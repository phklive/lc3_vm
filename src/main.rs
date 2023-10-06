pub mod hardware;
use hardware::registers::Registers;

const MEMORY_MAX: usize = 1 << 16;

fn main() {
    let mut _memory: [u16; MEMORY_MAX] = [0; MEMORY_MAX];
    let mut _running: u16 = 1;
    let _reg: Registers = Registers::new();
}
