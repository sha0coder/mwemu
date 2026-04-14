use crate::*;

// INSERTPS - Insert Scalar Single Precision Floating-Point Value
// Opcode: 66 0F 3A 21 /r ib       INSERTPS xmm1, xmm2/m32, imm8
//
// IMM8 format:
// - Bits 7-6: COUNT_S (source element selection, 0-3)
// - Bits 5-4: COUNT_D (destination element selection, 0-3)
// - Bits 3-0: ZMASK (zero mask for destination elements)

const DATA_ADDR: u64 = 0x3000;

// ============================================================================
// INSERTPS - Source selection tests (bits 7-6)
// ============================================================================

#[test]
fn test_insertps_xmm0_xmm1_src0_dst0() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x21, 0xc1, 0x00, 0xf4]; // INSERTPS XMM0, XMM1, 0x00 (src[0]->dst[0])
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_insertps_xmm0_xmm1_src1_dst0() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x21, 0xc1, 0x40, 0xf4]; // INSERTPS XMM0, XMM1, 0x40 (src[1]->dst[0])
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_insertps_xmm0_xmm1_src2_dst0() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x21, 0xc1, 0x80, 0xf4]; // INSERTPS XMM0, XMM1, 0x80 (src[2]->dst[0])
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_insertps_xmm0_xmm1_src3_dst0() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x21, 0xc1, 0xc0, 0xf4]; // INSERTPS XMM0, XMM1, 0xC0 (src[3]->dst[0])
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// INSERTPS - Destination selection tests (bits 5-4)
// ============================================================================

#[test]
fn test_insertps_xmm0_xmm1_src0_dst0_nozmask() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x21, 0xc1, 0x00, 0xf4]; // INSERTPS XMM0, XMM1, 0x00 (src[0]->dst[0])
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_insertps_xmm0_xmm1_src0_dst1() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x21, 0xc1, 0x10, 0xf4]; // INSERTPS XMM0, XMM1, 0x10 (src[0]->dst[1])
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_insertps_xmm0_xmm1_src0_dst2() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x21, 0xc1, 0x20, 0xf4]; // INSERTPS XMM0, XMM1, 0x20 (src[0]->dst[2])
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_insertps_xmm0_xmm1_src0_dst3() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x21, 0xc1, 0x30, 0xf4]; // INSERTPS XMM0, XMM1, 0x30 (src[0]->dst[3])
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// INSERTPS - Zero mask tests (bits 3-0)
// ============================================================================

#[test]
fn test_insertps_xmm0_xmm1_zmask_0001() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x21, 0xc1, 0x01, 0xf4]; // INSERTPS XMM0, XMM1, 0x01 (zero dst[0])
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_insertps_xmm0_xmm1_zmask_0010() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x21, 0xc1, 0x02, 0xf4]; // INSERTPS XMM0, XMM1, 0x02 (zero dst[1])
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_insertps_xmm0_xmm1_zmask_0100() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x21, 0xc1, 0x04, 0xf4]; // INSERTPS XMM0, XMM1, 0x04 (zero dst[2])
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_insertps_xmm0_xmm1_zmask_1000() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x21, 0xc1, 0x08, 0xf4]; // INSERTPS XMM0, XMM1, 0x08 (zero dst[3])
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_insertps_xmm0_xmm1_zmask_0011() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x21, 0xc1, 0x03, 0xf4]; // INSERTPS XMM0, XMM1, 0x03 (zero dst[0,1])
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_insertps_xmm0_xmm1_zmask_1100() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x21, 0xc1, 0x0c, 0xf4]; // INSERTPS XMM0, XMM1, 0x0C (zero dst[2,3])
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_insertps_xmm0_xmm1_zmask_1111() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x21, 0xc1, 0x0f, 0xf4]; // INSERTPS XMM0, XMM1, 0x0F (zero all)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// INSERTPS - Combined source, destination, and zero mask tests
// ============================================================================

#[test]
fn test_insertps_xmm0_xmm1_src1_dst2_zmask_0001() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x21, 0xc1, 0x61, 0xf4]; // INSERTPS XMM0, XMM1, 0x61 (src[1]->dst[2], zero dst[0])
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_insertps_xmm0_xmm1_src2_dst1_zmask_1000() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x21, 0xc1, 0x98, 0xf4]; // INSERTPS XMM0, XMM1, 0x98 (src[2]->dst[1], zero dst[3])
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_insertps_xmm0_xmm1_src3_dst3_zmask_0111() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x21, 0xc1, 0xf7, 0xf4]; // INSERTPS XMM0, XMM1, 0xF7 (src[3]->dst[3], zero dst[0,1,2])
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// INSERTPS - Different register combinations
// ============================================================================

#[test]
fn test_insertps_xmm2_xmm3_src0_dst1() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x21, 0xd3, 0x10, 0xf4]; // INSERTPS XMM2, XMM3, 0x10
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_insertps_xmm4_xmm5_src2_dst3() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x21, 0xe5, 0xb0, 0xf4]; // INSERTPS XMM4, XMM5, 0xB0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_insertps_xmm6_xmm7_src1_dst0_zmask_0110() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x21, 0xf7, 0x46, 0xf4]; // INSERTPS XMM6, XMM7, 0x46
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// INSERTPS - Extended XMM registers
// ============================================================================

#[test]
fn test_insertps_xmm8_xmm1_src0_dst0() {
    let mut emu = emu64();
    let code = [0x66, 0x44, 0x0f, 0x3a, 0x21, 0xc1, 0x00, 0xf4]; // INSERTPS XMM8, XMM1, 0x00
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_insertps_xmm15_xmm7_src3_dst2() {
    let mut emu = emu64();
    let code = [0x66, 0x44, 0x0f, 0x3a, 0x21, 0xff, 0xe0, 0xf4]; // INSERTPS XMM15, XMM7, 0xE0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_insertps_xmm0_xmm8_src1_dst1() {
    let mut emu = emu64();
    let code = [0x66, 0x41, 0x0f, 0x3a, 0x21, 0xc0, 0x50, 0xf4]; // INSERTPS XMM0, XMM8, 0x50
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_insertps_xmm0_xmm15_src2_dst3() {
    let mut emu = emu64();
    let code = [0x66, 0x41, 0x0f, 0x3a, 0x21, 0xc7, 0xb0, 0xf4]; // INSERTPS XMM0, XMM15, 0xB0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// INSERTPS - Memory source operand
// ============================================================================

#[test]
fn test_insertps_xmm0_mem_dst0() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x21, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0x00, 0xf4
    ]; // INSERTPS XMM0, [0x3000], 0x00 (mem->dst[0])
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_insertps_xmm0_mem_dst1() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x21, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0x10, 0xf4
    ]; // INSERTPS XMM0, [0x3000], 0x10 (mem->dst[1])
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_insertps_xmm0_mem_dst2() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x21, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0x20, 0xf4
    ]; // INSERTPS XMM0, [0x3000], 0x20 (mem->dst[2])
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_insertps_xmm0_mem_dst3() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x21, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0x30, 0xf4
    ]; // INSERTPS XMM0, [0x3000], 0x30 (mem->dst[3])
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_insertps_xmm1_mem_dst0_zmask_1110() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x21, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, 0x0e, 0xf4
    ]; // INSERTPS XMM1, [0x3000], 0x0E (mem->dst[0], zero dst[1,2,3])
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_insertps_xmm7_mem_dst2_zmask_1001() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x3a, 0x21, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0x29, 0xf4
    ]; // INSERTPS XMM7, [0x3000], 0x29 (mem->dst[2], zero dst[0,3])
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// INSERTPS - Edge cases and comprehensive combinations
// ============================================================================

#[test]
fn test_insertps_xmm0_xmm1_src0_dst0_zmask_0000() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x21, 0xc1, 0x00, 0xf4]; // INSERTPS XMM0, XMM1, 0x00 (no zeros)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_insertps_xmm1_xmm2_src1_dst1_zmask_0101() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x21, 0xca, 0x55, 0xf4]; // INSERTPS XMM1, XMM2, 0x55 (src[1]->dst[1], zero dst[0,2])
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_insertps_xmm2_xmm3_src2_dst2_zmask_1010() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x21, 0xd3, 0xaa, 0xf4]; // INSERTPS XMM2, XMM3, 0xAA (src[2]->dst[2], zero dst[1,3])
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_insertps_xmm3_xmm4_src3_dst3_zmask_1111() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x21, 0xdc, 0xff, 0xf4]; // INSERTPS XMM3, XMM4, 0xFF (src[3]->dst[3], zero all)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_insertps_xmm7_xmm6_src0_dst3_zmask_0111() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x21, 0xfe, 0x37, 0xf4]; // INSERTPS XMM7, XMM6, 0x37 (src[0]->dst[3], zero dst[0,1,2])
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_insertps_xmm5_xmm5_src2_dst1_zmask_0100() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x21, 0xed, 0x94, 0xf4]; // INSERTPS XMM5, XMM5, 0x94 (src[2]->dst[1], zero dst[2])
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_insertps_xmm4_xmm3_src1_dst3_zmask_1100() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x21, 0xe3, 0x7c, 0xf4]; // INSERTPS XMM4, XMM3, 0x7C (src[1]->dst[3], zero dst[2,3])
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_insertps_xmm8_xmm9_src2_dst1() {
    let mut emu = emu64();
    let code = [0x66, 0x45, 0x0f, 0x3a, 0x21, 0xc1, 0x90, 0xf4]; // INSERTPS XMM8, XMM9, 0x90 (src[2]->dst[1])
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_insertps_xmm15_xmm14_src3_dst0_zmask_1110() {
    let mut emu = emu64();
    let code = [0x66, 0x45, 0x0f, 0x3a, 0x21, 0xfe, 0xce, 0xf4]; // INSERTPS XMM15, XMM14, 0xCE (src[3]->dst[0], zero dst[1,2,3])
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
