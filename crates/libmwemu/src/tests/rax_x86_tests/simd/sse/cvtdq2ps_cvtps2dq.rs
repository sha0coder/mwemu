use crate::*;

// CVTDQ2PS - Convert Packed Doubleword Integers to Packed Single Precision
// CVTPS2DQ - Convert Packed Single Precision to Packed Signed Doubleword Integers
// Opcode: NP 0F 5B /r         CVTDQ2PS xmm1, xmm2/m128
//         66 0F 5B /r         CVTPS2DQ xmm1, xmm2/m128

const DATA_ADDR: u64 = 0x3000;

// CVTDQ2PS xmm, xmm - Convert 4x int32 to 4x float32
#[test]
fn test_cvtdq2ps_xmm0_xmm1() {
    let mut emu = emu64();
    let code = [0x0f, 0x5b, 0xc1, 0xf4]; // CVTDQ2PS XMM0, XMM1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtdq2ps_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [0x0f, 0x5b, 0xd3, 0xf4]; // CVTDQ2PS XMM2, XMM3
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtdq2ps_xmm7_xmm0() {
    let mut emu = emu64();
    let code = [0x0f, 0x5b, 0xf8, 0xf4]; // CVTDQ2PS XMM7, XMM0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtdq2ps_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [0x45, 0x0f, 0x5b, 0xc1, 0xf4]; // CVTDQ2PS XMM8, XMM9
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtdq2ps_xmm15_xmm14() {
    let mut emu = emu64();
    let code = [0x45, 0x0f, 0x5b, 0xfe, 0xf4]; // CVTDQ2PS XMM15, XMM14
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// CVTDQ2PS xmm, m128 - Convert from memory
#[test]
fn test_cvtdq2ps_xmm0_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x0f, 0x5b, 0x00, 0xf4]); // CVTDQ2PS XMM0, [RAX]

    emu.load_code_bytes(&full_code);
    let vals: [i32; 4] = [1, 2, 3, 4];
    for (i, &val) in vals.iter().enumerate() {
        emu.maps.write_bytes_slice(DATA_ADDR + (i * 4) as u64, &val.to_le_bytes());
    }
    emu.run(None).unwrap();
}

#[test]
fn test_cvtdq2ps_positive_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x0f, 0x5b, 0x00, 0xf4]); // CVTDQ2PS XMM0, [RAX]

    emu.load_code_bytes(&full_code);
    let vals: [i32; 4] = [100, 1000, 10000, 100000];
    for (i, &val) in vals.iter().enumerate() {
        emu.maps.write_bytes_slice(DATA_ADDR + (i * 4) as u64, &val.to_le_bytes());
    }
    emu.run(None).unwrap();
}

#[test]
fn test_cvtdq2ps_negative_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x0f, 0x5b, 0x00, 0xf4]); // CVTDQ2PS XMM0, [RAX]

    emu.load_code_bytes(&full_code);
    let vals: [i32; 4] = [-100, -1000, -10000, -100000];
    for (i, &val) in vals.iter().enumerate() {
        emu.maps.write_bytes_slice(DATA_ADDR + (i * 4) as u64, &val.to_le_bytes());
    }
    emu.run(None).unwrap();
}

#[test]
fn test_cvtdq2ps_mixed_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x0f, 0x5b, 0x00, 0xf4]); // CVTDQ2PS XMM0, [RAX]

    emu.load_code_bytes(&full_code);
    let vals: [i32; 4] = [-42, 0, 42, 123456];
    for (i, &val) in vals.iter().enumerate() {
        emu.maps.write_bytes_slice(DATA_ADDR + (i * 4) as u64, &val.to_le_bytes());
    }
    emu.run(None).unwrap();
}

#[test]
fn test_cvtdq2ps_zeros() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x0f, 0x5b, 0x00, 0xf4]); // CVTDQ2PS XMM0, [RAX]

    emu.load_code_bytes(&full_code);
    let vals: [i32; 4] = [0, 0, 0, 0];
    for (i, &val) in vals.iter().enumerate() {
        emu.maps.write_bytes_slice(DATA_ADDR + (i * 4) as u64, &val.to_le_bytes());
    }
    emu.run(None).unwrap();
}

#[test]
fn test_cvtdq2ps_max_min() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x0f, 0x5b, 0x00, 0xf4]); // CVTDQ2PS XMM0, [RAX]

    emu.load_code_bytes(&full_code);
    let vals: [i32; 4] = [i32::MAX, i32::MIN, 0, 1];
    for (i, &val) in vals.iter().enumerate() {
        emu.maps.write_bytes_slice(DATA_ADDR + (i * 4) as u64, &val.to_le_bytes());
    }
    emu.run(None).unwrap();
}

#[test]
fn test_cvtdq2ps_powers_of_two() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x0f, 0x5b, 0x00, 0xf4]); // CVTDQ2PS XMM0, [RAX]

    emu.load_code_bytes(&full_code);
    let vals: [i32; 4] = [1, 256, 65536, 16777216];
    for (i, &val) in vals.iter().enumerate() {
        emu.maps.write_bytes_slice(DATA_ADDR + (i * 4) as u64, &val.to_le_bytes());
    }
    emu.run(None).unwrap();
}

// CVTPS2DQ xmm, xmm - Convert 4x float32 to 4x int32
#[test]
fn test_cvtps2dq_xmm0_xmm1() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x5b, 0xc1, 0xf4]; // CVTPS2DQ XMM0, XMM1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2dq_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x5b, 0xd3, 0xf4]; // CVTPS2DQ XMM2, XMM3
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2dq_xmm7_xmm0() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x5b, 0xf8, 0xf4]; // CVTPS2DQ XMM7, XMM0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2dq_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [0x66, 0x45, 0x0f, 0x5b, 0xc1, 0xf4]; // CVTPS2DQ XMM8, XMM9
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2dq_xmm15_xmm14() {
    let mut emu = emu64();
    let code = [0x66, 0x45, 0x0f, 0x5b, 0xfe, 0xf4]; // CVTPS2DQ XMM15, XMM14
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// CVTPS2DQ xmm, m128 - Convert from memory
#[test]
fn test_cvtps2dq_xmm0_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0x5b, 0x00, 0xf4]); // CVTPS2DQ XMM0, [RAX]

    emu.load_code_bytes(&full_code);
    let vals: [f32; 4] = [1.0, 2.0, 3.0, 4.0];
    for (i, &val) in vals.iter().enumerate() {
        emu.maps.write_bytes_slice(DATA_ADDR + (i * 4) as u64, &val.to_le_bytes());
    }
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2dq_positive_floats() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0x5b, 0x00, 0xf4]); // CVTPS2DQ XMM0, [RAX]

    emu.load_code_bytes(&full_code);
    let vals: [f32; 4] = [10.5, 100.7, 1000.3, 9999.9];
    for (i, &val) in vals.iter().enumerate() {
        emu.maps.write_bytes_slice(DATA_ADDR + (i * 4) as u64, &val.to_le_bytes());
    }
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2dq_negative_floats() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0x5b, 0x00, 0xf4]); // CVTPS2DQ XMM0, [RAX]

    emu.load_code_bytes(&full_code);
    let vals: [f32; 4] = [-10.5, -100.7, -1000.3, -9999.9];
    for (i, &val) in vals.iter().enumerate() {
        emu.maps.write_bytes_slice(DATA_ADDR + (i * 4) as u64, &val.to_le_bytes());
    }
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2dq_rounding_nearest_even() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0x5b, 0x00, 0xf4]); // CVTPS2DQ XMM0, [RAX]

    emu.load_code_bytes(&full_code);
    let vals: [f32; 4] = [2.5, 3.5, -2.5, -3.5];
    for (i, &val) in vals.iter().enumerate() {
        emu.maps.write_bytes_slice(DATA_ADDR + (i * 4) as u64, &val.to_le_bytes());
    }
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2dq_rounding_up() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0x5b, 0x00, 0xf4]); // CVTPS2DQ XMM0, [RAX]

    emu.load_code_bytes(&full_code);
    let vals: [f32; 4] = [1.1, 2.9, 10.6, 99.8];
    for (i, &val) in vals.iter().enumerate() {
        emu.maps.write_bytes_slice(DATA_ADDR + (i * 4) as u64, &val.to_le_bytes());
    }
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2dq_rounding_down() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0x5b, 0x00, 0xf4]); // CVTPS2DQ XMM0, [RAX]

    emu.load_code_bytes(&full_code);
    let vals: [f32; 4] = [1.2, 2.3, 10.4, 99.1];
    for (i, &val) in vals.iter().enumerate() {
        emu.maps.write_bytes_slice(DATA_ADDR + (i * 4) as u64, &val.to_le_bytes());
    }
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2dq_zeros() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0x5b, 0x00, 0xf4]); // CVTPS2DQ XMM0, [RAX]

    emu.load_code_bytes(&full_code);
    let vals: [f32; 4] = [0.0, -0.0, 0.0, 0.0];
    for (i, &val) in vals.iter().enumerate() {
        emu.maps.write_bytes_slice(DATA_ADDR + (i * 4) as u64, &val.to_le_bytes());
    }
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2dq_whole_numbers() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0x5b, 0x00, 0xf4]); // CVTPS2DQ XMM0, [RAX]

    emu.load_code_bytes(&full_code);
    let vals: [f32; 4] = [1.0, -1.0, 100.0, -100.0];
    for (i, &val) in vals.iter().enumerate() {
        emu.maps.write_bytes_slice(DATA_ADDR + (i * 4) as u64, &val.to_le_bytes());
    }
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2dq_large_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0x5b, 0x00, 0xf4]); // CVTPS2DQ XMM0, [RAX]

    emu.load_code_bytes(&full_code);
    let vals: [f32; 4] = [1000000.0, -1000000.0, 8388608.0, -8388608.0];
    for (i, &val) in vals.iter().enumerate() {
        emu.maps.write_bytes_slice(DATA_ADDR + (i * 4) as u64, &val.to_le_bytes());
    }
    emu.run(None).unwrap();
}

// Test overflow handling - values too large for int32
#[test]
fn test_cvtps2dq_overflow_positive() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0x5b, 0x00, 0xf4]); // CVTPS2DQ XMM0, [RAX]

    emu.load_code_bytes(&full_code);
    let vals: [f32; 4] = [3e9, 1e10, f32::MAX, 2147483648.0];
    for (i, &val) in vals.iter().enumerate() {
        emu.maps.write_bytes_slice(DATA_ADDR + (i * 4) as u64, &val.to_le_bytes());
    }
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2dq_overflow_negative() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0x5b, 0x00, 0xf4]); // CVTPS2DQ XMM0, [RAX]

    emu.load_code_bytes(&full_code);
    let vals: [f32; 4] = [-3e9, -1e10, f32::MIN, -2147483649.0];
    for (i, &val) in vals.iter().enumerate() {
        emu.maps.write_bytes_slice(DATA_ADDR + (i * 4) as u64, &val.to_le_bytes());
    }
    emu.run(None).unwrap();
}

// Test special float values
#[test]
fn test_cvtps2dq_infinity() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0x5b, 0x00, 0xf4]); // CVTPS2DQ XMM0, [RAX]

    emu.load_code_bytes(&full_code);
    let vals: [f32; 4] = [f32::INFINITY, f32::NEG_INFINITY, 0.0, 1.0];
    for (i, &val) in vals.iter().enumerate() {
        emu.maps.write_bytes_slice(DATA_ADDR + (i * 4) as u64, &val.to_le_bytes());
    }
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2dq_nan() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0x5b, 0x00, 0xf4]); // CVTPS2DQ XMM0, [RAX]

    emu.load_code_bytes(&full_code);
    let vals: [f32; 4] = [f32::NAN, 1.0, 2.0, 3.0];
    for (i, &val) in vals.iter().enumerate() {
        emu.maps.write_bytes_slice(DATA_ADDR + (i * 4) as u64, &val.to_le_bytes());
    }
    emu.run(None).unwrap();
}

// Roundtrip tests
#[test]
fn test_cvtdq2ps_cvtps2dq_roundtrip() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x5b, 0x00,       // CVTDQ2PS XMM0, [RAX]
        0x66, 0x0f, 0x5b, 0xc8, // CVTPS2DQ XMM1, XMM0
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let vals: [i32; 4] = [42, -42, 1000, -1000];
    for (i, &val) in vals.iter().enumerate() {
        emu.maps.write_bytes_slice(DATA_ADDR + (i * 4) as u64, &val.to_le_bytes());
    }
    emu.run(None).unwrap();
}

// Test with different XMM registers
#[test]
fn test_cvtdq2ps_xmm10_xmm11() {
    let mut emu = emu64();
    let code = [0x45, 0x0f, 0x5b, 0xd3, 0xf4]; // CVTDQ2PS XMM10, XMM11
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2dq_xmm12_xmm13() {
    let mut emu = emu64();
    let code = [0x66, 0x45, 0x0f, 0x5b, 0xe5, 0xf4]; // CVTPS2DQ XMM12, XMM13
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Test memory with displacement
#[test]
fn test_cvtdq2ps_mem_disp() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(DATA_ADDR - 16).to_le_bytes());
    full_code.extend_from_slice(&[0x0f, 0x5b, 0x40, 0x10, 0xf4]); // CVTDQ2PS XMM0, [RAX+16]

    emu.load_code_bytes(&full_code);
    let vals: [i32; 4] = [123, 456, 789, 1011];
    for (i, &val) in vals.iter().enumerate() {
        emu.maps.write_bytes_slice(DATA_ADDR + (i * 4) as u64, &val.to_le_bytes());
    }
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2dq_mem_disp() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(DATA_ADDR - 16).to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0x5b, 0x40, 0x10, 0xf4]); // CVTPS2DQ XMM0, [RAX+16]

    emu.load_code_bytes(&full_code);
    let vals: [f32; 4] = [123.5, 456.7, 789.1, 1011.9];
    for (i, &val) in vals.iter().enumerate() {
        emu.maps.write_bytes_slice(DATA_ADDR + (i * 4) as u64, &val.to_le_bytes());
    }
    emu.run(None).unwrap();
}

// Test fraction boundary cases
#[test]
fn test_cvtps2dq_fraction_0_25() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0x5b, 0x00, 0xf4]); // CVTPS2DQ XMM0, [RAX]

    emu.load_code_bytes(&full_code);
    let vals: [f32; 4] = [0.25, -0.25, 1.25, -1.25];
    for (i, &val) in vals.iter().enumerate() {
        emu.maps.write_bytes_slice(DATA_ADDR + (i * 4) as u64, &val.to_le_bytes());
    }
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2dq_fraction_0_75() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x66, 0x0f, 0x5b, 0x00, 0xf4]); // CVTPS2DQ XMM0, [RAX]

    emu.load_code_bytes(&full_code);
    let vals: [f32; 4] = [0.75, -0.75, 1.75, -1.75];
    for (i, &val) in vals.iter().enumerate() {
        emu.maps.write_bytes_slice(DATA_ADDR + (i * 4) as u64, &val.to_le_bytes());
    }
    emu.run(None).unwrap();
}
