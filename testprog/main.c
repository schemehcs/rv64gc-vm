#include <stdint.h>

// Global initialized (.data) and uninitialized (.bss) variables
volatile uint32_t g_data_check = 0x12345678;
volatile uint32_t g_bss_check;

volatile uint64_t g_data_check_d = 0x1234567800000000;
volatile uint64_t g_bss_check_d;

// Helper function to test jal / jalr, stack frames, and recursion
uint32_t fibonacci(uint32_t n) {
    if (n <= 1) return n;
    return fibonacci(n - 1) + fibonacci(n - 2);
}

uint64_t fibonacci_d(uint64_t n) {
    if (n <= 1) return n;
    return fibonacci_d(n - 1) + fibonacci_d(n - 2);
}

int wcheck(void) {
    // 1. Validate .data and .bss memory initialization
    if (g_data_check != 0x12345678) return 0x1;
    g_bss_check = 0xCAFEBABE;
    if (g_bss_check != 0xCAFEBABE) return 0x2;

    // 2. ALU: Arithmetic & Logical (add, sub, xor, or, and)
    volatile int32_t a = 15;
    volatile int32_t b = -5;
    if ((a + b) != 10) return 0x3;
    if ((a - b) != 20) return 0x4;
    if ((a ^ 0xFF) != 0xF0) return 0x5;
    if ((a | 0xF0) != 0xFF) return 0x6;
    if ((a & 0x06) != 0x06) return 0x7;

    // 3. Shifts: Logical vs Arithmetic (sll, srl, sra)
    volatile uint32_t u_val = 0x80000010;
    volatile int32_t  s_val = -16; // 0xFFFFFFF0
    if ((u_val >> 4) != 0x08000001) return 0x8;  // Tests SRL (zero-fill)
    if ((s_val >> 2) != -4)         return 0x9;  // Tests SRA (sign-extend)
    if ((1u << 5) != 32)            return 0xa; // Tests SLL

    // 4. Signed vs Unsigned Comparisons (slt, sltu, blt, bltu, bge, bgeu)
    volatile int32_t  neg = -1;
    volatile uint32_t pos = 1;
    if (!(neg < (int32_t)pos))   return 0xb; // Signed: -1 < 1 is true
    if (!((uint32_t)neg > pos))  return 0xc; // Unsigned: 0xFFFFFFFF > 1 is true

    // 5. Sub-word Memory Access & Sign Extension (lb, lbu, lh, lhu, sb, sh, lw, sw)
    volatile uint8_t mem[4] = {0x80, 0x7F, 0xFF, 0x01};
    int8_t  signed_byte   = *(volatile int8_t *)&mem[0];
    uint8_t unsigned_byte = *(volatile uint8_t *)&mem[0];
    int16_t signed_half   = *(volatile int16_t *)&mem[2]; // 0x01FF
    if (signed_byte != -128)     return 0xd; // Tests LB sign-extension
    if (unsigned_byte != 128)    return 0xe; // Tests LBU zero-extension
    if (signed_half != 0x01FF)   return 0xf; // Tests LH little-endian load

    // 6. Function Calls & Stack Management (jal, jalr, sw/lw on sp)
    // fibonacci(7) = 13
    if (fibonacci(7) != 13)      return 0x10;

    volatile int32_t x = 15;
    volatile int32_t y = -4;
    if (x * y != -60) return 0x11;
    if (x / y != -3) return 0x12;
    if (x % y != 3) return 0x13;

    volatile uint32_t ux = 15;
    volatile uint32_t uy = 4;
    if (ux * uy != 60) return 0x14;
    if (ux / uy != 3) return 0x15;
    if (ux % uy != 3) return 0x16;

    // All tests passed! Write a magic signature to memory and return 0.
    g_bss_check = 0x0000600D;
    return 0;
}

int dcheck(void) {
    // 1. Validate .data and .bss memory initialization
    if (g_data_check_d != 0x1234567800000000) return 0x61;
    g_bss_check_d = 0xCAFEBABE;
    if (g_bss_check_d != 0xCAFEBABE) return 0x62;

    // 2. ALU: Arithmetic & Logical (add, sub, xor, or, and)
    volatile int64_t a = 15;
    volatile int64_t b = -5;
    if ((a + b) != 10) return 0x63;
    if ((a - b) != 20) return 0x64;
    if ((a ^ 0xFF) != 0xF0) return 0x65;
    if ((a | 0xF0) != 0xFF) return 0x66;
    if ((a & 0x06) != 0x06) return 0x67;

    // 3. Shifts: Logical vs Arithmetic (sll, srl, sra)
    volatile uint64_t u_val = 0x80000010;
    volatile int64_t  s_val = -16; // 0xFFFFFFF0
    if ((u_val >> 4) != 0x08000001) return 0x68;  // Tests SRL (zero-fill)
    if ((s_val >> 2) != -4)         return 0x69;  // Tests SRA (sign-extend)
    if ((1u << 5) != 32)            return 0x6a; // Tests SLL

    // 4. Signed vs Unsigned Comparisons (slt, sltu, blt, bltu, bge, bgeu)
    volatile int64_t  neg = -1;
    volatile uint64_t pos = 1;
    if (!(neg < (int64_t)pos))   return 0x6b; // Signed: -1 < 1 is true
    if (!((uint64_t)neg > pos))  return 0x6c; // Unsigned: 0xFFFFFFFF > 1 is true

    // 5. Sub-word Memory Access & Sign Extension (lb, lbu, lh, lhu, sb, sh, lw, sw)
    volatile uint8_t mem[4] = {0x80, 0x7F, 0xFF, 0x01};
    int8_t  signed_byte   = *(volatile int8_t *)&mem[0];
    uint8_t unsigned_byte = *(volatile uint8_t *)&mem[0];
    int16_t signed_half   = *(volatile int16_t *)&mem[2]; // 0x01FF
    if (signed_byte != -128)     return 0x6d; // Tests LB sign-extension
    if (unsigned_byte != 128)    return 0x6e; // Tests LBU zero-extension
    if (signed_half != 0x01FF)   return 0x6f; // Tests LH little-endian load

    // 6. Function Calls & Stack Management (jal, jalr, sw/lw on sp)
    // fibonacci(7) = 13
    if (fibonacci_d(7) != 13)      return 0x70;

    volatile int64_t x = 15;
    volatile int64_t y = -4;
    if (x * y != -60) return 0x71;
    if (x / y != -3) return 0x72;
    if (x % y != 3) return 0x73;

    volatile uint64_t ux = 15;
    volatile uint64_t uy = 4;
    if (ux * uy != 60) return 0x14;
    if (ux / uy != 3) return 0x15;
    if (ux % uy != 3) return 0x16;

    // All tests passed! Write a magic signature to memory and return 0.
    g_bss_check_d = 0x0000600D;
    return 0;
}

int main(void) {
    int wres = wcheck();
    if (wres != 0) {
        return wres;
    }
    return dcheck();
}
