mod mbc1;
mod mbc2;
mod mbc3;
mod mbc5;

use std::str::from_utf8;
use mbc1::{mbc1_read_byte, mbc1_write_byte};
use mbc2::{mbc2_read_byte, mbc2_write_byte};
use mbc3::{mbc3_read_byte, mbc3_write_byte};
use mbc5::{mbc5_read_byte, mbc5_write_byte};

const ROM_BANK_SIZE: usize = 0x4000;
const RAM_BANK_SIZE: usize = 0x2000;
const MAX_RAM_SIZE: usize = 32 * 1024; // 32 KiB
const MBC5_MAX_RAM_SIZE: usize = 128 * 1024; // 128 KiB

pub const ROM_START: u16        = 0x0000;
pub const ROM_STOP: u16         = 0x7FFF;

const RAM_ENABLE_START: u16     = ROM_START;
const RAM_ENABLE_STOP: u16      = 0x1FFF;
const ROM_BANK_NUM_START: u16   = RAM_ENABLE_STOP + 1;
const ROM_BANK_NUM_STOP: u16    = 0x3FFF;
const RAM_BANK_NUM_START: u16   = ROM_BANK_NUM_STOP + 1;
const RAM_BANK_NUM_STOP: u16    = 0x5FFF;
const ROM_RAM_MODE_START: u16   = RAM_BANK_NUM_STOP + 1;
const ROM_RAM_MODE_STOP: u16    = 0x7FFF;
pub const EXT_RAM_START: u16    = 0xA000;
pub const EXT_RAM_STOP: u16     = 0xBFFF;

const TITLE_ADDR: usize = 0x0134;
const DMG_TITLE_ADDR_END: usize = 0x013F;
const CGB_FLAG_ADDR: usize = 0x0143;
const MBC_TYPE_ADDR: usize = 0x0147;

/*
 * ROM Header Layout
 * Header runs from $0100-$014F
 *
 * +-------------------------+ $100
 * |       Start Vector      |
 * +-------------------------+ $104
 * |      Nintendo Logo      |
 * +-------------------------+ $134
 * |       Game Title        |
 * +-------------------------+ $13F
 * | Manufacturer Code (GBC) |
 * +-------------------------+ $143
 * |        GBC Flag         |
 * +-------------------------+ $144
 * |    New Licensee Code    |
 * +-------------------------+ $146
 * |        SGB Flag         |
 * +-------------------------+ $147
 * |     Cartridge Type      |
 * +-------------------------+ $148
 * |        ROM Size         |
 * +-------------------------+ $149
 * |        RAM Size         |
 * +-------------------------+ $14A
 * |     Destination Code    |
 * +-------------------------+ $14B
 * |    Old Licensee Code    |
 * +-------------------------+ $14C
 * |      ROM Version        |
 * +-------------------------+ $14D
 * |    Header Checksum      |
 * +-------------------------+ $14E
 * |    Global Checksum      |
 * +-------------------------+ $14F
 *
 */

#[derive(Copy, Clone, PartialEq)]
pub enum MBC {
    NONE,
    MBC1,
    MBC2,
    MBC3,
    HuC1,
    MBC5
}

pub struct Cart {
    mbc: MBC,
    rom_bank: u16,
    ram_bank: u8,
    rom: Vec<u8>,
    ram: Vec<u8>,
    ext_ram_enable: bool,
    rom_mode: bool,
    cgb: bool,
}

// ==================
// = Public Methods =
// ==================
impl Cart {
    pub fn new() -> Cart {
        Cart {
            mbc: MBC::NONE,
            rom_bank: 1,
            ram_bank: 0,
            rom: Vec::new(),
            ram: Vec::new(),
            ext_ram_enable: false,
            rom_mode: true,
            cgb: false,
        }
    }

    /// ```
    /// Get external RAM
    ///
    /// Returns a slice to the external RAM object, used for battery saving
    ///
    /// Output:
    ///     External RAM, as a slice (&[u8])
    /// ```
    pub fn get_ext_ram(&self) -> &[u8] {
        &self.ram
    }

    /// ```
    /// Load cartridge
    ///
    /// Loads the game from file into Cartridge object
    ///
    /// Input:
    ///     Array of game data
    /// ```
    pub fn load_cart(&mut self, rom: &[u8]) {
        for i in 0..rom.len() {
            self.rom.push(rom[i]);
        }
        self.set_mbc();
        self.set_cgb();
        self.init_ext_ram();
    }

    /// ```
    /// Read from cart
    ///
    /// Returns the byte at the specified address in the ROM
    ///
    /// Input:
    ///     Address in ROM (u16)
    ///
    /// Output:
    ///     Byte at specified address (u8)
    /// ```
    pub fn read_cart(&self, address: u16) -> u8 {
        if address < ROM_BANK_SIZE as u16 {
            // If in Bank 0, simply read value
            self.rom[address as usize]
        } else if address < ROM_STOP {
            // If in other rom bank, need to obey bank switching
            // NOTE: MBC2 only goes up to 16 banks
            let rel_address = (address as usize) - ROM_BANK_SIZE;
            let bank_address = (self.rom_bank as usize) * ROM_BANK_SIZE + rel_address;
            self.rom[bank_address as usize]
        } else {
            match self.mbc {
                // TODO: What to return if no MBC or ext. RAM disabled?
                MBC::MBC1 => { mbc1_read_byte(self, address) },
                MBC::MBC2 => { mbc2_read_byte(self, address) },
                MBC::MBC3 => { mbc3_read_byte(self, address) },
                MBC::MBC5 => { mbc5_read_byte(self, address) },
                _ => { 0 }
            }
        }
    }

    /// ```
    /// Write to cart
    ///
    /// Writes value to ROM ($0000-$7FFF) or external RAM ($A000-$BFFF) area of memory
    ///
    /// Inputs:
    ///     Address to write to (u16)
    ///     Value to write (u8)
    ///
    /// Output:
    ///     Whether data was written to battery saved-memory (bool)
    /// ```
    pub fn write_cart(&mut self, addr: u16, val: u8) -> bool {
        match self.mbc {
            MBC::MBC1 => { mbc1_write_byte(self, addr, val) },
            MBC::MBC2 => { mbc2_write_byte(self, addr, val) },
            MBC::MBC3 => { mbc3_write_byte(self, addr, val) },
            MBC::MBC5 => { mbc5_write_byte(self, addr, val) },
            _ => { false }
        }
    }

    /// ```
    /// Get Game Title
    ///
    /// Returns the title of the game
    ///
    /// Output:
    ///     Title of the game, from ROM (&str)
    /// ```
    pub fn get_title(&self) -> &str {
        let data = if self.cgb {
            &self.rom[TITLE_ADDR..DMG_TITLE_ADDR_END]
        } else {
            &self.rom[TITLE_ADDR..CGB_FLAG_ADDR]
        };
        from_utf8(data).unwrap()
    }
}

// ===================
// = Private Methods =
// ===================
impl Cart {
    /// ```
    /// Get MBC type
    ///
    /// Gets the Memory Bank Controller type for this game
    /// ```
    fn set_mbc(&mut self) {
        let val = self.rom[MBC_TYPE_ADDR];
        let mbc = match val {
            0x00 =>        { MBC::NONE },
            0x01..=0x03 => { MBC::MBC1 },
            0x05..=0x06 => { MBC::MBC2 },
            0x0F..=0x13 => { MBC::MBC3 },
            _ =>           { MBC::NONE }
        };

        self.mbc = mbc;
    }

    fn set_cgb(&mut self) {
        let val = self.rom[CGB_FLAG_ADDR];
        self.cgb = (val == 0x80) || (val == 0xC0);
    }

    /// ```
    /// Initialize external RAM
    ///
    /// Sets RAM vector to be the correct size
    /// ```
    fn init_ext_ram(&mut self) {
        // NOTE: This originally sized the RAM vector based on ROM header information
        // However, some ROMs *cough Blargg tests cough* don't report correctly
        // So now, simply assume we need maximum size
        if self.mbc == MBC::MBC5 {
            self.ram = vec![0; MBC5_MAX_RAM_SIZE];
        } else {
            self.ram = vec![0; MAX_RAM_SIZE];
        }
    }
}
