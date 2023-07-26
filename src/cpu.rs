use crate::opcodes::{references, Opcode};

enum Flags {
    C = (1 << 0), // Carry Bit
    Z = (1 << 1), // Zero
    I = (1 << 2), // Disable Interrupts
    D = (1 << 3), // Decimal Mode (unused in this implementation)
    B = (1 << 4), // Break
    U = (1 << 5), // Unused
    V = (1 << 6), // Overflow
    N = (1 << 7), // Negative
}

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
                _ => panic!("Addressing mode not implemented"),
            };

            self.cycles += additional_cycle1 & additional_cycle2;
        }

        self.cycles -= 1;
    }
   
}
