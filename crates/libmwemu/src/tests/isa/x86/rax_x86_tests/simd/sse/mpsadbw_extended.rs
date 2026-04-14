use crate::*;

// MPSADBW Extended Tests - Additional comprehensive coverage
//
// Compute Multiple Packed Sums of Absolute Difference
// Opcode: 66 0F 3A 42 /r ib

const ALIGNED_ADDR: u64 = 0x3000;

#[test]
fn test_mpsadbw_xmm2_xmm4_offset_0() {
    let mut emu = emu64();
    // MPSADBW XMM2, XMM4, 0
    let code = [0x66, 0x0f, 0x3a, 0x42, 0xd4, 0x00,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm3_xmm5_offset_1() {
    let mut emu = emu64();
    // MPSADBW XMM3, XMM5, 1
    let code = [0x66, 0x0f, 0x3a, 0x42, 0xdd, 0x01,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm4_xmm6_offset_2() {
    let mut emu = emu64();
    // MPSADBW XMM4, XMM6, 2
    let code = [0x66, 0x0f, 0x3a, 0x42, 0xe6, 0x02,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm5_xmm7_offset_3() {
    let mut emu = emu64();
    // MPSADBW XMM5, XMM7, 3
    let code = [0x66, 0x0f, 0x3a, 0x42, 0xef, 0x03,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm6_xmm0_offset_4() {
    let mut emu = emu64();
    // MPSADBW XMM6, XMM0, 4
    let code = [0x66, 0x0f, 0x3a, 0x42, 0xf0, 0x04,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm7_xmm1_offset_5() {
    let mut emu = emu64();
    // MPSADBW XMM7, XMM1, 5
    let code = [0x66, 0x0f, 0x3a, 0x42, 0xf9, 0x05,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm0_xmm2_offset_6() {
    let mut emu = emu64();
    // MPSADBW XMM0, XMM2, 6
    let code = [0x66, 0x0f, 0x3a, 0x42, 0xc2, 0x06,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm1_xmm3_offset_7() {
    let mut emu = emu64();
    // MPSADBW XMM1, XMM3, 7
    let code = [0x66, 0x0f, 0x3a, 0x42, 0xcb, 0x07,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm8_xmm10_offset_0() {
    let mut emu = emu64();
    // MPSADBW XMM8, XMM10, 0
    let code = [0x66, 0x45, 0x0f, 0x3a, 0x42, 0xc2, 0x00,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm9_xmm12_offset_1() {
    let mut emu = emu64();
    // MPSADBW XMM9, XMM12, 1
    let code = [0x66, 0x45, 0x0f, 0x3a, 0x42, 0xcc, 0x01,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm11_xmm14_offset_2() {
    let mut emu = emu64();
    // MPSADBW XMM11, XMM14, 2
    let code = [0x66, 0x45, 0x0f, 0x3a, 0x42, 0xde, 0x02,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm13_xmm15_offset_3() {
    let mut emu = emu64();
    // MPSADBW XMM13, XMM15, 3
    let code = [0x66, 0x45, 0x0f, 0x3a, 0x42, 0xef, 0x03,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm15_xmm8_offset_4() {
    let mut emu = emu64();
    // MPSADBW XMM15, XMM8, 4
    let code = [0x66, 0x45, 0x0f, 0x3a, 0x42, 0xf8, 0x04,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm0_xmm8_offset_5() {
    let mut emu = emu64();
    // MPSADBW XMM0, XMM8, 5
    let code = [0x66, 0x41, 0x0f, 0x3a, 0x42, 0xc0, 0x05,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm1_xmm9_offset_6() {
    let mut emu = emu64();
    // MPSADBW XMM1, XMM9, 6
    let code = [0x66, 0x41, 0x0f, 0x3a, 0x42, 0xc9, 0x06,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm2_xmm10_offset_7() {
    let mut emu = emu64();
    // MPSADBW XMM2, XMM10, 7
    let code = [0x66, 0x41, 0x0f, 0x3a, 0x42, 0xd2, 0x07,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm3_xmm11_offset_0() {
    let mut emu = emu64();
    // MPSADBW XMM3, XMM11, 0
    let code = [0x66, 0x41, 0x0f, 0x3a, 0x42, 0xdb, 0x00,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm4_xmm12_offset_1() {
    let mut emu = emu64();
    // MPSADBW XMM4, XMM12, 1
    let code = [0x66, 0x41, 0x0f, 0x3a, 0x42, 0xe4, 0x01,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm5_xmm13_offset_2() {
    let mut emu = emu64();
    // MPSADBW XMM5, XMM13, 2
    let code = [0x66, 0x41, 0x0f, 0x3a, 0x42, 0xed, 0x02,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm6_xmm14_offset_3() {
    let mut emu = emu64();
    // MPSADBW XMM6, XMM14, 3
    let code = [0x66, 0x41, 0x0f, 0x3a, 0x42, 0xf6, 0x03,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm7_xmm15_offset_4() {
    let mut emu = emu64();
    // MPSADBW XMM7, XMM15, 4
    let code = [0x66, 0x41, 0x0f, 0x3a, 0x42, 0xff, 0x04,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm8_xmm0_offset_5() {
    let mut emu = emu64();
    // MPSADBW XMM8, XMM0, 5
    let code = [0x66, 0x44, 0x0f, 0x3a, 0x42, 0xc0, 0x05,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm9_xmm1_offset_6() {
    let mut emu = emu64();
    // MPSADBW XMM9, XMM1, 6
    let code = [0x66, 0x44, 0x0f, 0x3a, 0x42, 0xc9, 0x06,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm10_xmm2_offset_7() {
    let mut emu = emu64();
    // MPSADBW XMM10, XMM2, 7
    let code = [0x66, 0x44, 0x0f, 0x3a, 0x42, 0xd2, 0x07,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm11_xmm3_offset_0() {
    let mut emu = emu64();
    // MPSADBW XMM11, XMM3, 0
    let code = [0x66, 0x44, 0x0f, 0x3a, 0x42, 0xdb, 0x00,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm12_xmm4_offset_1() {
    let mut emu = emu64();
    // MPSADBW XMM12, XMM4, 1
    let code = [0x66, 0x44, 0x0f, 0x3a, 0x42, 0xe4, 0x01,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm13_xmm5_offset_2() {
    let mut emu = emu64();
    // MPSADBW XMM13, XMM5, 2
    let code = [0x66, 0x44, 0x0f, 0x3a, 0x42, 0xed, 0x02,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm14_xmm6_offset_3() {
    let mut emu = emu64();
    // MPSADBW XMM14, XMM6, 3
    let code = [0x66, 0x44, 0x0f, 0x3a, 0x42, 0xf6, 0x03,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_xmm15_xmm7_offset_4() {
    let mut emu = emu64();
    // MPSADBW XMM15, XMM7, 4
    let code = [0x66, 0x44, 0x0f, 0x3a, 0x42, 0xff, 0x04,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_sequential_offsets() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x42, 0xc1, 0x00, // MPSADBW XMM0, XMM1, 0
        0x66, 0x0f, 0x3a, 0x42, 0xd3, 0x01, // MPSADBW XMM2, XMM3, 1
        0x66, 0x0f, 0x3a, 0x42, 0xe5, 0x02, // MPSADBW XMM4, XMM5, 2
        0x66, 0x0f, 0x3a, 0x42, 0xf7, 0x03, // MPSADBW XMM6, XMM7, 3
       
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_all_high_regs_pattern() {
    let mut emu = emu64();
    let code = [0x66, 0x45, 0x0f, 0x3a, 0x42, 0xc1, 0x00, // MPSADBW XMM8, XMM9, 0
        0x66, 0x45, 0x0f, 0x3a, 0x42, 0xd3, 0x02, // MPSADBW XMM10, XMM11, 2
        0x66, 0x45, 0x0f, 0x3a, 0x42, 0xe5, 0x04, // MPSADBW XMM12, XMM13, 4
        0x66, 0x45, 0x0f, 0x3a, 0x42, 0xf7, 0x06, // MPSADBW XMM14, XMM15, 6
       
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_cross_boundary_mix() {
    let mut emu = emu64();
    let code = [0x66, 0x44, 0x0f, 0x3a, 0x42, 0xc7, 0x01, // MPSADBW XMM8, XMM7, 1
        0x66, 0x41, 0x0f, 0x3a, 0x42, 0xf8, 0x03, // MPSADBW XMM7, XMM8, 3
        0x66, 0x44, 0x0f, 0x3a, 0x42, 0xce, 0x05, // MPSADBW XMM9, XMM6, 5
       
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mpsadbw_alternating_pattern() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x3a, 0x42, 0xc1, 0x00, // MPSADBW XMM0, XMM1, 0
        0x66, 0x45, 0x0f, 0x3a, 0x42, 0xc1, 0x04, // MPSADBW XMM8, XMM9, 4
        0x66, 0x0f, 0x3a, 0x42, 0xd3, 0x01, // MPSADBW XMM2, XMM3, 1
        0x66, 0x45, 0x0f, 0x3a, 0x42, 0xd3, 0x05, // MPSADBW XMM10, XMM11, 5
       
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

