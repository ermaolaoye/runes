use crate::cartridge::Cartridge;


// Memory addresses
const RAM: u16 = 0x0000;
const RAM_MIRRORS_END: u16 = 0x1FFF;
const PPU_REGISTERS: u16 = 0x2000;
const PPU_REGISTERS_MIRRORS_END: u16 = 0x3FFF;

pub struct Bus {
    pub cpu_vram: [u8; 2048],
    pub cartridge: Cartridge,
}

impl Bus {
    pub fn new(cartridge: Cartridge) -> Bus {
        Bus {
            cpu_vram: [0; 2048],
            cartridge,
        }
    }
}

impl Bus {
    pub fn mem_read(&self, addr: u16) -> u8 {
        match addr {
            RAM..=RAM_MIRRORS_END => {
                let mirror_down_addr = addr & 0b00000111_11111111; // basically we do a mod 0x0800
                self.cpu_vram[mirror_down_addr as usize]
            },
            PPU_REGISTERS..=PPU_REGISTERS_MIRRORS_END => {
                let mirror_down_addr = addr & 0b00100000_00000111;
                todo!("PPU")
            },

            0x8000..=0xFFFF => {
                0
            },

            _ => {
                println!("Unmapped memory address: {:#X}", addr);
                0
            }

        }
    }

    pub fn mem_write(&mut self, addr: u16, data: u8) {
        match addr {
            RAM..=RAM_MIRRORS_END => {
                let mirror_down_addr = addr & 0b11111111_11111111;
                self.cpu_vram[mirror_down_addr as usize] = data;
            },
            PPU_REGISTERS..=PPU_REGISTERS_MIRRORS_END => {
                let mirror_down_addr = addr & 0b00100000_00000111;
                todo!("PPU")
            },

            _ => {
                println!("Unmapped memory address: {:#X}", addr);
            }

        }
    }
}
