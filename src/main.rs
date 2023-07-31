pub mod cpu;
pub mod bus;
pub mod opcodes;

use cpu::CPU;

fn main() {
    let cpu = CPU::new();
}
