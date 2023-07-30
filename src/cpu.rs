use crate::opcodes::{references, Opcode};

enum StatusFlag {
    C = (1 << 0), // Carry Bit
    Z = (1 << 1), // Zero
    I = (1 << 2), // Disable Interrupts
    D = (1 << 3), // Decimal Mode (unused in this implementation)
    B = (1 << 4), // Break
    U = (1 << 5), // Unused
    V = (1 << 6), // Overflow
    N = (1 << 7), // Negative
}

#[derive(PartialEq)]
pub enum AddressingMode {
    IMP, // Implied
    IMM, // Immediate
    ZP0, // Zero Page
    ZPX, // Zero Page with X Offset
    ZPY, // Zero Page with Y Offset
    REL, // Relative
    ABS, // Absolute
    ABX, // Absolute with X Offset
    ABY, // Absolute with Y Offset
    IND, // Indirect
    IZX, // Indirect with X Offset
    IZY, // Indirect with Y Offset
}

pub struct CPU {
    accumulator: u8, // Accumulator Register
    x_register: u8, // X Register
    y_register: u8, // Y Register
    stack_pointer: u8, // Stack Pointer (points to location on bus)
    program_counter: u16, // Program Counter

    status: u8,

    fetched: u8, // Represents the working input value to the ALU

    addr_abs: u16, // All used memory addresses end up in here
    addr_rel: u16, // Represents absolute address following a branch
    opcode: u8, // Instruction opcode is fetched here
    cycles: u8, // Counts how many cycles the instruction has remaining
}
    
impl CPU {
    pub fn new() -> Self {
        CPU {
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            stack_pointer: 0x00,
            program_counter: 0x0000,
            status: 0x00,

            fetched: 0x00,

            addr_abs: 0x0000,
            addr_rel: 0x0000,
            opcode: 0x00,
            cycles: 0x00,

        } 
    }

    pub fn read(&self, addr: u16, b_read_only: bool) -> u8 {
        todo!("Implement read");
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        todo!("Implement write");
    }

    pub fn clock(&mut self) {
        if self.cycles == 0 {
            self.opcode = self.read(self.program_counter, false);
            self.program_counter += 1;
            
            let operate = &references::INSTRUCTION_LOOKUP[self.opcode as usize].operate;
            let addressing_mode = &references::INSTRUCTION_LOOKUP[self.opcode as usize].addrmode;

            let additional_cycle1: u8 = match operate {
                _ => panic!("Opcode not implemented"),
            };

            let additional_cycle2: u8 = match addressing_mode {
                AddressingMode::IMP => self.imp(),
                AddressingMode::IMM => self.imm(),
                AddressingMode::ZP0 => self.zp0(),
                AddressingMode::ZPX => self.zpx(),
                AddressingMode::ZPY => self.zpy(),
                AddressingMode::REL => self.rel(),
                AddressingMode::ABS => self.abs(),
                AddressingMode::ABX => self.abx(),
                AddressingMode::ABY => self.aby(),
                AddressingMode::IND => self.ind(),
                AddressingMode::IZX => self.izx(),
                AddressingMode::IZY => self.izy(),
            };

            self.cycles += additional_cycle1 & additional_cycle2;
        }

        self.cycles -= 1;
    }
}

impl CPU {
    // Flags Functions
    fn set_flag(&mut self, flag: StatusFlag, value: bool) {
        if value {
            self.status |= flag as u8;
        } else {
            self.status &= !(flag as u8);
        }
    }

    fn get_flag(&self, flag: StatusFlag) -> bool {
        (self.status & (flag as u8)) > 0
    }

    // Addressing Modes
    // Returned integer is the additional number of cycles required for the Instruction

    fn imp(&mut self) -> u8{
        // Some instructions use the accumulator's value as operand
        self.fetched = self.accumulator;
        0
    }

    fn imm(&mut self) -> u8 {
        self.addr_abs = self.program_counter;
        self.program_counter += 1;
        0
    }

    fn zp0(&mut self) -> u8 {
        self.addr_abs = self.read(self.program_counter, false) as u16;
        self.program_counter += 1;
        self.addr_abs &= 0x00FF;
        0
    }

    fn zpx(&mut self) -> u8 {
        self.addr_abs = (self.read(self.program_counter, false) + self.x_register) as u16;
        self.program_counter += 1;
        self.addr_abs &= 0x00FF;
        0
    }

    fn zpy(&mut self) -> u8 {
        self.addr_abs = (self.read(self.program_counter, false) + self.y_register) as u16;
        self.program_counter += 1;
        self.addr_abs &= 0x00FF;
        0
    }

    fn abs(&mut self) -> u8 {
        //6502 stores memory address in little endian format
        let lo = self.read(self.program_counter, false) as u16;
        self.program_counter += 1;

        let hi = self.read(self.program_counter, false) as u16;
        self.program_counter += 1;

        self.addr_abs = (hi << 8) | lo;
        0
    }

    fn abx(&mut self) -> u8 {
        let lo = self.read(self.program_counter, false) as u16;
        self.program_counter += 1;

        let hi = self.read(self.program_counter, false) as u16;
        self.program_counter += 1;

        self.addr_abs = (hi << 8) | lo;
        self.addr_abs += self.x_register as u16;

        // If the addition of the offset causes a change in the high byte, an additional cycle is required
        if (self.addr_abs & 0xFF00) != (hi << 8) {
            1
        } else {
            0
        }
    }

    fn aby(&mut self) -> u8 {
        let lo = self.read(self.program_counter, false) as u16;
        self.program_counter += 1;

        let hi = self.read(self.program_counter, false) as u16;
        self.program_counter += 1;

        self.addr_abs = (hi << 8) | lo;
        self.addr_abs += self.y_register as u16;

        // If the addition of the offset causes a change in the high byte, an additional cycle is required
        if (self.addr_abs & 0xFF00) != (hi << 8) {
            1
        } else {
            0
        }
    }

    fn ind(&mut self) -> u8 {
        let ptr_lo = self.read(self.program_counter, false) as u16;
        self.program_counter += 1;

        let ptr_hi = self.read(self.program_counter, false) as u16;
        self.program_counter += 1;

        let ptr = (ptr_hi << 8) | ptr_lo;

        // 6502 bug where if the low byte of the supplied address is 0xFF, the high byte is fetched from the low byte of the supplied address
        // This is added for bug for bug compatibility
        if ptr_lo == 0x00FF {
            self.addr_abs = (self.read(ptr & 0xFF00, false) as u16) << 8 | self.read(ptr, false) as u16;
        } else {
            self.addr_abs = (self.read(ptr + 1, false) as u16) << 8 | self.read(ptr, false) as u16;
        }

        0
    }

    fn izx(&mut self) -> u8 {
        let t = self.read(self.program_counter, false) as u16;
        self.program_counter += 1;

        let lo = self.read((t + self.x_register as u16) & 0x00FF, false) as u16;
        let hi = self.read((t + self.x_register as u16 + 1) & 0x00FF, false) as u16;

        self.addr_abs = (hi << 8) | lo;
        0
    }

    fn izy(&mut self) -> u8 {
        let t = self.read(self.program_counter, false) as u16;
        self.program_counter += 1;

        let lo = self.read(t & 0x00FF, false) as u16;
        let hi = self.read((t + 1) & 0x00FF, false) as u16;

        self.addr_abs = (hi << 8) | lo;
        self.addr_abs += self.y_register as u16;

        // If the addition of the offset causes a change in the high byte, an additional cycle is required
        if (self.addr_abs & 0xFF00) != (hi << 8) {
            1
        } else {
            0
        }
    }

    fn rel(&mut self) -> u8 {
        self.addr_rel = self.read(self.program_counter, false) as u16;
        self.program_counter += 1;

        // two's complement to convert to signed integer
        if (self.addr_rel & 0x80) != 0 {
            self.addr_rel |= 0xFF00;
        }

        0
    }
    // fetches data from memory using the address mode
    fn fetch(&mut self) -> u8 {
        if references::INSTRUCTION_LOOKUP[self.opcode as usize].addrmode != AddressingMode::IMP {
            self.fetched = self.read(self.addr_abs, false);
        }

        self.fetched
    }

    // Instructions
    fn and(&mut self) -> u8 {
        self.fetch();
        self.accumulator &= self.fetched;
        self.set_flag(StatusFlag::Z, self.accumulator == 0x00);
        self.set_flag(StatusFlag::N, self.accumulator & 0x80 != 0);
        1
    }


    fn bcs(&mut self) -> u8 {
        if self.get_flag(StatusFlag::C) {
            self.cycles += 1;
            self.addr_abs = self.program_counter + self.addr_rel;

            // If the branch crosses a page boundary, an additional cycle is required
            if (self.addr_abs & 0xFF00) != (self.program_counter & 0xFF00) {
                self.cycles += 1;
            }

            self.program_counter = self.addr_abs;
        }

        0
    }

    fn bcc(&mut self) -> u8 {
        if !self.get_flag(StatusFlag::C) {
            self.cycles += 1;
            self.addr_abs = self.program_counter + self.addr_rel;

            // If the branch crosses a page boundary, an additional cycle is required
            if (self.addr_abs & 0xFF00) != (self.program_counter & 0xFF00) {
                self.cycles += 1;
            }

            self.program_counter = self.addr_abs;
        }

        0
    }

    fn beq(&mut self) -> u8 {
        if self.get_flag(StatusFlag::Z) {
            self.cycles += 1;
            self.addr_abs = self.program_counter + self.addr_rel;

            // If the branch crosses a page boundary, an additional cycle is required
            if (self.addr_abs & 0xFF00) != (self.program_counter & 0xFF00) {
                self.cycles += 1;
            }

            self.program_counter = self.addr_abs;
        }

        0
    }

    fn bmi(&mut self) -> u8 {
        if self.get_flag(StatusFlag::N) {
            self.cycles += 1;
            self.addr_abs = self.program_counter + self.addr_rel;

            // If the branch crosses a page boundary, an additional cycle is required
            if (self.addr_abs & 0xFF00) != (self.program_counter & 0xFF00) {
                self.cycles += 1;
            }

            self.program_counter = self.addr_abs;
        }

        0
    }

    fn bne(&mut self) -> u8 {
        if !self.get_flag(StatusFlag::Z) {
            self.cycles += 1;
            self.addr_abs = self.program_counter + self.addr_rel;

            // If the branch crosses a page boundary, an additional cycle is required
            if (self.addr_abs & 0xFF00) != (self.program_counter & 0xFF00) {
                self.cycles += 1;
            }

            self.program_counter = self.addr_abs;
        }

        0
    }

    fn bpl(&mut self) -> u8 {
        if !self.get_flag(StatusFlag::N) {
            self.cycles += 1;
            self.addr_abs = self.program_counter + self.addr_rel;

            // If the branch crosses a page boundary, an additional cycle is required
            if (self.addr_abs & 0xFF00) != (self.program_counter & 0xFF00) {
                self.cycles += 1;
            }

            self.program_counter = self.addr_abs;
        }

        0
    }

    fn bvc(&mut self) -> u8 {
        if !self.get_flag(StatusFlag::V) {
            self.cycles += 1;
            self.addr_abs = self.program_counter + self.addr_rel;

            // If the branch crosses a page boundary, an additional cycle is required
            if (self.addr_abs & 0xFF00) != (self.program_counter & 0xFF00) {
                self.cycles += 1;
            }

            self.program_counter = self.addr_abs;
        }

        0
    }

    fn bvs(&mut self) -> u8 {
        if self.get_flag(StatusFlag::V) {
            self.cycles += 1;
            self.addr_abs = self.program_counter + self.addr_rel;

            // If the branch crosses a page boundary, an additional cycle is required
            if (self.addr_abs & 0xFF00) != (self.program_counter & 0xFF00) {
                self.cycles += 1;
            }

            self.program_counter = self.addr_abs;
        }

        0
    }

    fn clc(&mut self) -> u8 {
        self.set_flag(StatusFlag::C, false);
        0
    }

    fn cld(&mut self) -> u8 {
        self.set_flag(StatusFlag::D, false);
        0
    }

    fn cli(&mut self) -> u8 {
        self.set_flag(StatusFlag::I, false);
        0
    }

    fn clv(&mut self) -> u8 {
        self.set_flag(StatusFlag::V, false);
        0
    }

    fn adc(&mut self) -> u8 {
        self.fetch();
        let temp: u16 = self.accumulator as u16 + self.fetched as u16 + self.get_flag(StatusFlag::C) as u16;
        self.set_flag(StatusFlag::C, temp > 255);
        self.set_flag(StatusFlag::Z, (temp & 0x00FF) == 0);
        self.set_flag(StatusFlag::N, (temp & 0x80) != 0);
        self.set_flag(StatusFlag::V, ((!(self.accumulator ^ self.fetched) & (self.accumulator ^ temp as u8)) & 0x80) != 0);
        self.accumulator = temp as u8;
        1
    }

    fn sbc(&mut self) -> u8 {
        self.fetch();
        let value: u16 = (self.fetched as u16) ^ 0x00FF;
        let temp: u16 = self.accumulator as u16 + value + self.get_flag(StatusFlag::C) as u16;
        self.set_flag(StatusFlag::C, (temp & 0xFF00) != 0);
        self.set_flag(StatusFlag::Z, (temp & 0x00FF) == 0);
        self.set_flag(StatusFlag::V, ((temp ^ self.accumulator as u16) & (temp ^ value) & 0x0080) != 0);
        self.set_flag(StatusFlag::N, (temp & 0x0080) != 0);
        self.accumulator = temp as u8;
        1
    }

    fn pha(&mut self) -> u8 {
        self.write(0x0100 + self.stack_pointer as u16, self.accumulator);
        self.stack_pointer -= 1;
        0
    }

    fn pla(&mut self) -> u8 {
        self.stack_pointer += 1;
        self.accumulator = self.read(0x0100 + self.stack_pointer as u16, false);
        self.set_flag(StatusFlag::Z, self.accumulator == 0x00);
        self.set_flag(StatusFlag::N, (self.accumulator & 0x80) != 0);
        0
    }

    fn irq(&mut self) -> u8 {
        if !self.get_flag(StatusFlag::I) {
            self.write(0x0100 + self.stack_pointer as u16, ((self.program_counter >> 8) & 0x00FF) as u8);
            self.stack_pointer -= 1;
            self.write(0x0100 + self.stack_pointer as u16, (self.program_counter & 0x00FF) as u8);
            self.stack_pointer -= 1;

            self.set_flag(StatusFlag::B, false);
            self.set_flag(StatusFlag::U, true);
            self.set_flag(StatusFlag::I, true);
            self.write(0x0100 + self.stack_pointer as u16, self.status);
            self.stack_pointer -= 1;

            self.addr_abs = 0xFFFE;
            let lo = self.read(self.addr_abs, false) as u16;
            let hi = self.read(self.addr_abs + 1, false) as u16;
            self.program_counter = (hi << 8) | lo;

            self.cycles = 7;
        }

        0
    }

    fn nmi(&mut self) -> u8 {
        self.write(0x0100 + self.stack_pointer as u16, ((self.program_counter >> 8) & 0x00FF) as u8);
        self.stack_pointer -= 1;
        self.write(0x0100 + self.stack_pointer as u16, (self.program_counter & 0x00FF) as u8);
        self.stack_pointer -= 1;

        self.set_flag(StatusFlag::B, false);
        self.set_flag(StatusFlag::U, true);
        self.set_flag(StatusFlag::I, true);
        self.write(0x0100 + self.stack_pointer as u16, self.status);
        self.stack_pointer -= 1;

        self.addr_abs = 0xFFFA;
        let lo = self.read(self.addr_abs, false) as u16;
        let hi = self.read(self.addr_abs + 1, false) as u16;
        self.program_counter = (hi << 8) | lo;

        self.cycles = 8;
        0
    }

    fn rti(&mut self) -> u8 {
        self.stack_pointer += 1;
        self.status = self.read(0x0100 + self.stack_pointer as u16, false);
        self.status &= !(StatusFlag::B as u8);
        self.status &= !(StatusFlag::U as u8);

        self.stack_pointer += 1;
        let lo = self.read(0x0100 + self.stack_pointer as u16, false) as u16;
        self.stack_pointer += 1;
        let hi = self.read(0x0100 + self.stack_pointer as u16, false) as u16;
        self.program_counter = (hi << 8) | lo;

        0
    }

    fn rts(&mut self) -> u8 {
        self.stack_pointer += 1;
        let lo = self.read(0x0100 + self.stack_pointer as u16, false) as u16;
        self.stack_pointer += 1;
        let hi = self.read(0x0100 + self.stack_pointer as u16, false) as u16;
        self.program_counter = (hi << 8) | lo;

        self.program_counter += 1;
        0
    }

    fn sec(&mut self) -> u8 {
        self.set_flag(StatusFlag::C, true);
        0
    }

    fn sed(&mut self) -> u8 {
        self.set_flag(StatusFlag::D, true);
        0
    }

    fn sei(&mut self) -> u8 {
        self.set_flag(StatusFlag::I, true);
        0
    }

    fn sta(&mut self) -> u8 {
        self.write(self.addr_abs, self.accumulator);
        0
    }

    fn stx(&mut self) -> u8 {
        self.write(self.addr_abs, self.x_register);
        0
    }

    fn sty(&mut self) -> u8 {
        self.write(self.addr_abs, self.y_register);
        0
    }

    fn tax(&mut self) -> u8 {
        self.x_register = self.accumulator;
        self.set_flag(StatusFlag::Z, self.x_register == 0x00);
        self.set_flag(StatusFlag::N, (self.x_register & 0x80) != 0);
        0
    }

    fn tay(&mut self) -> u8 {
        self.y_register = self.accumulator;
        self.set_flag(StatusFlag::Z, self.y_register == 0x00);
        self.set_flag(StatusFlag::N, (self.y_register & 0x80) != 0);
        0
    }

    fn tsx(&mut self) -> u8 {
        self.x_register = self.stack_pointer;
        self.set_flag(StatusFlag::Z, self.x_register == 0x00);
        self.set_flag(StatusFlag::N, (self.x_register & 0x80) != 0);
        0
    }

    fn txa(&mut self) -> u8 {
        self.accumulator = self.x_register;
        self.set_flag(StatusFlag::Z, self.accumulator == 0x00);
        self.set_flag(StatusFlag::N, (self.accumulator & 0x80) != 0);
        0
    }

    fn txs(&mut self) -> u8 {
        self.stack_pointer = self.x_register;
        0
    }

    fn tya(&mut self) -> u8 {
        self.accumulator = self.y_register;
        self.set_flag(StatusFlag::Z, self.accumulator == 0x00);
        self.set_flag(StatusFlag::N, (self.accumulator & 0x80) != 0);
        0
    }

    fn xxx(&mut self) -> u8 {
        0
    }
    
    fn reset(&mut self) {
        self.addr_abs = 0xFFFC;
        let lo = self.read(self.addr_abs, false) as u16;
        let hi = self.read(self.addr_abs + 1, false) as u16;
        self.program_counter = (hi << 8) | lo;
        
        self.accumulator = 0;
        self.x_register = 0;
        self.y_register = 0;
        self.stack_pointer = 0xFD;
        self.status = 0x00 | StatusFlag::U as u8;

        self.addr_rel = 0x0000;
        self.addr_abs = 0x0000;
        self.fetched = 0x00;

        self.cycles = 8;
    }


}    
