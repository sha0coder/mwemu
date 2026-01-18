use crate::*;

// AVX-512 FP16 Arithmetic Instructions
//
// VADDPH - Add Packed Half-Precision Floating-Point Values
// VSUBPH - Subtract Packed Half-Precision Floating-Point Values
// VMULPH - Multiply Packed Half-Precision Floating-Point Values
// VDIVPH - Divide Packed Half-Precision Floating-Point Values
//
// These instructions operate on IEEE 754 half-precision (16-bit) floating-point values
// Requires AVX-512-FP16 support (CPUID: EAX=07H, ECX=0: EDX[bit 23])
//
// Encodings:
//   EVEX.128.NP.0F.W0 58 /r   VADDPH xmm1{k1}{z}, xmm2, xmm3/m128/m16bcst
//   EVEX.256.NP.0F.W0 58 /r   VADDPH ymm1{k1}{z}, ymm2, ymm3/m256/m16bcst
//   EVEX.512.NP.0F.W0 58 /r   VADDPH zmm1{k1}{z}, zmm2, zmm3/m512/m16bcst{er}
//
//   EVEX.128.NP.0F.W0 5C /r   VSUBPH xmm1{k1}{z}, xmm2, xmm3/m128/m16bcst
//   EVEX.256.NP.0F.W0 5C /r   VSUBPH ymm1{k1}{z}, ymm2, ymm3/m256/m16bcst
//   EVEX.512.NP.0F.W0 5C /r   VSUBPH zmm1{k1}{z}, zmm2, zmm3/m512/m16bcst{er}
//
//   EVEX.128.NP.0F.W0 59 /r   VMULPH xmm1{k1}{z}, xmm2, xmm3/m128/m16bcst
//   EVEX.256.NP.0F.W0 59 /r   VMULPH ymm1{k1}{z}, ymm2, ymm3/m256/m16bcst
//   EVEX.512.NP.0F.W0 59 /r   VMULPH zmm1{k1}{z}, zmm2, zmm3/m512/m16bcst{er}
//
//   EVEX.128.NP.0F.W0 5E /r   VDIVPH xmm1{k1}{z}, xmm2, xmm3/m128/m16bcst
//   EVEX.256.NP.0F.W0 5E /r   VDIVPH ymm1{k1}{z}, ymm2, ymm3/m256/m16bcst
//   EVEX.512.NP.0F.W0 5E /r   VDIVPH zmm1{k1}{z}, zmm2, zmm3/m512/m16bcst{er}

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// VADDPH XMM Tests (128-bit, 8 FP16 values)
// ============================================================================

#[test]
fn test_vaddph_xmm_basic() {
    let mut emu = emu64();
    // VADDPH XMM0, XMM1, XMM2
    let code = [
        // VADDPH XMM0, XMM1, XMM2
        0x62, 0xf5, 0x7c, 0x08, 0x58, 0xc2, // VADDPH XMM0, XMM1, XMM2

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddph_xmm_zero() {
    let mut emu = emu64();
    let code = [
        // Zero out XMM2
        0x62, 0xf1, 0x7c, 0x08, 0x57, 0xd2, // VXORPS XMM2, XMM2, XMM2

        // VADDPH XMM0, XMM1, XMM2
        0x62, 0xf5, 0x7c, 0x08, 0x58, 0xc2, // VADDPH XMM0, XMM1, XMM2

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddph_xmm_different_regs() {
    let mut emu = emu64();
    let code = [
        // VADDPH XMM3, XMM4, XMM5
        0x62, 0xf5, 0x5c, 0x08, 0x58, 0xdd, // VADDPH XMM3, XMM4, XMM5

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddph_xmm_memory_operand() {
    let mut emu = emu64();
    // VADDPH with memory operand
    let code = [
        // VADDPH XMM0, XMM1, [0x3000]
        0x62, 0xf5, 0x7c, 0x08, 0x58, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // VADDPH XMM0, XMM1, [0x3000]

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddph_xmm_broadcast() {
    let mut emu = emu64();
    // VADDPH with broadcast (1-to-8)
    let code = [
        // VADDPH XMM0, XMM1, [0x3000]{1to8}
        0x62, 0xf5, 0x7c, 0x18, 0x58, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // VADDPH XMM0, XMM1, [0x3000]{1to8}

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VADDPH YMM Tests (256-bit, 16 FP16 values)
// ============================================================================

#[test]
fn test_vaddph_ymm_basic() {
    let mut emu = emu64();
    // VADDPH YMM0, YMM1, YMM2
    let code = [
        // VADDPH YMM0, YMM1, YMM2
        0x62, 0xf5, 0x7c, 0x28, 0x58, 0xc2, // VADDPH YMM0, YMM1, YMM2

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddph_ymm_different_regs() {
    let mut emu = emu64();
    // VADDPH YMM6, YMM7, YMM8
    let code = [
        // VADDPH YMM6, YMM7, YMM8
        0x62, 0xd5, 0x44, 0x28, 0x58, 0xf0, // VADDPH YMM6, YMM7, YMM8

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VADDPH ZMM Tests (512-bit, 32 FP16 values)
// ============================================================================

#[test]
fn test_vaddph_zmm_basic() {
    let mut emu = emu64();
    // VADDPH ZMM0, ZMM1, ZMM2
    let code = [
        // VADDPH ZMM0, ZMM1, ZMM2
        0x62, 0xf5, 0x7c, 0x48, 0x58, 0xc2, // VADDPH ZMM0, ZMM1, ZMM2

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddph_zmm_with_rounding() {
    let mut emu = emu64();
    // VADDPH ZMM0, ZMM1, ZMM2, {rn-sae}
    let code = [
        // VADDPH ZMM0, ZMM1, ZMM2, {rn-sae}
        0x62, 0xf5, 0x7c, 0x18, 0x58, 0xc2, // VADDPH ZMM0, ZMM1, ZMM2, {rn-sae}

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VSUBPH XMM Tests
// ============================================================================

#[test]
fn test_vsubph_xmm_basic() {
    let mut emu = emu64();
    // VSUBPH XMM0, XMM1, XMM2
    let code = [
        // VSUBPH XMM0, XMM1, XMM2
        0x62, 0xf5, 0x7c, 0x08, 0x5c, 0xc2, // VSUBPH XMM0, XMM1, XMM2

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubph_xmm_same_operand() {
    let mut emu = emu64();
    // VSUBPH XMM0, XMM1, XMM1 (result should be zero)
    let code = [
        // VSUBPH XMM0, XMM1, XMM1
        0x62, 0xf5, 0x7c, 0x08, 0x5c, 0xc1, // VSUBPH XMM0, XMM1, XMM1

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubph_xmm_memory() {
    let mut emu = emu64();
    // VSUBPH XMM0, XMM1, [0x3000]
    let code = [
        // VSUBPH XMM0, XMM1, [0x3000]
        0x62, 0xf5, 0x7c, 0x08, 0x5c, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // VSUBPH XMM0, XMM1, [0x3000]

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VSUBPH YMM Tests
// ============================================================================

#[test]
fn test_vsubph_ymm_basic() {
    let mut emu = emu64();
    // VSUBPH YMM0, YMM1, YMM2
    let code = [
        // VSUBPH YMM0, YMM1, YMM2
        0x62, 0xf5, 0x7c, 0x28, 0x5c, 0xc2, // VSUBPH YMM0, YMM1, YMM2

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubph_ymm_different_regs() {
    let mut emu = emu64();
    // VSUBPH YMM3, YMM4, YMM5
    let code = [
        // VSUBPH YMM3, YMM4, YMM5
        0x62, 0xf5, 0x5c, 0x28, 0x5c, 0xdd, // VSUBPH YMM3, YMM4, YMM5

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VSUBPH ZMM Tests
// ============================================================================

#[test]
fn test_vsubph_zmm_basic() {
    let mut emu = emu64();
    // VSUBPH ZMM0, ZMM1, ZMM2
    let code = [
        // VSUBPH ZMM0, ZMM1, ZMM2
        0x62, 0xf5, 0x7c, 0x48, 0x5c, 0xc2, // VSUBPH ZMM0, ZMM1, ZMM2

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vsubph_zmm_rounding_rz() {
    let mut emu = emu64();
    // VSUBPH ZMM0, ZMM1, ZMM2, {rz-sae}
    let code = [
        // VSUBPH ZMM0, ZMM1, ZMM2, {rz-sae}
        0x62, 0xf5, 0x7c, 0x78, 0x5c, 0xc2, // VSUBPH ZMM0, ZMM1, ZMM2, {rz-sae}

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VMULPH XMM Tests
// ============================================================================

#[test]
fn test_vmulph_xmm_basic() {
    let mut emu = emu64();
    // VMULPH XMM0, XMM1, XMM2
    let code = [
        // VMULPH XMM0, XMM1, XMM2
        0x62, 0xf5, 0x7c, 0x08, 0x59, 0xc2, // VMULPH XMM0, XMM1, XMM2

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulph_xmm_by_one() {
    let mut emu = emu64();
    let code = [
        // Setup would require FP16 value 1.0 (0x3C00)
        // VMULPH XMM0, XMM1, XMM2
        0x62, 0xf5, 0x7c, 0x08, 0x59, 0xc2, // VMULPH XMM0, XMM1, XMM2

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulph_xmm_memory() {
    let mut emu = emu64();
    // VMULPH XMM0, XMM1, [0x3000]
    let code = [
        // VMULPH XMM0, XMM1, [0x3000]
        0x62, 0xf5, 0x7c, 0x08, 0x59, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // VMULPH XMM0, XMM1, [0x3000]

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulph_xmm_broadcast() {
    let mut emu = emu64();
    // VMULPH XMM0, XMM1, [0x3000]{1to8}
    let code = [
        // VMULPH XMM0, XMM1, [0x3000]{1to8}
        0x62, 0xf5, 0x7c, 0x18, 0x59, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // VMULPH XMM0, XMM1, [0x3000]{1to8}

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VMULPH YMM Tests
// ============================================================================

#[test]
fn test_vmulph_ymm_basic() {
    let mut emu = emu64();
    // VMULPH YMM0, YMM1, YMM2
    let code = [
        // VMULPH YMM0, YMM1, YMM2
        0x62, 0xf5, 0x7c, 0x28, 0x59, 0xc2, // VMULPH YMM0, YMM1, YMM2

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulph_ymm_different_regs() {
    let mut emu = emu64();
    // VMULPH YMM7, YMM6, YMM5
    let code = [
        // VMULPH YMM7, YMM6, YMM5
        0x62, 0xf5, 0x4c, 0x28, 0x59, 0xfd, // VMULPH YMM7, YMM6, YMM5

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VMULPH ZMM Tests
// ============================================================================

#[test]
fn test_vmulph_zmm_basic() {
    let mut emu = emu64();
    // VMULPH ZMM0, ZMM1, ZMM2
    let code = [
        // VMULPH ZMM0, ZMM1, ZMM2
        0x62, 0xf5, 0x7c, 0x48, 0x59, 0xc2, // VMULPH ZMM0, ZMM1, ZMM2

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulph_zmm_rounding_rd() {
    let mut emu = emu64();
    // VMULPH ZMM0, ZMM1, ZMM2, {rd-sae}
    let code = [
        // VMULPH ZMM0, ZMM1, ZMM2, {rd-sae}
        0x62, 0xf5, 0x7c, 0x38, 0x59, 0xc2, // VMULPH ZMM0, ZMM1, ZMM2, {rd-sae}

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VDIVPH XMM Tests
// ============================================================================

#[test]
fn test_vdivph_xmm_basic() {
    let mut emu = emu64();
    // VDIVPH XMM0, XMM1, XMM2
    let code = [
        // VDIVPH XMM0, XMM1, XMM2
        0x62, 0xf5, 0x7c, 0x08, 0x5e, 0xc2, // VDIVPH XMM0, XMM1, XMM2

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdivph_xmm_by_one() {
    let mut emu = emu64();
    let code = [
        // VDIVPH XMM0, XMM1, XMM2
        0x62, 0xf5, 0x7c, 0x08, 0x5e, 0xc2, // VDIVPH XMM0, XMM1, XMM2

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdivph_xmm_same_operand() {
    let mut emu = emu64();
    // VDIVPH XMM0, XMM1, XMM1 (result should be one, ignoring NaN/zero cases)
    let code = [
        // VDIVPH XMM0, XMM1, XMM1
        0x62, 0xf5, 0x7c, 0x08, 0x5e, 0xc1, // VDIVPH XMM0, XMM1, XMM1

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdivph_xmm_memory() {
    let mut emu = emu64();
    // VDIVPH XMM0, XMM1, [0x3000]
    let code = [
        // VDIVPH XMM0, XMM1, [0x3000]
        0x62, 0xf5, 0x7c, 0x08, 0x5e, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // VDIVPH XMM0, XMM1, [0x3000]

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VDIVPH YMM Tests
// ============================================================================

#[test]
fn test_vdivph_ymm_basic() {
    let mut emu = emu64();
    // VDIVPH YMM0, YMM1, YMM2
    let code = [
        // VDIVPH YMM0, YMM1, YMM2
        0x62, 0xf5, 0x7c, 0x28, 0x5e, 0xc2, // VDIVPH YMM0, YMM1, YMM2

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdivph_ymm_different_regs() {
    let mut emu = emu64();
    // VDIVPH YMM4, YMM5, YMM6
    let code = [
        // VDIVPH YMM4, YMM5, YMM6
        0x62, 0xf5, 0x54, 0x28, 0x5e, 0xe6, // VDIVPH YMM4, YMM5, YMM6

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VDIVPH ZMM Tests
// ============================================================================

#[test]
fn test_vdivph_zmm_basic() {
    let mut emu = emu64();
    // VDIVPH ZMM0, ZMM1, ZMM2
    let code = [
        // VDIVPH ZMM0, ZMM1, ZMM2
        0x62, 0xf5, 0x7c, 0x48, 0x5e, 0xc2, // VDIVPH ZMM0, ZMM1, ZMM2

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdivph_zmm_rounding_ru() {
    let mut emu = emu64();
    // VDIVPH ZMM0, ZMM1, ZMM2, {ru-sae}
    let code = [
        // VDIVPH ZMM0, ZMM1, ZMM2, {ru-sae}
        0x62, 0xf5, 0x7c, 0x58, 0x5e, 0xc2, // VDIVPH ZMM0, ZMM1, ZMM2, {ru-sae}

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Combined Operations Tests
// ============================================================================

#[test]
fn test_fp16_sequential_operations() {
    let mut emu = emu64();
    let code = [
        // VADDPH XMM0, XMM1, XMM2
        0x62, 0xf5, 0x7c, 0x08, 0x58, 0xc2, // VADDPH XMM0, XMM1, XMM2

        // VMULPH XMM3, XMM0, XMM4
        0x62, 0xf5, 0x7c, 0x08, 0x59, 0xdc, // VMULPH XMM3, XMM0, XMM4

        // VSUBPH XMM5, XMM3, XMM6
        0x62, 0xf5, 0x64, 0x08, 0x5c, 0xee, // VSUBPH XMM5, XMM3, XMM6

        // VDIVPH XMM7, XMM5, XMM1
        0x62, 0xf5, 0x54, 0x08, 0x5e, 0xf9, // VDIVPH XMM7, XMM5, XMM1

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_fp16_all_sizes() {
    let mut emu = emu64();
    let code = [
        // XMM: VADDPH XMM0, XMM1, XMM2
        0x62, 0xf5, 0x7c, 0x08, 0x58, 0xc2, // VADDPH XMM0, XMM1, XMM2

        // YMM: VADDPH YMM3, YMM4, YMM5
        0x62, 0xf5, 0x5c, 0x28, 0x58, 0xdd, // VADDPH YMM3, YMM4, YMM5

        // ZMM: VADDPH ZMM6, ZMM7, ZMM8
        0x62, 0xd5, 0x44, 0x48, 0x58, 0xf0, // VADDPH ZMM6, ZMM7, ZMM8

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_fp16_rounding_modes() {
    let mut emu = emu64();
    let code = [
        // Round nearest: VADDPH ZMM0, ZMM1, ZMM2, {rn-sae}
        0x62, 0xf5, 0x7c, 0x18, 0x58, 0xc2, // VADDPH ZMM0, ZMM1, ZMM2, {rn-sae}

        // Round down: VSUBPH ZMM3, ZMM4, ZMM5, {rd-sae}
        0x62, 0xf5, 0x5c, 0x38, 0x5c, 0xdd, // VSUBPH ZMM3, ZMM4, ZMM5, {rd-sae}

        // Round up: VMULPH ZMM6, ZMM7, ZMM8, {ru-sae}
        0x62, 0xd5, 0x44, 0x58, 0x59, 0xf0, // VMULPH ZMM6, ZMM7, ZMM8, {ru-sae}

        // Round zero: VDIVPH ZMM9, ZMM10, ZMM11, {rz-sae}
        0x62, 0x55, 0x2c, 0x78, 0x5e, 0xcb, // VDIVPH ZMM9, ZMM10, ZMM11, {rz-sae}

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_fp16_high_registers() {
    let mut emu = emu64();
    let code = [
        // VADDPH XMM8, XMM9, XMM10
        0x62, 0x75, 0x34, 0x08, 0x58, 0xc2, // VADDPH XMM8, XMM9, XMM10

        // VSUBPH XMM11, XMM12, XMM13
        0x62, 0x55, 0x1c, 0x08, 0x5c, 0xdd, // VSUBPH XMM11, XMM12, XMM13

        // VMULPH XMM14, XMM15, XMM8
        0x62, 0xd5, 0x04, 0x08, 0x59, 0xf0, // VMULPH XMM14, XMM15, XMM8

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
