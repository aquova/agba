use crate::utils::{GB, ModifyBits, TILESIZE};

/*
 * Object Attribute Memory (OAM) Layout
 *
 * Byte | Info
 * -----+------------------------------
 * 0    | Y-coord (stored y-coord - 16)
 * 1    | X-coord (stored x-coord - 8)
 * 2    | Data tile number
 * -----+------------------------------
 * 3    | Bit | Info
 * -----+-----+------------------------
 * 3    | 7   | Drawn above background when reset
 * 3    | 6   | Vertical flip when set
 * 3    | 5   | Horizontal flip when set
 * 3    | 4   | OBJ Palette 0/1
 * 3    | 3   | Tile VRAM Bank (CGB only)
 * 3    | 2-0 | Palette number (CGB only)
 *
**/

const X_OFFSET: i16 = 8;
const Y_OFFSET: i16 = 16;
const X_OFFSCREEN: i16 = 168;
const Y_OFFSCREEN: i16 = 160;

const Y_POS_BYTE: u16 = 0;
const X_POS_BYTE: u16 = 1;
const TILE_NUM_BYTE: u16 = 2;
const FLAG_BYTE: u16 = 3;
pub const OAM_BYTE_SIZE: u16 = 4;

const PAL_NUM_BIT: u8 = 4;
const X_FLIP_BIT: u8 = 5;
const Y_FLIP_BIT: u8 = 6;
const BG_PRIORITY_BIT: u8 = 7;

#[derive(Copy, Clone)]
pub struct Sprite {
    data: [u8; OAM_BYTE_SIZE as usize],
    tile_num: u8,
    x: i16,
    y: i16,
    above_bkgd: bool,
    x_flip: bool,
    y_flip: bool,
    palette: u8,
    vram_bank: u8,
}

impl Sprite {
    pub fn new() -> Sprite {
        Sprite {
            data: [0; OAM_BYTE_SIZE as usize],
            tile_num: 0,
            x: 0,
            y: 0,
            above_bkgd: true,
            x_flip: false,
            y_flip: false,
            palette: 0,
            vram_bank: 0,
        }
    }

    /// ```
    /// Update byte
    ///
    /// Updates metadata for this sprite
    ///
    /// Inputs:
    ///     Which metadata byte to edit (u16)
    ///     New byte value (u8)
    ///     Game Boy mode (GB)
    /// ```
    pub fn set_byte(&mut self, index: u16, byte: u8, mode: GB) {
        match index {
            Y_POS_BYTE =>    { self.parse_oam_byte1(byte); },
            X_POS_BYTE =>    { self.parse_oam_byte2(byte); },
            TILE_NUM_BYTE => { self.parse_oam_byte3(byte); },
            FLAG_BYTE =>     { self.parse_oam_byte4(byte, mode); },
            _ => { panic!("Byte offset can only be from 0-3"); }
        }

        self.data[index as usize] = byte;
    }

    /// ```
    /// Get byte
    ///
    /// Gets a byte from sprite metadata
    ///
    /// Input:
    ///     Which metadata byte to get (u16)
    ///
    /// Output:
    ///     Byte at that index (u8)
    /// ```
    pub fn get_byte(&self, index: u16) -> u8 {
        if index >= OAM_BYTE_SIZE {
            panic!("Byte offset can only be from 0-3");
        }

        self.data[index as usize]
    }

    /// ```
    /// Is onscreen
    ///
    /// Is sprite onscreen?
    ///
    /// Output:
    ///     Is sprite onscreen (bool)
    /// ```
    pub fn is_onscreen(&self) -> bool {
        let x_visible = self.x > -X_OFFSET && self.x < X_OFFSCREEN;
        let y_visible = self.y > -Y_OFFSET && self.y < Y_OFFSCREEN;

        x_visible || y_visible
    }

    /// ```
    /// Contains scanline?
    ///
    /// Whether the specified scanline passes thru this sprite
    ///
    /// Inputs:
    ///     Scanline in question (u8)
    ///     Are we in 8x16 sprite mode? (bool)
    ///
    /// Output:
    ///     Whether scanline passes thru this sprite (bool)
    /// ```
    pub fn contains_scanline(&self, scanline: u8, is_8x16: bool) -> bool {
        let line = scanline as i16;
        let spr_height = (if is_8x16 { 2 * TILESIZE } else { TILESIZE }) as i16;
        (self.y <= line) && (line < (self.y + spr_height))
    }

    /// ```
    /// Get tile num
    ///
    /// Gets associated tile number for this sprite
    ///
    /// Output:
    ///     Tile num (u8)
    /// ```
    pub fn get_tile_num(&self) -> u8 {
        self.tile_num
    }

    /// ```
    /// Get coords
    ///
    /// Get screen coordinates for this sprite
    ///
    /// Output:
    ///     Sprite coordinates ((i16, i16))
    /// ```
    pub fn get_coords(&self) -> (i16, i16) {
        (self.x, self.y)
    }

    /// ```
    /// Get palette
    ///
    /// Returns which palette index this sprite is using
    ///
    /// Output:
    ///     Which palette the sprite is using (u8)
    /// ```
    pub fn get_pal(&self) -> u8 {
        self.palette
    }

    /// ```
    /// Get VRAM bank
    ///
    /// Returns which VRAM bank the sprite is located in (CGB only)
    ///
    /// Output:
    ///     VRAM bank number (usize)
    /// ```
    pub fn get_vram_bank(&self) -> usize {
        self.vram_bank as usize
    }

    /// ```
    /// Is X flipped?
    ///
    /// Should the sprite be flipped in the X-direction?
    ///
    /// Output:
    ///     Whether sprite should be flipped
    /// ```
    pub fn is_x_flip(&self) -> bool {
        self.x_flip
    }

    /// ```
    /// Is Y flipped?
    ///
    /// Should the sprite be flipped in the Y-direction?
    ///
    /// Output:
    ///     Whether sprite should be flipped
    /// ```
    pub fn is_y_flip(&self) -> bool {
        self.y_flip
    }

    /// ```
    /// Is sprite above background?
    ///
    /// Should the sprite be draw above or below the background
    ///
    /// Output:
    ///     If sprite is drawn on top of background
    /// ```
    pub fn is_above_bkgd(&self) -> bool {
        self.above_bkgd
    }
}

impl Sprite {
    /// ```
    /// Parse OAM byte 1
    ///
    /// Parses byte corresponding to sprite Y-coordinate
    ///
    /// Input:
    ///     Value to parse (u8)
    /// ```
    fn parse_oam_byte1(&mut self, val: u8) {
        self.y = (val as i16) - Y_OFFSET;
    }

    /// ```
    /// Parse OAM byte 2
    ///
    /// Parses byte corresponding to sprite X-coordinate
    ///
    /// Input:
    ///     Value to parse (u8)
    /// ```
    fn parse_oam_byte2(&mut self, val: u8) {
        self.x = (val as i16) - X_OFFSET;
    }

    /// ```
    /// Parse OAM byte 3
    ///
    /// Parses byte corresponding to sprite tile number
    ///
    /// Input:
    ///     Value to parse (u8)
    /// ```
    fn parse_oam_byte3(&mut self, val: u8) {
        self.tile_num = val;
    }

    /// ```
    /// Parse OAM byte 4
    ///
    /// Parses byte corresponding to X/Y flip, palette choice, and draw priority
    ///
    /// Input:
    ///     Value to parse (u8)
    ///     Game Boy mode (GB)
    /// ```
    fn parse_oam_byte4(&mut self, val: u8, mode: GB) {
        self.above_bkgd = !val.get_bit(BG_PRIORITY_BIT);
        self.y_flip = val.get_bit(Y_FLIP_BIT);
        self.x_flip = val.get_bit(X_FLIP_BIT);
        if mode == GB::CGB {
            self.palette = val & 0b111;
            self.vram_bank = (val & 0b1000) >> 3;
        } else {
            self.palette = if val.get_bit(PAL_NUM_BIT) { 1 } else { 0 };
        }
    }
}
