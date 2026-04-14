use crate::*;

// CMPPS - Compare Packed Single Precision Floating-Point Values
//
// Performs a SIMD compare of packed single-precision floating-point values.
// Returns a mask of all 1s (0xFFFFFFFF) if comparison is true, or all 0s if false.
//
// Opcode: NP 0F C2 /r ib - CMPPS xmm1, xmm2/m128, imm8
//
// Comparison predicates (imm8 bits 2:0 for SSE):
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

// Helper to create float bit patterns
fn float_bits(val: f32) -> [u8; 4] {
    val.to_le_bytes()
}

// Helper to create 4 floats
fn make_4floats(f0: f32, f1: f32, f2: f32, f3: f32) -> Vec<u8> {
    let mut data = Vec::new();
    data.extend_from_slice(&float_bits(f0));
    data.extend_from_slice(&float_bits(f1));
    data.extend_from_slice(&float_bits(f2));
    data.extend_from_slice(&float_bits(f3));
    data
}

// ============================================================================
// Predicate 0: EQ (Equal, ordered, non-signaling)
// ============================================================================

#[test]
fn test_cmpps_eq_all_equal() {
    let mut emu = emu64();
    // CMPPS XMM0, XMM1, 0 (EQ)
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
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0x0f, 0xc2, 0xc1, 0x00, // CMPPS XMM0, XMM1, 0 (EQ)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let data = make_4floats(1.0, 2.0, 3.0, 4.0);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data);

    emu.run(None).unwrap();
}

#[test]
fn test_cmpps_eq_none_equal() {
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
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0x0f, 0xc2, 0xc1, 0x00, // CMPPS XMM0, XMM1, 0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_4floats(1.0, 2.0, 3.0, 4.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_4floats(5.0, 6.0, 7.0, 8.0));

    emu.run(None).unwrap();
}

#[test]
fn test_cmpps_eq_partial_equal() {
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
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0x0f, 0xc2, 0xc1, 0x00, // CMPPS XMM0, XMM1, 0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_4floats(1.0, 2.0, 3.0, 4.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_4floats(1.0, 5.0, 3.0, 6.0));

    emu.run(None).unwrap();
}

#[test]
fn test_cmpps_eq_with_zeros() {
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
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0x0f, 0xc2, 0xc1, 0x00, // CMPPS XMM0, XMM1, 0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_4floats(0.0, 0.0, 0.0, 0.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_4floats(0.0, 0.0, 0.0, 0.0));

    emu.run(None).unwrap();
}

#[test]
fn test_cmpps_eq_with_negatives() {
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
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0x0f, 0xc2, 0xc1, 0x00, // CMPPS XMM0, XMM1, 0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_4floats(-1.0, -2.0, -3.0, -4.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_4floats(-1.0, -2.0, -3.0, -4.0));

    emu.run(None).unwrap();
}

// ============================================================================
// Predicate 1: LT (Less-than, ordered, signaling)
// ============================================================================

#[test]
fn test_cmpps_lt_all_less() {
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
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0x0f, 0xc2, 0xc1, 0x01, // CMPPS XMM0, XMM1, 1 (LT)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_4floats(1.0, 2.0, 3.0, 4.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_4floats(5.0, 6.0, 7.0, 8.0));

    emu.run(None).unwrap();
}

#[test]
fn test_cmpps_lt_none_less() {
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
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0x0f, 0xc2, 0xc1, 0x01, // CMPPS XMM0, XMM1, 1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_4floats(5.0, 6.0, 7.0, 8.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_4floats(1.0, 2.0, 3.0, 4.0));

    emu.run(None).unwrap();
}

#[test]
fn test_cmpps_lt_partial_less() {
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
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0x0f, 0xc2, 0xc1, 0x01, // CMPPS XMM0, XMM1, 1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    // 1.0 < 5.0, 6.0 > 2.0, 3.0 < 7.0, 8.0 > 4.0
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_4floats(1.0, 6.0, 3.0, 8.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_4floats(5.0, 2.0, 7.0, 4.0));

    emu.run(None).unwrap();
}

#[test]
fn test_cmpps_lt_with_negatives() {
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
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0x0f, 0xc2, 0xc1, 0x01, // CMPPS XMM0, XMM1, 1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_4floats(-5.0, -4.0, -3.0, -2.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_4floats(-1.0, -0.5, 0.0, 1.0));

    emu.run(None).unwrap();
}

#[test]
fn test_cmpps_lt_equal_not_less() {
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
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0x0f, 0xc2, 0xc1, 0x01, // CMPPS XMM0, XMM1, 1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_4floats(1.0, 2.0, 3.0, 4.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_4floats(1.0, 2.0, 3.0, 4.0));

    emu.run(None).unwrap();
}

// ============================================================================
// Predicate 2: LE (Less-than-or-equal, ordered, signaling)
// ============================================================================

#[test]
fn test_cmpps_le_all_less_or_equal() {
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
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0x0f, 0xc2, 0xc1, 0x02, // CMPPS XMM0, XMM1, 2 (LE)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_4floats(1.0, 2.0, 3.0, 4.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_4floats(1.0, 2.0, 3.0, 4.0));

    emu.run(None).unwrap();
}

#[test]
fn test_cmpps_le_mixed() {
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
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0x0f, 0xc2, 0xc1, 0x02, // CMPPS XMM0, XMM1, 2
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_4floats(1.0, 5.0, 3.0, 10.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_4floats(5.0, 5.0, 7.0, 2.0));

    emu.run(None).unwrap();
}

#[test]
fn test_cmpps_le_none() {
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
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0x0f, 0xc2, 0xc1, 0x02, // CMPPS XMM0, XMM1, 2
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_4floats(10.0, 20.0, 30.0, 40.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_4floats(1.0, 2.0, 3.0, 4.0));

    emu.run(None).unwrap();
}

// ============================================================================
// Predicate 3: UNORD (Unordered, non-signaling)
// ============================================================================

#[test]
fn test_cmpps_unord_with_nan_first() {
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
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0x0f, 0xc2, 0xc1, 0x03, // CMPPS XMM0, XMM1, 3 (UNORD)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_4floats(f32::NAN, 2.0, 3.0, 4.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_4floats(1.0, 2.0, 3.0, 4.0));

    emu.run(None).unwrap();
}

#[test]
fn test_cmpps_unord_with_nan_second() {
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
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0x0f, 0xc2, 0xc1, 0x03, // CMPPS XMM0, XMM1, 3
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_4floats(1.0, 2.0, 3.0, 4.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_4floats(f32::NAN, 2.0, 3.0, 4.0));

    emu.run(None).unwrap();
}

#[test]
fn test_cmpps_unord_with_nan_both() {
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
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0x0f, 0xc2, 0xc1, 0x03, // CMPPS XMM0, XMM1, 3
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_4floats(f32::NAN, f32::NAN, 3.0, 4.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_4floats(f32::NAN, 2.0, f32::NAN, 4.0));

    emu.run(None).unwrap();
}

#[test]
fn test_cmpps_unord_no_nan() {
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
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0x0f, 0xc2, 0xc1, 0x03, // CMPPS XMM0, XMM1, 3
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_4floats(1.0, 2.0, 3.0, 4.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_4floats(5.0, 6.0, 7.0, 8.0));

    emu.run(None).unwrap();
}

// ============================================================================
// Predicate 4: NEQ (Not-equal, unordered, non-signaling)
// ============================================================================

#[test]
fn test_cmpps_neq_all_different() {
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
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0x0f, 0xc2, 0xc1, 0x04, // CMPPS XMM0, XMM1, 4 (NEQ)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_4floats(1.0, 2.0, 3.0, 4.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_4floats(5.0, 6.0, 7.0, 8.0));

    emu.run(None).unwrap();
}

#[test]
fn test_cmpps_neq_all_same() {
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
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0x0f, 0xc2, 0xc1, 0x04, // CMPPS XMM0, XMM1, 4
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_4floats(1.0, 2.0, 3.0, 4.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_4floats(1.0, 2.0, 3.0, 4.0));

    emu.run(None).unwrap();
}

#[test]
fn test_cmpps_neq_with_nan() {
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
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0x0f, 0xc2, 0xc1, 0x04, // CMPPS XMM0, XMM1, 4
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    // NEQ is "unordered" so NaN comparisons should be true
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_4floats(f32::NAN, 2.0, 3.0, 4.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_4floats(1.0, 2.0, 3.0, 4.0));

    emu.run(None).unwrap();
}

// ============================================================================
// Predicate 5: NLT (Not-less-than, unordered, signaling)
// ============================================================================

#[test]
fn test_cmpps_nlt_all_greater_or_equal() {
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
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0x0f, 0xc2, 0xc1, 0x05, // CMPPS XMM0, XMM1, 5 (NLT)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_4floats(10.0, 20.0, 30.0, 40.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_4floats(1.0, 2.0, 3.0, 4.0));

    emu.run(None).unwrap();
}

#[test]
fn test_cmpps_nlt_all_less() {
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
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0x0f, 0xc2, 0xc1, 0x05, // CMPPS XMM0, XMM1, 5
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_4floats(1.0, 2.0, 3.0, 4.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_4floats(10.0, 20.0, 30.0, 40.0));

    emu.run(None).unwrap();
}

#[test]
fn test_cmpps_nlt_with_nan() {
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
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0x0f, 0xc2, 0xc1, 0x05, // CMPPS XMM0, XMM1, 5
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_4floats(f32::NAN, 20.0, 3.0, 4.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_4floats(10.0, 2.0, 30.0, 40.0));

    emu.run(None).unwrap();
}

// ============================================================================
// Predicate 6: NLE (Not-less-than-or-equal, unordered, signaling)
// ============================================================================

#[test]
fn test_cmpps_nle_all_greater() {
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
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0x0f, 0xc2, 0xc1, 0x06, // CMPPS XMM0, XMM1, 6 (NLE)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_4floats(10.0, 20.0, 30.0, 40.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_4floats(1.0, 2.0, 3.0, 4.0));

    emu.run(None).unwrap();
}

#[test]
fn test_cmpps_nle_all_equal() {
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
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0x0f, 0xc2, 0xc1, 0x06, // CMPPS XMM0, XMM1, 6
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_4floats(1.0, 2.0, 3.0, 4.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_4floats(1.0, 2.0, 3.0, 4.0));

    emu.run(None).unwrap();
}

#[test]
fn test_cmpps_nle_with_nan() {
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
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0x0f, 0xc2, 0xc1, 0x06, // CMPPS XMM0, XMM1, 6
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_4floats(f32::NAN, 2.0, 3.0, 4.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_4floats(1.0, 2.0, 3.0, 4.0));

    emu.run(None).unwrap();
}

// ============================================================================
// Predicate 7: ORD (Ordered, non-signaling)
// ============================================================================

#[test]
fn test_cmpps_ord_all_ordered() {
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
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0x0f, 0xc2, 0xc1, 0x07, // CMPPS XMM0, XMM1, 7 (ORD)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_4floats(1.0, 2.0, 3.0, 4.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_4floats(5.0, 6.0, 7.0, 8.0));

    emu.run(None).unwrap();
}

#[test]
fn test_cmpps_ord_with_nan_first() {
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
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0x0f, 0xc2, 0xc1, 0x07, // CMPPS XMM0, XMM1, 7
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_4floats(f32::NAN, 2.0, 3.0, 4.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_4floats(1.0, 2.0, 3.0, 4.0));

    emu.run(None).unwrap();
}

#[test]
fn test_cmpps_ord_with_nan_second() {
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
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0x0f, 0xc2, 0xc1, 0x07, // CMPPS XMM0, XMM1, 7
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_4floats(1.0, 2.0, 3.0, 4.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_4floats(1.0, f32::NAN, 3.0, 4.0));

    emu.run(None).unwrap();
}

#[test]
fn test_cmpps_ord_with_nan_both() {
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
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0x0f, 0xc2, 0xc1, 0x07, // CMPPS XMM0, XMM1, 7
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_4floats(f32::NAN, 2.0, 3.0, 4.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_4floats(f32::NAN, 2.0, 3.0, 4.0));

    emu.run(None).unwrap();
}

// ============================================================================
// Special Value Tests
// ============================================================================

#[test]
fn test_cmpps_with_infinity() {
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
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0x0f, 0xc2, 0xc1, 0x01, // CMPPS XMM0, XMM1, 1 (LT)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_4floats(1.0, f32::NEG_INFINITY, 3.0, 4.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_4floats(f32::INFINITY, 0.0, 7.0, 8.0));

    emu.run(None).unwrap();
}

#[test]
fn test_cmpps_positive_negative_zero() {
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
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0x0f, 0xc2, 0xc1, 0x00, // CMPPS XMM0, XMM1, 0 (EQ)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_4floats(0.0, 2.0, 3.0, 4.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_4floats(-0.0, 2.0, 3.0, 4.0));

    emu.run(None).unwrap();
}

#[test]
fn test_cmpps_denormal_values() {
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
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0x0f, 0xc2, 0xc1, 0x01, // CMPPS XMM0, XMM1, 1 (LT)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let denorm = f32::from_bits(0x00000001); // Smallest positive denormal
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_4floats(denorm, 2.0, 3.0, 4.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_4floats(1.0, 2.0, 3.0, 4.0));

    emu.run(None).unwrap();
}

// ============================================================================
// Memory Operand Tests
// ============================================================================

#[test]
fn test_cmpps_xmm_mem_eq() {
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
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0xc2, 0x03, 0x00, // CMPPS XMM0, [RBX], 0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let data = make_4floats(1.0, 2.0, 3.0, 4.0);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data);

    emu.run(None).unwrap();
}

#[test]
fn test_cmpps_xmm_mem_lt() {
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
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0xc2, 0x03, 0x01, // CMPPS XMM0, [RBX], 1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_4floats(1.0, 2.0, 3.0, 4.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_4floats(5.0, 6.0, 7.0, 8.0));

    emu.run(None).unwrap();
}

// ============================================================================
// Different Register Combinations
// ============================================================================

#[test]
fn test_cmpps_xmm2_xmm3() {
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
        0x0f, 0x28, 0x10, // MOVAPS XMM2, [RAX]
        0x0f, 0x28, 0x1b, // MOVAPS XMM3, [RBX]
        0x0f, 0xc2, 0xd3, 0x00, // CMPPS XMM2, XMM3, 0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let data = make_4floats(1.0, 2.0, 3.0, 4.0);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data);

    emu.run(None).unwrap();
}

#[test]
fn test_cmpps_xmm7_xmm6() {
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
        0x0f, 0x28, 0x38, // MOVAPS XMM7, [RAX]
        0x0f, 0x28, 0x33, // MOVAPS XMM6, [RBX]
        0x0f, 0xc2, 0xfe, 0x01, // CMPPS XMM7, XMM6, 1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_4floats(1.0, 2.0, 3.0, 4.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_4floats(5.0, 6.0, 7.0, 8.0));

    emu.run(None).unwrap();
}

#[test]
fn test_cmpps_xmm15_xmm8() {
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
        0x44, 0x0f, 0x28, 0x38, // MOVAPS XMM15, [RAX]
        0x44, 0x0f, 0x28, 0x03, // MOVAPS XMM8, [RBX]
        0x45, 0x0f, 0xc2, 0xf8, 0x02, // CMPPS XMM15, XMM8, 2
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_4floats(1.0, 2.0, 3.0, 4.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_4floats(1.0, 2.0, 3.0, 4.0));

    emu.run(None).unwrap();
}

// ============================================================================
// Complex Patterns
// ============================================================================

#[test]
fn test_cmpps_alternating_pattern() {
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
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0x0f, 0xc2, 0xc1, 0x00, // CMPPS XMM0, XMM1, 0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_4floats(1.0, 5.0, 3.0, 7.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_4floats(1.0, 2.0, 3.0, 4.0));

    emu.run(None).unwrap();
}

#[test]
fn test_cmpps_large_values() {
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
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0x0f, 0xc2, 0xc1, 0x01, // CMPPS XMM0, XMM1, 1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_4floats(1e30, 2e30, 3e30, 4e30));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_4floats(5e30, 6e30, 7e30, 8e30));

    emu.run(None).unwrap();
}

#[test]
fn test_cmpps_small_values() {
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
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0x0f, 0xc2, 0xc1, 0x01, // CMPPS XMM0, XMM1, 1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_4floats(1e-30, 2e-30, 3e-30, 4e-30));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_4floats(5e-30, 6e-30, 7e-30, 8e-30));

    emu.run(None).unwrap();
}

#[test]
fn test_cmpps_mixed_signs() {
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
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0x0f, 0xc2, 0xc1, 0x01, // CMPPS XMM0, XMM1, 1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_4floats(-1.0, 2.0, -3.0, 4.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_4floats(1.0, -2.0, 3.0, -4.0));

    emu.run(None).unwrap();
}

// ============================================================================
// Chained Comparisons
// ============================================================================

#[test]
fn test_cmpps_multiple_comparisons() {
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
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0x0f, 0xc2, 0xc1, 0x00, // CMPPS XMM0, XMM1, 0 (EQ)
        0x0f, 0x28, 0x10, // MOVAPS XMM2, [RAX]
        0x0f, 0xc2, 0xd3, 0x01, // CMPPS XMM2, XMM3, 1 (LT)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &make_4floats(1.0, 2.0, 3.0, 4.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &make_4floats(1.0, 2.0, 3.0, 4.0));

    emu.run(None).unwrap();
}
