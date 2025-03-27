const SPD: f32 = 4.19;
const DSPD: f32 = 8.38;

pub mod cpu_architecture {
    use std::default;

    use super::SPD;

    #[derive(Debug, Default)]
    pub struct FrequencyMHz(f32);

    #[derive(Debug, Default)]
    pub struct CPU {
        name: String,
        mode: ProcessorMode,
        data_bus: u8,
        address_bus: u16,
        ram: [u8; 16], // 8 * 16 == 128 ram size is 127
    }

    #[derive(Debug)]
    enum ProcessorMode {
        Normal(FrequencyMHz),
        Dual(FrequencyMHz),
    }

    impl CPU {
        pub fn new() -> Self {
            Self {
                name: "z80".into(),
                ..Default::default()
            }
        }
    }

    impl Default for ProcessorMode {
        fn default() -> Self {
            Self::Normal(FrequencyMHz(SPD))
        }
    }

    struct Instruction {
        register_1: Register,
        register_2: Register,
    }

    enum Register {
        A,
        B,
        C,
        D,
    }
}
