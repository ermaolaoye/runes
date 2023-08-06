pub mod cpu;
pub mod bus;
pub mod opcodes;
pub mod ui;

use cpu::CPU;
use ui::ui;

use std::boxed::Box;
fn main() {
    let cpu = CPU::new();
    ui(cpu).unwrap();
}
