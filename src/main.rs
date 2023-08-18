pub mod cpu;
pub mod ppu;
pub mod bus;
pub mod opcodes;
pub mod ui;
pub mod cartridge;

use cpu::CPU;
use ui::ui;
use cartridge::Cartridge;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let cartridge_path = &args[1];
    let cpu = CPU::new(Cartridge::new(cartridge_path).unwrap());
    ui(cpu).unwrap();
}
