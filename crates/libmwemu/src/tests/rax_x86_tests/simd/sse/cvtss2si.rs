use crate::*;

// CVTSS2SI - Convert Scalar Single Precision to Signed Integer
// Opcode: F3 0F 2D /r    CVTSS2SI r32, xmm1/m32
//         F3 REX.W 0F 2D /r    CVTSS2SI r64, xmm1/m32

const DATA_ADDR: u64 = 0x3000;

// Register Tests
#[test]
fn test_cvtss2si_eax_xmm0() {
    let mut emu = emu64();
    let code = [0xf3, 0x0f, 0x2d, 0xc0, 0xf4]; // CVTSS2SI EAX, XMM0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtss2si_ebx_xmm1() {
    let mut emu = emu64();
    let code = [0xf3, 0x0f, 0x2d, 0xd9, 0xf4]; // CVTSS2SI EBX, XMM1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtss2si_ecx_xmm2() {
    let mut emu = emu64();
    let code = [0xf3, 0x0f, 0x2d, 0xca, 0xf4]; // CVTSS2SI ECX, XMM2
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtss2si_edx_xmm3() {
    let mut emu = emu64();
    let code = [0xf3, 0x0f, 0x2d, 0xd3, 0xf4]; // CVTSS2SI EDX, XMM3
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtss2si_esi_xmm7() {
    let mut emu = emu64();
    let code = [0xf3, 0x0f, 0x2d, 0xf7, 0xf4]; // CVTSS2SI ESI, XMM7
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// 64-bit Tests
#[test]
fn test_cvtss2si_rax_xmm0_64() {
    let mut emu = emu64();
    let code = [0xf3, 0x48, 0x0f, 0x2d, 0xc0, 0xf4]; // CVTSS2SI RAX, XMM0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtss2si_rbx_xmm1_64() {
    let mut emu = emu64();
    let code = [0xf3, 0x48, 0x0f, 0x2d, 0xd9, 0xf4]; // CVTSS2SI RBX, XMM1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtss2si_r9_xmm8_64() {
    let mut emu = emu64();
    let code = [0xf3, 0x4d, 0x0f, 0x2d, 0xc8, 0xf4]; // CVTSS2SI R9, XMM8
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Memory Tests
#[test]
fn test_cvtss2si_mem32() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2d, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let f1: f32 = 42.0;
    emu.maps.write_bytes_slice(DATA_ADDR, &f1.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtss2si_mem64() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x48, 0x0f, 0x2d, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let f1: f32 = 1234567.0;
    emu.maps.write_bytes_slice(DATA_ADDR, &f1.to_le_bytes());
    emu.run(None).unwrap();
}

// Special Values
#[test]
fn test_cvtss2si_zero() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2d, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let f1: f32 = 0.0;
    emu.maps.write_bytes_slice(DATA_ADDR, &f1.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtss2si_negative_zero() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2d, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let f1: f32 = -0.0;
    emu.maps.write_bytes_slice(DATA_ADDR, &f1.to_le_bytes());
    emu.run(None).unwrap();
}

// Rounding Tests
#[test]
fn test_cvtss2si_round_down() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2d, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let f1: f32 = 42.3;
    emu.maps.write_bytes_slice(DATA_ADDR, &f1.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtss2si_round_up() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2d, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let f1: f32 = 42.7;
    emu.maps.write_bytes_slice(DATA_ADDR, &f1.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtss2si_round_half_even() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2d, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let f1: f32 = 42.5;
    emu.maps.write_bytes_slice(DATA_ADDR, &f1.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtss2si_round_negative() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2d, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let f1: f32 = -42.7;
    emu.maps.write_bytes_slice(DATA_ADDR, &f1.to_le_bytes());
    emu.run(None).unwrap();
}

// Normal Values
#[test]
fn test_cvtss2si_one() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2d, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let f1: f32 = 1.0;
    emu.maps.write_bytes_slice(DATA_ADDR, &f1.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtss2si_minus_one() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2d, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let f1: f32 = -1.0;
    emu.maps.write_bytes_slice(DATA_ADDR, &f1.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtss2si_positive_large() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2d, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let f1: f32 = 1000000.0;
    emu.maps.write_bytes_slice(DATA_ADDR, &f1.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtss2si_negative_large() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2d, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let f1: f32 = -1000000.0;
    emu.maps.write_bytes_slice(DATA_ADDR, &f1.to_le_bytes());
    emu.run(None).unwrap();
}

// Overflow Cases (should return indefinite integer)
#[test]
fn test_cvtss2si_overflow_positive() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2d, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let f1: f32 = 3.0e9; // Larger than i32::MAX
    emu.maps.write_bytes_slice(DATA_ADDR, &f1.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtss2si_overflow_negative() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2d, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let f1: f32 = -3.0e9; // Less than i32::MIN
    emu.maps.write_bytes_slice(DATA_ADDR, &f1.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtss2si_infinity_positive() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2d, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let f1: f32 = f32::INFINITY;
    emu.maps.write_bytes_slice(DATA_ADDR, &f1.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtss2si_infinity_negative() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2d, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let f1: f32 = f32::NEG_INFINITY;
    emu.maps.write_bytes_slice(DATA_ADDR, &f1.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtss2si_nan() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2d, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let f1: f32 = f32::NAN;
    emu.maps.write_bytes_slice(DATA_ADDR, &f1.to_le_bytes());
    emu.run(None).unwrap();
}

// Edge Cases
#[test]
fn test_cvtss2si_near_max_i32() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2d, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let f1: f32 = 2147483000.0; // Near i32::MAX
    emu.maps.write_bytes_slice(DATA_ADDR, &f1.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtss2si_near_min_i32() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2d, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let f1: f32 = -2147483000.0; // Near i32::MIN
    emu.maps.write_bytes_slice(DATA_ADDR, &f1.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtss2si_small_fractional() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2d, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let f1: f32 = 0.9;
    emu.maps.write_bytes_slice(DATA_ADDR, &f1.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtss2si_very_small() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2d, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let f1: f32 = 0.000001;
    emu.maps.write_bytes_slice(DATA_ADDR, &f1.to_le_bytes());
    emu.run(None).unwrap();
}

// Misc
#[test]
fn test_cvtss2si_100_point_99() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2d, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let f1: f32 = 100.99;
    emu.maps.write_bytes_slice(DATA_ADDR, &f1.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtss2si_minus_100_point_99() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2d, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let f1: f32 = -100.99;
    emu.maps.write_bytes_slice(DATA_ADDR, &f1.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtss2si_displacement() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(DATA_ADDR - 16).to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2d, 0x40, 0x10, 0xf4]);

    emu.load_code_bytes(&full_code);
    let f1: f32 = 777.0;
    emu.maps.write_bytes_slice(DATA_ADDR, &f1.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtss2si_multiple() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x2d, 0x00, // CVTSS2SI EAX, [RAX]
        0xf3, 0x0f, 0x2d, 0x08, // CVTSS2SI ECX, [RAX]
        0xf3, 0x0f, 0x2d, 0x10, // CVTSS2SI EDX, [RAX]
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let f1: f32 = 456.0;
    emu.maps.write_bytes_slice(DATA_ADDR, &f1.to_le_bytes());
    emu.run(None).unwrap();
}
