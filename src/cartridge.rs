use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

struct INesHeader {
    name: [u8; 4],
    prg_rom_size: u8,
    chr_rom_size: u8,
    mapper_1: u8,
    mapper_2: u8,
    prg_ram_size: u8,
    tv_system_1: u8,
    tv_system_2: u8,
    unused: [u8; 5],
}

pub struct Cartridge {
    header: INesHeader,
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
    mapper: u8,
}

impl Cartridge {
    pub fn new(filename: &str) -> Result<Cartridge, String> {
        let mut file = File::open(filename).unwrap();
        let mut header_buffer: Vec<u8> = vec![0; 16];

        file.read_exact(&mut header_buffer).unwrap();

        let header = INesHeader {
            name: [header_buffer[0], header_buffer[1], header_buffer[2], header_buffer[3]],
            prg_rom_size: header_buffer[4],
            chr_rom_size: header_buffer[5],
            mapper_1: header_buffer[6],
            mapper_2: header_buffer[7],
            prg_ram_size: header_buffer[8],
            tv_system_1: header_buffer[9],
            tv_system_2: header_buffer[10],
            unused: [header_buffer[11], header_buffer[12], header_buffer[13], header_buffer[14], header_buffer[15]],
        };

        if header.name != [0x4E, 0x45, 0x53, 0x1A] {
            return Err("File is not in iNES file format".to_string());
        }

        // Skip the trainer data if header.mapper_1 is 0x04
        if header.mapper_1 & 0x04 == 0x04 {
            file.seek(SeekFrom::Current(512)).unwrap();
        }

        let mapper = (header.mapper_2 & 0xF0) | (header.mapper_1 >> 4);

        let prg_bank_size = 16384 * header.prg_rom_size as usize;
        let mut prg_rom = vec![0; prg_bank_size];
        file.read_exact(&mut prg_rom).unwrap();

        let chr_bank_size = 8192 * header.chr_rom_size as usize;
        let mut chr_rom = vec![0; chr_bank_size];
        file.read_exact(&mut chr_rom).unwrap();

        Ok(Cartridge {
            header,
            prg_rom,
            chr_rom,
            mapper,
        })
    }
}

