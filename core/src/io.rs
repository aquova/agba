// Input/Output functions

pub enum Buttons {
    START,
    SELECT,
    B,
    A,
    DOWN,
    UP,
    LEFT,
    RIGHT
}

impl Buttons {
    // I really hate this.
    pub fn get_index(&self) -> usize {
        match self {
            Buttons::START =>  { 0 },
            Buttons::SELECT => { 1 },
            Buttons::B =>      { 2 },
            Buttons::A =>      { 3 },
            Buttons::DOWN =>   { 4 },
            Buttons::UP =>     { 5 },
            Buttons::LEFT =>   { 6 },
            Buttons::RIGHT =>  { 7 },
        }
    }
}

pub struct IO {
    btns: [bool; 8],
    get_btn_keys: bool,
    get_dir_keys: bool
}

impl IO {
    pub fn new() -> IO {
        IO {
            btns: [false; 8],
            get_btn_keys: false,
            get_dir_keys: false
        }
    }

    pub fn btn_pressed(&mut self, btn: Buttons) {
        let i = btn.get_index();
        self.btns[i] = true;
    }

    pub fn btn_released(&mut self, btn: Buttons) {
        let i = btn.get_index();
        self.btns[i] = false;
    }

    pub fn set_btns(&mut self, val: u8) {
        self.get_btn_keys = (val & 0b0010_0000) != 0;
        self.get_dir_keys = (val & 0b0001_0000) != 0;
    }

    pub fn read_btns(&self) -> u8 {
        // AFAIK, the system can't ask for both values
        if self.get_btn_keys {
            self.pack_btn_keys()
        } else if self.get_dir_keys {
            self.pack_dir_keys()
        } else {
            0
        }
    }

    // TODO: See if these functions can be merged
    fn pack_btn_keys(&self) -> u8 {
        let mut output = 0;
        for i in 0..4 {
            let pressed = self.btns[i];
            output |= (pressed as u8) << (4 - i);
        }

        output
    }

    fn pack_dir_keys(&self) -> u8 {
        let mut output = 0;
        for i in 4..8 {
            let pressed = self.btns[i];
            output |= (pressed as u8) << (8 - i);
        }

        output
    }
}
