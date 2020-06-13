use crate::utils::ModifyBits;

pub const DIV_REG: u16 = 0xFF04; // Divider register
pub const CNT_REG: u16 = 0xFF05; // Counter register
pub const MOD_REG: u16 = 0xFF06; // Modulo register
pub const CON_REG: u16 = 0xFF07; // Control register

const TIMER_SPEED_IN_CYCLES: u8 = 16;
const COUNT_SPEED_IN_CYCLES: [u8; 4] = [64, 1, 4, 16];

pub struct Timer {
    running: bool,
    div_cycles: u8,
    cnt_cycles: u8,
    cnt_index: usize,
    div_reg: u8, // $FF04
    cnt_reg: u8, // $FF05
    mod_reg: u8, // $FF06
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            running: false,
            div_cycles: 0,
            cnt_cycles: 0,
            cnt_index: 0,
            div_reg: 0,
            cnt_reg: 0,
            mod_reg: 0,
        }
    }

    pub fn tick(&mut self) -> bool {
        let mut interrupt = false;

        // Timer clock runs slower than CPU clock
        // So timer registers only increment on set multiple of clock cycles
        if self.running {
            self.div_cycles += 1;
            self.cnt_cycles += 1;

            if self.div_cycles == TIMER_SPEED_IN_CYCLES {
                self.div_reg = self.div_reg.wrapping_add(1);
                self.div_cycles = 0;
            }

            if self.cnt_cycles == COUNT_SPEED_IN_CYCLES[self.cnt_index] {
                let overflow = self.cnt_reg.checked_add(1);
                // If overflow, set Timer counter to Timer Modulo value
                if overflow.is_none() {
                    self.cnt_reg = self.mod_reg;
                    interrupt = true;
                    self.cnt_cycles = 0;
                } else {
                    self.cnt_reg += 1;
                }
            }
        }

        interrupt
    }

    pub fn read_timer(&self, addr: u16) -> u8 {
        let val = match addr {
            DIV_REG => { self.div_reg },
            CNT_REG => { self.cnt_reg },
            MOD_REG => { self.mod_reg },
            CON_REG => {
                let running_val = if self.running { 0b100 } else { 0 };
                let output = running_val | (self.cnt_index as u8);
                output
            },
            _ => { panic!("Trying to read a non-timer register") }
        };

        val
    }

    pub fn write_timer(&mut self, addr: u16, val: u8) {
        match addr {
            DIV_REG => { self.div_reg = 0 },
            CNT_REG => { self.cnt_reg = 0 },
            MOD_REG => { self.mod_reg = val },
            CON_REG => {
                self.running = val.get_bit(2);

                let clock_spd = val & 0x3;
                self.cnt_index = clock_spd as usize;
            },
            _ => {
                panic!("Trying to write to non-timer register")
            }
        };
    }
}
