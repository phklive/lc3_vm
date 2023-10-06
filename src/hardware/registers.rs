const PC_START: u16 = 0x3000;

pub struct Registers {
    pub r_00: u16,   // General purpose register
    pub r_01: u16,   // General purpose register
    pub r_02: u16,   // General purpose register
    pub r_03: u16,   // General purpose register
    pub r_04: u16,   // General purpose register
    pub r_05: u16,   // General purpose register
    pub r_06: u16,   // General purpose register
    pub r_07: u16,   // General purpose register
    pub r_pc: u16,   // Program counter
    pub r_cond: u16, // Conditional flag
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            r_00: 0,
            r_01: 0,
            r_02: 0,
            r_03: 0,
            r_04: 0,
            r_05: 0,
            r_06: 0,
            r_07: 0,
            r_pc: PC_START,
            r_cond: 0,
        }
    }
}
