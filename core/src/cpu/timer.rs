use crate::utils::ModifyBits;

pub const DIV: u16 = 0xFF04;  // Divider register
pub const TIMA: u16 = 0xFF05; // Counter register
pub const TMA: u16 = 0xFF06;  // Modulo register
pub const TAC: u16 = 0xFF07;  // Control register

const TAC_ENABLE_BIT: u8 = 3;
const TIMA_COOLDOWN_OVERFLOW: u8 = 4;

pub struct Timer {
    div: u16,   // $FF04
    tima: u8,   // $FF05
    tma: u8,    // $FF06
    tac: u8,    // $FF07
    tima_cooldown: u8,
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
            tima_cooldown: 0,
        }
    }

    fn get_tima_period(&self) -> u16 {
        match self.tac & 0b11 {
            0b00 => (1 << 9),
            0b01 => (1 << 3),
            0b10 => (1 << 5),
            0b11 => (1 << 7),
            _ => unreachable!()
        }
    }

    fn tima_tick(&self) -> bool {
        (self.div & self.get_tima_period()) != 0
    }

    // A good source on timer behavior here: https://hacktix.github.io/GBEDG/timers/
    pub fn tick(&mut self, m_cycles: u8) -> bool {
        let mut interrupt = false;
        let t_cycles = 4 * m_cycles;

        for _ in 0..t_cycles {
            let old_bit = self.tima_tick();
            self.div = self.div.wrapping_add(1);
            let new_bit = self.tima_tick();
            let enabled = self.tac.get_bit(TAC_ENABLE_BIT);

            if self.tima_cooldown != 0 {
                self.tima_cooldown -= 1;
                if self.tima_cooldown == 0 {
                    self.tima = self.tma;
                    interrupt = true;
                }
            } else if (old_bit && enabled) && !(new_bit && enabled) {
                let (new_tima, overflow) = self.tima.overflowing_add(1);
                self.tima = new_tima;
                if overflow {
                    self.tima = 0;
                    self.tima_cooldown = TIMA_COOLDOWN_OVERFLOW;
                }
            }
        }

        interrupt
    }

    pub fn read_timer(&self, addr: u16) -> u8 {
        match addr {
            DIV => (self.div >> 8) as u8,
            TIMA => self.tima,
            TMA => self.tma,
            TAC => self.tac,
            _ => { panic!("Trying to read a non-timer register") }
        }
    }

    pub fn write_timer(&mut self, addr: u16, val: u8) {
        match addr {
            DIV => { self.div = 0 },
            TIMA => {
                self.tima = val;
                self.tima_cooldown = 0;
            },
            TMA => { self.tma = val },
            TAC => { self.tac = val },
            _ => panic!("Trying to write to non-timer register")
        };
    }
}
