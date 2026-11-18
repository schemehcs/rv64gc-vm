# RV64GC Instruction Set Opcode Reference Card

**RV64GC** = **RV64I** + **M** + **A** + **F** + **D** + **Zicsr** + **Zifencei** + **C** (16-bit Compressed).
Standard 32-bit instructions end with `opcode[1:0] = 11`; 16-bit compressed instructions end with `op[1:0] = 00`, `01`, or `10`.

---

## 1. RV64I — Base Integer Instruction Set (52 Instructions)

| Mnemonic | Fmt | `opcode` `[6:0]` (Hex) | `funct3` `[14:12]` | `funct7` `[31:25]` / Imm | Operation |
| :--- | :---: | :---: | :---: | :---: | :--- |
| **`lui`** | U | `0110111` (`0x37`) | — | `imm[31:12]` | `rd = sext(imm[31:12] << 12)` |
| **`auipc`** | U | `0010111` (`0x17`) | — | `imm[31:12]` | `rd = pc + sext(imm[31:12] << 12)` |
| **`jal`** | J | `1101111` (`0x6F`) | — | `imm[20|10:1|11|19:12]` | `rd = pc + 4; pc += sext(imm)` |
| **`jalr`** | I | `1100111` (`0x67`) | `000` | `imm[11:0]` | `rd = pc + 4; pc = (rs1 + sext(imm)) & ~1` |
| **`beq`** | B | `1100011` (`0x63`) | `000` | `imm[12|10:5|4:1|11]` | `if (rs1 == rs2) pc += sext(imm)` |
| **`bne`** | B | `1100011` (`0x63`) | `001` | `imm[12|10:5|4:1|11]` | `if (rs1 != rs2) pc += sext(imm)` |
| **`blt`** | B | `1100011` (`0x63`) | `100` | `imm[12|10:5|4:1|11]` | `if (rs1 < rs2) pc += sext(imm)` (signed) |
| **`bge`** | B | `1100011` (`0x63`) | `101` | `imm[12|10:5|4:1|11]` | `if (rs1 >= rs2) pc += sext(imm)` (signed) |
| **`bltu`** | B | `1100011` (`0x63`) | `110` | `imm[12|10:5|4:1|11]` | `if (rs1 < rs2) pc += sext(imm)` (unsigned) |
| **`bgeu`** | B | `1100011` (`0x63`) | `111` | `imm[12|10:5|4:1|11]` | `if (rs1 >= rs2) pc += sext(imm)` (unsigned) |
| **`lb`** | I | `0000011` (`0x03`) | `000` | `imm[11:0]` | `rd = sext(M[rs1 + sext(imm)][7:0])` |
| **`lh`** | I | `0000011` (`0x03`) | `001` | `imm[11:0]` | `rd = sext(M[rs1 + sext(imm)][15:0])` |
| **`lw`** | I | `0000011` (`0x03`) | `010` | `imm[11:0]` | `rd = sext(M[rs1 + sext(imm)][31:0])` |
| **`ld`** | I | `0000011` (`0x03`) | `011` | `imm[11:0]` | `rd = M[rs1 + sext(imm)][63:0]` |
| **`lbu`** | I | `0000011` (`0x03`) | `100` | `imm[11:0]` | `rd = zext(M[rs1 + sext(imm)][7:0])` |
| **`lhu`** | I | `0000011` (`0x03`) | `101` | `imm[11:0]` | `rd = zext(M[rs1 + sext(imm)][15:0])` |
| **`lwu`** | I | `0000011` (`0x03`) | `110` | `imm[11:0]` | `rd = zext(M[rs1 + sext(imm)][31:0])` |
| **`sb`** | S | `0100011` (`0x23`) | `000` | `imm[11:5|4:0]` | `M[rs1 + sext(imm)][7:0] = rs2[7:0]` |
| **`sh`** | S | `0100011` (`0x23`) | `001` | `imm[11:5|4:0]` | `M[rs1 + sext(imm)][15:0] = rs2[15:0]` |
| **`sw`** | S | `0100011` (`0x23`) | `010` | `imm[11:5|4:0]` | `M[rs1 + sext(imm)][31:0] = rs2[31:0]` |
| **`sd`** | S | `0100011` (`0x23`) | `011` | `imm[11:5|4:0]` | `M[rs1 + sext(imm)][63:0] = rs2[63:0]` |
| **`addi`** | I | `0010011` (`0x13`) | `000` | `imm[11:0]` | `rd = rs1 + sext(imm)` |
| **`slti`** | I | `0010011` (`0x13`) | `010` | `imm[11:0]` | `rd = (rs1 < sext(imm)) ? 1 : 0` (signed) |
| **`sltiu`** | I | `0010011` (`0x13`) | `011` | `imm[11:0]` | `rd = (rs1 < sext(imm)) ? 1 : 0` (unsigned) |
| **`xori`** | I | `0010011` (`0x13`) | `100` | `imm[11:0]` | `rd = rs1 ^ sext(imm)` |
| **`ori`** | I | `0010011` (`0x13`) | `110` | `imm[11:0]` | `rd = rs1 | sext(imm)` |
| **`andi`** | I | `0010011` (`0x13`) | `111` | `imm[11:0]` | `rd = rs1 & sext(imm)` |
| **`slli`** | I | `0010011` (`0x13`) | `001` | `000000` + `shamt[5:0]` | `rd = rs1 << shamt` |
| **`srli`** | I | `0010011` (`0x13`) | `101` | `000000` + `shamt[5:0]` | `rd = rs1 >> shamt` (logical) |
| **`srai`** | I | `0010011` (`0x13`) | `101` | `010000` + `shamt[5:0]` | `rd = rs1 >> shamt` (arithmetic) |
| **`addiw`** | I | `0011011` (`0x1B`) | `000` | `imm[11:0]` | `rd = sext((rs1 + sext(imm))[31:0])` |
| **`slliw`** | I | `0011011` (`0x1B`) | `001` | `0000000` + `shamt[4:0]` | `rd = sext((rs1[31:0] << shamt)[31:0])` |
| **`srliw`** | I | `0011011` (`0x1B`) | `101` | `0000000` + `shamt[4:0]` | `rd = sext(rs1[31:0] >> shamt)` (logical) |
| **`sraiw`** | I | `0011011` (`0x1B`) | `101` | `0100000` + `shamt[4:0]` | `rd = sext(rs1[31:0] >> shamt)` (arithmetic) |
| **`add`** | R | `0110011` (`0x33`) | `000` | `0000000` (`0x00`) | `rd = rs1 + rs2` |
| **`sub`** | R | `0110011` (`0x33`) | `000` | `0100000` (`0x20`) | `rd = rs1 - rs2` |
| **`sll`** | R | `0110011` (`0x33`) | `001` | `0000000` (`0x00`) | `rd = rs1 << rs2[5:0]` |
| **`slt`** | R | `0110011` (`0x33`) | `010` | `0000000` (`0x00`) | `rd = (rs1 < rs2) ? 1 : 0` (signed) |
| **`sltu`** | R | `0110011` (`0x33`) | `011` | `0000000` (`0x00`) | `rd = (rs1 < rs2) ? 1 : 0` (unsigned) |
| **`xor`** | R | `0110011` (`0x33`) | `100` | `0000000` (`0x00`) | `rd = rs1 ^ rs2` |
| **`srl`** | R | `0110011` (`0x33`) | `101` | `0000000` (`0x00`) | `rd = rs1 >> rs2[5:0]` (logical) |
| **`sra`** | R | `0110011` (`0x33`) | `101` | `0100000` (`0x20`) | `rd = rs1 >> rs2[5:0]` (arithmetic) |
| **`or`** | R | `0110011` (`0x33`) | `110` | `0000000` (`0x00`) | `rd = rs1 | rs2` |
| **`and`** | R | `0110011` (`0x33`) | `111` | `0000000` (`0x00`) | `rd = rs1 & rs2` |
| **`addw`** | R | `0111011` (`0x3B`) | `000` | `0000000` (`0x00`) | `rd = sext((rs1 + rs2)[31:0])` |
| **`subw`** | R | `0111011` (`0x3B`) | `000` | `0100000` (`0x20`) | `rd = sext((rs1 - rs2)[31:0])` |
| **`sllw`** | R | `0111011` (`0x3B`) | `001` | `0000000` (`0x00`) | `rd = sext((rs1[31:0] << rs2[4:0])[31:0])` |
| **`srlw`** | R | `0111011` (`0x3B`) | `101` | `0000000` (`0x00`) | `rd = sext(rs1[31:0] >> rs2[4:0])` (logical) |
| **`sraw`** | R | `0111011` (`0x3B`) | `101` | `0100000` (`0x20`) | `rd = sext(rs1[31:0] >> rs2[4:0])` (arithmetic) |
| **`fence`** | I | `0001111` (`0x0F`) | `000` | `fm[3:0]|pred[3:0]|succ[3:0]` | Memory/I/O ordering fence |
| **`ecall`** | I | `1110011` (`0x73`) | `000` | `000000000000` (`0x000`) | Environment call (trap to OS/EEI) |
| **`ebreak`** | I | `1110011` (`0x73`) | `000` | `000000000001` (`0x001`) | Breakpoint exception |

---

## 2. Zifencei & Zicsr — Instruction-Fetch Fence & CSR Access (7 Instructions)

| Mnemonic | Fmt | `opcode` `[6:0]` (Hex) | `funct3` `[14:12]` | `imm` `[31:20]` | Operation |
| :--- | :---: | :---: | :---: | :---: | :--- |
| **`fence.i`** | I | `0001111` (`0x0F`) | `001` | `000000000000` (`0x000`) | Synchronize instruction & data streams |
| **`csrrw`** | I | `1110011` (`0x73`) | `001` | `csr[11:0]` | `t = CSRs[csr]; CSRs[csr] = rs1; rd = t` |
| **`csrrs`** | I | `1110011` (`0x73`) | `010` | `csr[11:0]` | `t = CSRs[csr]; CSRs[csr] = t | rs1; rd = t` |
| **`csrrc`** | I | `1110011` (`0x73`) | `011` | `csr[11:0]` | `t = CSRs[csr]; CSRs[csr] = t & ~rs1; rd = t` |
| **`csrrwi`** | I | `1110011` (`0x73`) | `101` | `csr[11:0]` | `rd = CSRs[csr]; CSRs[csr] = zext(uimm[4:0])` |
| **`csrrsi`** | I | `1110011` (`0x73`) | `110` | `csr[11:0]` | `t = CSRs[csr]; CSRs[csr] = t | zext(uimm[4:0]); rd = t` |
| **`csrrci`** | I | `1110011` (`0x73`) | `111` | `csr[11:0]` | `t = CSRs[csr]; CSRs[csr] = t & ~zext(uimm[4:0]); rd = t` |

---

## 3. RV64M — Integer Multiplication & Division (13 Instructions)

All RV64M instructions are **R-Type** with `funct7 = 0000001` (`0x01`).

| Mnemonic | Fmt | `opcode` `[6:0]` (Hex) | `funct3` `[14:12]` | `funct7` `[31:25]` | Operation |
| :--- | :---: | :---: | :---: | :---: | :--- |
| **`mul`** | R | `0110011` (`0x33`) | `000` | `0000001` (`0x01`) | `rd = (rs1 * rs2)[63:0]` |
| **`mulh`** | R | `0110011` (`0x33`) | `001` | `0000001` (`0x01`) | `rd = (rs1 * rs2)[127:64]` (signed × signed) |
| **`mulhsu`** | R | `0110011` (`0x33`) | `010` | `0000001` (`0x01`) | `rd = (rs1 * rs2)[127:64]` (signed × unsigned) |
| **`mulhu`** | R | `0110011` (`0x33`) | `011` | `0000001` (`0x01`) | `rd = (rs1 * rs2)[127:64]` (unsigned × unsigned) |
| **`div`** | R | `0110011` (`0x33`) | `100` | `0000001` (`0x01`) | `rd = rs1 / rs2` (signed) |
| **`divu`** | R | `0110011` (`0x33`) | `101` | `0000001` (`0x01`) | `rd = rs1 / rs2` (unsigned) |
| **`rem`** | R | `0110011` (`0x33`) | `110` | `0000001` (`0x01`) | `rd = rs1 % rs2` (signed) |
| **`remu`** | R | `0110011` (`0x33`) | `111` | `0000001` (`0x01`) | `rd = rs1 % rs2` (unsigned) |
| **`mulw`** | R | `0111011` (`0x3B`) | `000` | `0000001` (`0x01`) | `rd = sext((rs1[31:0] * rs2[31:0])[31:0])` |
| **`divw`** | R | `0111011` (`0x3B`) | `100` | `0000001` (`0x01`) | `rd = sext(rs1[31:0] / rs2[31:0])` (signed) |
| **`divuw`** | R | `0111011` (`0x3B`) | `101` | `0000001` (`0x01`) | `rd = sext(rs1[31:0] / rs2[31:0])` (unsigned) |
| **`remw`** | R | `0111011` (`0x3B`) | `110` | `0000001` (`0x01`) | `rd = sext(rs1[31:0] % rs2[31:0])` (signed) |
| **`remuw`** | R | `0111011` (`0x3B`) | `111` | `0000001` (`0x01`) | `rd = sext(rs1[31:0] % rs2[31:0])` (unsigned) |

---

## 4. RV64A — Atomic Memory Operations (22 Instructions)

All RV64A instructions use **R-Type** with `opcode = 0101111` (`0x2F`).
- **Width (`funct3`):** `010` = `.w` (32-bit, sign-extended to 64-bit `rd`), `011` = `.d` (64-bit).
- **Ordering (`inst[26:25]`):** `aq` (`inst[26]`, acquire), `rl` (`inst[25]`, release).
- **Function (`funct5 = inst[31:27]`):**

| Mnemonic (`.w` / `.d`) | Fmt | `opcode` `[6:0]` (Hex) | `funct3` | `funct5` `[31:27]` | Operation (`*rs1` = `M[rs1]`) |
| :--- | :---: | :---: | :---: | :---: | :--- |
| **`lr.w` / `lr.d`** | R | `0101111` (`0x2F`) | `010` / `011` | `00010` (`0x02`) | Load-reserved (`rs2 = 00000`); sets reservation |
| **`sc.w` / `sc.d`** | R | `0101111` (`0x2F`) | `010` / `011` | `00011` (`0x03`) | Store-conditional `rs2`; `rd = 0` on success, `!= 0` on fail |
| **`amoswap.w` / `.d`** | R | `0101111` (`0x2F`) | `010` / `011` | `00001` (`0x01`) | `t = *rs1; *rs1 = rs2; rd = t` |
| **`amoadd.w` / `.d`** | R | `0101111` (`0x2F`) | `010` / `011` | `00000` (`0x00`) | `t = *rs1; *rs1 = t + rs2; rd = t` |
| **`amoxor.w` / `.d`** | R | `0101111` (`0x2F`) | `010` / `011` | `00100` (`0x04`) | `t = *rs1; *rs1 = t ^ rs2; rd = t` |
| **`amoand.w` / `.d`** | R | `0101111` (`0x2F`) | `010` / `011` | `01100` (`0x0C`) | `t = *rs1; *rs1 = t & rs2; rd = t` |
| **`amoor.w` / `.d`** | R | `0101111` (`0x2F`) | `010` / `011` | `01000` (`0x08`) | `t = *rs1; *rs1 = t | rs2; rd = t` |
| **`amomin.w` / `.d`** | R | `0101111` (`0x2F`) | `010` / `011` | `10000` (`0x10`) | `t = *rs1; *rs1 = min(t, rs2); rd = t` (signed) |
| **`amomax.w` / `.d`** | R | `0101111` (`0x2F`) | `010` / `011` | `10100` (`0x14`) | `t = *rs1; *rs1 = max(t, rs2); rd = t` (signed) |
| **`amominu.w` / `.d`** | R | `0101111` (`0x2F`) | `010` / `011` | `11000` (`0x18`) | `t = *rs1; *rs1 = min(t, rs2); rd = t` (unsigned) |
| **`amomaxu.w` / `.d`** | R | `0101111` (`0x2F`) | `010` / `011` | `11100` (`0x1C`) | `t = *rs1; *rs1 = max(t, rs2); rd = t` (unsigned) |

---

## 5. RV64F & RV64D — Single & Double Precision Floating-Point (62 Instructions)

- **Format (`fmt`, `inst[26:25]`):** `00` = `.s` (32-bit single, NaN-boxed in `f0`–`f31`), `01` = `.d` (64-bit double).
- **Rounding Mode (`rm`, `inst[14:12]`):** `000` RNE, `001` RTZ, `010` RDN, `011` RUP, `100` RMM, `111` DYN (use `frm` CSR).

| Mnemonic (`.s` / `.d`) | Fmt | `opcode` `[6:0]` (Hex) | `funct3` / `rm` | `funct7` `[31:25]` / `rs2` | Operation |
| :--- | :---: | :---: | :---: | :---: | :--- |
| **`flw` / `fld`** | I | `0000111` (`0x07`) | `010` / `011` | `imm[11:0]` | `rd = M[rs1 + sext(imm)]` (FP load) |
| **`fsw` / `fsd`** | S | `0100111` (`0x27`) | `010` / `011` | `imm[11:5|4:0]` | `M[rs1 + sext(imm)] = rs2` (FP store) |
| **`fmadd.s` / `.d`** | R4 | `1000011` (`0x43`) | `rm` | `rs3[31:27]` + `fmt[26:25]` | `rd = (rs1 * rs2) + rs3` |
| **`fmsub.s` / `.d`** | R4 | `1000111` (`0x47`) | `rm` | `rs3[31:27]` + `fmt[26:25]` | `rd = (rs1 * rs2) - rs3` |
| **`fnmsub.s` / `.d`** | R4 | `1001011` (`0x4B`) | `rm` | `rs3[31:27]` + `fmt[26:25]` | `rd = -(rs1 * rs2) + rs3` |
| **`fnmadd.s` / `.d`** | R4 | `1001111` (`0x4F`) | `rm` | `rs3[31:27]` + `fmt[26:25]` | `rd = -(rs1 * rs2) - rs3` |
| **`fadd.s` / `.d`** | R | `1010011` (`0x53`) | `rm` | `00000` + `fmt` | `rd = rs1 + rs2` |
| **`fsub.s` / `.d`** | R | `1010011` (`0x53`) | `rm` | `00001` + `fmt` | `rd = rs1 - rs2` |
| **`fmul.s` / `.d`** | R | `1010011` (`0x53`) | `rm` | `00010` + `fmt` | `rd = rs1 * rs2` |
| **`fdiv.s` / `.d`** | R | `1010011` (`0x53`) | `rm` | `00011` + `fmt` | `rd = rs1 / rs2` |
| **`fsqrt.s` / `.d`** | R | `1010011` (`0x53`) | `rm` | `01011` + `fmt` (`rs2=00000`) | `rd = sqrt(rs1)` |
| **`fsgnj.s` / `.d`** | R | `1010011` (`0x53`) | `000` | `00100` + `fmt` | `rd = abs(rs1)` with sign of `rs2` |
| **`fsgnjn.s` / `.d`** | R | `1010011` (`0x53`) | `001` | `00100` + `fmt` | `rd = abs(rs1)` with negated sign of `rs2` |
| **`fsgnjx.s` / `.d`** | R | `1010011` (`0x53`) | `010` | `00100` + `fmt` | `rd = rs1` with sign `sign(rs1) ^ sign(rs2)` |
| **`fmin.s` / `.d`** | R | `1010011` (`0x53`) | `000` | `00101` + `fmt` | `rd = min(rs1, rs2)` |
| **`fmax.s` / `.d`** | R | `1010011` (`0x53`) | `001` | `00101` + `fmt` | `rd = max(rs1, rs2)` |
| **`fcvt.s.d`** | R | `1010011` (`0x53`) | `rm` | `0100000` (`rs2=00001`) | Convert double `rs1` to single `rd` |
| **`fcvt.d.s`** | R | `1010011` (`0x53`) | `rm` | `0100001` (`rs2=00000`) | Convert single `rs1` to double `rd` |
| **`feq.s` / `.d`** | R | `1010011` (`0x53`) | `010` | `10100` + `fmt` | `rd (int) = (rs1 == rs2) ? 1 : 0` |
| **`flt.s` / `.d`** | R | `1010011` (`0x53`) | `001` | `10100` + `fmt` | `rd (int) = (rs1 < rs2) ? 1 : 0` |
| **`fle.s` / `.d`** | R | `1010011` (`0x53`) | `000` | `10100` + `fmt` | `rd (int) = (rs1 <= rs2) ? 1 : 0` |
| **`fclass.s` / `.d`** | R | `1010011` (`0x53`) | `001` | `11100` + `fmt` (`rs2=00000`) | `rd (int) = 10-bit FP classification mask` |
| **`fcvt.w.s` / `.d`** | R | `1010011` (`0x53`) | `rm` | `11000` + `fmt` (`rs2=00000`) | FP `rs1` → signed 32-bit int (sext to `rd`) |
| **`fcvt.wu.s` / `.d`** | R | `1010011` (`0x53`) | `rm` | `11000` + `fmt` (`rs2=00001`) | FP `rs1` → unsigned 32-bit int (sext to `rd`) |
| **`fcvt.l.s` / `.d`** | R | `1010011` (`0x53`) | `rm` | `11000` + `fmt` (`rs2=00010`) | FP `rs1` → signed 64-bit int `rd` |
| **`fcvt.lu.s` / `.d`** | R | `1010011` (`0x53`) | `rm` | `11000` + `fmt` (`rs2=00011`) | FP `rs1` → unsigned 64-bit int `rd` |
| **`fcvt.s.w` / `fcvt.d.w`** | R | `1010011` (`0x53`) | `rm` | `11010` + `fmt` (`rs2=00000`) | Signed 32-bit int `rs1` → FP `rd` |
| **`fcvt.s.wu` / `fcvt.d.wu`** | R | `1010011` (`0x53`) | `rm` | `11010` + `fmt` (`rs2=00001`) | Unsigned 32-bit int `rs1` → FP `rd` |
| **`fcvt.s.l` / `fcvt.d.l`** | R | `1010011` (`0x53`) | `rm` | `11010` + `fmt` (`rs2=00010`) | Signed 64-bit int `rs1` → FP `rd` |
| **`fcvt.s.lu` / `fcvt.d.lu`** | R | `1010011` (`0x53`) | `rm` | `11010` + `fmt` (`rs2=00011`) | Unsigned 64-bit int `rs1` → FP `rd` |
| **`fmv.x.w` / `fmv.x.d`** | R | `1010011` (`0x53`) | `000` | `1110000` / `1110001` (`rs2=00000`) | Move raw FP bits `rs1` → int `rd` (`.w` sexts) |
| **`fmv.w.x` / `fmv.d.x`** | R | `1010011` (`0x53`) | `000` | `1111000` / `1111001` (`rs2=00000`) | Move raw int bits `rs1` → FP `rd` |

---

## 6. RVC (RV64C) — 16-Bit Compressed Extension

Compressed instructions are **16 bits wide** (`inst[15:0]`) and expand 1-to-1 into standard 32-bit RV64GC instructions.
- **Quadrant (`op = inst[1:0]`):** `00` (C0), `01` (C1), `10` (C2). (`11` is reserved for >=32-bit instructions).
- **3-bit Compressed Registers (`rd'`, `rs1'`, `rs2'`):** Map `000`–`111` to `x8`–`x15` (or `f8`–`f15` for FP).

### Quadrant 0 (`op[1:0] = 00`)

| Mnemonic | Fmt | `funct3` `[15:13]` | Distinguishing Bits | Equivalent 32-Bit Instruction |
| :--- | :---: | :---: | :---: | :--- |
| **`c.addi4spn`** | CIW | `000` | `nzuimm[9:2] != 0` (`imm=0` is illegal) | `addi rd', x2, nzuimm` |
| **`c.fld`** | CL | `001` | `uimm[7:3]` | `fld rd', uimm(rs1')` |
| **`c.lw`** | CL | `010` | `uimm[6:2]` | `lw rd', uimm(rs1')` |
| **`c.ld`** | CL | `011` | `uimm[7:3]` | `ld rd', uimm(rs1')` |
| **`c.fsd`** | CS | `101` | `uimm[7:3]` | `fsd rs2', uimm(rs1')` |
| **`c.sw`** | CS | `110` | `uimm[6:2]` | `sw rs2', uimm(rs1')` |
| **`c.sd`** | CS | `111` | `uimm[7:3]` | `sd rs2', uimm(rs1')` |

### Quadrant 1 (`op[1:0] = 01`)

| Mnemonic | Fmt | `funct3` `[15:13]` | Distinguishing Bits | Equivalent 32-Bit Instruction |
| :--- | :---: | :---: | :---: | :--- |
| **`c.nop`** | CI | `000` | `rd = 0`, `imm = 0` | `addi x0, x0, 0` |
| **`c.addi`** | CI | `000` | `rd != 0`, `nzimm[5:0] != 0` | `addi rd, rd, nzimm` |
| **`c.addiw`** | CI | `001` | `rd != 0`, `imm[5:0]` | `addiw rd, rd, imm` |
| **`c.li`** | CI | `010` | `rd != 0`, `imm[5:0]` | `addi rd, x0, imm` |
| **`c.addi16sp`** | CI | `011` | `rd = 2` (`x2`), `nzimm[9:4] != 0` | `addi x2, x2, nzimm` |
| **`c.lui`** | CI | `011` | `rd != {0, 2}`, `nzimm[17:12] != 0` | `lui rd, nzimm` |
| **`c.srli`** | CB | `100` | `inst[11:10] = 00`, `shamt[5:0] != 0` | `srli rd', rd', shamt` |
| **`c.srai`** | CB | `100` | `inst[11:10] = 01`, `shamt[5:0] != 0` | `srai rd', rd', shamt` |
| **`c.andi`** | CB | `100` | `inst[11:10] = 10`, `imm[5:0]` | `andi rd', rd', imm` |
| **`c.sub`** | CA | `100` | `inst[12:10] = 011`, `inst[6:5] = 00` | `sub rd', rd', rs2'` |
| **`c.xor`** | CA | `100` | `inst[12:10] = 011`, `inst[6:5] = 01` | `xor rd', rd', rs2'` |
| **`c.or`** | CA | `100` | `inst[12:10] = 011`, `inst[6:5] = 10` | `or rd', rd', rs2'` |
| **`c.and`** | CA | `100` | `inst[12:10] = 011`, `inst[6:5] = 11` | `and rd', rd', rs2'` |
| **`c.subw`** | CA | `100` | `inst[12:10] = 111`, `inst[6:5] = 00` | `subw rd', rd', rs2'` |
| **`c.addw`** | CA | `100` | `inst[12:10] = 111`, `inst[6:5] = 01` | `addw rd', rd', rs2'` |
| **`c.j`** | CJ | `101` | `imm[11:1]` | `jal x0, imm` |
| **`c.beqz`** | CB | `110` | `imm[8:1]` | `beq rs1', x0, imm` |
| **`c.bnez`** | CB | `111` | `imm[8:1]` | `bne rs1', x0, imm` |

### Quadrant 2 (`op[1:0] = 10`)

| Mnemonic | Fmt | `funct3` `[15:13]` | Distinguishing Bits | Equivalent 32-Bit Instruction |
| :--- | :---: | :---: | :---: | :--- |
| **`c.slli`** | CI | `000` | `rd != 0`, `shamt[5:0] != 0` | `slli rd, rd, shamt` |
| **`c.fldsp`** | CI | `001` | `uimm[8:3]` | `fld rd, uimm(x2)` |
| **`c.lwsp`** | CI | `010` | `rd != 0`, `uimm[7:2]` | `lw rd, uimm(x2)` |
| **`c.ldsp`** | CI | `011` | `rd != 0`, `uimm[8:3]` | `ld rd, uimm(x2)` |
| **`c.jr`** | CR | `100` | `inst = 0`, `rs1 != 0`, `rs2 = 0` | `jalr x0, 0(rs1)` |
| **`c.mv`** | CR | `100` | `inst = 0`, `rd != 0`, `rs2 != 0` | `add rd, x0, rs2` |
| **`c.ebreak`** | CR | `100` | `inst = 1`, `rs1 = 0`, `rs2 = 0` | `ebreak` |
| **`c.jalr`** | CR | `100` | `inst = 1`, `rs1 != 0`, `rs2 = 0` | `jalr x1, 0(rs1)` |
| **`c.add`** | CR | `100` | `inst = 1`, `rd != 0`, `rs2 != 0` | `add rd, rd, rs2` |
| **`c.fsdsp`** | CSS | `101` | `uimm[8:3]` | `fsd rs2, uimm(x2)` |
| **`c.swsp`** | CSS | `110` | `uimm[7:2]` | `sw rs2, uimm(x2)` |
| **`c.sdsp`** | CSS | `111` | `uimm[8:3]` | `sd rs2, uimm(x2)` |

---

## 7. Bit-Field Layout Cheat Sheet

### 32-Bit Standard Formats (`inst[31:0]`)
- **R :** `[31:25] funct7` | `[24:20] rs2` | `[19:15] rs1` | `[14:12] funct3` | `[11:7] rd` | `[6:0] opcode`
- **R4:** `[31:27] rs3` | `[26:25] fmt` | `[24:20] rs2` | `[19:15] rs1` | `[14:12] rm` | `[11:7] rd` | `[6:0] opcode`
- **I :** `[31:20] imm[11:0]` | `[19:15] rs1` | `[14:12] funct3` | `[11:7] rd` | `[6:0] opcode`
- **S :** `[31:25] imm[11:5]` | `[24:20] rs2` | `[19:15] rs1` | `[14:12] funct3` | `[11:7] imm[4:0]` | `[6:0] opcode`
- **B :** ` imm` | `[30:25] imm[10:5]` | `[24:20] rs2` | `[19:15] rs1` | `[14:12] funct3` | `[11:8] imm[4:1]` | ` imm` | `[6:0] opcode`
- **U :** `[31:12] imm[31:12]` | `[11:7] rd` | `[6:0] opcode`
- **J :** ` imm` | `[30:21] imm[10:1]` | ` imm` | `[19:12] imm[19:12]` | `[11:7] rd` | `[6:0] opcode`

### 16-Bit Compressed Formats (`inst[15:0]`)
- **CR :** `[15:12] funct4` | `[11:7] rd/rs1` | `[6:2] rs2` | `[1:0] op`
- **CI :** `[15:13] funct3` | ` imm` | `[11:7] rd/rs1` | `[6:2] imm` | `[1:0] op`
- **CSS:** `[15:13] funct3` | `[12:7] imm` | `[6:2] rs2` | `[1:0] op`
- **CIW:** `[15:13] funct3` | `[12:5] imm` | `[4:2] rd'` | `[1:0] op`
- **CL :** `[15:13] funct3` | `[12:10] imm` | `[9:7] rs1'` | `[6:5] imm` | `[4:2] rd'` | `[1:0] op`
- **CS :** `[15:13] funct3` | `[12:10] imm` | `[9:7] rs1'` | `[6:5] imm` | `[4:2] rs2'` | `[1:0] op`
- **CA :** `[15:10] funct6` | `[9:7] rd'/rs1'` | `[6:5] funct2` | `[4:2] rs2'` | `[1:0] op`
- **CB :** `[15:13] funct3` | `[12:10] offset` | `[9:7] rd'/rs1'` | `[6:2] offset` | `[1:0] op`
- **CJ :** `[15:13] funct3` | `[12:2] jump_target` | `[1:0] op`
