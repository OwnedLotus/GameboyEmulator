const SPD: f32 = 4.19;
const DSPD: f32 = 8.38;

pub mod cpu_architecture {
    use std::default;

    use super::SPD;

    #[derive(Debug, Default)]
    pub struct FrequencyMHz(f32);

    #[derive(Debug)]
    pub struct CPU {
        name: String,
        mode: ProcessorMode,
        register: Regitser,
        memory: [u8; 0xFFFF],
        instruction: u8,
    }

    #[derive(Debug)]
    enum ProcessorMode {
        Normal(FrequencyMHz),
        Dual(FrequencyMHz),
    }

    impl CPU {
        pub fn new() -> Self {
            todo!()
        }

        //https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html
        pub fn execute_instruction(&mut self, immed1: u8, immed2: u16) {
            match self.instruction {
                0x00 => (), // noop
                0x01 => {
                    // LD BC, d16
                    // load d16 half into b half into C
                    self.register.B = (immed2 >> 8) as u8;
                    self.register.C = (immed2 & 0xFF) as u8;
                }
                0x02 => todo!(),
                0x03 => todo!(),
                0x04 => todo!(),
                0x05 => todo!(),
                0x06 => todo!(),
                0x07 => todo!(),
                0x08 => todo!(),
                0x09 => todo!(),
                0x0A => todo!(),
                0x0B => todo!(),
                0x0C => todo!(),
                0x0D => todo!(),
                0x0E => todo!(),
                0x0F => todo!(),

                0x10 => todo!(), // STOP 0
                0x11 => {
                    self.register.D = (immed2 >> 8) as u8;
                    self.register.E = (immed2 & 0xFF) as u8;
                }
                0x12 => todo!(),
                0x13 => todo!(),
                0x14 => todo!(),
                0x15 => todo!(),
                0x16 => todo!(),
                0x17 => todo!(),
                0x18 => todo!(),
                0x19 => todo!(),
                0x1A => todo!(),
                0x1B => todo!(),
                0x1C => todo!(),
                0x1D => todo!(),
                0x1E => todo!(),
                0x1F => todo!(),

                0x20 => todo!(),
                0x21 => {
                    self.register.D = (immed2 >> 8) as u8;
                    self.register.E = (immed2 & 0xFF) as u8;
                }
                0x22 => todo!(),
                0x23 => todo!(),
                0x24 => todo!(),
                0x25 => todo!(),
                0x26 => todo!(),
                0x27 => todo!(),
                0x28 => todo!(),
                0x29 => todo!(),
                0x2A => todo!(),
                0x2B => todo!(),
                0x2C => todo!(),
                0x2D => todo!(),
                0x2E => todo!(),
                0x2F => todo!(),

                0x30 => todo!(),
                0x31 => {
                    self.register.SP = immed2;
                }
                0x32 => todo!(),
                0x33 => todo!(),
                0x34 => todo!(),
                0x35 => todo!(),
                0x36 => todo!(),
                0x37 => todo!(),
                0x38 => todo!(),
                0x39 => todo!(),
                0x3A => todo!(),
                0x3B => todo!(),
                0x3C => todo!(),
                0x3D => todo!(),
                0x3E => todo!(),
                0x3F => todo!(),

                0x40 => self.register.B = self.register.B,
                0x41 => self.register.B = self.register.C,
                0x42 => self.register.B = self.register.D,
                0x43 => self.register.B = self.register.E,
                0x44 => self.register.B = self.register.H,
                0x45 => self.register.B = self.register.L,
                0x46 => todo!(),
                0x47 => todo!(),
                0x48 => todo!(),
                0x49 => todo!(),
                0x4A => todo!(),
                0x4B => todo!(),
                0x4C => todo!(),
                0x4D => todo!(),
                0x4E => todo!(),
                0x4F => todo!(),

                0x50 => self.register.D = self.register.B,
                0x51 => self.register.D = self.register.C,
                0x52 => self.register.D = self.register.D,
                0x53 => self.register.D = self.register.E,
                0x54 => self.register.D = self.register.H,
                0x55 => self.register.D = self.register.L,
                0x56 => todo!(),
                0x57 => todo!(),
                0x58 => todo!(),
                0x59 => todo!(),
                0x5A => todo!(),
                0x5B => todo!(),
                0x5C => todo!(),
                0x5D => todo!(),
                0x5E => todo!(),
                0x5F => todo!(),

                0x60 => self.register.H = self.register.B,
                0x61 => self.register.H = self.register.C,
                0x62 => self.register.H = self.register.D,
                0x63 => self.register.H = self.register.E,
                0x64 => self.register.H = self.register.H,
                0x65 => self.register.H = self.register.L,
                0x66 => todo!(),
                0x67 => todo!(),
                0x68 => todo!(),
                0x69 => todo!(),
                0x6A => todo!(),
                0x6B => todo!(),
                0x6C => todo!(),
                0x6D => todo!(),
                0x6E => todo!(),
                0x6F => todo!(),

                0x70 => todo!(),
                0x71 => todo!(),
                0x72 => todo!(),
                0x73 => todo!(),
                0x74 => todo!(),
                0x75 => todo!(),
                0x76 => todo!(),
                0x77 => todo!(),
                0x78 => todo!(),
                0x79 => todo!(),
                0x7A => todo!(),
                0x7B => todo!(),
                0x7C => todo!(),
                0x7D => todo!(),
                0x7E => todo!(),
                0x7F => todo!(),

                0x80 => todo!(),
                0x81 => todo!(),
                0x82 => todo!(),
                0x83 => todo!(),
                0x84 => todo!(),
                0x85 => todo!(),
                0x86 => todo!(),
                0x87 => todo!(),
                0x88 => todo!(),
                0x89 => todo!(),
                0x8A => todo!(),
                0x8B => todo!(),
                0x8C => todo!(),
                0x8D => todo!(),
                0x8E => todo!(),
                0x8F => todo!(),

                0x90 => todo!(),
                0x91 => todo!(),
                0x92 => todo!(),
                0x93 => todo!(),
                0x94 => todo!(),
                0x95 => todo!(),
                0x96 => todo!(),
                0x97 => todo!(),
                0x98 => todo!(),
                0x99 => todo!(),
                0x9A => todo!(),
                0x9B => todo!(),
                0x9C => todo!(),
                0x9D => todo!(),
                0x9E => todo!(),
                0x9F => todo!(),

                0xA0 => todo!(),
                0xA1 => todo!(),
                0xA2 => todo!(),
                0xA3 => todo!(),
                0xA4 => todo!(),
                0xA5 => todo!(),
                0xA6 => todo!(),
                0xA7 => todo!(),
                0xA8 => todo!(),
                0xA9 => todo!(),
                0xAA => todo!(),
                0xAB => todo!(),
                0xAC => todo!(),
                0xAD => todo!(),
                0xAE => todo!(),
                0xAF => todo!(),

                0xB0 => todo!(),
                0xB1 => todo!(),
                0xB2 => todo!(),
                0xB3 => todo!(),
                0xB4 => todo!(),
                0xB5 => todo!(),
                0xB6 => todo!(),
                0xB7 => todo!(),
                0xB8 => todo!(),
                0xB9 => todo!(),
                0xBA => todo!(),
                0xBB => todo!(),
                0xBC => todo!(),
                0xBD => todo!(),
                0xBE => todo!(),
                0xBF => todo!(),

                0xC0 => todo!(),
                0xC1 => todo!(),
                0xC2 => todo!(),
                0xC3 => todo!(),
                0xC4 => todo!(),
                0xC5 => todo!(),
                0xC6 => todo!(),
                0xC7 => todo!(),
                0xC8 => todo!(),
                0xC9 => todo!(),
                0xCA => todo!(),
                0xCB => todo!(),
                0xCC => todo!(),
                0xCD => todo!(),
                0xCE => todo!(),
                0xCF => todo!(),

                0xD0 => todo!(),
                0xD1 => todo!(),
                0xD2 => todo!(),
                0xD3 => todo!(),
                0xD4 => todo!(),
                0xD5 => todo!(),
                0xD6 => todo!(),
                0xD7 => todo!(),
                0xD8 => todo!(),
                0xD9 => todo!(),
                0xDA => todo!(),
                0xDB => todo!(),
                0xDC => todo!(),
                0xDD => todo!(),
                0xDE => todo!(),
                0xDF => todo!(),

                0xE0 => todo!(),
                0xE1 => todo!(),
                0xE2 => todo!(),
                0xE3 => todo!(),
                0xE4 => todo!(),
                0xE5 => todo!(),
                0xE6 => todo!(),
                0xE7 => todo!(),
                0xE8 => todo!(),
                0xE9 => todo!(),
                0xEA => todo!(),
                0xEB => todo!(),
                0xEC => todo!(),
                0xED => todo!(),
                0xEE => todo!(),
                0xEF => todo!(),

                0xF0 => todo!(),
                0xF1 => todo!(),
                0xF2 => todo!(),
                0xF3 => todo!(),
                0xF4 => todo!(),
                0xF5 => todo!(),
                0xF6 => todo!(),
                0xF7 => todo!(),
                0xF8 => todo!(),
                0xF9 => todo!(),
                0xFA => todo!(),
                0xFB => todo!(),
                0xFC => todo!(),
                0xFD => todo!(),
                0xFE => todo!(),
                0xFF => todo!(),
            }
        }

        fn loadr8(&mut self, num1: u8, num2: u8) {
            todo!()
        }
        fn loadr16(&mut self, num: u16) {
            todo!()
        }
        fn loadaddr8(&mut self, num: u8) {
            todo!()
        }
        fn loadaddr16(&mut self, num: u16) {
            todo!()
        }
    }

    impl Default for ProcessorMode {
        fn default() -> Self {
            Self::Normal(FrequencyMHz(SPD))
        }
    }

    #[derive(Debug, Default, Clone, Copy)]
    struct Regitser {
        A: u8,
        B: u8,
        C: u8,
        D: u8,
        E: u8,
        F: u8,
        H: u8,
        L: u8,
        PC: u16,
        SP: u16,
    }
}
