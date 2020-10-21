use crate::utils::COLOR_CHANNELS;

pub const DMG_PAL_SIZE: usize = 4;
pub const CGB_PAL_SIZE: usize = 32;

#[allow(non_camel_case_types)]
#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Palettes {
    GRAYSCALE,
    BROWN,
    BLUE,
    PASTEL,
    GREEN,
    RED,
    DARK_BLUE,
    ORANGE,
    DARK_GREEN,
    DARK_BROWN,
    YELLOW,
    INVERTED
}

pub struct Palette {
    sys_pal: Palettes,
}

impl Palette {
    pub fn new() -> Palette {
        Palette {
            sys_pal: Palettes::GRAYSCALE,
        }
    }

    /// ```
    /// Set system palette
    ///
    /// Set which palette the DMG should use
    ///
    /// Input:
    ///     Which palette to use (Palettes)
    /// ```
    pub fn set_sys_pal(&mut self, pal: Palettes) {
        self.sys_pal = pal;
    }

    /// ```
    /// Get BG palette
    ///
    /// Get which palette is being used for the background
    ///
    /// Output:
    ///     RGBA values for palette ([4x[RGBA u8 values]])
    /// ```
    pub fn get_bg_pal(&self) -> [[u8; COLOR_CHANNELS]; DMG_PAL_SIZE] {
        match self.sys_pal {
            Palettes::GRAYSCALE => {
                [[255, 255, 255, 255],
                 [128, 128, 128, 255],
                 [64,  64,  64,  255],
                 [0,   0,   0,   255]]
            },
            Palettes::BROWN => {
                [[255, 255, 255, 255],
                 [255, 173, 99,  255],
                 [131, 49,  0,   255],
                 [0,   0,   0,   255]]
            },
            Palettes::RED => {
                [[255, 255, 255, 255],
                 [255, 133, 132, 255],
                 [148, 58,  58,  255],
                 [0,   0,   0,   255]]
            },
            Palettes::DARK_BROWN => {
                [[255, 231, 197, 255],
                 [206, 156, 133, 255],
                 [132, 107, 41,  255],
                 [91,  49,  9,   255]]
            },
            Palettes::BLUE => {
                [[255, 255, 255, 255],
                 [101, 164, 155, 255],
                 [0,   0,   254, 255],
                 [0,   0,   0,   255]]
            },
            Palettes::DARK_BLUE => {
                [[255, 255, 255, 255],
                 [139, 140, 222, 255],
                 [83,  82,  140, 255],
                 [0,   0,   0,   255]]
            },
            Palettes::PASTEL => {
                [[255, 255, 165, 255],
                 [254, 148, 148, 255],
                 [147, 148, 254, 255],
                 [0,   0,   0,   255]]
            },
            Palettes::ORANGE => {
                [[255, 255, 255, 255],
                 [255, 255, 0,   255],
                 [254, 0,   0,   255],
                 [0,   0,   0,   255]]
            },
            Palettes::YELLOW => {
                [[255, 255, 255, 255],
                 [255, 255, 0,   255],
                 [125, 73,  0,   255],
                 [0,   0,   0,   255]]
            },
            Palettes::GREEN => {
                [[255, 255, 255, 255],
                 [81,  255, 0,   255],
                 [255, 66,  0,   255],
                 [0,   0,   0,   255]]
            },
            Palettes::DARK_GREEN => {
                [[255, 255, 255, 255],
                 [81,  255, 0,   255],
                 [1,   99,  198, 255],
                 [0,   0,   0,   255]]
            },
            Palettes::INVERTED => {
                [[0,   0,   0,   255],
                 [0,   132, 134, 255],
                 [255, 222, 0,   255],
                 [255, 255, 255, 255]]
            },
        }
    }

    /// ```
    /// Get sprite palette
    ///
    /// Get which palette is being used for the sprites
    ///
    /// Output:
    ///     RGBA values for palette ([4x[RGBA u8 values]])
    /// ```
    pub fn get_spr_pal(&self, pal: u8) -> [[u8; COLOR_CHANNELS]; DMG_PAL_SIZE] {
        match pal {
            0 => { self.get_obj0_pal() },
            1 => { self.get_obj1_pal() },
            _ => { unreachable!("DMG palette index cannot be greater than 1"); }
        }
    }

    /// ```
    /// Get the Object 0 palette
    ///
    /// Get which palette is being used for obj 0
    ///
    /// Output:
    ///     RGBA values for palette ([4x[RGBA u8 values]])
    /// ```
    fn get_obj0_pal(&self) -> [[u8; COLOR_CHANNELS]; DMG_PAL_SIZE] {
        match self.sys_pal {
            Palettes::GRAYSCALE => { self.get_bg_pal() },
            Palettes::BROWN =>     { self.get_bg_pal() },
            Palettes::PASTEL =>    { self.get_bg_pal() },
            Palettes::ORANGE =>    { self.get_bg_pal() },
            Palettes::GREEN =>     { self.get_bg_pal() },
            Palettes::INVERTED =>  { self.get_bg_pal() },
            Palettes::RED => {
                [[255, 255, 255, 255],
                 [123, 255, 48,  255],
                 [0,   131, 0,   255],
                 [0,   0,   0,   255]]
            },
            Palettes::DARK_BROWN => {
                [[255, 255, 255, 255],
                 [255, 173, 99,  255],
                 [131, 49,  0,   255],
                 [0,   0,   0,   255]]
            },
            Palettes::BLUE => {
                [[255, 255, 255, 255],
                 [255, 133, 132, 255],
                 [131, 49,  0,   255],
                 [0,   0,   0,   255]]
            },
            Palettes::DARK_BLUE => {
                [[255, 255, 255, 255],
                 [255, 133, 132, 255],
                 [148, 58,  58,  255],
                 [0,   0,   0,   255]]
            },
            Palettes::YELLOW => {
                [[255, 255, 255, 255],
                 [101, 164, 155, 255],
                 [0,   0,   254, 255],
                 [0,   0,   0,   255]]
            },
            Palettes::DARK_GREEN => {
                [[255, 255, 255, 255],
                 [255, 133, 132, 255],
                 [148, 58,  58,  255],
                 [0,   0,   0,   255]]
            },
        }
    }

    /// ```
    /// Get the Object 1 palette
    ///
    /// Get which palette is being used for obj 1
    ///
    /// Output:
    ///     RGBA values for palette ([4x[RGBA u8 values]])
    /// ```
    fn get_obj1_pal(&self) -> [[u8; COLOR_CHANNELS]; DMG_PAL_SIZE] {
        match self.sys_pal {
            Palettes::GRAYSCALE =>  { self.get_bg_pal() },
            Palettes::BROWN =>      { self.get_bg_pal() },
            Palettes::PASTEL =>     { self.get_bg_pal() },
            Palettes::ORANGE =>     { self.get_bg_pal() },
            Palettes::GREEN =>      { self.get_bg_pal() },
            Palettes::INVERTED =>   { self.get_bg_pal() },
            Palettes::DARK_BROWN => { self.get_obj0_pal() },
            Palettes::DARK_GREEN => { self.get_obj0_pal() },
            Palettes::RED => {
                [[255, 255, 255, 255],
                 [101, 164, 155, 255],
                 [0,   0,   254, 255],
                 [0,   0,   0,   255]]
            },
            Palettes::BLUE => {
                [[255, 255, 255, 255],
                 [123, 255, 48,  255],
                 [0,   131, 0,   255],
                 [0,   0,   0,   255]]
            },
            Palettes::DARK_BLUE => {
                [[255, 255, 255, 255],
                 [101, 164, 155, 255],
                 [0,   0,   254, 255],
                 [0,   0,   0,   255]]
            },
            Palettes::YELLOW => {
                [[255, 255, 255, 255],
                 [123, 255, 48,  255],
                 [0,   131, 0,   255],
                 [0,   0,   0,   255]]
            },
        }
    }
}

/// ```
/// GBC encoding to RGBA values
///
/// Helper function to convert 15-bit to 32-bit color
///
/// Input:
///     GBC 15-bit color value (u16)
///
/// Output:
///     Array of RGBA values ([u8; 4])
/// ```
pub fn gbc2rgba(gbc: u16) -> [u8; COLOR_CHANNELS] {
    let mut rgba = [0xFF; COLOR_CHANNELS];
    rgba[0] = five_bit_to_eight_bit((gbc & 0x1F) as u8);
    rgba[1] = five_bit_to_eight_bit(((gbc & 0b00000_11111_0000) >> 5) as u8);
    rgba[2] = five_bit_to_eight_bit(((gbc & 0b11111_00000_00000) >> 10) as u8);

    rgba
}

/// ```
/// 5-bit to 8-bit
///
/// Helper function to convert a single 5-bit color channel value to 8-bits
///
/// Input:
///     5-bit color channel value (u8)
///
/// Output:
///     8-bit color channel value (u8)
/// ```
fn five_bit_to_eight_bit(five_bit: u8) -> u8 {
    (five_bit / 0x1F) * 0xFF as u8
}
