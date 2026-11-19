use crate::inst::Instruction;
use anyhow::{Ok, bail};
use std::ops::{Range, Shl, Shr};

pub enum RetCode {
    Cont,
    Halt,
}

pub struct Machine {
    register: [i64; 32],
    memory: Memory,
    pc: usize,
}

pub struct Memory {
    ram: Vec<u8>,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Self { ram: vec![0; size] }
    }

    pub fn guard_addr(&self, addr: usize) -> anyhow::Result<()> {
        if addr >= self.ram.len() {
            bail!("memory addr out of range:{}", addr)
        }
        Ok(())
    }

    pub fn slice(&self, addr: usize, len: usize) -> anyhow::Result<&[u8]> {
        self.guard_addr(addr + len - 1)?;
        Ok(&self.ram[addr..addr + len])
    }

    #[allow(dead_code)]
    pub fn slice_range(&self, range: Range<usize>) -> anyhow::Result<&[u8]> {
        self.guard_addr(range.end)?;
        Ok(&self.ram[range])
    }

    pub fn slice_mut(&mut self, addr: usize, len: usize) -> anyhow::Result<&mut [u8]> {
        self.guard_addr(addr + len - 1)?;
        Ok(&mut self.ram[addr..addr + len])
    }

    pub fn write_slice(&mut self, addr: usize, slice: &[u8]) -> anyhow::Result<()> {
        let sl_len = slice.len();
        self.guard_addr(addr + sl_len - 1)?;
        self.ram[addr..addr + sl_len].copy_from_slice(slice);
        Ok(())
    }

    pub fn read_u8(&self, addr: usize) -> anyhow::Result<u8> {
        self.guard_addr(addr)?;
        Ok(self.ram[addr])
    }

    pub fn write_u8(&mut self, addr: usize, byte: u8) -> anyhow::Result<()> {
        self.guard_addr(addr)?;
        self.ram[addr] = byte;
        Ok(())
    }

    pub fn read_u16(&self, addr: usize) -> anyhow::Result<u16> {
        Ok(u16::from_le_bytes(self.slice(addr, 2)?.try_into()?))
    }

    pub fn read_u32(&self, addr: usize) -> anyhow::Result<u32> {
        Ok(u32::from_le_bytes(self.slice(addr, 4)?.try_into()?))
    }

    pub fn read_u64(&self, addr: usize) -> anyhow::Result<u64> {
        Ok(u64::from_le_bytes(self.slice(addr, 8)?.try_into()?))
    }
}

impl Machine {
    pub fn new(memory_size: usize) -> Self {
        Self {
            register: [0; 32],
            memory: Memory::new(memory_size),
            pc: 0,
        }
    }

    pub fn load_bin(&mut self, bin: &[u8]) -> anyhow::Result<()> {
        self.memory.slice_mut(0, bin.len())?.copy_from_slice(bin);
        Ok(())
    }

    fn rr(&self, xx: usize) -> i64 {
        self.register[xx]
    }

    fn rrw(&self, xx: usize) -> i32 {
        self.register[xx] as i32
    }

    fn rru(&self, xx: usize) -> u64 {
        self.register[xx] as u64
    }

    fn rruw(&self, xx: usize) -> u32 {
        self.register[xx] as u32
    }

    fn wr(&mut self, xx: usize, val: i64) {
        if xx == 0 {
            return;
        }
        self.register[xx] = val;
    }

    fn wrw(&mut self, xx: usize, val: i32) {
        self.wr(xx, val as i64);
    }

    fn wru(&mut self, xx: usize, val: u64) {
        self.wr(xx, val as i64)
    }

    fn advance_pc(&mut self) {
        self.pc = self.pc.wrapping_add(4);
    }

    fn jump(&mut self, offset: i64) {
        self.pc = self.pc.wrapping_add_signed(offset as isize);
    }

    fn effective_addr(&self, rs1: usize, imm: i64) -> usize {
        self.rru(rs1).wrapping_add_signed(imm) as usize
    }

    fn parse_inst(&self) -> anyhow::Result<Instruction> {
        let word = self.memory.read_u32(self.pc)?;
        Instruction::try_from(word)
    }

    pub fn exec_inst(&mut self, inst: &Instruction) -> anyhow::Result<RetCode> {
        use Instruction::*;
        match inst {
            Add { rd, rs1, rs2 } => {
                self.wr(*rd, self.rr(*rs1).wrapping_add(self.rr(*rs2)));
                self.advance_pc();
            }
            Sub { rd, rs1, rs2 } => {
                self.wr(*rd, self.rr(*rs1).wrapping_sub(self.rr(*rs2)));
                self.advance_pc();
            }
            Xor { rd, rs1, rs2 } => {
                self.wr(*rd, self.rr(*rs1) ^ self.rr(*rs2));
                self.advance_pc();
            }
            Or { rd, rs1, rs2 } => {
                self.wr(*rd, self.rr(*rs1) | self.rr(*rs2));
                self.advance_pc();
            }
            And { rd, rs1, rs2 } => {
                self.wr(*rd, self.rr(*rs1) & self.rr(*rs2));
                self.advance_pc();
            }
            Sll { rd, rs1, rs2 } => {
                self.wr(*rd, self.rr(*rs1) << (self.rru(*rs2) & 0x3f));
                self.advance_pc();
            }
            Srl { rd, rs1, rs2 } => {
                self.wr(*rd, (self.rru(*rs1) >> (self.rru(*rs2) & 0x3f)) as i64);
                self.advance_pc();
            }
            Sra { rd, rs1, rs2 } => {
                self.wr(*rd, self.rr(*rs1) >> (self.rru(*rs2) & 0x3f));
                self.advance_pc();
            }
            Slt { rd, rs1, rs2 } => {
                if self.rr(*rs1) < self.rr(*rs2) {
                    self.wr(*rd, 1);
                } else {
                    self.wr(*rd, 0);
                }
                self.advance_pc();
            }
            Sltu { rd, rs1, rs2 } => {
                if self.rru(*rs1) < self.rru(*rs2) {
                    self.wr(*rd, 1);
                } else {
                    self.wr(*rd, 0);
                }
                self.advance_pc();
            }
            Mul { rd, rs1, rs2 } => {
                let rs1_val = self.rr(*rs1);
                let rs2_val = self.rr(*rs2);
                let rd_val = rs1_val.wrapping_mul(rs2_val);
                self.wr(*rd, rd_val);
                self.advance_pc();
            }
            Mulh { rd, rs1, rs2 } => {
                let rs1_val = self.rr(*rs1) as i128;
                let rs2_val = self.rr(*rs2) as i128;
                let rd_val = ((rs1_val * rs2_val) >> 64) as i64;
                self.wr(*rd, rd_val);
                self.advance_pc();
            }
            Mulhsu { rd, rs1, rs2 } => {
                let rs1_val = self.rr(*rs1) as i128;
                let rs2_val = self.rru(*rs2) as i128;
                let rd_val = ((rs1_val * rs2_val) >> 64) as i64;
                self.wr(*rd, rd_val);
                self.advance_pc();
            }
            Mulhu { rd, rs1, rs2 } => {
                let rs1_val = self.rru(*rs1) as u128;
                let rs2_val = self.rru(*rs2) as u128;
                let rd_val = ((rs1_val * rs2_val) >> 64) as u64;
                self.wru(*rd, rd_val);
                self.advance_pc();
            }
            Div { rd, rs1, rs2 } => {
                let rd_val = self.rr(*rs1).wrapping_div(self.rr(*rs2));
                self.wr(*rd, rd_val);
                self.advance_pc();
            }
            Divu { rd, rs1, rs2 } => {
                let rd_val = self.rru(*rs1) / self.rru(*rs2);
                self.wru(*rd, rd_val);
                self.advance_pc();
            }
            Rem { rd, rs1, rs2 } => {
                let rd_val = self.rr(*rs1).wrapping_rem(self.rr(*rs2));
                self.wr(*rd, rd_val);
                self.advance_pc();
            }
            Remu { rd, rs1, rs2 } => {
                let rd_val = self.rru(*rs1) % self.rru(*rs2);
                self.wru(*rd, rd_val);
                self.advance_pc();
            }
            Addw { rd, rs1, rs2 } => {
                let rd_val = self.rrw(*rs1).wrapping_add(self.rrw(*rs2));
                self.wrw(*rd, rd_val);
                self.advance_pc();
            }
            Subw { rd, rs1, rs2 } => {
                let rd_val = self.rrw(*rs1).wrapping_sub(self.rrw(*rs2));
                self.wrw(*rd, rd_val);
                self.advance_pc();
            }
            Sllw { rd, rs1, rs2 } => {
                let rd_val = self.rrw(*rs1).shl(self.rrw(*rs2) & 0x1f);
                self.wrw(*rd, rd_val);
                self.advance_pc();
            }
            Srlw { rd, rs1, rs2 } => {
                let rd_val = self.rruw(*rs1).shr(self.rruw(*rs2) & 0x1f);
                self.wrw(*rd, rd_val as i32);
                self.advance_pc();
            }
            Sraw { rd, rs1, rs2 } => {
                let rd_val = self.rrw(*rs1).shr(self.rrw(*rs2) & 0x1f);
                self.wrw(*rd, rd_val);
                self.advance_pc();
            }
            Mulw { rd, rs1, rs2 } => {
                let rd_val = self.rrw(*rs1).wrapping_mul(self.rrw(*rs2));
                self.wrw(*rd, rd_val);
                self.advance_pc();
            }
            Divw { rd, rs1, rs2 } => {
                let rd_val = self.rrw(*rs1).wrapping_div(self.rrw(*rs2));
                self.wrw(*rd, rd_val);
                self.advance_pc();
            }
            Divuw { rd, rs1, rs2 } => {
                let rd_val = self.rruw(*rs1) / self.rruw(*rs2);
                self.wrw(*rd, rd_val as i32);
                self.advance_pc();
            }
            Remw { rd, rs1, rs2 } => {
                let rd_val = self.rrw(*rs1).wrapping_rem(self.rrw(*rs2));
                self.wrw(*rd, rd_val);
                self.advance_pc();
            }
            Remuw { rd, rs1, rs2 } => {
                let rd_val = self.rruw(*rs1).wrapping_rem(self.rruw(*rs2));
                self.wrw(*rd, rd_val as i32);
                self.advance_pc();
            }
            Addi { rd, rs1, imm } => {
                self.wr(*rd, self.rr(*rs1).wrapping_add(*imm));
                self.advance_pc();
            }
            Xori { rd, rs1, imm } => {
                self.wr(*rd, self.rr(*rs1) ^ *imm);
                self.advance_pc();
            }
            Ori { rd, rs1, imm } => {
                self.wr(*rd, self.rr(*rs1) | *imm);
                self.advance_pc();
            }
            Andi { rd, rs1, imm } => {
                self.wr(*rd, self.rr(*rs1) & *imm);
                self.advance_pc();
            }
            Slli { rd, rs1, imm } => {
                self.wru(*rd, self.rru(*rs1).wrapping_shl(*imm));
                self.advance_pc();
            }
            Srli { rd, rs1, imm } => {
                self.wru(*rd, self.rru(*rs1).wrapping_shr(*imm));
                self.advance_pc();
            }
            Srai { rd, rs1, imm } => {
                self.wr(*rd, self.rr(*rs1).wrapping_shr(*imm));
                self.advance_pc();
            }
            Slti { rd, rs1, imm } => {
                self.wr(*rd, if self.rr(*rs1) < *imm { 1 } else { 0 });
                self.advance_pc();
            }
            Sltiu { rd, rs1, imm } => {
                self.wr(*rd, if self.rru(*rs1) < *imm { 1 } else { 0 });
                self.advance_pc();
            }
            Addiw { rd, rs1, imm } => {
                let rs1_val = self.rr(*rs1) as i32;
                let rd_val = rs1_val.wrapping_add(*imm as i32);
                self.wr(*rd, rd_val as i64);
                self.advance_pc();
            }
            Slliw { rd, rs1, imm } => {
                let rs1_val = self.rr(*rs1) as i32;
                let rd_val = rs1_val.shl(*imm & 0x1f);
                self.wr(*rd, rd_val as i64);
                self.advance_pc();
            }
            Srliw { rd, rs1, imm } => {
                let rs1_val = self.rr(*rs1) as u32;
                let rd_val = rs1_val.shr(*imm & 0x1f);
                self.wr(*rd, rd_val as i32 as i64);
                self.advance_pc();
            }
            Sraiw { rd, rs1, imm } => {
                let rs1_val = self.rr(*rs1) as i32;
                let rd_val = rs1_val.shr(*imm & 0x1f);
                self.wr(*rd, rd_val as i64);
                self.advance_pc();
            }
            Lb { rd, rs1, imm } => {
                let addr = self.effective_addr(*rs1, *imm);
                let byte = self.memory.read_u8(addr)?;
                self.wr(*rd, byte as i8 as i64);
                self.advance_pc();
            }
            Lh { rd, rs1, imm } => {
                let addr = self.effective_addr(*rs1, *imm);
                let hw = self.memory.read_u16(addr)?;
                self.wr(*rd, hw as i16 as i64);
                self.advance_pc();
            }
            Lw { rd, rs1, imm } => {
                let addr = self.effective_addr(*rs1, *imm);
                let wd = self.memory.read_u32(addr)?;
                self.wr(*rd, wd as i32 as i64);
                self.advance_pc();
            }
            Ld { rd, rs1, imm } => {
                let addr = self.effective_addr(*rs1, *imm);
                let dd = self.memory.read_u64(addr)?;
                self.wru(*rd, dd);
                self.advance_pc();
            }
            Lbu { rd, rs1, imm } => {
                let addr = self.effective_addr(*rs1, *imm);
                self.wru(*rd, self.memory.read_u8(addr)? as u64);
                self.advance_pc();
            }
            Lhu { rd, rs1, imm } => {
                let addr = self.effective_addr(*rs1, *imm);
                self.wru(*rd, self.memory.read_u16(addr)? as u64);
                self.advance_pc();
            }
            Lwu { rd, rs1, imm } => {
                let addr = self.effective_addr(*rs1, *imm);
                self.wru(*rd, self.memory.read_u32(addr)? as u64);
                self.advance_pc();
            }
            Sb { rs2, imm, rs1 } => {
                let addr = self.effective_addr(*rs1, *imm);
                let byte = self.rr(*rs2) as u8;
                self.memory.write_u8(addr, byte)?;
                self.advance_pc();
            }
            Sh { rs2, imm, rs1 } => {
                let addr = self.effective_addr(*rs1, *imm);
                let val = self.rr(*rs2) as u16;
                self.memory.write_slice(addr, &val.to_le_bytes())?;
                self.advance_pc();
            }
            Sw { rs2, imm, rs1 } => {
                let addr = self.effective_addr(*rs1, *imm);
                let val = self.rr(*rs2) as u32;
                self.memory.write_slice(addr, &val.to_le_bytes())?;
                self.advance_pc();
            }
            Sd { rs2, imm, rs1 } => {
                let addr = self.effective_addr(*rs1, *imm);
                let val = self.rr(*rs2);
                self.memory.write_slice(addr, &val.to_le_bytes())?;
                self.advance_pc();
            }
            Beq { rs1, rs2, imm } => {
                if self.rr(*rs1) == self.rr(*rs2) {
                    self.jump(*imm);
                } else {
                    self.advance_pc();
                }
            }
            Bne { rs1, rs2, imm } => {
                if self.rr(*rs1) != self.rr(*rs2) {
                    self.jump(*imm);
                } else {
                    self.advance_pc();
                }
            }
            Blt { rs1, rs2, imm } => {
                if self.rr(*rs1) < self.rr(*rs2) {
                    self.jump(*imm);
                } else {
                    self.advance_pc();
                }
            }
            Bge { rs1, rs2, imm } => {
                if self.rr(*rs1) >= self.rr(*rs2) {
                    self.jump(*imm);
                } else {
                    self.advance_pc();
                }
            }
            Bltu { rs1, rs2, imm } => {
                if self.rru(*rs1) < self.rru(*rs2) {
                    self.jump(*imm);
                } else {
                    self.advance_pc();
                }
            }
            Bgeu { rs1, rs2, imm } => {
                if self.rru(*rs1) >= self.rru(*rs2) {
                    self.jump(*imm);
                } else {
                    self.advance_pc();
                }
            }
            Jal { rd, imm } => {
                self.wru(*rd, (self.pc as u64).wrapping_add(4));
                self.jump(*imm);
            }
            Jalr { rd, rs1, imm } => {
                let target = self.effective_addr(*rs1, *imm) & !1;
                self.wru(*rd, (self.pc as u64).wrapping_add(4));
                self.pc = target;
            }
            Lui { rd, imm } => {
                self.wr(*rd, *imm);
                self.advance_pc();
            }
            Auipc { rd, imm } => {
                let val = (self.pc as u64).wrapping_add_signed(*imm);
                self.wru(*rd, val);
                self.advance_pc();
            }
            Ecall => bail!("ecall not supported"),
            Ebreak => return Ok(RetCode::Halt),
            Fence => self.advance_pc(),
        }
        Ok(RetCode::Cont)
    }

    pub fn step(&mut self) -> anyhow::Result<RetCode> {
        self.exec_inst(&self.parse_inst()?)
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        loop {
            if matches!(self.step()?, RetCode::Halt) {
                return Ok(());
            }
        }
    }

    pub fn dump_reg(&self) {
        for i in 0..32 {
            if i % 4 == 0 {
                print!("{:02}: ", i);
            }
            print!("{:016x} ", self.register[i]);
            if i % 4 == 3 {
                println!();
            }
        }
    }

    pub fn debug(&self) {
        self.dump_reg();
    }
}

#[cfg(test)]
mod inst_tests {
    use super::{Instruction::*, Machine};

    #[test]
    fn store_halfword_writes_two_bytes() {
        let mut machine = Machine::new(16);
        machine.register[1] = 4;
        machine.register[2] = 0x1234_5678;

        machine
            .exec_inst(&Sh {
                rs2: 2,
                imm: 0,
                rs1: 1,
            })
            .unwrap();

        assert_eq!(machine.memory.slice_range(4..6).unwrap(), &[0x78, 0x56]);
        assert_eq!(machine.pc, 4);
    }

    #[test]
    fn store_word_writes_four_bytes() {
        let mut machine = Machine::new(16);
        machine.register[1] = 4;
        machine.register[2] = 0x1234_5678;

        machine
            .exec_inst(&Sw {
                rs2: 2,
                imm: 4,
                rs1: 1,
            })
            .unwrap();

        assert_eq!(
            &machine.memory.slice_range(8..12).unwrap(),
            &[0x78, 0x56, 0x34, 0x12]
        );
        assert_eq!(machine.pc, 4);
    }

    #[test]
    fn sll_uses_only_the_low_six_shift_bits() {
        let mut machine = Machine::new(16);
        machine.register[1] = 0x8000_0000_0000_0001_u64 as i64;
        machine.register[2] = 0x41;
        machine
            .exec_inst(&Sll {
                rd: 3,
                rs1: 1,
                rs2: 2,
            })
            .unwrap();
        assert_eq!(machine.rru(3), 2);
    }

    #[test]
    fn srl_uses_only_the_low_six_shift_bits() {
        let mut machine = Machine::new(16);
        machine.register[1] = 0x8000_0000_0000_0001_u64 as i64;
        machine.register[2] = 0x41;
        machine
            .exec_inst(&Srl {
                rd: 4,
                rs1: 1,
                rs2: 2,
            })
            .unwrap();
        assert_eq!(machine.rru(4), 0x4000_0000_0000_0000);
    }

    #[test]
    fn sra_uses_only_the_low_six_shift_bits() {
        let mut machine = Machine::new(16);
        machine.register[1] = 0x8000_0000_0000_0001_u64 as i64;
        machine.register[2] = 0x41;
        machine
            .exec_inst(&Sra {
                rd: 5,
                rs1: 1,
                rs2: 2,
            })
            .unwrap();
        assert_eq!(machine.rr(5), 0xc000_0000_0000_0000_u64 as i64);
    }

    #[test]
    fn jalr_reads_source_before_link_write_and_clears_bit_zero() {
        let mut machine = Machine::new(16);
        machine.register[1] = 101;

        machine
            .exec_inst(&Jalr {
                rd: 1,
                rs1: 1,
                imm: 2,
            })
            .unwrap();

        assert_eq!(machine.register[1], 4);
        assert_eq!(machine.pc, 102);
    }

    #[test]
    fn memory_addresses_wrap_as_rv32_addresses() {
        let mut machine = Machine::new(8);
        machine.register[1] = -1;
        machine.register[2] = 0x5a;

        machine
            .exec_inst(&Sb {
                rs2: 2,
                imm: 1,
                rs1: 1,
            })
            .unwrap();

        assert_eq!(machine.memory.read_u8(0).unwrap(), 0x5a);
    }

    #[test]
    fn out_of_bounds_store_returns_error_without_advancing_pc() {
        let mut machine = Machine::new(4);
        machine.register[1] = 3;
        machine.register[2] = 0x1234;

        assert!(
            machine
                .exec_inst(&Sh {
                    rs2: 2,
                    imm: 0,
                    rs1: 1
                })
                .is_err()
        );
        assert_eq!(machine.pc, 0);
    }

    #[test]
    fn out_of_bounds_load_returns_error_without_advancing_pc() {
        let mut machine = Machine::new(4);
        machine.register[1] = 3;
        assert!(
            machine
                .exec_inst(&Lw {
                    rd: 3,
                    rs1: 1,
                    imm: 0
                })
                .is_err()
        );
        assert_eq!(machine.pc, 0);
    }

    #[test]
    fn instruction_fetch_out_of_bounds_returns_error() {
        let mut machine = Machine::new(4);
        machine.pc = 4;
        assert!(machine.parse_inst().is_err());
    }

    #[test]
    fn fence_is_a_supported_no_op() {
        let mut machine = Machine::new(4);
        machine.exec_inst(&Fence).unwrap();
        assert_eq!(machine.pc, 4);
    }
}

#[cfg(test)]
mod display_tests {
    use super::Instruction::*;

    #[test]
    fn formats_riscv_assembly_operands() {
        assert_eq!(
            Add {
                rd: 1,
                rs1: 2,
                rs2: 3
            }
            .to_string(),
            "add x1, x2, x3"
        );
        assert_eq!(
            Lw {
                rd: 5,
                rs1: 6,
                imm: -12
            }
            .to_string(),
            "lw x5, -12(x6)"
        );
        assert_eq!(
            Sw {
                rs2: 7,
                imm: 16,
                rs1: 8
            }
            .to_string(),
            "sw x7, 16(x8)"
        );
        assert_eq!(
            Sltiu {
                rd: 9,
                rs1: 10,
                imm: 0xffff_f800
            }
            .to_string(),
            "sltiu x9, x10, -2048"
        );
        assert_eq!(
            Lui {
                rd: 11,
                imm: 0x1234_5000
            }
            .to_string(),
            "lui x11, 0x12345"
        );
        assert_eq!(Ecall.to_string(), "ecall");
    }

    #[test]
    fn displays_decoded_instructions() {
        let instruction = super::Instruction::try_from(0xfe1f_f0ef).unwrap();
        assert_eq!(instruction.to_string(), "jal x1, -32");
    }
}
