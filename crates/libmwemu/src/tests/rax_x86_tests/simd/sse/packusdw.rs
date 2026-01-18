use crate::*;

// PACKUSDW - Pack with Unsigned Saturation (Dwords to Words)
//
// Converts 4 signed dword integers (32-bit) from dest and 4 from src
// into 8 unsigned word integers (16-bit) with unsigned saturation.
// Range: 0 to 65535 (0xFFFF)
// Negative values -> 0, values > 65535 -> 65535
//
// Opcode:
// 66 0F 38 2B /r    PACKUSDW xmm1, xmm2/m128

const ALIGNED_ADDR: u64 = 0x3000;

#[test]
fn test_packusdw_xmm0_xmm1_basic() {
    let mut emu = emu64();
    // PACKUSDW XMM0, XMM1
    let code = [0x66, 0x0f, 0x38, 0x2b, 0xc1,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_xmm1_xmm2_basic() {
    let mut emu = emu64();
    // PACKUSDW XMM1, XMM2
    let code = [0x66, 0x0f, 0x38, 0x2b, 0xca,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_xmm2_xmm3_basic() {
    let mut emu = emu64();
    // PACKUSDW XMM2, XMM3
    let code = [0x66, 0x0f, 0x38, 0x2b, 0xd3,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_xmm3_xmm4_basic() {
    let mut emu = emu64();
    // PACKUSDW XMM3, XMM4
    let code = [0x66, 0x0f, 0x38, 0x2b, 0xdc,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_xmm4_xmm5_basic() {
    let mut emu = emu64();
    // PACKUSDW XMM4, XMM5
    let code = [0x66, 0x0f, 0x38, 0x2b, 0xe5,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_xmm5_xmm6_basic() {
    let mut emu = emu64();
    // PACKUSDW XMM5, XMM6
    let code = [0x66, 0x0f, 0x38, 0x2b, 0xee,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_xmm6_xmm7_basic() {
    let mut emu = emu64();
    // PACKUSDW XMM6, XMM7
    let code = [0x66, 0x0f, 0x38, 0x2b, 0xf7,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_xmm7_xmm0_basic() {
    let mut emu = emu64();
    // PACKUSDW XMM7, XMM0
    let code = [0x66, 0x0f, 0x38, 0x2b, 0xf8,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_xmm8_xmm9() {
    let mut emu = emu64();
    // PACKUSDW XMM8, XMM9
    let code = [0x66, 0x45, 0x0f, 0x38, 0x2b, 0xc1,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_xmm9_xmm10() {
    let mut emu = emu64();
    // PACKUSDW XMM9, XMM10
    let code = [0x66, 0x45, 0x0f, 0x38, 0x2b, 0xca,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_xmm10_xmm11() {
    let mut emu = emu64();
    // PACKUSDW XMM10, XMM11
    let code = [0x66, 0x45, 0x0f, 0x38, 0x2b, 0xd3,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_xmm11_xmm12() {
    let mut emu = emu64();
    // PACKUSDW XMM11, XMM12
    let code = [0x66, 0x45, 0x0f, 0x38, 0x2b, 0xdc,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_xmm12_xmm13() {
    let mut emu = emu64();
    // PACKUSDW XMM12, XMM13
    let code = [0x66, 0x45, 0x0f, 0x38, 0x2b, 0xe5,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_xmm13_xmm14() {
    let mut emu = emu64();
    // PACKUSDW XMM13, XMM14
    let code = [0x66, 0x45, 0x0f, 0x38, 0x2b, 0xee,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_xmm14_xmm15() {
    let mut emu = emu64();
    // PACKUSDW XMM14, XMM15
    let code = [0x66, 0x45, 0x0f, 0x38, 0x2b, 0xf7,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_xmm15_xmm8() {
    let mut emu = emu64();
    // PACKUSDW XMM15, XMM8
    let code = [0x66, 0x45, 0x0f, 0x38, 0x2b, 0xf8,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_xmm0_xmm8() {
    let mut emu = emu64();
    // PACKUSDW XMM0, XMM8
    let code = [0x66, 0x41, 0x0f, 0x38, 0x2b, 0xc0,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_xmm1_xmm9() {
    let mut emu = emu64();
    // PACKUSDW XMM1, XMM9
    let code = [0x66, 0x41, 0x0f, 0x38, 0x2b, 0xc9,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_xmm2_xmm10() {
    let mut emu = emu64();
    // PACKUSDW XMM2, XMM10
    let code = [0x66, 0x41, 0x0f, 0x38, 0x2b, 0xd2,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_xmm3_xmm11() {
    let mut emu = emu64();
    // PACKUSDW XMM3, XMM11
    let code = [0x66, 0x41, 0x0f, 0x38, 0x2b, 0xdb,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_xmm4_xmm12() {
    let mut emu = emu64();
    // PACKUSDW XMM4, XMM12
    let code = [0x66, 0x41, 0x0f, 0x38, 0x2b, 0xe4,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_xmm5_xmm13() {
    let mut emu = emu64();
    // PACKUSDW XMM5, XMM13
    let code = [0x66, 0x41, 0x0f, 0x38, 0x2b, 0xed,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_xmm6_xmm14() {
    let mut emu = emu64();
    // PACKUSDW XMM6, XMM14
    let code = [0x66, 0x41, 0x0f, 0x38, 0x2b, 0xf6,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_xmm7_xmm15() {
    let mut emu = emu64();
    // PACKUSDW XMM7, XMM15
    let code = [0x66, 0x41, 0x0f, 0x38, 0x2b, 0xff,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_xmm8_xmm0() {
    let mut emu = emu64();
    // PACKUSDW XMM8, XMM0
    let code = [0x66, 0x44, 0x0f, 0x38, 0x2b, 0xc0,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_xmm9_xmm1() {
    let mut emu = emu64();
    // PACKUSDW XMM9, XMM1
    let code = [0x66, 0x44, 0x0f, 0x38, 0x2b, 0xc9,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_xmm10_xmm2() {
    let mut emu = emu64();
    // PACKUSDW XMM10, XMM2
    let code = [0x66, 0x44, 0x0f, 0x38, 0x2b, 0xd2,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_xmm11_xmm3() {
    let mut emu = emu64();
    // PACKUSDW XMM11, XMM3
    let code = [0x66, 0x44, 0x0f, 0x38, 0x2b, 0xdb,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_xmm12_xmm4() {
    let mut emu = emu64();
    // PACKUSDW XMM12, XMM4
    let code = [0x66, 0x44, 0x0f, 0x38, 0x2b, 0xe4,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_xmm13_xmm5() {
    let mut emu = emu64();
    // PACKUSDW XMM13, XMM5
    let code = [0x66, 0x44, 0x0f, 0x38, 0x2b, 0xed,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_xmm14_xmm6() {
    let mut emu = emu64();
    // PACKUSDW XMM14, XMM6
    let code = [0x66, 0x44, 0x0f, 0x38, 0x2b, 0xf6,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_xmm15_xmm7() {
    let mut emu = emu64();
    // PACKUSDW XMM15, XMM7
    let code = [0x66, 0x44, 0x0f, 0x38, 0x2b, 0xff,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_xmm0_xmm0_same() {
    let mut emu = emu64();
    // PACKUSDW XMM0, XMM0
    let code = [0x66, 0x0f, 0x38, 0x2b, 0xc0,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_xmm8_xmm8_same() {
    let mut emu = emu64();
    // PACKUSDW XMM8, XMM8
    let code = [0x66, 0x45, 0x0f, 0x38, 0x2b, 0xc0,
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_sequential() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x38, 0x2b, 0xc1, // PACKUSDW XMM0, XMM1
        0x66, 0x0f, 0x38, 0x2b, 0xd3, // PACKUSDW XMM2, XMM3
        0x66, 0x0f, 0x38, 0x2b, 0xe5, // PACKUSDW XMM4, XMM5
       
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_chain() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x38, 0x2b, 0xc1, // PACKUSDW XMM0, XMM1
        0x66, 0x0f, 0x38, 0x2b, 0xd0, // PACKUSDW XMM2, XMM0
        0x66, 0x0f, 0x38, 0x2b, 0xda, // PACKUSDW XMM3, XMM2
       
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_all_high_regs() {
    let mut emu = emu64();
    let code = [0x66, 0x45, 0x0f, 0x38, 0x2b, 0xc1, // PACKUSDW XMM8, XMM9
        0x66, 0x45, 0x0f, 0x38, 0x2b, 0xd3, // PACKUSDW XMM10, XMM11
        0x66, 0x45, 0x0f, 0x38, 0x2b, 0xe5, // PACKUSDW XMM12, XMM13
       
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_cross_boundary() {
    let mut emu = emu64();
    let code = [0x66, 0x44, 0x0f, 0x38, 0x2b, 0xc7, // PACKUSDW XMM8, XMM7
        0x66, 0x41, 0x0f, 0x38, 0x2b, 0xf8, // PACKUSDW XMM7, XMM8
       
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_alternating() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x38, 0x2b, 0xc1, // PACKUSDW XMM0, XMM1
        0x66, 0x45, 0x0f, 0x38, 0x2b, 0xc1, // PACKUSDW XMM8, XMM9
        0x66, 0x0f, 0x38, 0x2b, 0xd3, // PACKUSDW XMM2, XMM3
       
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_bidirectional() {
    let mut emu = emu64();
    let code = [0x66, 0x0f, 0x38, 0x2b, 0xc1, // PACKUSDW XMM0, XMM1
        0x66, 0x0f, 0x38, 0x2b, 0xc8, // PACKUSDW XMM1, XMM0
       
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

