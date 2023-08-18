use crate::cartridge::Mirroring;

pub enum PPUStatusFlags {
    SpriteOverflow = (1 << 5),
    SpriteZeroHit = (1 << 6),
    VerticalBlank = (1 << 7),
}

pub struct PPU {
    pub chr_rom: Vec<u8>,
    pub vram: [u8; 2048],
    pub oam: [u8; 256],
    pub palette: [u8; 32],

    pub address_register: u16,
    address_latch: bool,

    pub control_register: u8,

    pub mask_register: u8,

    pub status_register: u8,

    pub data_buffer: u8,

    pub mirroring: Mirroring,
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

            mask_register: 0b0000_0000,

            status_register: 0b0000_0000,

            data_buffer: 0b0000_0000,

            mirroring,
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
    pub fn update_mask_register(&mut self, data: u8) {
        self.mask_register = data;
    }
    

    // PPU Read & Write
    pub fn read_data(&mut self) -> u8 {
        self.increment_vram_addr();

        match self.address_register {
            0..=0x1FFF => todo!("read from CHR ROM"),
            0x2000..=0x2FFF => todo!("read from nametable"),
            0x3000..=0x3EFF => todo!("read from nametable mirror"),
            0x3F00..=0x3FFF => {
                self.palette[(self.address_register - 0x3f00) as usize]
            },
            _ => panic!("Invalid address register: {:#X}", self.address_register),
        }
    }

    pub fn write_data(&mut self, data: u8) {
        self.vram[self.address_register as usize] = data;
        self.increment_vram_addr();
    }

}
