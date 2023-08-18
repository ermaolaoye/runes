use crate::cartridge::Cartridge;
use crate::ppu::PPU;


// Memory addresses
const RAM: u16 = 0x0000;
const RAM_MIRRORS_END: u16 = 0x1FFF;
const PPU_REGISTERS: u16 = 0x2000;
const PPU_REGISTERS_MIRRORS_END: u16 = 0x3FFF;

pub struct Bus {
    pub cpu_vram: [u8; 2048],
    pub cartridge: Cartridge,
    pub ppu: PPU,
}

impl Bus {
    pub fn new(cartridge: Cartridge) -> Bus {
        Bus {
            cpu_vram: [0; 2048],
            ppu: PPU::new(cartridge.chr_rom.clone(), cartridge.mirror.clone()),
            cartridge,
        }
    }
}

impl Bus {
    pub fn mem_read(&mut self, addr: u16) -> u8 {
        match addr {
            RAM..=RAM_MIRRORS_END => {
                let mirror_down_addr = addr & 0x07FF;                
                self.cpu_vram[mirror_down_addr as usize]
            },

            // PPU
            0x2000 | 0x2001 | 0x2003 | 0x2005 | 0x2006 | 0x4014 => {
                panic!("PPU write-only register read attempted at address {:#X}", addr);
            },

            0x2002 => self.ppu.read_status_register(),

            0x2007 => self.ppu.read_data(),

            0x2008..=PPU_REGISTERS_MIRRORS_END => {
                let mirror_down_addr = addr & 0x0007;
                self.mem_read(mirror_down_addr)
            },

            // ROM(Cartridge)
            0x8000..=0xFFFF => self.read_prg_rom(addr),
            

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

            0x2000 => self.ppu.write_to_control_register(data),

            0x2001 => self.ppu.write_to_mask_register(data),

            0x2002 => panic!("PPU read-only register write attempted at address {:#X}", addr),

            0x2003 => todo!("OAMADDR"),

            0x2004 => todo!("OAMDATA"),

            0x2005 => todo!("PPUSCROLL"),

            0x2006 => self.ppu.write_to_address_register(data),

            0x2007 => self.ppu.write_data(data),


            PPU_REGISTERS..=PPU_REGISTERS_MIRRORS_END => {
                let mirror_down_addr = addr & 0b00100000_00000111;
                self.mem_write(mirror_down_addr, data);
            },

            0x8000..=0xFFFF => {
                panic!("Cannot write to ROM");
            },

            _ => {
                println!("Unmapped memory address: {:#X}", addr);
            }

        }
    }

    pub fn read_prg_rom(&self, mut addr: u16) -> u8 {
        addr -= 0x8000;
        if self.cartridge.prg_rom.len() == 0x4000 && addr >= 0x4000 {
            // Mirror
            addr -= 0x4000;
        }

        self.cartridge.prg_rom[addr as usize]
    }
    
}
