use crate::cartridge::Mirroring;
pub enum PPUStatusFlags {
    SpriteOverflow = (1 << 5),
    SpriteZeroHit = (1 << 6),
    VerticalBlank = (1 << 7),
}

pub enum PPUControlFlags {
    NametableX = (1 << 0),
    NametableY = (1 << 1),
    IncrementMode = (1 << 2),
    PatternSprite = (1 << 3),
    PatternBackground = (1 << 4),
    SpriteSize = (1 << 5),
    SlaveMode = (1 << 6),
    EnableNMI = (1 << 7),
}

// In the format of (R,G,B)
pub static SYSTEM_PALLETE: [(u8,u8,u8); 64] = [
   (0x80, 0x80, 0x80), (0x00, 0x3D, 0xA6), (0x00, 0x12, 0xB0), (0x44, 0x00, 0x96), (0xA1, 0x00, 0x5E),
   (0xC7, 0x00, 0x28), (0xBA, 0x06, 0x00), (0x8C, 0x17, 0x00), (0x5C, 0x2F, 0x00), (0x10, 0x45, 0x00),
   (0x05, 0x4A, 0x00), (0x00, 0x47, 0x2E), (0x00, 0x41, 0x66), (0x00, 0x00, 0x00), (0x05, 0x05, 0x05),
   (0x05, 0x05, 0x05), (0xC7, 0xC7, 0xC7), (0x00, 0x77, 0xFF), (0x21, 0x55, 0xFF), (0x82, 0x37, 0xFA),
   (0xEB, 0x2F, 0xB5), (0xFF, 0x29, 0x50), (0xFF, 0x22, 0x00), (0xD6, 0x32, 0x00), (0xC4, 0x62, 0x00),
   (0x35, 0x80, 0x00), (0x05, 0x8F, 0x00), (0x00, 0x8A, 0x55), (0x00, 0x99, 0xCC), (0x21, 0x21, 0x21),
   (0x09, 0x09, 0x09), (0x09, 0x09, 0x09), (0xFF, 0xFF, 0xFF), (0x0F, 0xD7, 0xFF), (0x69, 0xA2, 0xFF),
   (0xD4, 0x80, 0xFF), (0xFF, 0x45, 0xF3), (0xFF, 0x61, 0x8B), (0xFF, 0x88, 0x33), (0xFF, 0x9C, 0x12),
   (0xFA, 0xBC, 0x20), (0x9F, 0xE3, 0x0E), (0x2B, 0xF0, 0x35), (0x0C, 0xF0, 0xA4), (0x05, 0xFB, 0xFF),
   (0x5E, 0x5E, 0x5E), (0x0D, 0x0D, 0x0D), (0x0D, 0x0D, 0x0D), (0xFF, 0xFF, 0xFF), (0xA6, 0xFC, 0xFF),
   (0xB3, 0xEC, 0xFF), (0xDA, 0xAB, 0xEB), (0xFF, 0xA8, 0xF9), (0xFF, 0xAB, 0xB3), (0xFF, 0xD2, 0xB0),
   (0xFF, 0xEF, 0xA6), (0xFF, 0xF7, 0x9C), (0xD7, 0xE8, 0x95), (0xA6, 0xED, 0xAF), (0xA2, 0xF2, 0xDA),
   (0x99, 0xFF, 0xFC), (0xDD, 0xDD, 0xDD), (0x11, 0x11, 0x11), (0x11, 0x11, 0x11)
];

pub struct PPU {
    pub chr_rom: Vec<u8>,
    pub vram: [u8; 2048],
    pub oam: [u8; 256],
    pub palette: [u8; 32],

    // PPU Registers

    pub address_register: u16,
    address_latch: bool,

    pub control_register: u8,
    pub nmi: bool,

    pub mask_register: u8,

    pub status_register: u8,


    // Data Buffer

    pub data_buffer: u8,

    pub mirroring: Mirroring,

    // Miscs
    pub scanline: u16,
    pub cycle: u16,

}

impl PPU {
    pub fn new(chr_rom: Vec<u8>, mirroring: Mirroring) -> PPU {
        PPU {
            chr_rom,
            vram: [0; 2048],
            oam: [0; 256],
            palette: [0; 32],
            address_register: 0b0000_0000_0000_0000,
            address_latch: true,

            control_register: 0b0000_0000,
            nmi: false,

            mask_register: 0b0000_0000,

            status_register: 0b0000_0000,

            data_buffer: 0b0000_0000,

            mirroring,

            scanline: 0,
            cycle: 0,
        }
    }

    // Mirroring
    pub fn mirror_vram_addr(&mut self, addr: u16) -> u16 {
        let mirrored_vram = addr & 0b1011_1111_1111_1111;
        let vram_index = mirrored_vram & 0x2000;
        let name_table = vram_index / 0x0400;

        match (&self.mirroring, name_table) {
            (Mirroring::Vertical, 2) | (Mirroring::Vertical, 3) => vram_index - 0x800,
            (Mirroring::Horizontal, 2) | (Mirroring::Horizontal, 1) => vram_index - 0x400,
            (Mirroring::Horizontal, 3) => vram_index - 0x800,
            _ => vram_index,
        }
    }

    // Address Register

    pub fn write_to_address_register(&mut self, data: u8) {
        if self.address_latch {
            self.address_register = (self.address_register & 0x00FF) | ((data as u16) << 8);
        } else {
            self.address_register = (self.address_register & 0xFF00) | (data as u16);
        }

        // Mirroring
        if self.address_register >= 0x3FFF {
            self.address_register &= 0x3FFF;
        }

        self.address_latch = !self.address_latch;
    }

    pub fn increment_address_register(&mut self, increment: u8) {
        self.address_register += increment as u16;

        // Mirroring
        if self.address_register >= 0x3FFF {
            self.address_register &= 0x3FFF;
        }
    }

    pub fn reset_address_latch(&mut self) {
        self.address_latch = true;
    }

    // Control Register

    pub fn write_to_control_register(&mut self, data: u8) {
        self.control_register = data;
    }


    pub fn get_control_flag(&mut self, flag: PPUControlFlags) -> bool {
        self.control_register & (flag as u8) != 0
    }

    pub fn increment_vram_addr(&mut self) {
        let increment: u8;
        if self.control_register & 0b0000_0100 == 0 {
            increment = 1
        } else {
            increment = 32
        }

        self.increment_address_register(increment);
    }

    // Status register
    pub fn read_status_register(&mut self) -> u8 {
        let status = (self.status_register & 0xE0) | (self.data_buffer & 0x1F); // Noise
        self.set_status_flag(PPUStatusFlags::VerticalBlank, false);
        self.address_latch = true;
        status
    }

    pub fn set_status_flag(&mut self, flag: PPUStatusFlags, value: bool) {
        if value {
            self.status_register |= flag as u8;
        } else {
            self.status_register &= !(flag as u8);
        }
    }


    // Mask Register
    pub fn write_to_mask_register(&mut self, data: u8) {
        self.mask_register = data;
    }
    

    // PPU Read & Write
    pub fn read_data(&mut self) -> u8 {
        self.increment_vram_addr();

        match self.address_register {
            0..=0x1FFF => {
                // Read from CHR ROM
                let data = self.data_buffer;
                self.data_buffer = self.chr_rom[self.address_register as usize];
                data
            }, 
            0x2000..=0x2FFF => {
                // Read from VRAM
                let data = self.data_buffer;
                self.data_buffer = self.vram[self.mirror_vram_addr(self.address_register) as usize];
                data
            },
            0x3000..=0x3EFF => panic!("PPU invalid read with address register: {:#X}", self.address_register),

            0x3F00..=0x3FFF => {
                self.palette[(self.address_register - 0x3f00) as usize]
            },

            _ => panic!("PPU invalid read with address register: {:#X}", self.address_register),
        }
    }

    pub fn write_data(&mut self, data: u8) {
        self.vram[self.address_register as usize] = data;
        self.increment_vram_addr();
    }

    pub fn clock(&mut self) {
        match self.scanline {
            0..=239 => {
                match self.cycle {
                    1..=256 => {
                        // Background Rendering
                    },
                    257 => {
                        // Sprite Evaluation
                    },
                    321..=336 => {
                        // Background Rendering
                    },
                    _ => {},
                }
            },
            240 => {
                // Post Render Scanline - Do Nothing
            },
            241..=260 => {
                if self.scanline == 241 && self.cycle == 1 {
                    self.set_status_flag(PPUStatusFlags::VerticalBlank, true);

                    if self.get_control_flag(PPUControlFlags::EnableNMI) {
                        self.nmi = true;
                    }
                }
            },

            _ => {},
        }

        self.cycle += 1;

        if self.cycle >= 341 {
            self.cycle = 0;
            self.scanline += 1;

            if self.scanline > 261 {
                self.scanline = 0;
                self.set_status_flag(PPUStatusFlags::VerticalBlank, false);
            }
        }
    }

}
