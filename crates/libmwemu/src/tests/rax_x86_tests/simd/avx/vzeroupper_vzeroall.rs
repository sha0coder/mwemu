use crate::*;

// VZEROUPPER - Zero Upper Bits of YMM Registers
// VZEROALL - Zero All YMM Registers
//
// VZEROUPPER zeros the upper 128 bits (bits 255:128) of all YMM registers.
// The lower 128 bits of the YMM registers (the corresponding XMM registers) are unmodified.
//
// VZEROALL zeros all YMM registers (YMM0-YMM15).
//
// Opcodes:
// VEX.128.0F.WIG 77       VZEROUPPER  - Zero upper 128 bits of YMM0-YMM15
// VEX.256.0F.WIG 77       VZEROALL    - Zero all YMM registers

const ALIGNED_ADDR: u64 = 0x3000; // 32-byte aligned address for testing

// ============================================================================
// VZEROUPPER Tests
// ============================================================================

#[test]
fn test_vzeroupper_basic() {
    let mut emu = emu64();
    // VZEROUPPER
    let code = [
        0xc5, 0xf8, 0x77, // VZEROUPPER
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vzeroupper_after_ymm_operation() {
    let mut emu = emu64();
    // VADDPS YMM0, YMM1, YMM2 followed by VZEROUPPER
    let code = [
        0xc5, 0xf4, 0x58, 0xc2, // VADDPS YMM0, YMM1, YMM2
        0xc5, 0xf8, 0x77, // VZEROUPPER
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vzeroupper_multiple_ymm_ops() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf4, 0x58, 0xc2, // VADDPS YMM0, YMM1, YMM2
        0xc5, 0xec, 0x58, 0xcb, // VADDPS YMM1, YMM2, YMM3
        0xc5, 0xe4, 0x58, 0xd4, // VADDPS YMM2, YMM3, YMM4
        0xc5, 0xf8, 0x77, // VZEROUPPER
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vzeroupper_after_extended_ymm() {
    let mut emu = emu64();
    // YMM8-YMM15 operations followed by VZEROUPPER
    let code = [
        0xc4, 0x41, 0x34, 0x58, 0xc2, // VADDPS YMM8, YMM9, YMM10
        0xc4, 0x41, 0x2c, 0x58, 0xcb, // VADDPS YMM9, YMM10, YMM11
        0xc5, 0xf8, 0x77, // VZEROUPPER
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vzeroupper_before_xmm_operation() {
    let mut emu = emu64();
    // VZEROUPPER followed by XMM operation
    let code = [
        0xc5, 0xf8, 0x77, // VZEROUPPER
        0xc5, 0xf0, 0x58, 0xc2, // VADDPS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vzeroupper_repeated() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf8, 0x77, // VZEROUPPER
        0xc5, 0xf8, 0x77, // VZEROUPPER
        0xc5, 0xf8, 0x77, // VZEROUPPER
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vzeroupper_interleaved_ops() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf4, 0x58, 0xc2, // VADDPS YMM0, YMM1, YMM2
        0xc5, 0xf8, 0x77, // VZEROUPPER
        0xc5, 0xec, 0x58, 0xcb, // VADDPS YMM1, YMM2, YMM3
        0xc5, 0xf8, 0x77, // VZEROUPPER
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vzeroupper_after_blend() {
    let mut emu = emu64();
    // VBLENDPS followed by VZEROUPPER
    let code = [
        0xc4, 0xe3, 0x75, 0x0c, 0xc2, 0xaa, // VBLENDPS YMM0, YMM1, YMM2, 0xAA
        0xc5, 0xf8, 0x77, // VZEROUPPER
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vzeroupper_after_perm() {
    let mut emu = emu64();
    // VPERM2F128 followed by VZEROUPPER
    let code = [
        0xc4, 0xe3, 0x75, 0x06, 0xc2, 0x20, // VPERM2F128 YMM0, YMM1, YMM2, 0x20
        0xc5, 0xf8, 0x77, // VZEROUPPER
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vzeroupper_after_broadcast() {
    let mut emu = emu64();
    // VBROADCASTSS followed by VZEROUPPER
    let code = [
        0xc4, 0xe2, 0x7d, 0x18, 0x05, 0x00, 0x40, 0x00, 0x00, // VBROADCASTSS YMM0, [rip + 0x4000]
        0xc5, 0xf8, 0x77, // VZEROUPPER
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 4] = [0x00, 0x00, 0x80, 0x3f]; // 1.0
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vzeroupper_after_insert() {
    let mut emu = emu64();
    // VINSERTF128 followed by VZEROUPPER
    let code = [
        0xc4, 0xe3, 0x7d, 0x18, 0xc1, 0x01, // VINSERTF128 YMM0, YMM0, XMM1, 1
        0xc5, 0xf8, 0x77, // VZEROUPPER
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vzeroupper_after_extract() {
    let mut emu = emu64();
    // VEXTRACTF128 followed by VZEROUPPER
    let code = [
        0xc4, 0xe3, 0x7d, 0x19, 0xc1, 0x01, // VEXTRACTF128 XMM1, YMM0, 1
        0xc5, 0xf8, 0x77, // VZEROUPPER
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vzeroupper_after_logic_ops() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf4, 0x54, 0xc2, // VANDPS YMM0, YMM1, YMM2
        0xc5, 0xec, 0x56, 0xcb, // VORPS YMM1, YMM2, YMM3
        0xc5, 0xe4, 0x57, 0xd4, // VXORPS YMM2, YMM3, YMM4
        0xc5, 0xf8, 0x77, // VZEROUPPER
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vzeroupper_after_arithmetic() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf4, 0x58, 0xc2, // VADDPS YMM0, YMM1, YMM2
        0xc5, 0xec, 0x5c, 0xcb, // VSUBPS YMM1, YMM2, YMM3
        0xc5, 0xe4, 0x59, 0xd4, // VMULPS YMM2, YMM3, YMM4
        0xc5, 0xdc, 0x5e, 0xdd, // VDIVPS YMM3, YMM4, YMM5
        0xc5, 0xf8, 0x77, // VZEROUPPER
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vzeroupper_after_comparison() {
    let mut emu = emu64();
    // VCMPPS followed by VZEROUPPER
    let code = [
        0xc5, 0xf4, 0xc2, 0xc2, 0x00, // VCMPPS YMM0, YMM1, YMM2, 0 (EQ)
        0xc5, 0xf8, 0x77, // VZEROUPPER
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vzeroupper_after_shuffle() {
    let mut emu = emu64();
    // VSHUFPS followed by VZEROUPPER
    let code = [
        0xc5, 0xf4, 0xc6, 0xc2, 0xe4, // VSHUFPS YMM0, YMM1, YMM2, 0xE4
        0xc5, 0xf8, 0x77, // VZEROUPPER
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vzeroupper_after_unpack() {
    let mut emu = emu64();
    // VUNPCKLPS followed by VZEROUPPER
    let code = [
        0xc5, 0xf4, 0x14, 0xc2, // VUNPCKLPS YMM0, YMM1, YMM2
        0xc5, 0xf8, 0x77, // VZEROUPPER
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vzeroupper_after_hadd() {
    let mut emu = emu64();
    // VHADDPS followed by VZEROUPPER
    let code = [
        0xc5, 0xf7, 0x7c, 0xc2, // VHADDPS YMM0, YMM1, YMM2
        0xc5, 0xf8, 0x77, // VZEROUPPER
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vzeroupper_after_hsub() {
    let mut emu = emu64();
    // VHSUBPS followed by VZEROUPPER
    let code = [
        0xc5, 0xf7, 0x7d, 0xc2, // VHSUBPS YMM0, YMM1, YMM2
        0xc5, 0xf8, 0x77, // VZEROUPPER
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vzeroupper_after_sqrt() {
    let mut emu = emu64();
    // VSQRTPS followed by VZEROUPPER
    let code = [
        0xc5, 0xfc, 0x51, 0xc1, // VSQRTPS YMM0, YMM1
        0xc5, 0xf8, 0x77, // VZEROUPPER
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vzeroupper_after_minmax() {
    let mut emu = emu64();
    // VMINPS/VMAXPS followed by VZEROUPPER
    let code = [
        0xc5, 0xf4, 0x5d, 0xc2, // VMINPS YMM0, YMM1, YMM2
        0xc5, 0xec, 0x5f, 0xcb, // VMAXPS YMM1, YMM2, YMM3
        0xc5, 0xf8, 0x77, // VZEROUPPER
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VZEROALL Tests
// ============================================================================

#[test]
fn test_vzeroall_basic() {
    let mut emu = emu64();
    // VZEROALL
    let code = [
        0xc5, 0xfc, 0x77, // VZEROALL
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vzeroall_after_ymm_operation() {
    let mut emu = emu64();
    // VADDPS YMM0, YMM1, YMM2 followed by VZEROALL
    let code = [
        0xc5, 0xf4, 0x58, 0xc2, // VADDPS YMM0, YMM1, YMM2
        0xc5, 0xfc, 0x77, // VZEROALL
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vzeroall_multiple_ymm_ops() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf4, 0x58, 0xc2, // VADDPS YMM0, YMM1, YMM2
        0xc5, 0xec, 0x58, 0xcb, // VADDPS YMM1, YMM2, YMM3
        0xc5, 0xe4, 0x58, 0xd4, // VADDPS YMM2, YMM3, YMM4
        0xc5, 0xfc, 0x77, // VZEROALL
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vzeroall_after_extended_ymm() {
    let mut emu = emu64();
    // YMM8-YMM15 operations followed by VZEROALL
    let code = [
        0xc4, 0x41, 0x34, 0x58, 0xc2, // VADDPS YMM8, YMM9, YMM10
        0xc4, 0x41, 0x2c, 0x58, 0xcb, // VADDPS YMM9, YMM10, YMM11
        0xc4, 0x41, 0x24, 0x58, 0xd4, // VADDPS YMM10, YMM11, YMM12
        0xc5, 0xfc, 0x77, // VZEROALL
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vzeroall_before_xmm_operation() {
    let mut emu = emu64();
    // VZEROALL followed by XMM operation
    let code = [
        0xc5, 0xfc, 0x77, // VZEROALL
        0xc5, 0xf0, 0x58, 0xc2, // VADDPS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vzeroall_repeated() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xfc, 0x77, // VZEROALL
        0xc5, 0xfc, 0x77, // VZEROALL
        0xc5, 0xfc, 0x77, // VZEROALL
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vzeroall_interleaved_ops() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf4, 0x58, 0xc2, // VADDPS YMM0, YMM1, YMM2
        0xc5, 0xfc, 0x77, // VZEROALL
        0xc5, 0xec, 0x58, 0xcb, // VADDPS YMM1, YMM2, YMM3
        0xc5, 0xfc, 0x77, // VZEROALL
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vzeroall_after_blend() {
    let mut emu = emu64();
    // VBLENDPS followed by VZEROALL
    let code = [
        0xc4, 0xe3, 0x75, 0x0c, 0xc2, 0xaa, // VBLENDPS YMM0, YMM1, YMM2, 0xAA
        0xc5, 0xfc, 0x77, // VZEROALL
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vzeroall_after_perm() {
    let mut emu = emu64();
    // VPERM2F128 followed by VZEROALL
    let code = [
        0xc4, 0xe3, 0x75, 0x06, 0xc2, 0x20, // VPERM2F128 YMM0, YMM1, YMM2, 0x20
        0xc5, 0xfc, 0x77, // VZEROALL
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vzeroall_after_broadcast() {
    let mut emu = emu64();
    // VBROADCASTSS followed by VZEROALL
    let code = [
        0xc4, 0xe2, 0x7d, 0x18, 0x05, 0x00, 0x40, 0x00, 0x00, // VBROADCASTSS YMM0, [rip + 0x4000]
        0xc5, 0xfc, 0x77, // VZEROALL
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 4] = [0x00, 0x00, 0x80, 0x3f]; // 1.0
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vzeroall_after_logic_ops() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf4, 0x54, 0xc2, // VANDPS YMM0, YMM1, YMM2
        0xc5, 0xec, 0x56, 0xcb, // VORPS YMM1, YMM2, YMM3
        0xc5, 0xe4, 0x57, 0xd4, // VXORPS YMM2, YMM3, YMM4
        0xc5, 0xfc, 0x77, // VZEROALL
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vzeroall_after_arithmetic() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf4, 0x58, 0xc2, // VADDPS YMM0, YMM1, YMM2
        0xc5, 0xec, 0x5c, 0xcb, // VSUBPS YMM1, YMM2, YMM3
        0xc5, 0xe4, 0x59, 0xd4, // VMULPS YMM2, YMM3, YMM4
        0xc5, 0xdc, 0x5e, 0xdd, // VDIVPS YMM3, YMM4, YMM5
        0xc5, 0xfc, 0x77, // VZEROALL
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vzeroall_vs_vzeroupper() {
    let mut emu = emu64();
    // VZEROALL followed by VZEROUPPER
    let code = [
        0xc5, 0xfc, 0x77, // VZEROALL
        0xc5, 0xf8, 0x77, // VZEROUPPER
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vzeroupper_vs_vzeroall() {
    let mut emu = emu64();
    // VZEROUPPER followed by VZEROALL
    let code = [
        0xc5, 0xf8, 0x77, // VZEROUPPER
        0xc5, 0xfc, 0x77, // VZEROALL
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vzeroall_all_regs_used() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf4, 0x58, 0xc2, // VADDPS YMM0, YMM1, YMM2
        0xc5, 0xec, 0x58, 0xcb, // VADDPS YMM1, YMM2, YMM3
        0xc5, 0xe4, 0x58, 0xd4, // VADDPS YMM2, YMM3, YMM4
        0xc5, 0xdc, 0x58, 0xdd, // VADDPS YMM3, YMM4, YMM5
        0xc5, 0xd4, 0x58, 0xe6, // VADDPS YMM4, YMM5, YMM6
        0xc5, 0xcc, 0x58, 0xef, // VADDPS YMM5, YMM6, YMM7
        0xc4, 0x41, 0x3c, 0x58, 0xc1, // VADDPS YMM8, YMM8, YMM9
        0xc4, 0x41, 0x34, 0x58, 0xca, // VADDPS YMM9, YMM9, YMM10
        0xc5, 0xfc, 0x77, // VZEROALL
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vzeroupper_after_mixed_pd_ps() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf4, 0x58, 0xc2, // VADDPS YMM0, YMM1, YMM2
        0xc5, 0xed, 0x58, 0xcb, // VADDPD YMM1, YMM2, YMM3
        0xc5, 0xe4, 0x58, 0xd4, // VADDPS YMM2, YMM3, YMM4
        0xc5, 0xf8, 0x77, // VZEROUPPER
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vzeroall_after_mixed_pd_ps() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf4, 0x58, 0xc2, // VADDPS YMM0, YMM1, YMM2
        0xc5, 0xed, 0x58, 0xcb, // VADDPD YMM1, YMM2, YMM3
        0xc5, 0xe4, 0x58, 0xd4, // VADDPS YMM2, YMM3, YMM4
        0xc5, 0xfc, 0x77, // VZEROALL
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vzeroupper_context_switch_simulation() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf4, 0x58, 0xc2, // VADDPS YMM0, YMM1, YMM2
        0xc5, 0xec, 0x58, 0xcb, // VADDPS YMM1, YMM2, YMM3
        0xc5, 0xf8, 0x77, // VZEROUPPER (before context switch)
        0xc5, 0xf0, 0x58, 0xc2, // VADDPS XMM0, XMM1, XMM2 (in new context)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vzeroall_context_switch_simulation() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf4, 0x58, 0xc2, // VADDPS YMM0, YMM1, YMM2
        0xc5, 0xec, 0x58, 0xcb, // VADDPS YMM1, YMM2, YMM3
        0xc5, 0xfc, 0x77, // VZEROALL (before context switch)
        0xc5, 0xf0, 0x58, 0xc2, // VADDPS XMM0, XMM1, XMM2 (in new context)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
