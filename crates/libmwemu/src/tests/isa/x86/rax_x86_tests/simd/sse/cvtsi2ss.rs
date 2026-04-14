use crate::*;

// CVTSI2SS - Convert Signed Integer to Scalar Single Precision
// Opcode: F3 0F 2A /r    CVTSI2SS xmm1, r/m32
//         F3 REX.W 0F 2A /r    CVTSI2SS xmm1, r/m64

const DATA_ADDR: u64 = 0x3000;

// Register Tests
#[test]
fn test_cvtsi2ss_xmm0_eax() {
    let mut emu = emu64();
    let code = [0xf3, 0x0f, 0x2a, 0xc0, 0xf4]; // CVTSI2SS XMM0, EAX
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtsi2ss_xmm1_ebx() {
    let mut emu = emu64();
    let code = [0xf3, 0x0f, 0x2a, 0xcb, 0xf4]; // CVTSI2SS XMM1, EBX
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtsi2ss_xmm2_ecx() {
    let mut emu = emu64();
    let code = [0xf3, 0x0f, 0x2a, 0xd1, 0xf4]; // CVTSI2SS XMM2, ECX
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtsi2ss_xmm3_edx() {
    let mut emu = emu64();
    let code = [0xf3, 0x0f, 0x2a, 0xda, 0xf4]; // CVTSI2SS XMM3, EDX
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtsi2ss_xmm7_esi() {
    let mut emu = emu64();
    let code = [0xf3, 0x0f, 0x2a, 0xfe, 0xf4]; // CVTSI2SS XMM7, ESI
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// 64-bit Tests
#[test]
fn test_cvtsi2ss_xmm0_rax_64() {
    let mut emu = emu64();
    let code = [0xf3, 0x48, 0x0f, 0x2a, 0xc0, 0xf4]; // CVTSI2SS XMM0, RAX
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtsi2ss_xmm1_rbx_64() {
    let mut emu = emu64();
    let code = [0xf3, 0x48, 0x0f, 0x2a, 0xcb, 0xf4]; // CVTSI2SS XMM1, RBX
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtsi2ss_xmm8_r9_64() {
    let mut emu = emu64();
    let code = [0xf3, 0x4d, 0x0f, 0x2a, 0xc1, 0xf4]; // CVTSI2SS XMM8, R9
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Memory Tests
#[test]
fn test_cvtsi2ss_mem32() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2a, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let val: i32 = 42;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtsi2ss_mem64() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x48, 0x0f, 0x2a, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let val: i64 = 1234567890;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

// Special Values
#[test]
fn test_cvtsi2ss_zero() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2a, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let val: i32 = 0;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtsi2ss_max_i32() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2a, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let val: i32 = i32::MAX;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtsi2ss_min_i32() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2a, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let val: i32 = i32::MIN;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtsi2ss_max_i64() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x48, 0x0f, 0x2a, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let val: i64 = i64::MAX;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtsi2ss_min_i64() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x48, 0x0f, 0x2a, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let val: i64 = i64::MIN;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtsi2ss_positive_small() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2a, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let val: i32 = 1;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtsi2ss_negative_small() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2a, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let val: i32 = -1;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtsi2ss_positive_medium() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2a, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let val: i32 = 1000000;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtsi2ss_negative_medium() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2a, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let val: i32 = -1000000;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

// Powers of 2
#[test]
fn test_cvtsi2ss_power_of_2() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2a, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let val: i32 = 1024;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtsi2ss_negative_power_of_2() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2a, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let val: i32 = -1024;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

// Precision Loss Tests (large integers may lose precision in f32)
#[test]
fn test_cvtsi2ss_large_with_precision_loss() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2a, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let val: i32 = 16777217; // Larger than 2^24, may lose precision
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtsi2ss_64bit_large() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x48, 0x0f, 0x2a, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let val: i64 = 1000000000000;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

// Misc edge cases
#[test]
fn test_cvtsi2ss_100() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2a, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let val: i32 = 100;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtsi2ss_minus_100() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2a, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let val: i32 = -100;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtsi2ss_12345() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2a, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let val: i32 = 12345;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtsi2ss_minus_67890() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2a, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let val: i32 = -67890;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

// Addressing modes
#[test]
fn test_cvtsi2ss_displacement() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(DATA_ADDR - 16).to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2a, 0x40, 0x10, 0xf4]);

    emu.load_code_bytes(&full_code);
    let val: i32 = 999;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtsi2ss_multiple() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x2a, 0x00, // CVTSI2SS XMM0, [RAX]
        0xf3, 0x0f, 0x2a, 0x08, // CVTSI2SS XMM1, [RAX]
        0xf3, 0x0f, 0x2a, 0x10, // CVTSI2SS XMM2, [RAX]
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let val: i32 = 777;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtsi2ss_xmm15() {
    let mut emu = emu64();
    let code = [0xf3, 0x44, 0x0f, 0x2a, 0xf8, 0xf4]; // CVTSI2SS XMM15, EAX
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtsi2ss_r8d() {
    let mut emu = emu64();
    let code = [0xf3, 0x41, 0x0f, 0x2a, 0xc0, 0xf4]; // CVTSI2SS XMM0, R8D
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtsi2ss_boundary_8388608() {
    let mut emu = emu64();
    // 2^23 - exact boundary for f32 mantissa
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2a, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let val: i32 = 8388608;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}

#[test]
fn test_cvtsi2ss_boundary_16777216() {
    let mut emu = emu64();
    // 2^24 - largest exactly representable integer
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0xf3, 0x0f, 0x2a, 0x00, 0xf4]);

    emu.load_code_bytes(&full_code);
    let val: i32 = 16777216;
    emu.maps.write_bytes_slice(DATA_ADDR, &val.to_le_bytes());
    emu.run(None).unwrap();
}
