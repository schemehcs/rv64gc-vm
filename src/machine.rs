mod instruction;
mod memory;
mod register;

use instruction::Instruction;
use memory::Memory;
use register::Register;

use anyhow::{Ok, bail};

pub enum RetCode {
    Cont,
    Halt,
}

enum Mode {
    User = 0,
    Supervisor = 1,
    Machine = 3,
}

pub struct Machine {
    mode: Mode,
    register: Register,
    memory: Memory,
    pc: usize,
}

impl Machine {
    pub fn new(memory_size: usize) -> Self {
        Self {
            mode: Mode::Machine,
            register: Register::new(),
            memory: Memory::new(memory_size),
            pc: 0,
        }
    }

    pub fn load_bin(&mut self, addr: usize, bin: &[u8]) -> anyhow::Result<()> {
        self.memory.store_slice(addr, bin)
    }

    pub fn call_r<F: Fn(i64, i64) -> i64>(&mut self, rd: usize, rs1: usize, rs2: usize, f: F) {
        let (rs1_val, rs2_val) = self.register.read_pair(rs1, rs2);
        let rd_val = f(rs1_val, rs2_val);
        self.register.write(rd, rd_val);
    }

    pub fn call_ru<F: Fn(u64, u64) -> u64>(&mut self, rd: usize, rs1: usize, rs2: usize, f: F) {
        let (rs1_val, rs2_val) = self.register.read_pairu(rs1, rs2);
        let rd_val = f(rs1_val, rs2_val);
        self.register.write_u(rd, rd_val);
    }

    pub fn call_rw<F: Fn(i32, i32) -> i32>(&mut self, rd: usize, rs1: usize, rs2: usize, f: F) {
        let (rs1_val, rs2_val) = self.register.read_pairw(rs1, rs2);
        let rd_val = f(rs1_val, rs2_val);
        self.register.write_w(rd, rd_val);
    }

    pub fn call_rwu<F: Fn(u32, u32) -> u32>(&mut self, rd: usize, rs1: usize, rs2: usize, f: F) {
        let (rs1_val, rs2_val) = self.register.read_pairwu(rs1, rs2);
        let rd_val = f(rs1_val, rs2_val);
        self.register.write_wu(rd, rd_val);
    }

    pub fn call_i<F: Fn(i64, i64) -> i64>(&mut self, rd: usize, rs1: usize, imm: i64, f: F) {
        let rs1_val = self.register.read(rs1);
        let rd_val = f(rs1_val, imm);
        self.register.write(rd, rd_val);
    }

    pub fn call_iw<F: Fn(i32, i32) -> i32>(&mut self, rd: usize, rs1: usize, imm: i32, f: F) {
        let rs1_val = self.register.read_w(rs1);
        let rd_val = f(rs1_val, imm);
        self.register.write_w(rd, rd_val);
    }

    pub fn call_iu<F: Fn(u64, u64) -> u64>(&mut self, rd: usize, rs1: usize, imm: u64, f: F) {
        let rs1_val = self.register.read_u(rs1);
        let rd_val = f(rs1_val, imm);
        self.register.write_u(rd, rd_val);
    }

    pub fn call_iwu<F: Fn(u32, u32) -> u32>(&mut self, rd: usize, rs1: usize, imm: u32, f: F) {
        let rs1_val = self.register.read_wu(rs1);
        let rd_val = f(rs1_val, imm);
        self.register.write_wu(rd, rd_val);
    }

    fn advance_pc(&mut self) {
        self.pc = self.pc.wrapping_add(4);
    }

    fn jump(&mut self, offset: i64) {
        self.pc = self.pc.wrapping_add_signed(offset as isize);
    }

    fn effective_addr(&self, rs1: usize, imm: i64) -> usize {
        self.register.read_u(rs1).wrapping_add_signed(imm) as usize
    }

    fn div_i64(dividend: i64, divisor: i64) -> i64 {
        if divisor == 0 {
            -1
        } else if dividend == i64::MIN && divisor == -1 {
            i64::MIN
        } else {
            dividend / divisor
        }
    }

    fn div_u64(dividend: u64, divisor: u64) -> u64 {
        dividend.checked_div(divisor).unwrap_or(u64::MAX)
    }

    fn rem_i64(dividend: i64, divisor: i64) -> i64 {
        if divisor == 0 {
            dividend
        } else if dividend == i64::MIN && divisor == -1 {
            0
        } else {
            dividend % divisor
        }
    }

    fn rem_u64(dividend: u64, divisor: u64) -> u64 {
        dividend.checked_div(divisor).unwrap_or(dividend)
    }

    fn div_i32(dividend: i32, divisor: i32) -> i32 {
        if divisor == 0 {
            -1
        } else if dividend == i32::MIN && divisor == -1 {
            i32::MIN
        } else {
            dividend / divisor
        }
    }

    fn div_u32(dividend: u32, divisor: u32) -> u32 {
        dividend.checked_div(divisor).unwrap_or(u32::MAX)
    }

    fn rem_i32(dividend: i32, divisor: i32) -> i32 {
        if divisor == 0 {
            dividend
        } else if dividend == i32::MIN && divisor == -1 {
            0
        } else {
            dividend % divisor
        }
    }

    fn rem_u32(dividend: u32, divisor: u32) -> u32 {
        dividend.checked_rem(divisor).unwrap_or(dividend)
    }

    fn parse_ir(&self) -> anyhow::Result<Instruction> {
        let word = self.memory.load_u32(self.pc)?;
        Instruction::try_from(word)
    }

    pub fn execute_ir(&mut self, inst: &Instruction) -> anyhow::Result<RetCode> {
        use Instruction::*;
        match inst {
            Add { rd, rs1, rs2 } => {
                self.call_r(*rd, *rs1, *rs2, i64::wrapping_add);
                self.advance_pc();
            }
            Sub { rd, rs1, rs2 } => {
                self.call_r(*rd, *rs1, *rs2, i64::wrapping_sub);
                self.advance_pc();
            }
            Xor { rd, rs1, rs2 } => {
                self.call_r(*rd, *rs1, *rs2, |x, y| x ^ y);
                self.advance_pc();
            }
            Or { rd, rs1, rs2 } => {
                self.call_r(*rd, *rs1, *rs2, |x, y| x | y);
                self.advance_pc();
            }
            And { rd, rs1, rs2 } => {
                self.call_r(*rd, *rs1, *rs2, |x, y| x & y);
                self.advance_pc();
            }
            Sll { rd, rs1, rs2 } => {
                self.call_r(*rd, *rs1, *rs2, |x, y| x << (y & 0x3f));
                self.advance_pc();
            }
            Srl { rd, rs1, rs2 } => {
                self.call_ru(*rd, *rs1, *rs2, |x, y| x >> (y & 0x3f));
                self.advance_pc();
            }
            Sra { rd, rs1, rs2 } => {
                self.call_r(*rd, *rs1, *rs2, |x, y| x >> (y & 0x3f));
                self.advance_pc();
            }
            Slt { rd, rs1, rs2 } => {
                self.call_r(*rd, *rs1, *rs2, |x, y| if x < y { 1 } else { 0 });
                self.advance_pc();
            }
            Sltu { rd, rs1, rs2 } => {
                self.call_ru(*rd, *rs1, *rs2, |x, y| if x < y { 1 } else { 0 });
                self.advance_pc();
            }
            Mul { rd, rs1, rs2 } => {
                self.call_r(*rd, *rs1, *rs2, i64::wrapping_mul);
                self.advance_pc();
            }
            Mulh { rd, rs1, rs2 } => {
                self.call_r(*rd, *rs1, *rs2, |x, y| {
                    ((x as i128 * y as i128) >> 64) as i64
                });
                self.advance_pc();
            }
            Mulhsu { rd, rs1, rs2 } => {
                self.call_r(*rd, *rs1, *rs2, |x, y| {
                    ((x as i128 * y as i128) >> 64) as i64
                });
                self.advance_pc();
            }
            Mulhu { rd, rs1, rs2 } => {
                self.call_ru(*rd, *rs1, *rs2, |x, y| {
                    ((x as u128 * y as u128) >> 64) as u64
                });
                self.advance_pc();
            }
            Div { rd, rs1, rs2 } => {
                self.call_r(*rd, *rs1, *rs2, Self::div_i64);
                self.advance_pc();
            }
            Divu { rd, rs1, rs2 } => {
                self.call_ru(*rd, *rs1, *rs2, Self::div_u64);
                self.advance_pc();
            }
            Rem { rd, rs1, rs2 } => {
                self.call_r(*rd, *rs1, *rs2, Self::rem_i64);
                self.advance_pc();
            }
            Remu { rd, rs1, rs2 } => {
                self.call_ru(*rd, *rs1, *rs2, Self::rem_u64);
                self.advance_pc();
            }
            Addw { rd, rs1, rs2 } => {
                self.call_rw(*rd, *rs1, *rs2, i32::wrapping_add);
                self.advance_pc();
            }
            Subw { rd, rs1, rs2 } => {
                self.call_rw(*rd, *rs1, *rs2, i32::wrapping_sub);
                self.advance_pc();
            }
            Sllw { rd, rs1, rs2 } => {
                self.call_rw(*rd, *rs1, *rs2, |x, y| x << (y & 0x1f));
                self.advance_pc();
            }
            Srlw { rd, rs1, rs2 } => {
                self.call_rwu(*rd, *rs1, *rs2, |x, y| x >> (y & 0x1f));
                self.advance_pc();
            }
            Sraw { rd, rs1, rs2 } => {
                self.call_rw(*rd, *rs1, *rs2, |x, y| x >> (y & 0x1f));
                self.advance_pc();
            }
            Mulw { rd, rs1, rs2 } => {
                self.call_rw(*rd, *rs1, *rs2, i32::wrapping_mul);
                self.advance_pc();
            }
            Divw { rd, rs1, rs2 } => {
                self.call_rw(*rd, *rs1, *rs2, Self::div_i32);
                self.advance_pc();
            }
            Divuw { rd, rs1, rs2 } => {
                self.call_rwu(*rd, *rs1, *rs2, Self::div_u32);
                self.advance_pc();
            }
            Remw { rd, rs1, rs2 } => {
                self.call_rw(*rd, *rs1, *rs2, Self::rem_i32);
                self.advance_pc();
            }
            Remuw { rd, rs1, rs2 } => {
                self.call_rwu(*rd, *rs1, *rs2, Self::rem_u32);
                self.advance_pc();
            }
            Addi { rd, rs1, imm } => {
                self.call_i(*rd, *rs1, *imm, i64::wrapping_add);
                self.advance_pc();
            }
            Xori { rd, rs1, imm } => {
                self.call_i(*rd, *rs1, *imm, |x, y| x ^ y);
                self.advance_pc();
            }
            Ori { rd, rs1, imm } => {
                self.call_i(*rd, *rs1, *imm, |x, y| x | y);
                self.advance_pc();
            }
            Andi { rd, rs1, imm } => {
                self.call_i(*rd, *rs1, *imm, |x, y| x & y);
                self.advance_pc();
            }
            Slli { rd, rs1, imm } => {
                self.call_iu(*rd, *rs1, *imm as u64, |x, y| x << (y & 0x3f));
                self.advance_pc();
            }
            Srli { rd, rs1, imm } => {
                self.call_iu(*rd, *rs1, *imm as u64, |x, y| x >> (y & 0x3f));
                self.advance_pc();
            }
            Srai { rd, rs1, imm } => {
                self.call_i(*rd, *rs1, *imm as i64, |x, y| x >> (y & 0x3f));
                self.advance_pc();
            }
            Slti { rd, rs1, imm } => {
                self.call_i(*rd, *rs1, *imm, |x, y| if x < y { 1 } else { 0 });
                self.advance_pc();
            }
            Sltiu { rd, rs1, imm } => {
                self.call_iu(*rd, *rs1, *imm, |x, y| if x < y { 1 } else { 0 });
                self.advance_pc();
            }
            Addiw { rd, rs1, imm } => {
                self.call_iw(*rd, *rs1, *imm, i32::wrapping_add);
                self.advance_pc();
            }
            Slliw { rd, rs1, imm } => {
                self.call_iw(*rd, *rs1, *imm, |x, y| x << (y & 0x1f));
                self.advance_pc();
            }
            Srliw { rd, rs1, imm } => {
                self.call_iwu(*rd, *rs1, *imm, |x, y| x >> (y & 0x1f));
                self.advance_pc();
            }
            Sraiw { rd, rs1, imm } => {
                self.call_iw(*rd, *rs1, *imm, |x, y| x >> (y & 0x1f));
                self.advance_pc();
            }
            Lb { rd, rs1, imm } => {
                let addr = self.effective_addr(*rs1, *imm);
                let byte = self.memory.load_u8(addr)?;
                self.register.write(*rd, byte as i8 as i64);
                self.advance_pc();
            }
            Lh { rd, rs1, imm } => {
                let addr = self.effective_addr(*rs1, *imm);
                let hw = self.memory.load_u16(addr)?;
                self.register.write(*rd, hw as i16 as i64);
                self.advance_pc();
            }
            Lw { rd, rs1, imm } => {
                let addr = self.effective_addr(*rs1, *imm);
                let wd = self.memory.load_u32(addr)?;
                self.register.write(*rd, wd as i32 as i64);
                self.advance_pc();
            }
            Ld { rd, rs1, imm } => {
                let addr = self.effective_addr(*rs1, *imm);
                let dd = self.memory.load_u64(addr)?;
                self.register.write_u(*rd, dd);
                self.advance_pc();
            }
            Lbu { rd, rs1, imm } => {
                let addr = self.effective_addr(*rs1, *imm);
                let byte = self.memory.load_u8(addr)?;
                self.register.write_u(*rd, byte as u64);
                self.advance_pc();
            }
            Lhu { rd, rs1, imm } => {
                let addr = self.effective_addr(*rs1, *imm);
                let hw = self.memory.load_u16(addr)?;
                self.register.write_u(*rd, hw as u64);
                self.advance_pc();
            }
            Lwu { rd, rs1, imm } => {
                let addr = self.effective_addr(*rs1, *imm);
                let wd = self.memory.load_u32(addr)?;
                self.register.write_u(*rd, wd as u64);
                self.advance_pc();
            }
            Sb { rs2, imm, rs1 } => {
                let addr = self.effective_addr(*rs1, *imm);
                let byte = self.register.read(*rs2) as u8;
                self.memory.store_u8(addr, byte)?;
                self.advance_pc();
            }
            Sh { rs2, imm, rs1 } => {
                let addr = self.effective_addr(*rs1, *imm);
                let val = self.register.read(*rs2) as u16;
                self.memory.store_slice(addr, &val.to_le_bytes())?;
                self.advance_pc();
            }
            Sw { rs2, imm, rs1 } => {
                let addr = self.effective_addr(*rs1, *imm);
                let val = self.register.read(*rs2) as u32;
                self.memory.store_slice(addr, &val.to_le_bytes())?;
                self.advance_pc();
            }
            Sd { rs2, imm, rs1 } => {
                let addr = self.effective_addr(*rs1, *imm);
                let val = self.register.read(*rs2);
                self.memory.store_slice(addr, &val.to_le_bytes())?;
                self.advance_pc();
            }
            Beq { rs1, rs2, imm } => {
                let (rs1_val, rs2_val) = self.register.read_pair(*rs1, *rs2);
                if rs1_val == rs2_val {
                    self.jump(*imm);
                } else {
                    self.advance_pc();
                }
            }
            Bne { rs1, rs2, imm } => {
                let (rs1_val, rs2_val) = self.register.read_pair(*rs1, *rs2);
                if rs1_val != rs2_val {
                    self.jump(*imm);
                } else {
                    self.advance_pc();
                }
            }
            Blt { rs1, rs2, imm } => {
                let (rs1_val, rs2_val) = self.register.read_pair(*rs1, *rs2);
                if rs1_val < rs2_val {
                    self.jump(*imm);
                } else {
                    self.advance_pc();
                }
            }
            Bge { rs1, rs2, imm } => {
                let (rs1_val, rs2_val) = self.register.read_pair(*rs1, *rs2);
                if rs1_val >= rs2_val {
                    self.jump(*imm);
                } else {
                    self.advance_pc();
                }
            }
            Bltu { rs1, rs2, imm } => {
                let (rs1_val, rs2_val) = self.register.read_pair(*rs1, *rs2);
                if rs1_val < rs2_val {
                    self.jump(*imm);
                } else {
                    self.advance_pc();
                }
            }
            Bgeu { rs1, rs2, imm } => {
                let (rs1_val, rs2_val) = self.register.read_pairu(*rs1, *rs2);
                if rs1_val >= rs2_val {
                    self.jump(*imm);
                } else {
                    self.advance_pc();
                }
            }
            Jal { rd, imm } => {
                let next_pc = (self.pc as u64).wrapping_add(4);
                self.register.write_u(*rd, next_pc);
                self.jump(*imm);
            }
            Jalr { rd, rs1, imm } => {
                let target = self.effective_addr(*rs1, *imm) & !1;
                let next_pc = (self.pc as u64).wrapping_add(4);
                self.register.write_u(*rd, next_pc);
                self.pc = target;
            }
            Lui { rd, imm } => {
                self.register.write(*rd, *imm);
                self.advance_pc();
            }
            Auipc { rd, imm } => {
                let target = (self.pc as u64).wrapping_add_signed(*imm);
                self.register.write_u(*rd, target);
                self.advance_pc();
            }
            Ecall => bail!("ecall not supported"),
            Ebreak => return Ok(RetCode::Halt),
            Fence => self.advance_pc(),
            _ => todo!(),
        }
        Ok(RetCode::Cont)
    }

    pub fn step(&mut self) -> anyhow::Result<RetCode> {
        self.execute_ir(&self.parse_ir()?)
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
            print!("{:016x} ", self.register.read(i));
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
mod execute_ir_tests {
    use super::{Instruction::*, Machine};

    #[test]
    fn store_halfword_writes_two_bytes() {
        let mut machine = Machine::new(16);
        machine.register.write(1, 4);
        machine.register.write(2, 0x1234_5678);

        machine
            .execute_ir(&Sh {
                rs2: 2,
                imm: 0,
                rs1: 1,
            })
            .unwrap();

        assert_eq!(machine.memory.slice(4, 2).unwrap(), &[0x78, 0x56]);
        assert_eq!(machine.pc, 4);
    }

    #[test]
    fn store_word_writes_four_bytes() {
        let mut machine = Machine::new(16);
        machine.register.write(1, 4);
        machine.register.write(2, 0x1234_5678);

        machine
            .execute_ir(&Sw {
                rs2: 2,
                imm: 4,
                rs1: 1,
            })
            .unwrap();

        assert_eq!(
            &machine.memory.slice(8, 4).unwrap(),
            &[0x78, 0x56, 0x34, 0x12]
        );
        assert_eq!(machine.pc, 4);
    }

    #[test]
    fn sll_uses_only_the_low_six_shift_bits() {
        let mut machine = Machine::new(16);
        machine.register.write(1, 0x8000_0000_0000_0001_u64 as i64);
        machine.register.write(2, 0x41);
        machine
            .execute_ir(&Sll {
                rd: 3,
                rs1: 1,
                rs2: 2,
            })
            .unwrap();
        assert_eq!(machine.register.read_u(3), 2);
    }

    #[test]
    fn srl_uses_only_the_low_six_shift_bits() {
        let mut machine = Machine::new(16);
        machine.register.write(1, 0x8000_0000_0000_0001_u64 as i64);
        machine.register.write(2, 0x41);
        machine
            .execute_ir(&Srl {
                rd: 4,
                rs1: 1,
                rs2: 2,
            })
            .unwrap();
        assert_eq!(machine.register.read_u(4), 0x4000_0000_0000_0000);
    }

    #[test]
    fn sra_uses_only_the_low_six_shift_bits() {
        let mut machine = Machine::new(16);
        machine.register.write(1, 0x8000_0000_0000_0001_u64 as i64);
        machine.register.write(2, 0x41);
        machine
            .execute_ir(&Sra {
                rd: 5,
                rs1: 1,
                rs2: 2,
            })
            .unwrap();
        assert_eq!(machine.register.read(5), 0xc000_0000_0000_0000_u64 as i64);
    }

    #[test]
    fn jalr_reads_source_before_link_write_and_clears_bit_zero() {
        let mut machine = Machine::new(16);
        machine.register.write(1, 101);
        machine
            .execute_ir(&Jalr {
                rd: 1,
                rs1: 1,
                imm: 2,
            })
            .unwrap();

        assert_eq!(machine.register.read(1), 4);
        assert_eq!(machine.pc, 102);
    }

    #[test]
    fn memory_addresses_wrap_as_rv32_addresses() {
        let mut machine = Machine::new(8);
        machine.register.write(1, -1);
        machine.register.write(2, 0x5a);

        machine
            .execute_ir(&Sb {
                rs2: 2,
                imm: 1,
                rs1: 1,
            })
            .unwrap();

        assert_eq!(machine.memory.load_u8(0).unwrap(), 0x5a);
    }

    #[test]
    fn out_of_bounds_store_returns_error_without_advancing_pc() {
        let mut machine = Machine::new(4);
        machine.register.write(1, 3);
        machine.register.write(2, 0x1234);

        assert!(
            machine
                .execute_ir(&Sh {
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
        machine.register.write(1, 3);
        assert!(
            machine
                .execute_ir(&Lw {
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
        assert!(machine.parse_ir().is_err());
    }

    #[test]
    fn signed_division_handles_zero_and_overflow() {
        let mut machine = Machine::new(16);
        machine.register.write(1, i64::MIN);
        machine.register.write(2, -1);

        machine
            .execute_ir(&Div {
                rd: 3,
                rs1: 1,
                rs2: 2,
            })
            .unwrap();
        assert_eq!(machine.register.read(3), i64::MIN);

        machine.register.write(2, 0);
        machine
            .execute_ir(&Div {
                rd: 4,
                rs1: 1,
                rs2: 2,
            })
            .unwrap();
        assert_eq!(machine.register.read(4), -1);

        machine
            .execute_ir(&Rem {
                rd: 5,
                rs1: 1,
                rs2: 2,
            })
            .unwrap();
        assert_eq!(machine.register.read(5), i64::MIN);
    }

    #[test]
    fn unsigned_division_handles_zero() {
        let mut machine = Machine::new(16);
        machine.register.write(1, 7);
        machine.register.write(2, 0);

        machine
            .execute_ir(&Divu {
                rd: 3,
                rs1: 1,
                rs2: 2,
            })
            .unwrap();
        assert_eq!(machine.register.read_u(3), u64::MAX);

        machine
            .execute_ir(&Remu {
                rd: 4,
                rs1: 1,
                rs2: 2,
            })
            .unwrap();
        assert_eq!(machine.register.read(4), 7);
    }

    #[test]
    fn word_division_handles_zero_and_overflow() {
        let mut machine = Machine::new(16);
        machine.register.write(1, i32::MIN as i64);
        machine.register.write(2, -1);

        machine
            .execute_ir(&Divw {
                rd: 3,
                rs1: 1,
                rs2: 2,
            })
            .unwrap();
        assert_eq!(machine.register.read_w(3), i32::MIN);

        machine.register.write(2, 0);
        machine
            .execute_ir(&Remw {
                rd: 4,
                rs1: 1,
                rs2: 2,
            })
            .unwrap();
        assert_eq!(machine.register.read_w(4), i32::MIN);
    }

    #[test]
    fn word_division_by_zero_returns_specified_values() {
        let mut machine = Machine::new(16);
        machine.register.write(1, 0x1234_5678_8000_0007);
        machine.register.write(2, 0);

        machine
            .execute_ir(&Divw {
                rd: 3,
                rs1: 1,
                rs2: 2,
            })
            .unwrap();
        assert_eq!(machine.register.read(3), -1);

        machine
            .execute_ir(&Divuw {
                rd: 4,
                rs1: 1,
                rs2: 2,
            })
            .unwrap();
        assert_eq!(machine.register.read(4), -1);

        machine
            .execute_ir(&Remuw {
                rd: 5,
                rs1: 1,
                rs2: 2,
            })
            .unwrap();
        assert_eq!(machine.register.read(5), 0xffff_ffff_8000_0007_u64 as i64);
    }

    #[test]
    fn word_remainder_overflow_returns_zero() {
        let mut machine = Machine::new(16);
        machine.register.write(1, i32::MIN as i64);
        machine.register.write(2, -1);

        machine
            .execute_ir(&Remw {
                rd: 3,
                rs1: 1,
                rs2: 2,
            })
            .unwrap();
        assert_eq!(machine.register.read(3), 0);
    }

    #[test]
    fn memory_slice_rejects_overflow_and_allows_empty_at_end() {
        let mut memory = super::Memory::new(4);
        assert!(memory.slice(usize::MAX, 1).is_err());
        assert!(memory.store_slice(usize::MAX, &[]).is_err());
        assert!(memory.slice(4, 0).is_ok());
    }

    #[test]
    fn fence_is_a_supported_no_op() {
        let mut machine = Machine::new(4);
        machine.execute_ir(&Fence).unwrap();
        assert_eq!(machine.pc, 4);
    }
}
