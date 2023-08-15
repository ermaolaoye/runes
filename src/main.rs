pub mod cpu;
pub mod bus;
pub mod opcodes;
pub mod ui;
pub mod cartridge;

use cpu::CPU;
use ui::ui;

fn main() {
    let cpu = CPU::new();
    ui(cpu).unwrap();
}
