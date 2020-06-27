mod sprite;
mod tile;

use sprite::Sprite;
use tile::{Tile, TILE_BYTES};
use crate::cpu::clock::ModeTypes;
use crate::utils::*;
use std::ops::Range;

// =============
// = Constants =
// =============
const MAP_SIZE: usize = 32; // In tiles
const MAP_PIXELS: usize = MAP_SIZE * TILESIZE; // In pixels
const VRAM_SIZE: usize = 0x8000;
const VRAM_OFFSET: usize = 0x8000;
const TILE_NUM: usize = 384;
const OAM_SPR_NUM: usize = 40;

// VRAM registers
const LCDC: usize                    = 0xFF40 - VRAM_OFFSET;
const STAT: usize                    = 0xFF41 - VRAM_OFFSET;
const SCY: usize                     = 0xFF42 - VRAM_OFFSET;
const SCX: usize                     = 0xFF43 - VRAM_OFFSET;
const LY: usize                      = 0xFF44 - VRAM_OFFSET;
const LYC: usize                     = 0xFF45 - VRAM_OFFSET;
// 0xFF46 is DMA transfer, handled by Bus
const BGP: usize                     = 0xFF47 - VRAM_OFFSET;
const OBP0: usize                    = 0xFF48 - VRAM_OFFSET;
const OBP1: usize                    = 0xFF49 - VRAM_OFFSET;
const WY: usize                      = 0xFF4A - VRAM_OFFSET;
const WX: usize                      = 0xFF4B - VRAM_OFFSET;

// VRAM ranges
const DISPLAY_RAM_RANGE: Range<usize> = (0x8000 - VRAM_OFFSET)..(0xA000 - VRAM_OFFSET);
const OAM_MEM: u16                    = 0xFE00 - (VRAM_OFFSET as u16);
const OAM_MEM_END: u16                = 0xFE9F - (VRAM_OFFSET as u16); // Inclusive
const TILE_SET: u16                   = 0x8000 - (VRAM_OFFSET as u16);
const TILE_SET_END: u16               = 0x97FF - (VRAM_OFFSET as u16);

const TILE_MAP_0_RANGE: Range<usize> = (0x9800 - VRAM_OFFSET)..(0x9C00 - VRAM_OFFSET);
const TILE_MAP_1_RANGE: Range<usize> = (0x9C00 - VRAM_OFFSET)..(0xA000 - VRAM_OFFSET);

// Colors
const BLACK: [u8; COLOR_CHANNELS]            = [0,   0,   0,   255];
const LIGHT_GRAY: [u8; COLOR_CHANNELS]       = [148, 148, 165, 255];
const DARK_GRAY: [u8; COLOR_CHANNELS]        = [107, 107, 90,  255];
const WHITE: [u8; COLOR_CHANNELS]            = [255, 255, 255, 255];

const COLORS: [[u8; COLOR_CHANNELS]; 4] = [
    WHITE,
    LIGHT_GRAY,
    DARK_GRAY,
    BLACK,
];

pub struct PPU {
    vram: [u8; VRAM_SIZE],
    map_buffer: [u8; SCREEN_HEIGHT * SCREEN_WIDTH],
    tiles: [Tile; TILE_NUM],
    oam: [Sprite; OAM_SPR_NUM],
}

impl PPU {
    // ==================
    // = Public methods =
    // ==================
    pub fn new() -> PPU {
        PPU {
            vram: [0; VRAM_SIZE],
            map_buffer: [0; SCREEN_HEIGHT * SCREEN_WIDTH],
            tiles: [Tile::new(); TILE_NUM],
            oam: [Sprite::new(); OAM_SPR_NUM],
        }
    }

    /// ```
    /// Write VRAM
    ///
    /// Write value to specified address in VRAM
    ///
    /// Can't access OAM memory during OAM Interrupt
    /// Can't access OAM or VRAM during LCD transfer
    ///
    /// Input:
    ///     Address to write to (u16)
    ///     Value to write (u8)
    /// ```
    pub fn write_vram(&mut self, raw_addr: u16, val: u8) {
        let addr = raw_addr - VRAM_OFFSET as u16;

        if self.is_valid_status(raw_addr) {
            // Update OAM objects if needed
            if is_in_oam(addr) {
                let relative_addr = addr - OAM_MEM;
                let spr_num = relative_addr / 4;
                let byte_num = relative_addr % 4;
                self.oam[spr_num as usize].update_byte(byte_num, val);
            } else if is_in_tile_set(addr) {
                let offset = addr - TILE_SET;
                let tile_num = offset / TILE_BYTES;
                let byte_num = offset % TILE_BYTES;
                self.tiles[tile_num as usize].update_byte(byte_num, val);
            }

            self.vram[addr as usize] = val;
        }
    }

    /// ```
    /// Read VRAM
    ///
    /// Read value from given address in VRAM
    ///
    /// Input:
    ///     Address to read from (u16)
    ///
    /// Output:
    ///     Value at given address (u8)
    /// ```
    pub fn read_vram(&self, raw_addr: u16) -> u8 {
        let addr = raw_addr - VRAM_OFFSET as u16;
        self.vram[addr as usize]
    }

    /// ```
    /// Set LY register
    ///
    /// Sets the value at the LY RAM address
    ///
    /// Input:
    ///     Value to write (u8)
    ///
    /// Output:
    ///     Whether values in LY and LYC registers are equal
    /// ```
    pub fn set_ly(&mut self, line: u8) -> bool {
        self.vram[LY] = line;

        if self.vram[LY] == self.vram[LYC] {
            // If LY and LYC are equal:
            // - Set coincidence bit in STAT register
            // - Trigger LCDC status interrupt if enabled
            self.vram[STAT].set_bit(2);
            self.vram[STAT].get_bit(6)
        } else {
            false
        }
    }

    /// ```
    /// Render scanline
    ///
    /// Renders specified scanline to buffer
    /// ```
    pub fn render_scanline(&mut self) {
        let line = self.vram[LY];

        // Render current scanline
        let mut pixel_row = [0; SCREEN_WIDTH];

        // Limit scope to appease the Borrow Checker Gods
        {
            let tile_map = self.get_bkgd_tile_map();
            let palette = self.get_bkgd_palette();
            let screen_coords = self.get_scroll_coords();

            // Get the row of tiles containing our scanline
            let y = (screen_coords.y + line) as usize;
            let row = ((screen_coords.y + line) as usize) % TILESIZE;
            let start_x = screen_coords.x as usize;
            for x in 0..SCREEN_WIDTH {
                // Get coords for current tile
                let map_x = ((start_x + x) % MAP_PIXELS) / TILESIZE;
                let map_y = y / TILESIZE;
                let index = map_y * MAP_SIZE + map_x;
                // The tile indexes in the second tile pattern table ($8800-97ff) are signed
                let tile_index = if self.get_bkgd_wndw_tile_set_index() == 0 {
                    (256 + (tile_map[index] as i8 as isize)) as usize
                } else {
                    tile_map[index] as usize
                };
                let tile = &self.tiles[tile_index];
                let col = (start_x + x) % TILESIZE;
                let pixel = tile.get_row(row)[col];
                let corrected_pixel = palette[pixel as usize];
                pixel_row[x] = corrected_pixel;
            }
        }
        let start_index = line as usize * SCREEN_WIDTH;
        let end_index = (line + 1) as usize * SCREEN_WIDTH;
        self.map_buffer[start_index..end_index].copy_from_slice(&pixel_row);
    }

    /// ```
    /// Set status
    ///
    /// Sets the current value of the status register ($FF41)
    ///
    /// Input:
    ///     Current clock mode (u8)
    /// ```
    pub fn set_status(&mut self, mode: u8) {
        self.vram[STAT] &= 0b1111_1100;
        self.vram[STAT] |= mode;
    }

    /// ```
    /// Render screen
    ///
    /// Renders the current screen
    ///
    /// Output:
    ///     Array of pixels to draw ([u8])
    /// ```
    pub fn render_screen(&self) -> [u8; DISP_SIZE] {
        let mut map_array = [0; SCREEN_HEIGHT * SCREEN_WIDTH];

        if self.is_bkgd_dspl() {
            map_array.copy_from_slice(&self.map_buffer);
        }

        if self.is_wndw_dspl() {
            self.render_window(&mut map_array);
        }

        if self.is_sprt_dspl() {
            self.render_sprites(&mut map_array);
        }

        let screen = self.get_color(&map_array);

        screen
    }

    // ===================
    // = Private methods =
    // ===================

    /// ```
    /// Render window
    ///
    /// Renders the window tiles onto the pixel array
    ///
    /// Input:
    ///     Array of pixels to modify (&[u8])
    /// ```
    fn render_window(&self, pixel_array: &mut [u8]) {
        let wndw_coords = self.get_wndw_coords();
        let wndw_map = self.get_wndw_tile_map();
        let palette = self.get_bkgd_palette();

        let origin_x = wndw_coords.x as usize;
        let origin_y = wndw_coords.y as usize;

        // Iterate thru visible pixels in window
        for y in origin_y..SCREEN_HEIGHT {
            for x in origin_x..SCREEN_WIDTH {
                let tile_x = (x - origin_x) / TILESIZE;
                let tile_y = (y - origin_y) / TILESIZE;
                let index = tile_y * MAP_SIZE + tile_x;

                // The tile indexes in the second tile pattern table ($8800-97ff) are signed
                let tile_index = if self.get_bkgd_wndw_tile_set_index() == 0 {
                    (256 + (wndw_map[index] as i8 as isize)) as usize
                } else {
                    wndw_map[index] as usize
                };
                let tile = &self.tiles[tile_index];
                let col = (x - origin_x) % TILESIZE;
                let row = (y - origin_y) % TILESIZE;
                let pixel = tile.get_row(row)[col];

                let index = y * SCREEN_WIDTH + x;
                let corrected_pixel = palette[pixel as usize];
                pixel_array[index] = corrected_pixel;
            }
        }
    }

    /// ```
    /// Render sprites
    ///
    /// Renders the sprites onto the graphics array
    ///
    /// Input:
    ///     [u8] - Graphics array to render upon
    /// ```
    fn render_sprites(&self, pixel_array: &mut [u8]) {
        // Iterate through every sprite
        for i in 0..OAM_SPR_NUM {
            let spr = self.oam[i];
            if !spr.is_onscreen() {
                continue;
            }

            // If sprites are 8x8, just draw single tile
            // If sprites are 8x16, need to draw tile and adjacent tile below it
            // If sprites are 8x16 and Y-flipped, need to draw bottom tile on top
            let num_spr = if self.spr_are_8x16() { 2 } else { 1 };

            for i in 0..num_spr {
                let top_coords = spr.get_coords();
                let spr_coords = Point::new(top_coords.x, top_coords.y + (TILESIZE as u8 * i));
                let spr_offset = if spr.is_y_flip() { num_spr - i - 1 } else { i };
                let spr_num = spr.get_tile_num() + spr_offset;
                let tile = &self.tiles[spr_num as usize];
                self.draw_spr(pixel_array, tile, spr, spr_coords);
            }
        }
    }

    /// ```
    /// Draw sprite
    ///
    /// Draw sprite to screen
    ///
    /// Inputs:
    ///     Graphics array to render upon ([u8])
    ///     Tile to render (Tile)
    ///     Sprite metadata (Sprite)
    ///     Screen coordinates to draw to (Point)
    /// ```
    fn draw_spr(&self, pixel_array: &mut [u8], tile: &Tile, spr: Sprite, spr_coords: Point) {
        // TODO: Needs to handle sprite priority
        let palette = self.get_spr_palette(spr.is_pal_0());
        let flip_x = spr.is_x_flip();
        let flip_y = spr.is_y_flip();
        let above_bg = spr.is_above_bkgd();

        let spr_x = spr_coords.x as usize;
        let spr_y = spr_coords.y as usize;

        'draw_row: for row in 0..TILESIZE {
            let pixels = if flip_y {
                tile.get_row(TILESIZE - row - 1)
            } else {
                tile.get_row(row)
            };

            // Iterate through each pixel in row, applying the palette
            'draw_col: for col in 0..TILESIZE {
                let pixel = pixels[col as usize];
                let x_offset = if flip_x {
                    TILESIZE - col - 1
                } else {
                    col
                };

                let pixel_x = spr_x + x_offset;
                let pixel_y = spr_y + row;
                // Stop if pixel is going to be drawn off-screen
                if pixel_x >= SCREEN_WIDTH {
                    continue 'draw_col;
                } else if pixel_y >= SCREEN_HEIGHT {
                    continue 'draw_row;
                }

                let pixel_index = pixel_x + SCREEN_WIDTH * pixel_y;
                let corrected_pixel = palette[pixel as usize];

                // Only draw pixel if
                // - Sprite is above background, and the pixel being drawn isn't transparent
                // - Sprite is below background, and background has transparent color here
                let should_draw = (above_bg && pixel != 0) || (!above_bg && pixel_array[pixel_index] == 0);
                if should_draw {
                    pixel_array[pixel_index] = corrected_pixel;
                }
            }
        }
    }

    /// ```
    /// Get color
    ///
    /// Gets the pixel values for the pixels currently on screen
    ///
    /// Input:
    ///     160x144 screen pixel array (&[u8])
    ///
    /// Output:
    ///     RGB values for on-screen pixels ([u8])
    /// ```
    fn get_color(&self, pixel_array: &[u8]) -> [u8; DISP_SIZE] {
        let mut rgb_screen = [0; DISP_SIZE];
        // Iterate through every visible pixel
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let index = y * SCREEN_WIDTH + x;
                let pixel = pixel_array[index];

                let view_index = index * COLOR_CHANNELS;
                let color = COLORS[pixel as usize];
                for i in 0..color.len() {
                    rgb_screen[view_index + i] = color[i];
                }
            }
        }

        rgb_screen
    }

    /// ```
    /// Get background tile map
    ///
    /// Gets the pixel data for the background tiles
    ///
    /// Output:
    ///     Slice of tilemap values (&[u8])
    /// ```
    fn get_bkgd_tile_map(&self) -> &[u8] {
        // $00 for $9800-$9BFF
        // $01 for $9C00-$9FFF
        let tile_map = if self.get_bkgd_tile_map_index() == 0 {
            &self.vram[TILE_MAP_0_RANGE]
        } else {
            &self.vram[TILE_MAP_1_RANGE]
        };

        tile_map
    }

    /// ```
    /// Get window tile map
    ///
    /// Gets the pixel data for the window tiles
    ///
    /// Output:
    ///     Slice of tilemap values (&[u8])
    /// ```
    fn get_wndw_tile_map(&self) -> &[u8] {
        // $00 for $9800-$9BFF
        // $01 for $9C00-$9FFF
        let wndw_map = if self.get_wndw_tile_map_index() == 0 {
            &self.vram[TILE_MAP_0_RANGE]
        } else {
            &self.vram[TILE_MAP_1_RANGE]
        };

        wndw_map
    }

    /// ```
    /// Get background palette
    ///
    /// Gets the palette indices from the BGP register ($FF47)
    ///
    /// Output:
    ///     Palette indices ([u8])
    /// ```
    fn get_bkgd_palette(&self) -> [u8; 4] {
        unpack_u8(self.vram[BGP])
    }

    /// ```
    /// Get sprite palette
    ///
    /// Gets the palette indices for the sprites
    ///
    /// Input:
    ///     Whether to use palette 0 or 1 (bool)
    ///
    /// Output:
    ///     Palette indices ([u8])
    /// ```
    fn get_spr_palette(&self, pal_0: bool) -> [u8; 4] {
        let pal = if pal_0 {
            unpack_u8(self.vram[OBP0])
        } else {
            unpack_u8(self.vram[OBP1])
        };

        pal
    }

    /// ```
    /// Is background displayed
    ///
    /// Is background layer currently visible
    ///
    /// Output:
    ///     Whether or not background is displayed (bool)
    /// ```
    fn is_bkgd_dspl(&self) -> bool {
        let lcd_control = self.vram[LCDC];
        lcd_control.get_bit(0)
    }

    /// ```
    /// Is window displayed
    ///
    /// Is the window layer currently visible
    ///
    /// Output:
    ///     Whether window layer is visible (bool)
    /// ```
    fn is_wndw_dspl(&self) -> bool {
        let lcd_control = self.vram[LCDC];
        lcd_control.get_bit(5)
    }

    /// ```
    /// Are sprites displayed
    ///
    /// Is the sprite layer visible
    ///
    /// Output:
    ///     Whether the sprite layer is visible (bool)
    /// ```
    fn is_sprt_dspl(&self) -> bool {
        let lcd_control = self.vram[LCDC];
        lcd_control.get_bit(1)
    }

    /// ```
    /// Get background tileset index
    ///
    /// Returns which tileset is being used (0/1)
    ///
    /// Output:
    ///     Tileset index (u8)
    /// ```
    fn get_bkgd_wndw_tile_set_index(&self) -> u8 {
        let lcd_control = self.vram[LCDC];
        if lcd_control.get_bit(4) { return 1 } else { return 0 }
    }

    /// ```
    /// Get background tilemap index
    ///
    /// Returns which tilemap set is being used (0/1)
    ///
    /// Output:
    ///     Tilemap index (u8)
    /// ```
    fn get_bkgd_tile_map_index(&self) -> u8 {
        let lcd_control = self.vram[LCDC];
        if lcd_control.get_bit(3) { return 1 } else { return 0 }
    }

    /// ```
    /// Get window tilemap index
    ///
    /// Returns which window tilemap set is being used (0/1)
    ///
    /// Output:
    ///     Tilemap index (u8)
    /// ```
    fn get_wndw_tile_map_index(&self) -> u8 {
        let lcd_control = self.vram[LCDC];
        if lcd_control.get_bit(6) { return 1 } else { return 0 }
    }

    /// ```
    /// Are sprites 8x16?
    ///
    /// Returns true if sprites are to be drawn 8x16
    ///
    /// Output:
    ///     Whether spries are 8x16 (vs 8x8) (bool)
    /// ```
    fn spr_are_8x16(&self) -> bool {
        self.vram[LCDC].get_bit(2)
    }

    /// ```
    /// Get scroll coords
    ///
    /// Returns the values of the SCX and SCY registers
    ///
    /// Output:
    ///     SCX, SCY point (Point)
    /// ```
    fn get_scroll_coords(&self) -> Point {
        let scroll_x = self.vram[SCX];
        let scroll_y = self.vram[SCY];

        Point::new(scroll_x, scroll_y)
    }

    /// ```
    /// Get window coords
    ///
    /// Returns the window position from the WX and WY registers
    ///
    /// Output:
    ///     Location of the window (Point)
    fn get_wndw_coords(&self) -> Point {
        let wndw_x = self.vram[WX].saturating_sub(7);
        let wndw_y = self.vram[WY];

        Point::new(wndw_x, wndw_y)
    }

    /// ```
    /// Get LCDC Status
    ///
    /// Get the current clock mode from the LCD status register
    ///
    /// Output:
    ///     Current clock mode (ModeTypes)
    /// ```
    fn get_lcdc_status(&self) -> ModeTypes {
        let lcd_stat = self.vram[STAT];
        let mode = lcd_stat & 0b0000_0011;
        match mode {
            0 => { ModeTypes::HBLANK },
            1 => { ModeTypes::VBLANK },
            2 => { ModeTypes::OAMReadMode },
            3 => { ModeTypes::VRAMReadMode },
            _ => { panic!("Invalid mode") }
        }
    }

    /// ```
    /// Is valid status
    ///
    /// Can we write to the given address, given the clock mode?
    ///
    /// Input:
    ///     Address to write to (u16)
    ///
    /// Output:
    ///     Write status (bool)
    /// ```
    fn is_valid_status(&self, addr: u16) -> bool {
        let lcdc_status = self.get_lcdc_status();

        match lcdc_status {
            ModeTypes::OAMReadMode => {
                !is_in_oam(addr)
            },
            ModeTypes::VRAMReadMode => {
                !is_in_oam(addr) && !DISPLAY_RAM_RANGE.contains(&(addr as usize))
            },
            _ => {
                true
            }
        }
    }

}

/// ```
/// Is in OAM?
///
/// Helper function to determine if address being written to is in OAM memory
///
/// Inputs:
///     Address to write to (u16)
///
/// Outputs:
///     Whether the address is in OAM memory (bool)
/// ```
fn is_in_oam(addr: u16) -> bool {
    addr >= OAM_MEM && addr <= OAM_MEM_END
}

fn is_in_tile_set(addr: u16) -> bool {
    addr >= TILE_SET && addr <= TILE_SET_END
}
