use crate::*;

// CMPPD - Compare Packed Double Precision Floating-Point Values
//
// Performs a SIMD compare of packed double-precision floating-point values.
// Returns a mask of all 1s (0xFFFFFFFFFFFFFFFF) if comparison is true, or all 0s if false.
//
// Opcode: 66 0F C2 /r ib - CMPPD xmm1, xmm2/m128, imm8
//
// Comparison predicates (imm8 bits 2:0 for SSE2):
// 0 (EQ_OQ)   - Equal (ordered, non-signaling)
// 1 (LT_OS)   - Less-than (ordered, signaling)
// 2 (LE_OS)   - Less-than-or-equal (ordered, signaling)
// 3 (UNORD_Q) - Unordered (non-signaling)
// 4 (NEQ_UQ)  - Not-equal (unordered, non-signaling)
// 5 (NLT_US)  - Not-less-than (unordered, signaling)
// 6 (NLE_US)  - Not-less-than-or-equal (unordered, signaling)
// 7 (ORD_Q)   - Ordered (non-signaling)

const ALIGNED_ADDR: u64 = 0x3000;
const ALIGNED_ADDR2: u64 = 0x3100;

// Helper to create double bit patterns
fn double_bits(val: f64) -> [u8; 8] {
    val.to_le_bytes()
}

// Helper to create 2 doubles
fn make_2doubles(f0: f64, f1: f64) -> Vec<u8> {
    let mut data = Vec::new();
    data.extend_from_slice(&double_bits(f0));
    data.extend_from_slice(&double_bits(f1));
    data
}

// ============================================================================
// Predicate 0: EQ (Equal, ordered, non-signaling)
// ============================================================================

#[test]
fn test_cmppd_eq_all_equal() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x00, // MOVAPD XMM0, [RAX]
        0x66, 0x0f, 0x28, 0x0b, // MOVAPD XMM1, [RBX]
        0x66, 0x0f, 0xc2, 0xc1, 0x00, // CMPPD XMM0, XMM1, 0 (EQ)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let data = make_2doubles(1.0, 2.0);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data);

    emu.run(None).unwrap();
}

#[test]
fn test_cmppd_eq_none_equal() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x00, // MOVAPD XMM0, [RAX]
        0x66, 0x0f, 0x28, 0x0b, // MOVAPD XMM1, [RBX]
        0x66, 0x0f, 0xc2, 0xc1, 0x00, // CMPPD XMM0, XMM1, 0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_2doubles(1.0, 2.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_2doubles(3.0, 4.0));

    emu.run(None).unwrap();
}

#[test]
fn test_cmppd_eq_partial_equal() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x00, // MOVAPD XMM0, [RAX]
        0x66, 0x0f, 0x28, 0x0b, // MOVAPD XMM1, [RBX]
        0x66, 0x0f, 0xc2, 0xc1, 0x00, // CMPPD XMM0, XMM1, 0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_2doubles(1.0, 2.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_2doubles(1.0, 4.0));

    emu.run(None).unwrap();
}

#[test]
fn test_cmppd_eq_with_zeros() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x00, // MOVAPD XMM0, [RAX]
        0x66, 0x0f, 0x28, 0x0b, // MOVAPD XMM1, [RBX]
        0x66, 0x0f, 0xc2, 0xc1, 0x00, // CMPPD XMM0, XMM1, 0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_2doubles(0.0, 0.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_2doubles(0.0, 0.0));

    emu.run(None).unwrap();
}

#[test]
fn test_cmppd_eq_with_negatives() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x00, // MOVAPD XMM0, [RAX]
        0x66, 0x0f, 0x28, 0x0b, // MOVAPD XMM1, [RBX]
        0x66, 0x0f, 0xc2, 0xc1, 0x00, // CMPPD XMM0, XMM1, 0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_2doubles(-1.0, -2.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_2doubles(-1.0, -2.0));

    emu.run(None).unwrap();
}

#[test]
fn test_cmppd_eq_high_precision() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x00, // MOVAPD XMM0, [RAX]
        0x66, 0x0f, 0x28, 0x0b, // MOVAPD XMM1, [RBX]
        0x66, 0x0f, 0xc2, 0xc1, 0x00, // CMPPD XMM0, XMM1, 0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_2doubles(3.141592653589793, 2.718281828459045));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_2doubles(3.141592653589793, 2.718281828459045));

    emu.run(None).unwrap();
}

// ============================================================================
// Predicate 1: LT (Less-than, ordered, signaling)
// ============================================================================

#[test]
fn test_cmppd_lt_all_less() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x00, // MOVAPD XMM0, [RAX]
        0x66, 0x0f, 0x28, 0x0b, // MOVAPD XMM1, [RBX]
        0x66, 0x0f, 0xc2, 0xc1, 0x01, // CMPPD XMM0, XMM1, 1 (LT)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_2doubles(1.0, 2.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_2doubles(3.0, 4.0));

    emu.run(None).unwrap();
}

#[test]
fn test_cmppd_lt_none_less() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x00, // MOVAPD XMM0, [RAX]
        0x66, 0x0f, 0x28, 0x0b, // MOVAPD XMM1, [RBX]
        0x66, 0x0f, 0xc2, 0xc1, 0x01, // CMPPD XMM0, XMM1, 1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_2doubles(5.0, 6.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_2doubles(1.0, 2.0));

    emu.run(None).unwrap();
}

#[test]
fn test_cmppd_lt_partial_less() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x00, // MOVAPD XMM0, [RAX]
        0x66, 0x0f, 0x28, 0x0b, // MOVAPD XMM1, [RBX]
        0x66, 0x0f, 0xc2, 0xc1, 0x01, // CMPPD XMM0, XMM1, 1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_2doubles(1.0, 6.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_2doubles(5.0, 2.0));

    emu.run(None).unwrap();
}

#[test]
fn test_cmppd_lt_with_negatives() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x00, // MOVAPD XMM0, [RAX]
        0x66, 0x0f, 0x28, 0x0b, // MOVAPD XMM1, [RBX]
        0x66, 0x0f, 0xc2, 0xc1, 0x01, // CMPPD XMM0, XMM1, 1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_2doubles(-5.0, -4.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_2doubles(-1.0, -0.5));

    emu.run(None).unwrap();
}

#[test]
fn test_cmppd_lt_equal_not_less() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x00, // MOVAPD XMM0, [RAX]
        0x66, 0x0f, 0x28, 0x0b, // MOVAPD XMM1, [RBX]
        0x66, 0x0f, 0xc2, 0xc1, 0x01, // CMPPD XMM0, XMM1, 1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_2doubles(1.0, 2.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_2doubles(1.0, 2.0));

    emu.run(None).unwrap();
}

// ============================================================================
// Predicate 2: LE (Less-than-or-equal, ordered, signaling)
// ============================================================================

#[test]
fn test_cmppd_le_all_less_or_equal() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x00, // MOVAPD XMM0, [RAX]
        0x66, 0x0f, 0x28, 0x0b, // MOVAPD XMM1, [RBX]
        0x66, 0x0f, 0xc2, 0xc1, 0x02, // CMPPD XMM0, XMM1, 2 (LE)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_2doubles(1.0, 2.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_2doubles(1.0, 2.0));

    emu.run(None).unwrap();
}

#[test]
fn test_cmppd_le_mixed() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x00, // MOVAPD XMM0, [RAX]
        0x66, 0x0f, 0x28, 0x0b, // MOVAPD XMM1, [RBX]
        0x66, 0x0f, 0xc2, 0xc1, 0x02, // CMPPD XMM0, XMM1, 2
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_2doubles(1.0, 10.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_2doubles(5.0, 2.0));

    emu.run(None).unwrap();
}

#[test]
fn test_cmppd_le_none() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x00, // MOVAPD XMM0, [RAX]
        0x66, 0x0f, 0x28, 0x0b, // MOVAPD XMM1, [RBX]
        0x66, 0x0f, 0xc2, 0xc1, 0x02, // CMPPD XMM0, XMM1, 2
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_2doubles(10.0, 20.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_2doubles(1.0, 2.0));

    emu.run(None).unwrap();
}

// ============================================================================
// Predicate 3: UNORD (Unordered, non-signaling)
// ============================================================================

#[test]
fn test_cmppd_unord_with_nan_first() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x00, // MOVAPD XMM0, [RAX]
        0x66, 0x0f, 0x28, 0x0b, // MOVAPD XMM1, [RBX]
        0x66, 0x0f, 0xc2, 0xc1, 0x03, // CMPPD XMM0, XMM1, 3 (UNORD)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_2doubles(f64::NAN, 2.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_2doubles(1.0, 2.0));

    emu.run(None).unwrap();
}

#[test]
fn test_cmppd_unord_with_nan_second() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x00, // MOVAPD XMM0, [RAX]
        0x66, 0x0f, 0x28, 0x0b, // MOVAPD XMM1, [RBX]
        0x66, 0x0f, 0xc2, 0xc1, 0x03, // CMPPD XMM0, XMM1, 3
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_2doubles(1.0, 2.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_2doubles(f64::NAN, 2.0));

    emu.run(None).unwrap();
}

#[test]
fn test_cmppd_unord_with_nan_both() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x00, // MOVAPD XMM0, [RAX]
        0x66, 0x0f, 0x28, 0x0b, // MOVAPD XMM1, [RBX]
        0x66, 0x0f, 0xc2, 0xc1, 0x03, // CMPPD XMM0, XMM1, 3
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_2doubles(f64::NAN, f64::NAN));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_2doubles(f64::NAN, 2.0));

    emu.run(None).unwrap();
}

#[test]
fn test_cmppd_unord_no_nan() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x00, // MOVAPD XMM0, [RAX]
        0x66, 0x0f, 0x28, 0x0b, // MOVAPD XMM1, [RBX]
        0x66, 0x0f, 0xc2, 0xc1, 0x03, // CMPPD XMM0, XMM1, 3
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_2doubles(1.0, 2.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_2doubles(3.0, 4.0));

    emu.run(None).unwrap();
}

// ============================================================================
// Predicate 4: NEQ (Not-equal, unordered, non-signaling)
// ============================================================================

#[test]
fn test_cmppd_neq_all_different() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x00, // MOVAPD XMM0, [RAX]
        0x66, 0x0f, 0x28, 0x0b, // MOVAPD XMM1, [RBX]
        0x66, 0x0f, 0xc2, 0xc1, 0x04, // CMPPD XMM0, XMM1, 4 (NEQ)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_2doubles(1.0, 2.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_2doubles(3.0, 4.0));

    emu.run(None).unwrap();
}

#[test]
fn test_cmppd_neq_all_same() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x00, // MOVAPD XMM0, [RAX]
        0x66, 0x0f, 0x28, 0x0b, // MOVAPD XMM1, [RBX]
        0x66, 0x0f, 0xc2, 0xc1, 0x04, // CMPPD XMM0, XMM1, 4
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_2doubles(1.0, 2.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_2doubles(1.0, 2.0));

    emu.run(None).unwrap();
}

#[test]
fn test_cmppd_neq_with_nan() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x00, // MOVAPD XMM0, [RAX]
        0x66, 0x0f, 0x28, 0x0b, // MOVAPD XMM1, [RBX]
        0x66, 0x0f, 0xc2, 0xc1, 0x04, // CMPPD XMM0, XMM1, 4
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_2doubles(f64::NAN, 2.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_2doubles(1.0, 2.0));

    emu.run(None).unwrap();
}

// ============================================================================
// Predicate 5: NLT (Not-less-than, unordered, signaling)
// ============================================================================

#[test]
fn test_cmppd_nlt_all_greater_or_equal() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x00, // MOVAPD XMM0, [RAX]
        0x66, 0x0f, 0x28, 0x0b, // MOVAPD XMM1, [RBX]
        0x66, 0x0f, 0xc2, 0xc1, 0x05, // CMPPD XMM0, XMM1, 5 (NLT)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_2doubles(10.0, 20.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_2doubles(1.0, 2.0));

    emu.run(None).unwrap();
}

#[test]
fn test_cmppd_nlt_all_less() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x00, // MOVAPD XMM0, [RAX]
        0x66, 0x0f, 0x28, 0x0b, // MOVAPD XMM1, [RBX]
        0x66, 0x0f, 0xc2, 0xc1, 0x05, // CMPPD XMM0, XMM1, 5
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_2doubles(1.0, 2.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_2doubles(10.0, 20.0));

    emu.run(None).unwrap();
}

#[test]
fn test_cmppd_nlt_with_nan() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x00, // MOVAPD XMM0, [RAX]
        0x66, 0x0f, 0x28, 0x0b, // MOVAPD XMM1, [RBX]
        0x66, 0x0f, 0xc2, 0xc1, 0x05, // CMPPD XMM0, XMM1, 5
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_2doubles(f64::NAN, 20.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_2doubles(10.0, 2.0));

    emu.run(None).unwrap();
}

// ============================================================================
// Predicate 6: NLE (Not-less-than-or-equal, unordered, signaling)
// ============================================================================

#[test]
fn test_cmppd_nle_all_greater() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x00, // MOVAPD XMM0, [RAX]
        0x66, 0x0f, 0x28, 0x0b, // MOVAPD XMM1, [RBX]
        0x66, 0x0f, 0xc2, 0xc1, 0x06, // CMPPD XMM0, XMM1, 6 (NLE)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_2doubles(10.0, 20.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_2doubles(1.0, 2.0));

    emu.run(None).unwrap();
}

#[test]
fn test_cmppd_nle_all_equal() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x00, // MOVAPD XMM0, [RAX]
        0x66, 0x0f, 0x28, 0x0b, // MOVAPD XMM1, [RBX]
        0x66, 0x0f, 0xc2, 0xc1, 0x06, // CMPPD XMM0, XMM1, 6
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_2doubles(1.0, 2.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_2doubles(1.0, 2.0));

    emu.run(None).unwrap();
}

#[test]
fn test_cmppd_nle_with_nan() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x00, // MOVAPD XMM0, [RAX]
        0x66, 0x0f, 0x28, 0x0b, // MOVAPD XMM1, [RBX]
        0x66, 0x0f, 0xc2, 0xc1, 0x06, // CMPPD XMM0, XMM1, 6
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_2doubles(f64::NAN, 2.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_2doubles(1.0, 2.0));

    emu.run(None).unwrap();
}

// ============================================================================
// Predicate 7: ORD (Ordered, non-signaling)
// ============================================================================

#[test]
fn test_cmppd_ord_all_ordered() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x00, // MOVAPD XMM0, [RAX]
        0x66, 0x0f, 0x28, 0x0b, // MOVAPD XMM1, [RBX]
        0x66, 0x0f, 0xc2, 0xc1, 0x07, // CMPPD XMM0, XMM1, 7 (ORD)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_2doubles(1.0, 2.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_2doubles(3.0, 4.0));

    emu.run(None).unwrap();
}

#[test]
fn test_cmppd_ord_with_nan_first() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x00, // MOVAPD XMM0, [RAX]
        0x66, 0x0f, 0x28, 0x0b, // MOVAPD XMM1, [RBX]
        0x66, 0x0f, 0xc2, 0xc1, 0x07, // CMPPD XMM0, XMM1, 7
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_2doubles(f64::NAN, 2.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_2doubles(1.0, 2.0));

    emu.run(None).unwrap();
}

#[test]
fn test_cmppd_ord_with_nan_second() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x00, // MOVAPD XMM0, [RAX]
        0x66, 0x0f, 0x28, 0x0b, // MOVAPD XMM1, [RBX]
        0x66, 0x0f, 0xc2, 0xc1, 0x07, // CMPPD XMM0, XMM1, 7
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_2doubles(1.0, 2.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_2doubles(1.0, f64::NAN));

    emu.run(None).unwrap();
}

#[test]
fn test_cmppd_ord_with_nan_both() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x00, // MOVAPD XMM0, [RAX]
        0x66, 0x0f, 0x28, 0x0b, // MOVAPD XMM1, [RBX]
        0x66, 0x0f, 0xc2, 0xc1, 0x07, // CMPPD XMM0, XMM1, 7
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_2doubles(f64::NAN, 2.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_2doubles(f64::NAN, 2.0));

    emu.run(None).unwrap();
}

// ============================================================================
// Special Value Tests
// ============================================================================

#[test]
fn test_cmppd_with_infinity() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x00, // MOVAPD XMM0, [RAX]
        0x66, 0x0f, 0x28, 0x0b, // MOVAPD XMM1, [RBX]
        0x66, 0x0f, 0xc2, 0xc1, 0x01, // CMPPD XMM0, XMM1, 1 (LT)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_2doubles(1.0, f64::NEG_INFINITY));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_2doubles(f64::INFINITY, 0.0));

    emu.run(None).unwrap();
}

#[test]
fn test_cmppd_positive_negative_zero() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x00, // MOVAPD XMM0, [RAX]
        0x66, 0x0f, 0x28, 0x0b, // MOVAPD XMM1, [RBX]
        0x66, 0x0f, 0xc2, 0xc1, 0x00, // CMPPD XMM0, XMM1, 0 (EQ)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_2doubles(0.0, 2.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_2doubles(-0.0, 2.0));

    emu.run(None).unwrap();
}

#[test]
fn test_cmppd_denormal_values() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x00, // MOVAPD XMM0, [RAX]
        0x66, 0x0f, 0x28, 0x0b, // MOVAPD XMM1, [RBX]
        0x66, 0x0f, 0xc2, 0xc1, 0x01, // CMPPD XMM0, XMM1, 1 (LT)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let denorm = f64::from_bits(0x0000000000000001); // Smallest positive denormal
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_2doubles(denorm, 2.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_2doubles(1.0, 2.0));

    emu.run(None).unwrap();
}

// ============================================================================
// Memory Operand Tests
// ============================================================================

#[test]
fn test_cmppd_xmm_mem_eq() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x00, // MOVAPD XMM0, [RAX]
        0x66, 0x0f, 0xc2, 0x03, 0x00, // CMPPD XMM0, [RBX], 0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let data = make_2doubles(1.0, 2.0);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data);

    emu.run(None).unwrap();
}

#[test]
fn test_cmppd_xmm_mem_lt() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x00, // MOVAPD XMM0, [RAX]
        0x66, 0x0f, 0xc2, 0x03, 0x01, // CMPPD XMM0, [RBX], 1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_2doubles(1.0, 2.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_2doubles(3.0, 4.0));

    emu.run(None).unwrap();
}

// ============================================================================
// Different Register Combinations
// ============================================================================

#[test]
fn test_cmppd_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x10, // MOVAPD XMM2, [RAX]
        0x66, 0x0f, 0x28, 0x1b, // MOVAPD XMM3, [RBX]
        0x66, 0x0f, 0xc2, 0xd3, 0x00, // CMPPD XMM2, XMM3, 0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let data = make_2doubles(1.0, 2.0);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data);

    emu.run(None).unwrap();
}

#[test]
fn test_cmppd_xmm7_xmm6() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x38, // MOVAPD XMM7, [RAX]
        0x66, 0x0f, 0x28, 0x33, // MOVAPD XMM6, [RBX]
        0x66, 0x0f, 0xc2, 0xfe, 0x01, // CMPPD XMM7, XMM6, 1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_2doubles(1.0, 2.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_2doubles(3.0, 4.0));

    emu.run(None).unwrap();
}

#[test]
fn test_cmppd_xmm15_xmm8() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x28, 0x38, // MOVAPD XMM15, [RAX]
        0x66, 0x44, 0x0f, 0x28, 0x03, // MOVAPD XMM8, [RBX]
        0x66, 0x45, 0x0f, 0xc2, 0xf8, 0x02, // CMPPD XMM15, XMM8, 2
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_2doubles(1.0, 2.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_2doubles(1.0, 2.0));

    emu.run(None).unwrap();
}

// ============================================================================
// Complex Patterns
// ============================================================================

#[test]
fn test_cmppd_large_values() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x00, // MOVAPD XMM0, [RAX]
        0x66, 0x0f, 0x28, 0x0b, // MOVAPD XMM1, [RBX]
        0x66, 0x0f, 0xc2, 0xc1, 0x01, // CMPPD XMM0, XMM1, 1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_2doubles(1e200, 2e200));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_2doubles(3e200, 4e200));

    emu.run(None).unwrap();
}

#[test]
fn test_cmppd_small_values() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x00, // MOVAPD XMM0, [RAX]
        0x66, 0x0f, 0x28, 0x0b, // MOVAPD XMM1, [RBX]
        0x66, 0x0f, 0xc2, 0xc1, 0x01, // CMPPD XMM0, XMM1, 1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_2doubles(1e-200, 2e-200));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_2doubles(3e-200, 4e-200));

    emu.run(None).unwrap();
}

#[test]
fn test_cmppd_mixed_signs() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x00, // MOVAPD XMM0, [RAX]
        0x66, 0x0f, 0x28, 0x0b, // MOVAPD XMM1, [RBX]
        0x66, 0x0f, 0xc2, 0xc1, 0x01, // CMPPD XMM0, XMM1, 1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_2doubles(-1.0, 2.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_2doubles(1.0, -2.0));

    emu.run(None).unwrap();
}

// ============================================================================
// Chained Comparisons
// ============================================================================

#[test]
fn test_cmppd_multiple_comparisons() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x00, // MOVAPD XMM0, [RAX]
        0x66, 0x0f, 0x28, 0x0b, // MOVAPD XMM1, [RBX]
        0x66, 0x0f, 0xc2, 0xc1, 0x00, // CMPPD XMM0, XMM1, 0 (EQ)
        0x66, 0x0f, 0x28, 0x10, // MOVAPD XMM2, [RAX]
        0x66, 0x0f, 0xc2, 0xd3, 0x01, // CMPPD XMM2, XMM3, 1 (LT)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_2doubles(1.0, 2.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_2doubles(1.0, 2.0));

    emu.run(None).unwrap();
}
