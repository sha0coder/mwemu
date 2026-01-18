use crate::*;

// VCMPPS - Compare Packed Single Precision Floating-Point Values
// VCMPPD - Compare Packed Double Precision Floating-Point Values
//
// These instructions compare packed floating-point values and produce a mask result.
// The comparison predicate (imm8) determines the type of comparison performed.
//
// There are 32 different comparison predicates (0-31):
// 0x00 = EQ_OQ    (Equal, Ordered, Quiet)
// 0x01 = LT_OS    (Less Than, Ordered, Signaling)
// 0x02 = LE_OS    (Less Than or Equal, Ordered, Signaling)
// 0x03 = UNORD_Q  (Unordered, Quiet)
// 0x04 = NEQ_UQ   (Not Equal, Unordered, Quiet)
// 0x05 = NLT_US   (Not Less Than, Unordered, Signaling)
// 0x06 = NLE_US   (Not Less Than or Equal, Unordered, Signaling)
// 0x07 = ORD_Q    (Ordered, Quiet)
// 0x08 = EQ_UQ    (Equal, Unordered, Quiet)
// 0x09 = NGE_US   (Not Greater Than or Equal, Unordered, Signaling)
// 0x0A = NGT_US   (Not Greater Than, Unordered, Signaling)
// 0x0B = FALSE_OQ (False, Ordered, Quiet)
// 0x0C = NEQ_OQ   (Not Equal, Ordered, Quiet)
// 0x0D = GE_OS    (Greater Than or Equal, Ordered, Signaling)
// 0x0E = GT_OS    (Greater Than, Ordered, Signaling)
// 0x0F = TRUE_UQ  (True, Unordered, Quiet)
// 0x10 = EQ_OS    (Equal, Ordered, Signaling)
// 0x11 = LT_OQ    (Less Than, Ordered, Quiet)
// 0x12 = LE_OQ    (Less Than or Equal, Ordered, Quiet)
// 0x13 = UNORD_S  (Unordered, Signaling)
// 0x14 = NEQ_US   (Not Equal, Unordered, Signaling)
// 0x15 = NLT_UQ   (Not Less Than, Unordered, Quiet)
// 0x16 = NLE_UQ   (Not Less Than or Equal, Unordered, Quiet)
// 0x17 = ORD_S    (Ordered, Signaling)
// 0x18 = EQ_US    (Equal, Unordered, Signaling)
// 0x19 = NGE_UQ   (Not Greater Than or Equal, Unordered, Quiet)
// 0x1A = NGT_UQ   (Not Greater Than, Unordered, Quiet)
// 0x1B = FALSE_OS (False, Ordered, Signaling)
// 0x1C = NEQ_OS   (Not Equal, Ordered, Signaling)
// 0x1D = GE_OQ    (Greater Than or Equal, Ordered, Quiet)
// 0x1E = GT_OQ    (Greater Than, Ordered, Quiet)
// 0x1F = TRUE_US  (True, Unordered, Signaling)
//
// Opcodes:
// VEX.128.NP 0F C2 /r ib    VCMPPS xmm1, xmm2, xmm3/m128, imm8
// VEX.256.NP 0F C2 /r ib    VCMPPS ymm1, ymm2, ymm3/m256, imm8
// VEX.128.66 0F C2 /r ib    VCMPPD xmm1, xmm2, xmm3/m128, imm8
// VEX.256.66 0F C2 /r ib    VCMPPD ymm1, ymm2, ymm3/m256, imm8

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// VCMPPS Tests - 128-bit (4x float32)
// ============================================================================

#[test]
fn test_vcmpps_xmm_eq_oq() {
    let mut emu = emu64();
    // VCMPPS XMM1, XMM0, XMM2, 0x00 (Equal, Ordered, Quiet)
    let code = [
        0xc5, 0xf8, 0xc2, 0xca, 0x00, // VCMPPS XMM1, XMM0, XMM2, 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmpps_xmm_lt_os() {
    let mut emu = emu64();
    // VCMPPS XMM1, XMM0, XMM2, 0x01 (Less Than, Ordered, Signaling)
    let code = [
        0xc5, 0xf8, 0xc2, 0xca, 0x01, // VCMPPS XMM1, XMM0, XMM2, 0x01
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmpps_xmm_le_os() {
    let mut emu = emu64();
    // VCMPPS XMM1, XMM0, XMM2, 0x02 (Less Than or Equal, Ordered, Signaling)
    let code = [
        0xc5, 0xf8, 0xc2, 0xca, 0x02, // VCMPPS XMM1, XMM0, XMM2, 0x02
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmpps_xmm_unord_q() {
    let mut emu = emu64();
    // VCMPPS XMM1, XMM0, XMM2, 0x03 (Unordered, Quiet)
    let code = [
        0xc5, 0xf8, 0xc2, 0xca, 0x03, // VCMPPS XMM1, XMM0, XMM2, 0x03
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmpps_xmm_neq_uq() {
    let mut emu = emu64();
    // VCMPPS XMM1, XMM0, XMM2, 0x04 (Not Equal, Unordered, Quiet)
    let code = [
        0xc5, 0xf8, 0xc2, 0xca, 0x04, // VCMPPS XMM1, XMM0, XMM2, 0x04
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmpps_xmm_nlt_us() {
    let mut emu = emu64();
    // VCMPPS XMM1, XMM0, XMM2, 0x05 (Not Less Than, Unordered, Signaling)
    let code = [
        0xc5, 0xf8, 0xc2, 0xca, 0x05, // VCMPPS XMM1, XMM0, XMM2, 0x05
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmpps_xmm_nle_us() {
    let mut emu = emu64();
    // VCMPPS XMM1, XMM0, XMM2, 0x06 (Not Less Than or Equal, Unordered, Signaling)
    let code = [
        0xc5, 0xf8, 0xc2, 0xca, 0x06, // VCMPPS XMM1, XMM0, XMM2, 0x06
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmpps_xmm_ord_q() {
    let mut emu = emu64();
    // VCMPPS XMM1, XMM0, XMM2, 0x07 (Ordered, Quiet)
    let code = [
        0xc5, 0xf8, 0xc2, 0xca, 0x07, // VCMPPS XMM1, XMM0, XMM2, 0x07
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmpps_xmm_eq_uq() {
    let mut emu = emu64();
    // VCMPPS XMM1, XMM0, XMM2, 0x08 (Equal, Unordered, Quiet)
    let code = [
        0xc5, 0xf8, 0xc2, 0xca, 0x08, // VCMPPS XMM1, XMM0, XMM2, 0x08
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmpps_xmm_nge_us() {
    let mut emu = emu64();
    // VCMPPS XMM1, XMM0, XMM2, 0x09 (Not Greater Than or Equal, Unordered, Signaling)
    let code = [
        0xc5, 0xf8, 0xc2, 0xca, 0x09, // VCMPPS XMM1, XMM0, XMM2, 0x09
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmpps_xmm_ngt_us() {
    let mut emu = emu64();
    // VCMPPS XMM1, XMM0, XMM2, 0x0A (Not Greater Than, Unordered, Signaling)
    let code = [
        0xc5, 0xf8, 0xc2, 0xca, 0x0a, // VCMPPS XMM1, XMM0, XMM2, 0x0A
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmpps_xmm_false_oq() {
    let mut emu = emu64();
    // VCMPPS XMM1, XMM0, XMM2, 0x0B (False, Ordered, Quiet)
    let code = [
        0xc5, 0xf8, 0xc2, 0xca, 0x0b, // VCMPPS XMM1, XMM0, XMM2, 0x0B
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmpps_xmm_neq_oq() {
    let mut emu = emu64();
    // VCMPPS XMM1, XMM0, XMM2, 0x0C (Not Equal, Ordered, Quiet)
    let code = [
        0xc5, 0xf8, 0xc2, 0xca, 0x0c, // VCMPPS XMM1, XMM0, XMM2, 0x0C
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmpps_xmm_ge_os() {
    let mut emu = emu64();
    // VCMPPS XMM1, XMM0, XMM2, 0x0D (Greater Than or Equal, Ordered, Signaling)
    let code = [
        0xc5, 0xf8, 0xc2, 0xca, 0x0d, // VCMPPS XMM1, XMM0, XMM2, 0x0D
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmpps_xmm_gt_os() {
    let mut emu = emu64();
    // VCMPPS XMM1, XMM0, XMM2, 0x0E (Greater Than, Ordered, Signaling)
    let code = [
        0xc5, 0xf8, 0xc2, 0xca, 0x0e, // VCMPPS XMM1, XMM0, XMM2, 0x0E
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmpps_xmm_true_uq() {
    let mut emu = emu64();
    // VCMPPS XMM1, XMM0, XMM2, 0x0F (True, Unordered, Quiet)
    let code = [
        0xc5, 0xf8, 0xc2, 0xca, 0x0f, // VCMPPS XMM1, XMM0, XMM2, 0x0F
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmpps_xmm_eq_os() {
    let mut emu = emu64();
    // VCMPPS XMM1, XMM0, XMM2, 0x10 (Equal, Ordered, Signaling)
    let code = [
        0xc5, 0xf8, 0xc2, 0xca, 0x10, // VCMPPS XMM1, XMM0, XMM2, 0x10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmpps_xmm_lt_oq() {
    let mut emu = emu64();
    // VCMPPS XMM1, XMM0, XMM2, 0x11 (Less Than, Ordered, Quiet)
    let code = [
        0xc5, 0xf8, 0xc2, 0xca, 0x11, // VCMPPS XMM1, XMM0, XMM2, 0x11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmpps_xmm_le_oq() {
    let mut emu = emu64();
    // VCMPPS XMM1, XMM0, XMM2, 0x12 (Less Than or Equal, Ordered, Quiet)
    let code = [
        0xc5, 0xf8, 0xc2, 0xca, 0x12, // VCMPPS XMM1, XMM0, XMM2, 0x12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmpps_xmm_unord_s() {
    let mut emu = emu64();
    // VCMPPS XMM1, XMM0, XMM2, 0x13 (Unordered, Signaling)
    let code = [
        0xc5, 0xf8, 0xc2, 0xca, 0x13, // VCMPPS XMM1, XMM0, XMM2, 0x13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmpps_xmm_neq_us() {
    let mut emu = emu64();
    // VCMPPS XMM1, XMM0, XMM2, 0x14 (Not Equal, Unordered, Signaling)
    let code = [
        0xc5, 0xf8, 0xc2, 0xca, 0x14, // VCMPPS XMM1, XMM0, XMM2, 0x14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmpps_xmm_nlt_uq() {
    let mut emu = emu64();
    // VCMPPS XMM1, XMM0, XMM2, 0x15 (Not Less Than, Unordered, Quiet)
    let code = [
        0xc5, 0xf8, 0xc2, 0xca, 0x15, // VCMPPS XMM1, XMM0, XMM2, 0x15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmpps_xmm_nle_uq() {
    let mut emu = emu64();
    // VCMPPS XMM1, XMM0, XMM2, 0x16 (Not Less Than or Equal, Unordered, Quiet)
    let code = [
        0xc5, 0xf8, 0xc2, 0xca, 0x16, // VCMPPS XMM1, XMM0, XMM2, 0x16
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmpps_xmm_ord_s() {
    let mut emu = emu64();
    // VCMPPS XMM1, XMM0, XMM2, 0x17 (Ordered, Signaling)
    let code = [
        0xc5, 0xf8, 0xc2, 0xca, 0x17, // VCMPPS XMM1, XMM0, XMM2, 0x17
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmpps_xmm_eq_us() {
    let mut emu = emu64();
    // VCMPPS XMM1, XMM0, XMM2, 0x18 (Equal, Unordered, Signaling)
    let code = [
        0xc5, 0xf8, 0xc2, 0xca, 0x18, // VCMPPS XMM1, XMM0, XMM2, 0x18
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmpps_xmm_nge_uq() {
    let mut emu = emu64();
    // VCMPPS XMM1, XMM0, XMM2, 0x19 (Not Greater Than or Equal, Unordered, Quiet)
    let code = [
        0xc5, 0xf8, 0xc2, 0xca, 0x19, // VCMPPS XMM1, XMM0, XMM2, 0x19
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmpps_xmm_ngt_uq() {
    let mut emu = emu64();
    // VCMPPS XMM1, XMM0, XMM2, 0x1A (Not Greater Than, Unordered, Quiet)
    let code = [
        0xc5, 0xf8, 0xc2, 0xca, 0x1a, // VCMPPS XMM1, XMM0, XMM2, 0x1A
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmpps_xmm_false_os() {
    let mut emu = emu64();
    // VCMPPS XMM1, XMM0, XMM2, 0x1B (False, Ordered, Signaling)
    let code = [
        0xc5, 0xf8, 0xc2, 0xca, 0x1b, // VCMPPS XMM1, XMM0, XMM2, 0x1B
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmpps_xmm_neq_os() {
    let mut emu = emu64();
    // VCMPPS XMM1, XMM0, XMM2, 0x1C (Not Equal, Ordered, Signaling)
    let code = [
        0xc5, 0xf8, 0xc2, 0xca, 0x1c, // VCMPPS XMM1, XMM0, XMM2, 0x1C
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmpps_xmm_ge_oq() {
    let mut emu = emu64();
    // VCMPPS XMM1, XMM0, XMM2, 0x1D (Greater Than or Equal, Ordered, Quiet)
    let code = [
        0xc5, 0xf8, 0xc2, 0xca, 0x1d, // VCMPPS XMM1, XMM0, XMM2, 0x1D
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmpps_xmm_gt_oq() {
    let mut emu = emu64();
    // VCMPPS XMM1, XMM0, XMM2, 0x1E (Greater Than, Ordered, Quiet)
    let code = [
        0xc5, 0xf8, 0xc2, 0xca, 0x1e, // VCMPPS XMM1, XMM0, XMM2, 0x1E
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmpps_xmm_true_us() {
    let mut emu = emu64();
    // VCMPPS XMM1, XMM0, XMM2, 0x1F (True, Unordered, Signaling)
    let code = [
        0xc5, 0xf8, 0xc2, 0xca, 0x1f, // VCMPPS XMM1, XMM0, XMM2, 0x1F
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCMPPS Tests - 256-bit (8x float32)
// ============================================================================

#[test]
fn test_vcmpps_ymm_eq_oq() {
    let mut emu = emu64();
    // VCMPPS YMM1, YMM0, YMM2, 0x00 (Equal, Ordered, Quiet)
    let code = [
        0xc5, 0xfc, 0xc2, 0xca, 0x00, // VCMPPS YMM1, YMM0, YMM2, 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmpps_ymm_lt_os() {
    let mut emu = emu64();
    // VCMPPS YMM1, YMM0, YMM2, 0x01
    let code = [
        0xc5, 0xfc, 0xc2, 0xca, 0x01, // VCMPPS YMM1, YMM0, YMM2, 0x01
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmpps_ymm_le_os() {
    let mut emu = emu64();
    // VCMPPS YMM1, YMM0, YMM2, 0x02
    let code = [
        0xc5, 0xfc, 0xc2, 0xca, 0x02, // VCMPPS YMM1, YMM0, YMM2, 0x02
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmpps_ymm_unord_q() {
    let mut emu = emu64();
    // VCMPPS YMM1, YMM0, YMM2, 0x03
    let code = [
        0xc5, 0xfc, 0xc2, 0xca, 0x03, // VCMPPS YMM1, YMM0, YMM2, 0x03
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmpps_ymm_neq_uq() {
    let mut emu = emu64();
    // VCMPPS YMM1, YMM0, YMM2, 0x04
    let code = [
        0xc5, 0xfc, 0xc2, 0xca, 0x04, // VCMPPS YMM1, YMM0, YMM2, 0x04
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmpps_ymm_extended_regs() {
    let mut emu = emu64();
    // VCMPPS YMM8, YMM9, YMM10, 0x00
    let code = [
        0xc4, 0xc1, 0x34, 0xc2, 0xc2, 0x00, // VCMPPS YMM8, YMM9, YMM10, 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmpps_ymm_mem() {
    let mut emu = emu64();
    // VCMPPS YMM1, YMM0, [mem], 0x00
    let code = [
        0xc5, 0xfc, 0xc2, 0x0d, 0x00, 0x40, 0x00, 0x00, 0x00, // VCMPPS YMM1, YMM0, [rip+0x4000], 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [
        0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x00, 0x40,
        0x00, 0x00, 0x40, 0x40, 0x00, 0x00, 0x80, 0x40,
        0x00, 0x00, 0xa0, 0x40, 0x00, 0x00, 0xc0, 0x40,
        0x00, 0x00, 0xe0, 0x40, 0x00, 0x00, 0x00, 0x41,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// VCMPPD Tests - 128-bit (2x float64)
// ============================================================================

#[test]
fn test_vcmppd_xmm_eq_oq() {
    let mut emu = emu64();
    // VCMPPD XMM1, XMM0, XMM2, 0x00 (Equal, Ordered, Quiet)
    let code = [
        0xc5, 0xf9, 0xc2, 0xca, 0x00, // VCMPPD XMM1, XMM0, XMM2, 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmppd_xmm_lt_os() {
    let mut emu = emu64();
    // VCMPPD XMM1, XMM0, XMM2, 0x01
    let code = [
        0xc5, 0xf9, 0xc2, 0xca, 0x01, // VCMPPD XMM1, XMM0, XMM2, 0x01
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmppd_xmm_le_os() {
    let mut emu = emu64();
    // VCMPPD XMM1, XMM0, XMM2, 0x02
    let code = [
        0xc5, 0xf9, 0xc2, 0xca, 0x02, // VCMPPD XMM1, XMM0, XMM2, 0x02
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmppd_xmm_unord_q() {
    let mut emu = emu64();
    // VCMPPD XMM1, XMM0, XMM2, 0x03
    let code = [
        0xc5, 0xf9, 0xc2, 0xca, 0x03, // VCMPPD XMM1, XMM0, XMM2, 0x03
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmppd_xmm_neq_uq() {
    let mut emu = emu64();
    // VCMPPD XMM1, XMM0, XMM2, 0x04
    let code = [
        0xc5, 0xf9, 0xc2, 0xca, 0x04, // VCMPPD XMM1, XMM0, XMM2, 0x04
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmppd_xmm_nlt_us() {
    let mut emu = emu64();
    // VCMPPD XMM1, XMM0, XMM2, 0x05
    let code = [
        0xc5, 0xf9, 0xc2, 0xca, 0x05, // VCMPPD XMM1, XMM0, XMM2, 0x05
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmppd_xmm_nle_us() {
    let mut emu = emu64();
    // VCMPPD XMM1, XMM0, XMM2, 0x06
    let code = [
        0xc5, 0xf9, 0xc2, 0xca, 0x06, // VCMPPD XMM1, XMM0, XMM2, 0x06
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmppd_xmm_ord_q() {
    let mut emu = emu64();
    // VCMPPD XMM1, XMM0, XMM2, 0x07
    let code = [
        0xc5, 0xf9, 0xc2, 0xca, 0x07, // VCMPPD XMM1, XMM0, XMM2, 0x07
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmppd_xmm_ge_os() {
    let mut emu = emu64();
    // VCMPPD XMM1, XMM0, XMM2, 0x0D
    let code = [
        0xc5, 0xf9, 0xc2, 0xca, 0x0d, // VCMPPD XMM1, XMM0, XMM2, 0x0D
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmppd_xmm_gt_os() {
    let mut emu = emu64();
    // VCMPPD XMM1, XMM0, XMM2, 0x0E
    let code = [
        0xc5, 0xf9, 0xc2, 0xca, 0x0e, // VCMPPD XMM1, XMM0, XMM2, 0x0E
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmppd_xmm_true_uq() {
    let mut emu = emu64();
    // VCMPPD XMM1, XMM0, XMM2, 0x0F
    let code = [
        0xc5, 0xf9, 0xc2, 0xca, 0x0f, // VCMPPD XMM1, XMM0, XMM2, 0x0F
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCMPPD Tests - 256-bit (4x float64)
// ============================================================================

#[test]
fn test_vcmppd_ymm_eq_oq() {
    let mut emu = emu64();
    // VCMPPD YMM1, YMM0, YMM2, 0x00 (Equal, Ordered, Quiet)
    let code = [
        0xc5, 0xfd, 0xc2, 0xca, 0x00, // VCMPPD YMM1, YMM0, YMM2, 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmppd_ymm_lt_os() {
    let mut emu = emu64();
    // VCMPPD YMM1, YMM0, YMM2, 0x01
    let code = [
        0xc5, 0xfd, 0xc2, 0xca, 0x01, // VCMPPD YMM1, YMM0, YMM2, 0x01
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmppd_ymm_le_os() {
    let mut emu = emu64();
    // VCMPPD YMM1, YMM0, YMM2, 0x02
    let code = [
        0xc5, 0xfd, 0xc2, 0xca, 0x02, // VCMPPD YMM1, YMM0, YMM2, 0x02
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmppd_ymm_unord_q() {
    let mut emu = emu64();
    // VCMPPD YMM1, YMM0, YMM2, 0x03
    let code = [
        0xc5, 0xfd, 0xc2, 0xca, 0x03, // VCMPPD YMM1, YMM0, YMM2, 0x03
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmppd_ymm_neq_uq() {
    let mut emu = emu64();
    // VCMPPD YMM1, YMM0, YMM2, 0x04
    let code = [
        0xc5, 0xfd, 0xc2, 0xca, 0x04, // VCMPPD YMM1, YMM0, YMM2, 0x04
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmppd_ymm_extended_regs() {
    let mut emu = emu64();
    // VCMPPD YMM11, YMM12, YMM13, 0x00
    let code = [
        0xc4, 0xc1, 0x1d, 0xc2, 0xdd, 0x00, // VCMPPD YMM11, YMM12, YMM13, 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmppd_ymm_mem() {
    let mut emu = emu64();
    // VCMPPD YMM1, YMM0, [mem], 0x00
    let code = [
        0xc5, 0xfd, 0xc2, 0x0d, 0x00, 0x40, 0x00, 0x00, 0x00, // VCMPPD YMM1, YMM0, [rip+0x4000], 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0x3f,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x40,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x40,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vcmppd_xmm_extended_regs() {
    let mut emu = emu64();
    // VCMPPD XMM14, XMM15, XMM8, 0x01
    let code = [
        0xc4, 0xc1, 0x01, 0xc2, 0xf0, 0x01, // VCMPPD XMM14, XMM15, XMM8, 0x01
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmpps_xmm_different_regs() {
    let mut emu = emu64();
    // VCMPPS XMM3, XMM4, XMM5, 0x07
    let code = [
        0xc5, 0xd8, 0xc2, 0xdd, 0x07, // VCMPPS XMM3, XMM4, XMM5, 0x07
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmpps_ymm_different_regs() {
    let mut emu = emu64();
    // VCMPPS YMM5, YMM6, YMM7, 0x0E
    let code = [
        0xc5, 0xcc, 0xc2, 0xef, 0x0e, // VCMPPS YMM5, YMM6, YMM7, 0x0E
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmppd_xmm_different_regs() {
    let mut emu = emu64();
    // VCMPPD XMM6, XMM7, XMM2, 0x0C
    let code = [
        0xc5, 0xc9, 0xc2, 0xf2, 0x0c, // VCMPPD XMM6, XMM7, XMM2, 0x0C
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcmppd_ymm_different_regs() {
    let mut emu = emu64();
    // VCMPPD YMM3, YMM1, YMM4, 0x1F
    let code = [
        0xc5, 0xf5, 0xc2, 0xdc, 0x1f, // VCMPPD YMM3, YMM1, YMM4, 0x1F
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
