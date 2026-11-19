use anyhow::{Ok, bail};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Instruction {
    /// ADD rd = rs1 + rs2
    Add {
        rd: usize,
        rs1: usize,
        rs2: usize,
    },
    /// Sub rd = rs1 - rs2
    Sub {
        rd: usize,
        rs1: usize,
        rs2: usize,
    },
    /// XOR rd = rs1 ^ rs2
    Xor {
        rd: usize,
        rs1: usize,
        rs2: usize,
    },
    /// OR rd = rs1 | rs2
    Or {
        rd: usize,
        rs1: usize,
        rs2: usize,
    },
    /// AND rd = rs1 & rs2
    And {
        rd: usize,
        rs1: usize,
        rs2: usize,
    },
    /// Shift Left Logical rd = rs1 << rs2
    Sll {
        rd: usize,
        rs1: usize,
        rs2: usize,
    },
    /// Shift Right Logical rd = rs1 >> rs2
    Srl {
        rd: usize,
        rs1: usize,
        rs2: usize,
    },
    /// Shift Right Arith* rd = rs1 >> rs2
    Sra {
        rd: usize,
        rs1: usize,
        rs2: usize,
    },
    /// Set Less Than rd = (rs1 < rs2)?1:0
    Slt {
        rd: usize,
        rs1: usize,
        rs2: usize,
    },
    /// Set Less Than(U) rd = (rs1 < rs2)?1:0
    Sltu {
        rd: usize,
        rs1: usize,
        rs2: usize,
    },
    Mul {
        rd: usize,
        rs1: usize,
        rs2: usize,
    },
    Mulh {
        rd: usize,
        rs1: usize,
        rs2: usize,
    },
    Mulhsu {
        rd: usize,
        rs1: usize,
        rs2: usize,
    },
    Mulhu {
        rd: usize,
        rs1: usize,
        rs2: usize,
    },
    Div {
        rd: usize,
        rs1: usize,
        rs2: usize,
    },
    Divu {
        rd: usize,
        rs1: usize,
        rs2: usize,
    },
    Rem {
        rd: usize,
        rs1: usize,
        rs2: usize,
    },
    Remu {
        rd: usize,
        rs1: usize,
        rs2: usize,
    },
    /// Addw rd = sext(rs1[0:31] + rs2[0:31])
    Addw {
        rd: usize,
        rs1: usize,
        rs2: usize,
    },
    /// Subw rd = sext(rs1[0:31] - rs2[0:31])
    Subw {
        rd: usize,
        rs1: usize,
        rs2: usize,
    },
    /// Shift Left Logical word rd = rs1 << rs2
    Sllw {
        rd: usize,
        rs1: usize,
        rs2: usize,
    },
    /// Shift Right Logical word rd = rs1 >> rs2
    Srlw {
        rd: usize,
        rs1: usize,
        rs2: usize,
    },
    /// Shift Right Arith* word rd = rs1 >> rs2
    Sraw {
        rd: usize,
        rs1: usize,
        rs2: usize,
    },
    Mulw {
        rd: usize,
        rs1: usize,
        rs2: usize,
    },
    Divw {
        rd: usize,
        rs1: usize,
        rs2: usize,
    },
    Divuw {
        rd: usize,
        rs1: usize,
        rs2: usize,
    },
    Remw {
        rd: usize,
        rs1: usize,
        rs2: usize,
    },
    Remuw {
        rd: usize,
        rs1: usize,
        rs2: usize,
    },
    /// ADD Imm rd = rs1 + imm
    Addi {
        rd: usize,
        rs1: usize,
        imm: i64,
    },
    /// XOR Imm rd = rs1 ^ imm
    Xori {
        rd: usize,
        rs1: usize,
        imm: i64,
    },
    /// OR Imm rd = rs1 | imm
    Ori {
        rd: usize,
        rs1: usize,
        imm: i64,
    },
    /// AND Imm rd = rs1 & imm
    Andi {
        rd: usize,
        rs1: usize,
        imm: i64,
    },
    /// Shift Left Logical Imm rd = rs1 << imm
    Slli {
        rd: usize,
        rs1: usize,
        imm: u32,
    },
    /// Shift Right Logical Imm rd = rs1 >> imm
    Srli {
        rd: usize,
        rs1: usize,
        imm: u32,
    },
    /// Shift right Arith Imm rd = rs1 >> imm
    Srai {
        rd: usize,
        rs1: usize,
        imm: u32,
    },
    /// Set Less Than Imm rd = (rs1 < imm)?1:0
    Slti {
        rd: usize,
        rs1: usize,
        imm: i64,
    },
    /// Set Less Than Imm(U) rd = (rs1 < imm)?1:0
    Sltiu {
        rd: usize,
        rs1: usize,
        imm: u64,
    },
    /// Add Imm word rd = sign_extend(rs1[0:31] + Imm[0:31])
    Addiw {
        rd: usize,
        rs1: usize,
        imm: i32,
    },
    /// Shift Left Logical Imm word rd = rs1 << imm
    Slliw {
        rd: usize,
        rs1: usize,
        imm: i32,
    },
    /// Shift Right Logical Imm word rd = rs1 >> imm
    Srliw {
        rd: usize,
        rs1: usize,
        imm: u32,
    },
    /// Shift right Arith Imm word rd = rs1 >> imm
    Sraiw {
        rd: usize,
        rs1: usize,
        imm: i32,
    },
    /// Load Byte rd = M[rs1+imm][0:7]
    Lb {
        rd: usize,
        rs1: usize,
        imm: i64,
    },
    /// Load Half rd = M[rs1+imm][0:15]
    Lh {
        rd: usize,
        rs1: usize,
        imm: i64,
    },
    /// Load Word rd = M[rs1+imm][0:31]
    Lw {
        rd: usize,
        rs1: usize,
        imm: i64,
    },
    /// Load double word
    Ld {
        rd: usize,
        rs1: usize,
        imm: i64,
    },
    /// Load Byte(U) rd = M[rs1+imm][0:7]
    Lbu {
        rd: usize,
        rs1: usize,
        imm: i64,
    },
    /// Load Half(U) rd = M[rs1+imm][0:15]
    Lhu {
        rd: usize,
        rs1: usize,
        imm: i64,
    },
    /// Load Word unsigned rd = zext(M[rs1+imm][0:31])
    Lwu {
        rd: usize,
        rs1: usize,
        imm: i64,
    },
    /// Store Byte M[rs1+imm][0:7] = rs2[0:7]
    Sb {
        rs2: usize,
        imm: i64,
        rs1: usize,
    },
    /// Store Half M[rs1+imm][0:15] = rs2[0:15]
    Sh {
        rs2: usize,
        imm: i64,
        rs1: usize,
    },
    /// Store Word M[rs1+imm][0:31] = rs2[0:31]
    Sw {
        rs2: usize,
        imm: i64,
        rs1: usize,
    },
    /// Store Word M[rs1+imm][0:63] = rs2[0:63]
    Sd {
        rs2: usize,
        imm: i64,
        rs1: usize,
    },
    /// Branch == if(rs1==rs2) PC+=imm
    Beq {
        rs1: usize,
        rs2: usize,
        imm: i64,
    },
    /// Branch != if(rs1!=rs2) PC+=imm
    Bne {
        rs1: usize,
        rs2: usize,
        imm: i64,
    },
    /// Branch < if(rs1<rs2) PC+=imm
    Blt {
        rs1: usize,
        rs2: usize,
        imm: i64,
    },
    /// Branch > if(rs1>rs2) PC+=imm
    Bge {
        rs1: usize,
        rs2: usize,
        imm: i64,
    },
    /// Branch <(U) if(rs1<rs2) PC+=imm
    Bltu {
        rs1: usize,
        rs2: usize,
        imm: i64,
    },
    /// Branch >=(U) if(rs1>=rs2) PC+=imm
    Bgeu {
        rs1: usize,
        rs2: usize,
        imm: i64,
    },
    /// Jump and Link rd = PC+4; PC+=imm
    Jal {
        rd: usize,
        imm: i64,
    },
    /// Jump and Link Reg rd=PC+4; PC=rs1+imm
    Jalr {
        rd: usize,
        rs1: usize,
        imm: i64,
    },
    /// Load Upper Imm rd = imm << 12
    Lui {
        rd: usize,
        imm: i64,
    },
    /// Add Upper Imm to PC rd=PC+(imm<<12)
    Auipc {
        rd: usize,
        imm: i64,
    },
    /// Env Call Transfer control to OS
    Ecall,
    /// Env Break Transfer control to debugger
    Ebreak,
    /// Fence memory and I/O operations
    Fence,
    Csrrw {
        rd: usize,
        rs1: usize,
        csr: usize,
    },
    Csrrs {
        rd: usize,
        rs1: usize,
        csr: usize,
    },
    Csrrc {
        rd: usize,
        rs1: usize,
        csr: usize,
    },
    Csrrwi {
        rd: usize,
        imm: u64,
        csr: usize,
    },
    Csrrsi {
        rd: usize,
        imm: u64,
        csr: usize,
    },
    Csrrci {
        rd: usize,
        imm: u64,
        csr: usize,
    },
}

fn rd_field(word: u32) -> usize {
    ((word >> 7) & 0x1f) as usize
}

fn rs1_field(word: u32) -> usize {
    ((word >> 15) & 0x1f) as usize
}

fn rs2_field(word: u32) -> usize {
    ((word >> 20) & 0x1f) as usize
}

fn funct3_field(word: u32) -> u32 {
    (word >> 12) & 0x7
}

fn sign_extend(value: u32, width: u32) -> i64 {
    ((value << (32 - width)) as i32 as i64) >> (32 - width)
}

struct Rfmt {
    funct3: u32,
    funct7: u32,
    rd: usize,
    rs1: usize,
    rs2: usize,
}

impl From<u32> for Rfmt {
    fn from(value: u32) -> Self {
        Rfmt {
            funct3: funct3_field(value),
            funct7: value >> 25,
            rd: rd_field(value),
            rs1: rs1_field(value),
            rs2: rs2_field(value),
        }
    }
}

struct Ifmt {
    funct3: u32,
    rd: usize,
    rs1: usize,
    imm: i64,
}

impl From<u32> for Ifmt {
    fn from(value: u32) -> Self {
        Ifmt {
            funct3: funct3_field(value),
            rd: rd_field(value),
            rs1: rs1_field(value),
            imm: (value as i32 as i64) >> 20,
        }
    }
}

struct Csrfmt {
    funct3: u32,
    rd: usize,
    rs1: usize,
    csr: usize,
}

impl From<u32> for Csrfmt {
    fn from(value: u32) -> Self {
        Csrfmt {
            funct3: funct3_field(value),
            rd: rd_field(value),
            rs1: rs1_field(value),
            csr: (value >> 20) as usize,
        }
    }
}

struct Sfmt {
    funct3: u32,
    rs1: usize,
    rs2: usize,
    imm: i64,
}

impl From<u32> for Sfmt {
    fn from(value: u32) -> Self {
        let imm_0_4 = (value as i32 >> 7) & 0x1f;
        let imm_5_31 = (value as i32 >> 20) & !0x1f;
        let imm = (imm_0_4 | imm_5_31) as i64;
        Self {
            funct3: funct3_field(value),
            rs1: rs1_field(value),
            rs2: rs2_field(value),
            imm,
        }
    }
}

struct Bfmt {
    funct3: u32,
    rs1: usize,
    rs2: usize,
    imm: i64,
}

impl From<u32> for Bfmt {
    fn from(value: u32) -> Self {
        let imm = sign_extend(
            (((value >> 31) & 1) << 12)
                | (((value >> 7) & 1) << 11)
                | (((value >> 25) & 0x3f) << 5)
                | (((value >> 8) & 0xf) << 1),
            13,
        );
        Self {
            funct3: funct3_field(value),
            rs1: rs1_field(value),
            rs2: rs2_field(value),
            imm,
        }
    }
}

struct Ufmt {
    rd: usize,
    imm: i64,
}

impl From<u32> for Ufmt {
    fn from(value: u32) -> Self {
        Self {
            rd: rd_field(value),
            imm: (value & 0xffff_f000) as i32 as i64,
        }
    }
}

struct Jfmt {
    rd: usize,
    imm: i64,
}

impl From<u32> for Jfmt {
    fn from(value: u32) -> Self {
        let imm = sign_extend(
            (((value >> 31) & 1) << 20)
                | (((value >> 12) & 0xff) << 12)
                | (((value >> 20) & 1) << 11)
                | (((value >> 21) & 0x3ff) << 1),
            21,
        );
        Self {
            rd: rd_field(value),
            imm,
        }
    }
}

impl TryFrom<u32> for Instruction {
    type Error = anyhow::Error;
    fn try_from(value: u32) -> anyhow::Result<Self> {
        use Instruction::*;
        let opcode = value & 0x7f;
        match opcode {
            // Fence - I FMT
            0x0f => {
                let funct3 = funct3_field(value);
                let fm = value >> 28;
                let pred = (value >> 24) & 0xf;
                let succ = (value >> 20) & 0xf;
                let rd = rd_field(value);
                let rs1 = rs1_field(value);
                let valid_fm = fm == 0 || (fm == 0b1000 && pred == 0b0011 && succ == 0b0011);
                if funct3 != 0 || rd != 0 || rs1 != 0 || !valid_fm {
                    bail!("invalid fence instruction:{:08x}", value);
                }
                Ok(Fence)
            }
            // Arith - R FMT
            0x33 => {
                let Rfmt {
                    funct3,
                    funct7,
                    rd,
                    rs1,
                    rs2,
                } = Rfmt::from(value);
                match (funct3, funct7) {
                    (0x0, 0x00) => Ok(Add { rd, rs1, rs2 }),
                    (0x0, 0x20) => Ok(Sub { rd, rs1, rs2 }),
                    (0x4, 0x00) => Ok(Xor { rd, rs1, rs2 }),
                    (0x6, 0x00) => Ok(Or { rd, rs1, rs2 }),
                    (0x7, 0x00) => Ok(And { rd, rs1, rs2 }),
                    (0x1, 0x00) => Ok(Sll { rd, rs1, rs2 }),
                    (0x5, 0x00) => Ok(Srl { rd, rs1, rs2 }),
                    (0x5, 0x20) => Ok(Sra { rd, rs1, rs2 }),
                    (0x2, 0x00) => Ok(Slt { rd, rs1, rs2 }),
                    (0x3, 0x00) => Ok(Sltu { rd, rs1, rs2 }),
                    (0x0, 0x01) => Ok(Mul { rd, rs1, rs2 }),
                    (0x1, 0x01) => Ok(Mulh { rd, rs1, rs2 }),
                    (0x2, 0x01) => Ok(Mulhsu { rd, rs1, rs2 }),
                    (0x3, 0x01) => Ok(Mulhu { rd, rs1, rs2 }),
                    (0x4, 0x01) => Ok(Div { rd, rs1, rs2 }),
                    (0x5, 0x01) => Ok(Divu { rd, rs1, rs2 }),
                    (0x6, 0x01) => Ok(Rem { rd, rs1, rs2 }),
                    (0x7, 0x01) => Ok(Remu { rd, rs1, rs2 }),
                    _ => bail!(
                        "invalid (funct3, funct7):({:x},{:x}), opcode:{:b}",
                        funct3,
                        funct7,
                        opcode
                    ),
                }
            }
            // Airth word - R FMT
            0x3b => {
                let Rfmt {
                    funct3,
                    funct7,
                    rd,
                    rs1,
                    rs2,
                } = Rfmt::from(value);
                match (funct3, funct7) {
                    (0x0, 0x00) => Ok(Addw { rd, rs1, rs2 }),
                    (0x0, 0x20) => Ok(Subw { rd, rs1, rs2 }),
                    (0x1, 0x00) => Ok(Sllw { rd, rs1, rs2 }),
                    (0x5, 0x00) => Ok(Srlw { rd, rs1, rs2 }),
                    (0x5, 0x20) => Ok(Sraw { rd, rs1, rs2 }),
                    (0x0, 0x01) => Ok(Mulw { rd, rs1, rs2 }),
                    (0x4, 0x01) => Ok(Divw { rd, rs1, rs2 }),
                    (0x5, 0x01) => Ok(Divuw { rd, rs1, rs2 }),
                    (0x6, 0x01) => Ok(Remw { rd, rs1, rs2 }),
                    (0x7, 0x01) => Ok(Remuw { rd, rs1, rs2 }),
                    _ => bail!(
                        "invalid (funct3, funct7):({:x},{:x}), opcode:{:b}",
                        funct3,
                        funct7,
                        opcode
                    ),
                }
            }
            // Airth Imm - I FMT
            0x13 => {
                let Ifmt {
                    funct3,
                    rd,
                    rs1,
                    imm,
                } = Ifmt::from(value);
                match funct3 {
                    0x0 => Ok(Addi { rd, rs1, imm }),
                    0x4 => Ok(Xori { rd, rs1, imm }),
                    0x6 => Ok(Ori { rd, rs1, imm }),
                    0x7 => Ok(Andi { rd, rs1, imm }),
                    0x1 => Ok(Slli {
                        rd,
                        rs1,
                        imm: (imm as u32) & 0x3f,
                    }),
                    0x5 => match value >> 25 {
                        0x00 => Ok(Srli {
                            rd,
                            rs1,
                            imm: (imm as u32) & 0x3f,
                        }),
                        0x20 => Ok(Srai {
                            rd,
                            rs1,
                            imm: (imm as u32) & 0x3f,
                        }),
                        _ => bail!("invalid shift imm:{:x}", imm),
                    },
                    0x2 => Ok(Slti { rd, rs1, imm }),
                    0x3 => Ok(Sltiu {
                        rd,
                        rs1,
                        imm: imm as u64,
                    }),
                    _ => bail!("invalid funct3:{:x}", funct3),
                }
            }
            0x1b => {
                let Ifmt {
                    funct3,
                    rd,
                    rs1,
                    imm,
                } = Ifmt::from(value);
                match funct3 {
                    0x0 => Ok(Addiw {
                        rd,
                        rs1,
                        imm: imm as i32,
                    }),
                    0x1 => Ok(Slliw {
                        rd,
                        rs1,
                        imm: (imm as i32) & 0x1f,
                    }),
                    0x5 => match value >> 25 {
                        0x00 => Ok(Srliw {
                            rd,
                            rs1,
                            imm: (imm as u32) & 0x1f,
                        }),
                        0x20 => Ok(Sraiw {
                            rd,
                            rs1,
                            imm: (imm as i32) & 0x1f,
                        }),
                        _ => bail!("invalid shift imm:{:x}", imm),
                    },
                    _ => bail!("invalid funct3:{:x}", funct3),
                }
            }
            // Load - I FMT
            0x03 => {
                let Ifmt {
                    funct3,
                    rd,
                    rs1,
                    imm,
                } = Ifmt::from(value);
                match funct3 {
                    0x0 => Ok(Lb { rd, rs1, imm }),
                    0x1 => Ok(Lh { rd, rs1, imm }),
                    0x2 => Ok(Lw { rd, rs1, imm }),
                    0x3 => Ok(Ld { rd, rs1, imm }),
                    0x4 => Ok(Lbu { rd, rs1, imm }),
                    0x5 => Ok(Lhu { rd, rs1, imm }),
                    0x6 => Ok(Lwu { rd, rs1, imm }),
                    _ => bail!("invalid load funct3:{:x}", funct3),
                }
            }
            // Store - S FMT
            0x23 => {
                let Sfmt {
                    funct3,
                    rs1,
                    rs2,
                    imm,
                } = Sfmt::from(value);
                match funct3 {
                    0x0 => Ok(Sb { rs2, imm, rs1 }),
                    0x1 => Ok(Sh { rs2, imm, rs1 }),
                    0x2 => Ok(Sw { rs2, imm, rs1 }),
                    0x3 => Ok(Sd { rs2, imm, rs1 }),
                    _ => bail!("invalid store funct3:{:x}", funct3),
                }
            }
            // Branch - B FMT
            0x63 => {
                let Bfmt {
                    funct3,
                    rs1,
                    rs2,
                    imm,
                } = Bfmt::from(value);
                match funct3 {
                    0x0 => Ok(Beq { rs1, rs2, imm }),
                    0x1 => Ok(Bne { rs1, rs2, imm }),
                    0x4 => Ok(Blt { rs1, rs2, imm }),
                    0x5 => Ok(Bge { rs1, rs2, imm }),
                    0x6 => Ok(Bltu { rs1, rs2, imm }),
                    0x7 => Ok(Bgeu { rs1, rs2, imm }),
                    _ => bail!("invalid branch funct3:{:x}", funct3),
                }
            }
            // Jump And Link - J FMT
            0x6f => {
                let Jfmt { rd, imm } = Jfmt::from(value);
                Ok(Jal { rd, imm })
            }
            // Jump And Link Reg - I FMT
            0x67 => {
                let Ifmt {
                    funct3,
                    rd,
                    rs1,
                    imm,
                } = Ifmt::from(value);
                if funct3 != 0x00 {
                    bail!("invalid jalr funct3:{:x}", funct3);
                }
                Ok(Jalr { rd, rs1, imm })
            }
            // Load Upper Imm - U FMT
            0x37 => {
                let Ufmt { rd, imm } = Ufmt::from(value);
                Ok(Lui { rd, imm })
            }
            // Add Upper Imm to PC - U FMT
            0x17 => {
                let Ufmt { rd, imm } = Ufmt::from(value);
                Ok(Auipc { rd, imm })
            }
            // Environment
            0x73 => {
                let Csrfmt {
                    funct3,
                    rd,
                    rs1,
                    csr,
                } = Csrfmt::from(value);
                match funct3 {
                    0x00 => {
                        if rd != 0 || rs1 != 0 {
                            bail!("invalid environment ir:{:x}", value);
                        }
                        match csr {
                            0x0 => Ok(Ecall),
                            0x1 => Ok(Ebreak),
                            _ => bail!("invalid environment csr:{:x}", csr),
                        }
                    }
                    0x1 => Ok(Csrrw { rd, rs1, csr }),
                    0x2 => Ok(Csrrs { rd, rs1, csr }),
                    0x3 => Ok(Csrrc { rd, rs1, csr }),
                    0x5 => Ok(Csrrwi {
                        rd,
                        imm: rs1 as u64,
                        csr,
                    }),
                    0x6 => Ok(Csrrsi {
                        rd,
                        imm: rs1 as u64,
                        csr,
                    }),
                    0x7 => Ok(Csrrci {
                        rd,
                        imm: rs1 as u64,
                        csr,
                    }),
                    _ => bail!("invlaid environment funct3:{:x}", funct3),
                }
            }
            _ => bail!("invalid opcode:{:b}", opcode),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Instruction::*;
        match self {
            Add { rd, rs1, rs2 } => write!(f, "add x{rd}, x{rs1}, x{rs2}"),
            Sub { rd, rs1, rs2 } => write!(f, "sub x{rd}, x{rs1}, x{rs2}"),
            Xor { rd, rs1, rs2 } => write!(f, "xor x{rd}, x{rs1}, x{rs2}"),
            Or { rd, rs1, rs2 } => write!(f, "or x{rd}, x{rs1}, x{rs2}"),
            And { rd, rs1, rs2 } => write!(f, "and x{rd}, x{rs1}, x{rs2}"),
            Sll { rd, rs1, rs2 } => write!(f, "sll x{rd}, x{rs1}, x{rs2}"),
            Srl { rd, rs1, rs2 } => write!(f, "srl x{rd}, x{rs1}, x{rs2}"),
            Sra { rd, rs1, rs2 } => write!(f, "sra x{rd}, x{rs1}, x{rs2}"),
            Slt { rd, rs1, rs2 } => write!(f, "slt x{rd}, x{rs1}, x{rs2}"),
            Sltu { rd, rs1, rs2 } => write!(f, "sltu x{rd}, x{rs1}, x{rs2}"),
            Mul { rd, rs1, rs2 } => write!(f, "mul x{rd}, x{rs1}, x{rs2}"),
            Mulh { rd, rs1, rs2 } => write!(f, "mulh x{rd}, x{rs1}, x{rs2}"),
            Mulhsu { rd, rs1, rs2 } => write!(f, "mulhsu x{rd}, x{rs1}, x{rs2}"),
            Mulhu { rd, rs1, rs2 } => write!(f, "mulhu x{rd}, x{rs1}, x{rs2}"),
            Div { rd, rs1, rs2 } => write!(f, "div x{rd}, x{rs1}, x{rs2}"),
            Divu { rd, rs1, rs2 } => write!(f, "divu x{rd}, x{rs1}, x{rs2}"),
            Rem { rd, rs1, rs2 } => write!(f, "rem x{rd}, x{rs1}, x{rs2}"),
            Remu { rd, rs1, rs2 } => write!(f, "remu x{rd}, x{rs1}, x{rs2}"),
            Addw { rd, rs1, rs2 } => write!(f, "addw x{rd}, x{rs1}, x{rs2}"),
            Subw { rd, rs1, rs2 } => write!(f, "subw x{rd}, x{rs1}, x{rs2}"),
            Sllw { rd, rs1, rs2 } => write!(f, "sllw x{rd}, x{rs1}, x{rs2}"),
            Srlw { rd, rs1, rs2 } => write!(f, "srlw x{rd}, x{rs1}, x{rs2}"),
            Sraw { rd, rs1, rs2 } => write!(f, "sraw x{rd}, x{rs1}, x{rs2}"),
            Mulw { rd, rs1, rs2 } => write!(f, "mulw x{rd}, x{rs1}, x{rs2}"),
            Divw { rd, rs1, rs2 } => write!(f, "divw x{rd}, x{rs1}, x{rs2}"),
            Divuw { rd, rs1, rs2 } => write!(f, "divuw x{rd}, x{rs1}, x{rs2}"),
            Remw { rd, rs1, rs2 } => write!(f, "remw x{rd}, x{rs1}, x{rs2}"),
            Remuw { rd, rs1, rs2 } => write!(f, "remuw x{rd}, x{rs1}, x{rs2}"),
            Addi { rd, rs1, imm } => write!(f, "addi x{rd}, x{rs1}, {imm}"),
            Xori { rd, rs1, imm } => write!(f, "xori x{rd}, x{rs1}, {imm}"),
            Ori { rd, rs1, imm } => write!(f, "ori x{rd}, x{rs1}, {imm}"),
            Andi { rd, rs1, imm } => write!(f, "andi x{rd}, x{rs1}, {imm}"),
            Slli { rd, rs1, imm } => write!(f, "slli x{rd}, x{rs1}, {imm}"),
            Srli { rd, rs1, imm } => write!(f, "srli x{rd}, x{rs1}, {imm}"),
            Srai { rd, rs1, imm } => write!(f, "srai x{rd}, x{rs1}, {imm}"),
            Slti { rd, rs1, imm } => write!(f, "slti x{rd}, x{rs1}, {imm}"),
            Sltiu { rd, rs1, imm } => write!(f, "sltiu x{rd}, x{rs1}, {}", *imm as i32),
            Addiw { rd, rs1, imm } => write!(f, "addiw x{rd}, x{rs1}, {}", *imm),
            Slliw { rd, rs1, imm } => write!(f, "slliw x{rd}, x{rs1}, {}", *imm),
            Srliw { rd, rs1, imm } => write!(f, "srliw x{rd}, x{rs1}, {}", *imm),
            Sraiw { rd, rs1, imm } => write!(f, "sraiw x{rd}, x{rs1}, {}", *imm),
            Lb { rd, rs1, imm } => write!(f, "lb x{rd}, {imm}(x{rs1})"),
            Lh { rd, rs1, imm } => write!(f, "lh x{rd}, {imm}(x{rs1})"),
            Lw { rd, rs1, imm } => write!(f, "lw x{rd}, {imm}(x{rs1})"),
            Ld { rd, rs1, imm } => write!(f, "ld x{rd}, {imm}(x{rs1})"),
            Lbu { rd, rs1, imm } => write!(f, "lbu x{rd}, {imm}(x{rs1})"),
            Lhu { rd, rs1, imm } => write!(f, "lhu x{rd}, {imm}(x{rs1})"),
            Lwu { rd, rs1, imm } => write!(f, "lwu x{rd}, {imm}(x{rs1})"),
            Sb { rs2, imm, rs1 } => write!(f, "sb x{rs2}, {imm}(x{rs1})"),
            Sh { rs2, imm, rs1 } => write!(f, "sh x{rs2}, {imm}(x{rs1})"),
            Sw { rs2, imm, rs1 } => write!(f, "sw x{rs2}, {imm}(x{rs1})"),
            Sd { rs2, imm, rs1 } => write!(f, "sd x{rs2}, {imm}(x{rs1})"),
            Beq { rs1, rs2, imm } => write!(f, "beq x{rs1}, x{rs2}, {imm}"),
            Bne { rs1, rs2, imm } => write!(f, "bne x{rs1}, x{rs2}, {imm}"),
            Blt { rs1, rs2, imm } => write!(f, "blt x{rs1}, x{rs2}, {imm}"),
            Bge { rs1, rs2, imm } => write!(f, "bge x{rs1}, x{rs2}, {imm}"),
            Bltu { rs1, rs2, imm } => write!(f, "bltu x{rs1}, x{rs2}, {imm}"),
            Bgeu { rs1, rs2, imm } => write!(f, "bgeu x{rs1}, x{rs2}, {imm}"),
            Jal { rd, imm } => write!(f, "jal x{rd}, {imm}"),
            Jalr { rd, rs1, imm } => write!(f, "jalr x{rd}, {imm}(x{rs1})"),
            Lui { rd, imm } => write!(f, "lui x{rd}, {:#x}", imm >> 12),
            Auipc { rd, imm } => write!(f, "auipc x{rd}, {:#x}", imm >> 12),
            Ecall => f.write_str("ecall"),
            Ebreak => f.write_str("ebreak"),
            Fence => f.write_str("fence"),
            Csrrw { rd, rs1, csr } => write!(f, "csrrw x{rd}, x{csr}, x{rs1}"),
            Csrrs { rd, rs1, csr } => write!(f, "csrrs x{rd}, x{csr}, x{rs1}"),
            Csrrc { rd, rs1, csr } => write!(f, "csrrc x{rd}, x{csr}, x{rs1}"),
            Csrrwi { rd, imm, csr } => write!(f, "csrrwi x{rd}, x{csr}, {:x}", imm),
            Csrrsi { rd, imm, csr } => write!(f, "csrrsi x{rd}, x{csr}, {:x}", imm),
            Csrrci { rd, imm, csr } => write!(f, "csrrci x{rd}, x{csr}, {:x}", imm),
        }
    }
}

impl From<Instruction> for u32 {
    fn from(inst: Instruction) -> Self {
        use Instruction::*;

        fn encode_r(
            opcode: u32,
            funct7: u32,
            rs2: usize,
            rs1: usize,
            funct3: u32,
            rd: usize,
        ) -> u32 {
            (funct7 << 25)
                | ((rs2 as u32 & 0x1f) << 20)
                | ((rs1 as u32 & 0x1f) << 15)
                | (funct3 << 12)
                | ((rd as u32 & 0x1f) << 7)
                | opcode
        }

        fn encode_i(opcode: u32, funct3: u32, rd: usize, rs1: usize, imm: i64) -> u32 {
            let imm = imm as u32 & 0xfff;
            ((imm & 0xfff) << 20)
                | ((rs1 as u32 & 0x1f) << 15)
                | (funct3 << 12)
                | ((rd as u32 & 0x1f) << 7)
                | opcode
        }

        fn encode_s(funct3: u32, rs2: usize, imm: i64, rs1: usize) -> u32 {
            let imm = imm as u32 & 0xfff;
            ((imm >> 5) << 25)
                | ((rs2 as u32 & 0x1f) << 20)
                | ((rs1 as u32 & 0x1f) << 15)
                | (funct3 << 12)
                | ((imm & 0x1f) << 7)
                | 0x23
        }

        fn encode_b(funct3: u32, rs1: usize, rs2: usize, imm: i64) -> u32 {
            let imm = imm as u32 & 0x1fff;
            (((imm >> 12) & 1) << 31)
                | (((imm >> 5) & 0x3f) << 25)
                | ((rs2 as u32 & 0x1f) << 20)
                | ((rs1 as u32 & 0x1f) << 15)
                | (funct3 << 12)
                | (((imm >> 1) & 0xf) << 8)
                | (((imm >> 11) & 1) << 7)
                | 0x63
        }

        fn encode_j(rd: usize, imm: i64) -> u32 {
            let imm = imm as u32 & 0x1f_ffff;
            (((imm >> 20) & 1) << 31)
                | (((imm >> 1) & 0x3ff) << 21)
                | (((imm >> 11) & 1) << 20)
                | (((imm >> 12) & 0xff) << 12)
                | ((rd as u32 & 0x1f) << 7)
                | 0x6f
        }

        fn encode_csr(opcode: u32, funct3: u32, rd: usize, rs1: usize, csr: usize) -> u32 {
            let csr = csr as u32 & 0xfff;
            ((csr & 0xfff) << 20)
                | ((rs1 as u32 & 0x1f) << 15)
                | (funct3 << 12)
                | ((rd as u32 & 0x1f) << 7)
                | opcode
        }

        match inst {
            Add { rd, rs1, rs2 } => encode_r(0x33, 0, rs2, rs1, 0, rd),
            Sub { rd, rs1, rs2 } => encode_r(0x33, 0x20, rs2, rs1, 0, rd),
            Xor { rd, rs1, rs2 } => encode_r(0x33, 0, rs2, rs1, 4, rd),
            Or { rd, rs1, rs2 } => encode_r(0x33, 0, rs2, rs1, 6, rd),
            And { rd, rs1, rs2 } => encode_r(0x33, 0, rs2, rs1, 7, rd),
            Sll { rd, rs1, rs2 } => encode_r(0x33, 0, rs2, rs1, 1, rd),
            Srl { rd, rs1, rs2 } => encode_r(0x33, 0, rs2, rs1, 5, rd),
            Sra { rd, rs1, rs2 } => encode_r(0x33, 0x20, rs2, rs1, 5, rd),
            Slt { rd, rs1, rs2 } => encode_r(0x33, 0, rs2, rs1, 2, rd),
            Sltu { rd, rs1, rs2 } => encode_r(0x33, 0, rs2, rs1, 3, rd),
            Mul { rd, rs1, rs2 } => encode_r(0x33, 1, rs2, rs1, 0, rd),
            Mulh { rd, rs1, rs2 } => encode_r(0x33, 1, rs2, rs1, 1, rd),
            Mulhsu { rd, rs1, rs2 } => encode_r(0x33, 1, rs2, rs1, 2, rd),
            Mulhu { rd, rs1, rs2 } => encode_r(0x33, 1, rs2, rs1, 3, rd),
            Div { rd, rs1, rs2 } => encode_r(0x33, 1, rs2, rs1, 4, rd),
            Divu { rd, rs1, rs2 } => encode_r(0x33, 1, rs2, rs1, 5, rd),
            Rem { rd, rs1, rs2 } => encode_r(0x33, 1, rs2, rs1, 6, rd),
            Remu { rd, rs1, rs2 } => encode_r(0x33, 1, rs2, rs1, 7, rd),
            Addw { rd, rs1, rs2 } => encode_r(0x3b, 0, rs2, rs1, 0, rd),
            Subw { rd, rs1, rs2 } => encode_r(0x3b, 0x20, rs2, rs1, 0, rd),
            Sllw { rd, rs1, rs2 } => encode_r(0x3b, 0, rs2, rs1, 0x1, rd),
            Srlw { rd, rs1, rs2 } => encode_r(0x3b, 0, rs2, rs1, 0x5, rd),
            Sraw { rd, rs1, rs2 } => encode_r(0x3b, 0x20, rs2, rs1, 0x5, rd),
            Mulw { rd, rs1, rs2 } => encode_r(0x3b, 1, rs2, rs1, 0, rd),
            Divw { rd, rs1, rs2 } => encode_r(0x3b, 1, rs2, rs1, 4, rd),
            Divuw { rd, rs1, rs2 } => encode_r(0x3b, 1, rs2, rs1, 5, rd),
            Remw { rd, rs1, rs2 } => encode_r(0x3b, 1, rs2, rs1, 6, rd),
            Remuw { rd, rs1, rs2 } => encode_r(0x3b, 1, rs2, rs1, 7, rd),
            Addi { rd, rs1, imm } => encode_i(0x13, 0, rd, rs1, imm),
            Xori { rd, rs1, imm } => encode_i(0x13, 4, rd, rs1, imm),
            Ori { rd, rs1, imm } => encode_i(0x13, 6, rd, rs1, imm),
            Andi { rd, rs1, imm } => encode_i(0x13, 7, rd, rs1, imm),
            Slli { rd, rs1, imm } => encode_i(0x13, 1, rd, rs1, imm as i64),
            Srli { rd, rs1, imm } => encode_i(0x13, 5, rd, rs1, imm as i64),
            Srai { rd, rs1, imm } => encode_i(0x13, 5, rd, rs1, 0x400 | imm as i64 & 0x3f),
            Slti { rd, rs1, imm } => encode_i(0x13, 2, rd, rs1, imm),
            Sltiu { rd, rs1, imm } => encode_i(0x13, 3, rd, rs1, imm as i64),
            Addiw { rd, rs1, imm } => encode_i(0x1b, 0, rd, rs1, imm as i64),
            Slliw { rd, rs1, imm } => encode_i(0x1b, 1, rd, rs1, imm as i64),
            Srliw { rd, rs1, imm } => encode_i(0x1b, 5, rd, rs1, imm as i64),
            Sraiw { rd, rs1, imm } => encode_i(0x1b, 5, rd, rs1, 0x400 | imm as i64 & 0x1f),
            Lb { rd, rs1, imm } => encode_i(0x03, 0, rd, rs1, imm),
            Lh { rd, rs1, imm } => encode_i(0x03, 1, rd, rs1, imm),
            Lw { rd, rs1, imm } => encode_i(0x03, 2, rd, rs1, imm),
            Ld { rd, rs1, imm } => encode_i(0x03, 3, rd, rs1, imm),
            Lbu { rd, rs1, imm } => encode_i(0x03, 4, rd, rs1, imm),
            Lhu { rd, rs1, imm } => encode_i(0x03, 5, rd, rs1, imm),
            Lwu { rd, rs1, imm } => encode_i(0x03, 6, rd, rs1, imm),
            Sb { rs2, imm, rs1 } => encode_s(0, rs2, imm, rs1),
            Sh { rs2, imm, rs1 } => encode_s(1, rs2, imm, rs1),
            Sw { rs2, imm, rs1 } => encode_s(2, rs2, imm, rs1),
            Sd { rs2, imm, rs1 } => encode_s(3, rs2, imm, rs1),
            Beq { rs1, rs2, imm } => encode_b(0, rs1, rs2, imm),
            Bne { rs1, rs2, imm } => encode_b(1, rs1, rs2, imm),
            Blt { rs1, rs2, imm } => encode_b(4, rs1, rs2, imm),
            Bge { rs1, rs2, imm } => encode_b(5, rs1, rs2, imm),
            Bltu { rs1, rs2, imm } => encode_b(6, rs1, rs2, imm),
            Bgeu { rs1, rs2, imm } => encode_b(7, rs1, rs2, imm),
            Jal { rd, imm } => encode_j(rd, imm),
            Jalr { rd, rs1, imm } => encode_i(0x67, 0, rd, rs1, imm),
            Lui { rd, imm } => (imm as u32 & 0xffff_f000) | ((rd as u32 & 0x1f) << 7) | 0x37,
            Auipc { rd, imm } => (imm as u32 & 0xffff_f000) | ((rd as u32 & 0x1f) << 7) | 0x17,
            Ecall => 0x0000_0073,
            Ebreak => 0x0010_0073,
            Fence => 0x0000_000f,
            Csrrw { rd, rs1, csr } => encode_csr(0x73, 0x1, rd, rs1, csr),
            Csrrs { rd, rs1, csr } => encode_csr(0x73, 0x2, rd, rs1, csr),
            Csrrc { rd, rs1, csr } => encode_csr(0x73, 0x3, rd, rs1, csr),
            Csrrwi { rd, imm, csr } => encode_csr(0x73, 0x5, rd, imm as usize, csr),
            Csrrsi { rd, imm, csr } => encode_csr(0x73, 0x6, rd, imm as usize, csr),
            Csrrci { rd, imm, csr } => encode_csr(0x73, 0x7, rd, imm as usize, csr),
        }
    }
}

#[cfg(test)]
mod parser_tests {
    use super::Instruction::{self, *};

    #[inline]
    fn assert_round_trip(word: u32, expected: Instruction) {
        assert_eq!(Instruction::try_from(word).unwrap(), expected);
        assert_eq!(u32::from(expected), word);
    }

    #[test]
    fn decodes_add() {
        assert_round_trip(
            0b0000000_00011_00010_000_00001_0110011,
            Add {
                rd: 1,
                rs1: 2,
                rs2: 3,
            },
        );
    }

    #[test]
    fn decodes_sub() {
        assert_round_trip(
            0b0100000_00011_00010_000_00001_0110011,
            Sub {
                rd: 1,
                rs1: 2,
                rs2: 3,
            },
        );
    }

    #[test]
    fn decodes_xor() {
        assert_round_trip(
            0b0000000_00011_00010_100_00001_0110011,
            Xor {
                rd: 1,
                rs1: 2,
                rs2: 3,
            },
        );
    }

    #[test]
    fn decodes_or() {
        assert_round_trip(
            0b0000000_00011_00010_110_00001_0110011,
            Or {
                rd: 1,
                rs1: 2,
                rs2: 3,
            },
        );
    }

    #[test]
    fn decodes_and() {
        assert_round_trip(
            0b0000000_00011_00010_111_00001_0110011,
            And {
                rd: 1,
                rs1: 2,
                rs2: 3,
            },
        );
    }

    #[test]
    fn decodes_sll() {
        assert_round_trip(
            0b0000000_00011_00010_001_00001_0110011,
            Sll {
                rd: 1,
                rs1: 2,
                rs2: 3,
            },
        );
    }

    #[test]
    fn decodes_srl() {
        assert_round_trip(
            0b0000000_00011_00010_101_00001_0110011,
            Srl {
                rd: 1,
                rs1: 2,
                rs2: 3,
            },
        );
    }

    #[test]
    fn decodes_sra() {
        assert_round_trip(
            0b0100000_00011_00010_101_00001_0110011,
            Sra {
                rd: 1,
                rs1: 2,
                rs2: 3,
            },
        );
    }

    #[test]
    fn decodes_slt() {
        assert_round_trip(
            0b0000000_00011_00010_010_00001_0110011,
            Slt {
                rd: 1,
                rs1: 2,
                rs2: 3,
            },
        );
    }

    #[test]
    fn decodes_sltu() {
        assert_round_trip(
            0b0000000_00011_00010_011_00001_0110011,
            Sltu {
                rd: 1,
                rs1: 2,
                rs2: 3,
            },
        );
    }

    #[test]
    fn decodes_andw() {
        assert_round_trip(
            0b0000000_00011_00010_000_00001_0111011,
            Addw {
                rd: 1,
                rs1: 2,
                rs2: 3,
            },
        );
    }

    #[test]
    fn decodes_subw() {
        assert_round_trip(
            0b0100000_00011_00010_000_00001_0111011,
            Subw {
                rd: 1,
                rs1: 2,
                rs2: 3,
            },
        );
    }

    #[test]
    fn decodes_sllw() {
        assert_round_trip(
            0b0000000_00011_00010_001_00001_0111011,
            Sllw {
                rd: 1,
                rs1: 2,
                rs2: 3,
            },
        );
    }

    #[test]
    fn decodes_srlw() {
        assert_round_trip(
            0b0000000_00011_00010_101_00001_0111011,
            Srlw {
                rd: 1,
                rs1: 2,
                rs2: 3,
            },
        );
    }

    #[test]
    fn decodes_sraw() {
        assert_round_trip(
            0b0100000_00011_00010_101_00001_0111011,
            Sraw {
                rd: 1,
                rs1: 2,
                rs2: 3,
            },
        );
    }

    #[test]
    fn decodes_addi() {
        assert_round_trip(
            0b1000_0000_0000_00010_000_00001_0010011,
            Addi {
                rd: 1,
                rs1: 2,
                imm: -2048,
            },
        );
    }

    #[test]
    fn decodes_xori() {
        assert_round_trip(
            0b1000_0000_0000_00010_100_00001_0010011,
            Xori {
                rd: 1,
                rs1: 2,
                imm: -2048,
            },
        );
    }

    #[test]
    fn decodes_ori() {
        assert_round_trip(
            0b1000_0000_0000_00010_110_00001_0010011,
            Ori {
                rd: 1,
                rs1: 2,
                imm: -2048,
            },
        );
    }

    #[test]
    fn decodes_andi() {
        assert_round_trip(
            0b1000_0000_0000_00010_111_00001_0010011,
            Andi {
                rd: 1,
                rs1: 2,
                imm: -2048,
            },
        );
    }

    #[test]
    fn decodes_slli() {
        assert_round_trip(
            0b0000_0000_1000_00010_001_00001_0010011,
            Slli {
                rd: 1,
                rs1: 2,
                imm: 8,
            },
        );
    }

    #[test]
    fn decodes_srli() {
        assert_round_trip(
            0b0000_0000_1000_00010_101_00001_0010011,
            Srli {
                rd: 1,
                rs1: 2,
                imm: 8,
            },
        );
    }

    #[test]
    fn decodes_srai() {
        assert_round_trip(
            0b0100_0000_1000_00010_101_00001_0010011,
            Srai {
                rd: 1,
                rs1: 2,
                imm: 8,
            },
        );
    }

    #[test]
    fn decodes_slti() {
        assert_round_trip(
            0b1000_0000_0000_00010_010_00001_0010011,
            Slti {
                rd: 1,
                rs1: 2,
                imm: -2048,
            },
        );
    }

    #[test]
    fn decodes_sltiu() {
        assert_round_trip(
            0b1000_0000_0000_00010_011_00001_0010011,
            Sltiu {
                rd: 1,
                rs1: 2,
                imm: 0xffff_ffff_ffff_f800,
            },
        );
    }

    #[test]
    fn decodes_lb() {
        assert_round_trip(
            0b1000_0000_0000_00010_000_00001_0000011,
            Lb {
                rd: 1,
                rs1: 2,
                imm: -2048,
            },
        );
    }

    #[test]
    fn decodes_lh() {
        assert_round_trip(
            0b1000_0000_0000_00010_001_00001_0000011,
            Lh {
                rd: 1,
                rs1: 2,
                imm: -2048,
            },
        );
    }

    #[test]
    fn decodes_lw() {
        assert_round_trip(
            0b1000_0000_0000_00010_010_00001_0000011,
            Lw {
                rd: 1,
                rs1: 2,
                imm: -2048,
            },
        );
    }

    #[test]
    fn decodes_lbu() {
        assert_round_trip(
            0b1000_0000_0000_00010_100_00001_0000011,
            Lbu {
                rd: 1,
                rs1: 2,
                imm: -2048,
            },
        );
    }

    #[test]
    fn decodes_lhu() {
        assert_round_trip(
            0b1000_0000_0000_00010_101_00001_0000011,
            Lhu {
                rd: 1,
                rs1: 2,
                imm: -2048,
            },
        );
    }

    #[test]
    fn decodes_sb() {
        assert_round_trip(
            0b1000000_00011_00010_000_00000_0100011,
            Sb {
                rs2: 3,
                imm: -2048,
                rs1: 2,
            },
        );
    }

    #[test]
    fn decodes_sh() {
        assert_round_trip(
            0b1000000_00011_00010_001_00000_0100011,
            Sh {
                rs2: 3,
                imm: -2048,
                rs1: 2,
            },
        );
    }

    #[test]
    fn decodes_sw() {
        assert_round_trip(
            0b1000000_00011_00010_010_00000_0100011,
            Sw {
                rs2: 3,
                imm: -2048,
                rs1: 2,
            },
        );
    }

    #[test]
    fn decodes_beq() {
        assert_round_trip(
            0b1000000_00011_00010_000_00001_1100011,
            Beq {
                rs1: 2,
                rs2: 3,
                imm: -2048,
            },
        );
    }

    #[test]
    fn decodes_bne() {
        assert_round_trip(
            0b1000000_00011_00010_001_00001_1100011,
            Bne {
                rs1: 2,
                rs2: 3,
                imm: -2048,
            },
        );
    }

    #[test]
    fn decodes_blt() {
        assert_round_trip(
            0b1000000_00011_00010_100_00001_1100011,
            Blt {
                rs1: 2,
                rs2: 3,
                imm: -2048,
            },
        );
    }

    #[test]
    fn decodes_bge() {
        assert_round_trip(
            0b1000000_00011_00010_101_00001_1100011,
            Bge {
                rs1: 2,
                rs2: 3,
                imm: -2048,
            },
        );
    }

    #[test]
    fn decodes_bltu() {
        assert_round_trip(
            0b1000000_00011_00010_110_00001_1100011,
            Bltu {
                rs1: 2,
                rs2: 3,
                imm: -2048,
            },
        );
    }

    #[test]
    fn decodes_bgeu() {
        assert_round_trip(
            0b1000000_00011_00010_111_00001_1100011,
            Bgeu {
                rs1: 2,
                rs2: 3,
                imm: -2048,
            },
        );
    }

    #[test]
    fn decodes_jal() {
        assert_round_trip(
            0b1_0000000000_1_11111111_00001_1101111,
            Jal { rd: 1, imm: -2048 },
        );
    }

    #[test]
    fn decodes_jalr() {
        assert_round_trip(
            0b100000000000_00010_000_00001_1100111,
            Jalr {
                rd: 1,
                rs1: 2,
                imm: -2048,
            },
        );
    }

    #[test]
    fn decodes_lui() {
        assert_round_trip(
            0b00010010001101000101_00001_0110111,
            Lui {
                rd: 1,
                imm: 0x1234_5000,
            },
        );
    }

    #[test]
    fn decodes_auipc() {
        assert_round_trip(
            0b00010010001101000101_00001_0010111,
            Auipc {
                rd: 1,
                imm: 0x1234_5000,
            },
        );
    }

    #[test]
    fn decodes_fence() {
        assert_round_trip(0x0000_000f, Fence);
    }

    #[test]
    fn decodes_fence_tso() {
        assert_eq!(Instruction::try_from(0x8330_000f).unwrap(), Fence);
    }

    #[test]
    fn rejects_invalid_fence_encoding() {
        assert!(Instruction::try_from(0x8110_000f).is_err());
    }

    #[test]
    fn rejects_ecall_with_nonzero_rd() {
        assert!(Instruction::try_from(0x0000_00f3).is_err());
    }

    #[test]
    fn rejects_jalr_with_nonzero_funct3() {
        assert!(Instruction::try_from(0x0000_1067).is_err());
    }

    #[test]
    fn decodes_ecall() {
        assert_round_trip(0b0000_0000_0000_00000000_00000_1110011, Ecall);
    }

    #[test]
    fn decodes_ebreak() {
        assert_round_trip(0b0000_0000_0001_00000000_00000_1110011, Ebreak);
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
