use crate::cpu::AddressingMode;

pub enum Opcode {
    ADC, // Add with Carry
    AND, // Logical AND
    ASL, // Arithmetic Shift Left
    BCC, // Branch if Carry Clear
    BCS, // Branch if Carry Set 
    BEQ, // Branch if Equal
    BIT, // Bit Test
    BMI, // Branch if Minus
    BNE, // Branch if Not Equal
    BPL, // Branch if Positive
    BRK, // Force Interrupt
    BVC, // Branch if Overflow Clear
    BVS, // Branch if Overflow Set
    CLC, // Clear Carry Flag
    CLD, // Clear Decimal Mode
    CLI, // Clear Interrupt Disable
    CLV, // Clear Overflow Flag
    CMP, // Compare
    CPX, // Compare X Register
    CPY, // Compare Y Register
    DEC, // Decrement Memory
    DEX, // Decrement X Register
    DEY, // Decrement Y Register
    EOR, // Exclusive OR
    INC, // Increment Memory
    INX, // Increment X Register
    INY, // Increment Y Register
    JMP, // Jump
    JSR, // Jump to Subroutine
    LDA, // Load Accumulator
    LDX, // Load X Register
    LDY, // Load Y Register
    LSR, // Logical Shift Right
    NOP, // No Operation
    ORA, // Logical Inclusive OR
    PHA, // Push Accumulator
    PHP, // Push Processor Status
    PLA, // Pull Accumulator
    PLP, // Pull Processor Status
    ROL, // Rotate Left
    ROR, // Rotate Right
    RTI, // Return from Interrupt
    RTS, // Return from Subroutine
    SBC, // Subtract with Carry
    SEC, // Set Carry Flag
    SED, // Set Decimal Flag
    SEI, // Set Interrupt Disable
    STA, // Store Accumulator
    STX, // Store X Register
    STY, // Store Y Register
    TAX, // Transfer Accumulator to X
    TAY, // Transfer Accumulator to Y
    TSX, // Transfer Stack Pointer to X
    TXA, // Transfer X to Accumulator
    TXS, // Transfer X to Stack Pointer
    TYA, // Transfer Y to Accumulator

    XXX, // Unknown (Unofficial Opcodes)
    // Unofficial Opcodes are not implemented in this project (yet).
}

pub struct Instruction {
    pub hexcode: u8,
    pub operate: Opcode,
    pub addrmode: AddressingMode,
    pub cycles: u8,
}

pub mod references {
    use super::{Instruction, Opcode, AddressingMode};

    // 6502 Instruction Lookup Table
    // This is implemented as a compile time evaluated array, instead of HashMap for better performance.
    pub static INSTRUCTION_LOOKUP: [Instruction; 256] = [
        Instruction { hexcode: 0x00, operate: Opcode::BRK, addrmode: AddressingMode::IMP, cycles: 7 },
        Instruction { hexcode: 0x01, operate: Opcode::ORA, addrmode: AddressingMode::IZX, cycles: 6 },
        Instruction { hexcode: 0x02, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0x03, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 8 },
        Instruction { hexcode: 0x04, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 3 },
        Instruction { hexcode: 0x05, operate: Opcode::ORA, addrmode: AddressingMode::ZP0, cycles: 3 },
        Instruction { hexcode: 0x06, operate: Opcode::ASL, addrmode: AddressingMode::ZP0, cycles: 5 },
        Instruction { hexcode: 0x07, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 5 },
        Instruction { hexcode: 0x08, operate: Opcode::PHP, addrmode: AddressingMode::IMP, cycles: 3 },
        Instruction { hexcode: 0x09, operate: Opcode::ORA, addrmode: AddressingMode::IMM, cycles: 2 },
        Instruction { hexcode: 0x0A, operate: Opcode::ASL, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0x0B, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0x0C, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 4 },
        Instruction { hexcode: 0x0D, operate: Opcode::ORA, addrmode: AddressingMode::ABS, cycles: 4 },
        Instruction { hexcode: 0x0E, operate: Opcode::ASL, addrmode: AddressingMode::ABS, cycles: 6 },
        Instruction { hexcode: 0x0F, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 6 },

        Instruction { hexcode: 0x10, operate: Opcode::BPL, addrmode: AddressingMode::REL, cycles: 2 },
        Instruction { hexcode: 0x11, operate: Opcode::ORA, addrmode: AddressingMode::IZY, cycles: 5 },
        Instruction { hexcode: 0x12, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0x13, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 8 },
        Instruction { hexcode: 0x14, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 4 },
        Instruction { hexcode: 0x15, operate: Opcode::ORA, addrmode: AddressingMode::ZPX, cycles: 4 },
        Instruction { hexcode: 0x16, operate: Opcode::ASL, addrmode: AddressingMode::ZPX, cycles: 6 },
        Instruction { hexcode: 0x17, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 6 },
        Instruction { hexcode: 0x18, operate: Opcode::CLC, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0x19, operate: Opcode::ORA, addrmode: AddressingMode::ABY, cycles: 4 },
        Instruction { hexcode: 0x1A, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0x1B, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 7 },
        Instruction { hexcode: 0x1C, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 4 },
        Instruction { hexcode: 0x1D, operate: Opcode::ORA, addrmode: AddressingMode::ABX, cycles: 4 },
        Instruction { hexcode: 0x1E, operate: Opcode::ASL, addrmode: AddressingMode::ABX, cycles: 7 },
        Instruction { hexcode: 0x1F, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 7 },

        Instruction { hexcode: 0x20, operate: Opcode::JSR, addrmode: AddressingMode::ABS, cycles: 6 },
        Instruction { hexcode: 0x21, operate: Opcode::AND, addrmode: AddressingMode::IZX, cycles: 6 },
        Instruction { hexcode: 0x22, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0x23, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 8 },
        Instruction { hexcode: 0x24, operate: Opcode::BIT, addrmode: AddressingMode::ZP0, cycles: 3 },
        Instruction { hexcode: 0x25, operate: Opcode::AND, addrmode: AddressingMode::ZP0, cycles: 3 },
        Instruction { hexcode: 0x26, operate: Opcode::ROL, addrmode: AddressingMode::ZP0, cycles: 5 },
        Instruction { hexcode: 0x27, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 5 },
        Instruction { hexcode: 0x28, operate: Opcode::PLP, addrmode: AddressingMode::IMP, cycles: 4 },
        Instruction { hexcode: 0x29, operate: Opcode::AND, addrmode: AddressingMode::IMM, cycles: 2 },
        Instruction { hexcode: 0x2A, operate: Opcode::ROL, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0x2B, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0x2C, operate: Opcode::BIT, addrmode: AddressingMode::ABS, cycles: 4 },
        Instruction { hexcode: 0x2D, operate: Opcode::AND, addrmode: AddressingMode::ABS, cycles: 4 },
        Instruction { hexcode: 0x2E, operate: Opcode::ROL, addrmode: AddressingMode::ABS, cycles: 6 },
        Instruction { hexcode: 0x2F, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 6 },

        Instruction { hexcode: 0x30, operate: Opcode::BMI, addrmode: AddressingMode::REL, cycles: 2 },
        Instruction { hexcode: 0x31, operate: Opcode::AND, addrmode: AddressingMode::IZY, cycles: 5 },
        Instruction { hexcode: 0x32, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0x33, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 8 },
        Instruction { hexcode: 0x34, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 4 },
        Instruction { hexcode: 0x35, operate: Opcode::AND, addrmode: AddressingMode::ZPX, cycles: 4 },
        Instruction { hexcode: 0x36, operate: Opcode::ROL, addrmode: AddressingMode::ZPX, cycles: 6 },
        Instruction { hexcode: 0x37, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 6 },
        Instruction { hexcode: 0x38, operate: Opcode::SEC, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0x39, operate: Opcode::AND, addrmode: AddressingMode::ABY, cycles: 4 },
        Instruction { hexcode: 0x3A, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0x3B, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 7 },
        Instruction { hexcode: 0x3C, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 4 },
        Instruction { hexcode: 0x3D, operate: Opcode::AND, addrmode: AddressingMode::ABX, cycles: 4 },
        Instruction { hexcode: 0x3E, operate: Opcode::ROL, addrmode: AddressingMode::ABX, cycles: 7 },
        Instruction { hexcode: 0x3F, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 7 },

        Instruction { hexcode: 0x40, operate: Opcode::RTI, addrmode: AddressingMode::IMP, cycles: 6 },
        Instruction { hexcode: 0x41, operate: Opcode::EOR, addrmode: AddressingMode::IZX, cycles: 6 },
        Instruction { hexcode: 0x42, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0x43, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 8 },
        Instruction { hexcode: 0x44, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 3 },
        Instruction { hexcode: 0x45, operate: Opcode::EOR, addrmode: AddressingMode::ZP0, cycles: 3 },  
        Instruction { hexcode: 0x46, operate: Opcode::LSR, addrmode: AddressingMode::ZP0, cycles: 5 },
        Instruction { hexcode: 0x47, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 5 },
        Instruction { hexcode: 0x48, operate: Opcode::PHA, addrmode: AddressingMode::IMP, cycles: 3 },
        Instruction { hexcode: 0x49, operate: Opcode::EOR, addrmode: AddressingMode::IMM, cycles: 2 },
        Instruction { hexcode: 0x4A, operate: Opcode::LSR, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0x4B, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0x4C, operate: Opcode::JMP, addrmode: AddressingMode::ABS, cycles: 3 },
        Instruction { hexcode: 0x4D, operate: Opcode::EOR, addrmode: AddressingMode::ABS, cycles: 4 },
        Instruction { hexcode: 0x4E, operate: Opcode::LSR, addrmode: AddressingMode::ABS, cycles: 6 },
        Instruction { hexcode: 0x4F, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 6 },

        Instruction { hexcode: 0x50, operate: Opcode::BVC, addrmode: AddressingMode::REL, cycles: 2 },
        Instruction { hexcode: 0x51, operate: Opcode::EOR, addrmode: AddressingMode::IZY, cycles: 5 },
        Instruction { hexcode: 0x52, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0x53, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 8 },
        Instruction { hexcode: 0x54, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 4 },
        Instruction { hexcode: 0x55, operate: Opcode::EOR, addrmode: AddressingMode::ZPX, cycles: 4 },
        Instruction { hexcode: 0x56, operate: Opcode::LSR, addrmode: AddressingMode::ZPX, cycles: 6 },
        Instruction { hexcode: 0x57, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 6 },
        Instruction { hexcode: 0x58, operate: Opcode::CLI, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0x59, operate: Opcode::EOR, addrmode: AddressingMode::ABY, cycles: 4 },
        Instruction { hexcode: 0x5A, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0x5B, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 7 },
        Instruction { hexcode: 0x5C, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 7 },
        Instruction { hexcode: 0x5D, operate: Opcode::EOR, addrmode: AddressingMode::ABX, cycles: 4 },
        Instruction { hexcode: 0x5E, operate: Opcode::LSR, addrmode: AddressingMode::ABX, cycles: 7 },
        Instruction { hexcode: 0x5F, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 7 },

        Instruction { hexcode: 0x60, operate: Opcode::RTS, addrmode: AddressingMode::IMP, cycles: 6 },
        Instruction { hexcode: 0x61, operate: Opcode::ADC, addrmode: AddressingMode::IZX, cycles: 6 },
        Instruction { hexcode: 0x62, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0x63, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 8 },
        Instruction { hexcode: 0x64, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 3 },
        Instruction { hexcode: 0x65, operate: Opcode::ADC, addrmode: AddressingMode::ZP0, cycles: 3 },
        Instruction { hexcode: 0x66, operate: Opcode::ROR, addrmode: AddressingMode::ZP0, cycles: 5 },
        Instruction { hexcode: 0x67, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 5 },
        Instruction { hexcode: 0x68, operate: Opcode::PLA, addrmode: AddressingMode::IMP, cycles: 4 },
        Instruction { hexcode: 0x69, operate: Opcode::ADC, addrmode: AddressingMode::IMM, cycles: 2 },
        Instruction { hexcode: 0x6A, operate: Opcode::ROR, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0x6B, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0x6C, operate: Opcode::JMP, addrmode: AddressingMode::IND, cycles: 5 },
        Instruction { hexcode: 0x6D, operate: Opcode::ADC, addrmode: AddressingMode::ABS, cycles: 4 },
        Instruction { hexcode: 0x6E, operate: Opcode::ROR, addrmode: AddressingMode::ABS, cycles: 6 },
        Instruction { hexcode: 0x6F, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 6 },

        Instruction { hexcode: 0x70, operate: Opcode::BVS, addrmode: AddressingMode::REL, cycles: 2 },
        Instruction { hexcode: 0x71, operate: Opcode::ADC, addrmode: AddressingMode::IZY, cycles: 5 },
        Instruction { hexcode: 0x72, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0x73, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 8 },
        Instruction { hexcode: 0x74, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 4 },
        Instruction { hexcode: 0x75, operate: Opcode::ADC, addrmode: AddressingMode::ZPX, cycles: 4 },
        Instruction { hexcode: 0x76, operate: Opcode::ROR, addrmode: AddressingMode::ZPX, cycles: 6 },
        Instruction { hexcode: 0x77, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 6 },
        Instruction { hexcode: 0x78, operate: Opcode::SEI, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0x79, operate: Opcode::ADC, addrmode: AddressingMode::ABY, cycles: 4 },
        Instruction { hexcode: 0x7A, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0x7B, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 7 },
        Instruction { hexcode: 0x7C, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 4 },
        Instruction { hexcode: 0x7D, operate: Opcode::ADC, addrmode: AddressingMode::ABX, cycles: 4 },
        Instruction { hexcode: 0x7E, operate: Opcode::ROR, addrmode: AddressingMode::ABX, cycles: 7 },
        Instruction { hexcode: 0x7F, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 7 },

        Instruction { hexcode: 0x80, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0x81, operate: Opcode::STA, addrmode: AddressingMode::IZX, cycles: 6 },
        Instruction { hexcode: 0x82, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0x83, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 6 },
        Instruction { hexcode: 0x84, operate: Opcode::STY, addrmode: AddressingMode::ZP0, cycles: 3 },
        Instruction { hexcode: 0x85, operate: Opcode::STA, addrmode: AddressingMode::ZP0, cycles: 3 },
        Instruction { hexcode: 0x86, operate: Opcode::STX, addrmode: AddressingMode::ZP0, cycles: 3 },
        Instruction { hexcode: 0x87, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 3 },
        Instruction { hexcode: 0x88, operate: Opcode::DEY, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0x89, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0x8A, operate: Opcode::TXA, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0x8B, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0x8C, operate: Opcode::STY, addrmode: AddressingMode::ABS, cycles: 4 },
        Instruction { hexcode: 0x8D, operate: Opcode::STA, addrmode: AddressingMode::ABS, cycles: 4 },
        Instruction { hexcode: 0x8E, operate: Opcode::STX, addrmode: AddressingMode::ABS, cycles: 4 },
        Instruction { hexcode: 0x8F, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 4 },

        Instruction { hexcode: 0x90, operate: Opcode::BCC, addrmode: AddressingMode::REL, cycles: 2 },
        Instruction { hexcode: 0x91, operate: Opcode::STA, addrmode: AddressingMode::IZY, cycles: 6 },
        Instruction { hexcode: 0x92, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0x93, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 6 },
        Instruction { hexcode: 0x94, operate: Opcode::STY, addrmode: AddressingMode::ZPX, cycles: 4 },
        Instruction { hexcode: 0x95, operate: Opcode::STA, addrmode: AddressingMode::ZPX, cycles: 4 },
        Instruction { hexcode: 0x96, operate: Opcode::STX, addrmode: AddressingMode::ZPY, cycles: 4 },
        Instruction { hexcode: 0x97, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 4 },
        Instruction { hexcode: 0x98, operate: Opcode::TYA, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0x99, operate: Opcode::STA, addrmode: AddressingMode::ABY, cycles: 5 },
        Instruction { hexcode: 0x9A, operate: Opcode::TXS, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0x9B, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 5 },
        Instruction { hexcode: 0x9C, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 5 },
        Instruction { hexcode: 0x9D, operate: Opcode::STA, addrmode: AddressingMode::ABX, cycles: 5 },
        Instruction { hexcode: 0x9E, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 5 },
        Instruction { hexcode: 0x9F, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 5 },

        Instruction { hexcode: 0xA0, operate: Opcode::LDY, addrmode: AddressingMode::IMM, cycles: 2 },
        Instruction { hexcode: 0xA1, operate: Opcode::LDA, addrmode: AddressingMode::IZX, cycles: 6 },
        Instruction { hexcode: 0xA2, operate: Opcode::LDX, addrmode: AddressingMode::IMM, cycles: 2 },
        Instruction { hexcode: 0xA3, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 6 },
        Instruction { hexcode: 0xA4, operate: Opcode::LDY, addrmode: AddressingMode::ZP0, cycles: 3 },
        Instruction { hexcode: 0xA5, operate: Opcode::LDA, addrmode: AddressingMode::ZP0, cycles: 3 },
        Instruction { hexcode: 0xA6, operate: Opcode::LDX, addrmode: AddressingMode::ZP0, cycles: 3 },
        Instruction { hexcode: 0xA7, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 3 },
        Instruction { hexcode: 0xA8, operate: Opcode::TAY, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0xA9, operate: Opcode::LDA, addrmode: AddressingMode::IMM, cycles: 2 },
        Instruction { hexcode: 0xAA, operate: Opcode::TAX, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0xAB, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0xAC, operate: Opcode::LDY, addrmode: AddressingMode::ABS, cycles: 4 },
        Instruction { hexcode: 0xAD, operate: Opcode::LDA, addrmode: AddressingMode::ABS, cycles: 4 },
        Instruction { hexcode: 0xAE, operate: Opcode::LDX, addrmode: AddressingMode::ABS, cycles: 4 },
        Instruction { hexcode: 0xAF, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 4 },

        Instruction { hexcode: 0xB0, operate: Opcode::BCS, addrmode: AddressingMode::REL, cycles: 2 },
        Instruction { hexcode: 0xB1, operate: Opcode::LDA, addrmode: AddressingMode::IZY, cycles: 5 },
        Instruction { hexcode: 0xB2, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0xB3, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 5 },
        Instruction { hexcode: 0xB4, operate: Opcode::LDY, addrmode: AddressingMode::ZPX, cycles: 4 },
        Instruction { hexcode: 0xB5, operate: Opcode::LDA, addrmode: AddressingMode::ZPX, cycles: 4 },
        Instruction { hexcode: 0xB6, operate: Opcode::LDX, addrmode: AddressingMode::ZPY, cycles: 4 },
        Instruction { hexcode: 0xB7, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 4 },
        Instruction { hexcode: 0xB8, operate: Opcode::CLV, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0xB9, operate: Opcode::LDA, addrmode: AddressingMode::ABY, cycles: 4 },
        Instruction { hexcode: 0xBA, operate: Opcode::TSX, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0xBB, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 4 },
        Instruction { hexcode: 0xBC, operate: Opcode::LDY, addrmode: AddressingMode::ABX, cycles: 4 },
        Instruction { hexcode: 0xBD, operate: Opcode::LDA, addrmode: AddressingMode::ABX, cycles: 4 },
        Instruction { hexcode: 0xBE, operate: Opcode::LDX, addrmode: AddressingMode::ABY, cycles: 4 },
        Instruction { hexcode: 0xBF, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 4 },

        Instruction { hexcode: 0xC0, operate: Opcode::CPY, addrmode: AddressingMode::IMM, cycles: 2 },
        Instruction { hexcode: 0xC1, operate: Opcode::CMP, addrmode: AddressingMode::IZX, cycles: 6 },
        Instruction { hexcode: 0xC2, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0xC3, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 8 },
        Instruction { hexcode: 0xC4, operate: Opcode::CPY, addrmode: AddressingMode::ZP0, cycles: 3 },
        Instruction { hexcode: 0xC5, operate: Opcode::CMP, addrmode: AddressingMode::ZP0, cycles: 3 },
        Instruction { hexcode: 0xC6, operate: Opcode::DEC, addrmode: AddressingMode::ZP0, cycles: 5 },
        Instruction { hexcode: 0xC7, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 5 },
        Instruction { hexcode: 0xC8, operate: Opcode::INY, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0xC9, operate: Opcode::CMP, addrmode: AddressingMode::IMM, cycles: 2 },
        Instruction { hexcode: 0xCA, operate: Opcode::DEX, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0xCB, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0xCC, operate: Opcode::CPY, addrmode: AddressingMode::ABS, cycles: 4 },
        Instruction { hexcode: 0xCD, operate: Opcode::CMP, addrmode: AddressingMode::ABS, cycles: 4 },
        Instruction { hexcode: 0xCE, operate: Opcode::DEC, addrmode: AddressingMode::ABS, cycles: 6 },
        Instruction { hexcode: 0xCF, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 6 },

        Instruction { hexcode: 0xD0, operate: Opcode::BNE, addrmode: AddressingMode::REL, cycles: 2 },
        Instruction { hexcode: 0xD1, operate: Opcode::CMP, addrmode: AddressingMode::IZY, cycles: 5 },
        Instruction { hexcode: 0xD2, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0xD3, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 8 },
        Instruction { hexcode: 0xD4, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 4 },
        Instruction { hexcode: 0xD5, operate: Opcode::CMP, addrmode: AddressingMode::ZPX, cycles: 4 },
        Instruction { hexcode: 0xD6, operate: Opcode::DEC, addrmode: AddressingMode::ZPX, cycles: 6 },
        Instruction { hexcode: 0xD7, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 6 },
        Instruction { hexcode: 0xD8, operate: Opcode::CLD, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0xD9, operate: Opcode::CMP, addrmode: AddressingMode::ABY, cycles: 4 },
        Instruction { hexcode: 0xDA, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0xDB, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 7 },
        Instruction { hexcode: 0xDC, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 4 },
        Instruction { hexcode: 0xDD, operate: Opcode::CMP, addrmode: AddressingMode::ABX, cycles: 4 },
        Instruction { hexcode: 0xDE, operate: Opcode::DEC, addrmode: AddressingMode::ABX, cycles: 7 },
        Instruction { hexcode: 0xDF, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 7 },

        Instruction { hexcode: 0xE0, operate: Opcode::CPX, addrmode: AddressingMode::IMM, cycles: 2 },
        Instruction { hexcode: 0xE1, operate: Opcode::SBC, addrmode: AddressingMode::IZX, cycles: 6 },
        Instruction { hexcode: 0xE2, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0xE3, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 8 },
        Instruction { hexcode: 0xE4, operate: Opcode::CPX, addrmode: AddressingMode::ZP0, cycles: 3 },
        Instruction { hexcode: 0xE5, operate: Opcode::SBC, addrmode: AddressingMode::ZP0, cycles: 3 },
        Instruction { hexcode: 0xE6, operate: Opcode::INC, addrmode: AddressingMode::ZP0, cycles: 5 },
        Instruction { hexcode: 0xE7, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 5 },
        Instruction { hexcode: 0xE8, operate: Opcode::INX, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0xE9, operate: Opcode::SBC, addrmode: AddressingMode::IMM, cycles: 2 },
        Instruction { hexcode: 0xEA, operate: Opcode::NOP, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0xEB, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0xEC, operate: Opcode::CPX, addrmode: AddressingMode::ABS, cycles: 4 },
        Instruction { hexcode: 0xED, operate: Opcode::SBC, addrmode: AddressingMode::ABS, cycles: 4 },
        Instruction { hexcode: 0xEE, operate: Opcode::INC, addrmode: AddressingMode::ABS, cycles: 6 },
        Instruction { hexcode: 0xEF, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 6 },

        Instruction { hexcode: 0xF0, operate: Opcode::BEQ, addrmode: AddressingMode::REL, cycles: 2 },
        Instruction { hexcode: 0xF1, operate: Opcode::SBC, addrmode: AddressingMode::IZY, cycles: 5 },
        Instruction { hexcode: 0xF2, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0xF3, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 8 },
        Instruction { hexcode: 0xF4, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 4 },
        Instruction { hexcode: 0xF5, operate: Opcode::SBC, addrmode: AddressingMode::ZPX, cycles: 4 },
        Instruction { hexcode: 0xF6, operate: Opcode::INC, addrmode: AddressingMode::ZPX, cycles: 6 },
        Instruction { hexcode: 0xF7, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 6 },
        Instruction { hexcode: 0xF8, operate: Opcode::SED, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0xF9, operate: Opcode::SBC, addrmode: AddressingMode::ABY, cycles: 4 },
        Instruction { hexcode: 0xFA, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 2 },
        Instruction { hexcode: 0xFB, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 7 },
        Instruction { hexcode: 0xFC, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 4 },
        Instruction { hexcode: 0xFD, operate: Opcode::SBC, addrmode: AddressingMode::ABX, cycles: 4 },
        Instruction { hexcode: 0xFE, operate: Opcode::INC, addrmode: AddressingMode::ABX, cycles: 7 },
        Instruction { hexcode: 0xFF, operate: Opcode::XXX, addrmode: AddressingMode::IMP, cycles: 7 },


    ];
}
