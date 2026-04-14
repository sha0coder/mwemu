use crate::*;

// KMOV - Move to/from Mask Registers
//
// AVX-512 opmask register move instructions.
// k0-k7 are 8 opmask registers used for predication and masking.
// Different variants for different sizes:
// - KMOVB: 8-bit  (k1-k7)
// - KMOVW: 16-bit (k0-k7)
// - KMOVD: 32-bit (k0-k7)
// - KMOVQ: 64-bit (k0-k7)
//
// Opcodes:
// VEX.L0.0F.W0 90 /r       KMOVB k1, k2/m8        - Move byte from k2/m8 to k1
// VEX.L0.0F.W0 91 /r       KMOVB m8, k1           - Move byte from k1 to m8
// VEX.L0.0F.W0 92 /r       KMOVB k1, r32          - Move byte from r32 to k1
// VEX.L0.0F.W0 93 /r       KMOVB r32, k1          - Move byte from k1 to r32
// (Similar patterns for KMOVW, KMOVD, KMOVQ)

const DATA_ADDR: u64 = 0x2000;

// ============================================================================
// KMOVB Tests - 8-bit mask register moves
// ============================================================================

#[test]
fn test_kmovb_k1_k2() {
    let mut emu = emu64();
    // KMOVB K1, K2
    let code = [
        0xc5, 0xf9, 0x90, 0xca, // KMOVB K1, K2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kmovb_k2_k3() {
    let mut emu = emu64();
    // KMOVB K2, K3
    let code = [
        0xc5, 0xf9, 0x90, 0xd3, // KMOVB K2, K3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kmovb_k3_k4() {
    let mut emu = emu64();
    // KMOVB K3, K4
    let code = [
        0xc5, 0xf9, 0x90, 0xdc, // KMOVB K3, K4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kmovb_k4_k5() {
    let mut emu = emu64();
    // KMOVB K4, K5
    let code = [
        0xc5, 0xf9, 0x90, 0xe5, // KMOVB K4, K5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kmovb_k5_k6() {
    let mut emu = emu64();
    // KMOVB K5, K6
    let code = [
        0xc5, 0xf9, 0x90, 0xee, // KMOVB K5, K6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kmovb_k6_k7() {
    let mut emu = emu64();
    // KMOVB K6, K7
    let code = [
        0xc5, 0xf9, 0x90, 0xf7, // KMOVB K6, K7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kmovb_k7_k1() {
    let mut emu = emu64();
    // KMOVB K7, K1
    let code = [
        0xc5, 0xf9, 0x90, 0xf9, // KMOVB K7, K1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// KMOVB to/from GPR tests
#[test]
fn test_kmovb_k1_eax() {
    let mut emu = emu64();
    // KMOVB K1, EAX
    let code = [
        0xc5, 0xf9, 0x92, 0xc8, // KMOVB K1, EAX
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kmovb_eax_k1() {
    let mut emu = emu64();
    // KMOVB EAX, K1
    let code = [
        0xc5, 0xf9, 0x93, 0xc1, // KMOVB EAX, K1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kmovb_k2_ebx() {
    let mut emu = emu64();
    // KMOVB K2, EBX
    let code = [
        0xc5, 0xf9, 0x92, 0xd3, // KMOVB K2, EBX
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kmovb_ecx_k3() {
    let mut emu = emu64();
    // KMOVB ECX, K3
    let code = [
        0xc5, 0xf9, 0x93, 0xcb, // KMOVB ECX, K3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// KMOVW Tests - 16-bit mask register moves
// ============================================================================

#[test]
fn test_kmovw_k0_k1() {
    let mut emu = emu64();
    // KMOVW K0, K1 (k0 can be used with KMOVW/D/Q)
    let code = [
        0xc5, 0xf8, 0x90, 0xc1, // KMOVW K0, K1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kmovw_k1_k2() {
    let mut emu = emu64();
    // KMOVW K1, K2
    let code = [
        0xc5, 0xf8, 0x90, 0xca, // KMOVW K1, K2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kmovw_k2_k3() {
    let mut emu = emu64();
    // KMOVW K2, K3
    let code = [
        0xc5, 0xf8, 0x90, 0xd3, // KMOVW K2, K3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kmovw_k3_k4() {
    let mut emu = emu64();
    // KMOVW K3, K4
    let code = [
        0xc5, 0xf8, 0x90, 0xdc, // KMOVW K3, K4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kmovw_k4_k5() {
    let mut emu = emu64();
    // KMOVW K4, K5
    let code = [
        0xc5, 0xf8, 0x90, 0xe5, // KMOVW K4, K5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kmovw_k5_k6() {
    let mut emu = emu64();
    // KMOVW K5, K6
    let code = [
        0xc5, 0xf8, 0x90, 0xee, // KMOVW K5, K6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kmovw_k6_k7() {
    let mut emu = emu64();
    // KMOVW K6, K7
    let code = [
        0xc5, 0xf8, 0x90, 0xf7, // KMOVW K6, K7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kmovw_k7_k0() {
    let mut emu = emu64();
    // KMOVW K7, K0
    let code = [
        0xc5, 0xf8, 0x90, 0xf8, // KMOVW K7, K0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// KMOVW to/from GPR tests
#[test]
fn test_kmovw_k1_eax() {
    let mut emu = emu64();
    // KMOVW K1, EAX
    let code = [
        0xc5, 0xf8, 0x92, 0xc8, // KMOVW K1, EAX
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kmovw_eax_k1() {
    let mut emu = emu64();
    // KMOVW EAX, K1
    let code = [
        0xc5, 0xf8, 0x93, 0xc1, // KMOVW EAX, K1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kmovw_k0_ebx() {
    let mut emu = emu64();
    // KMOVW K0, EBX
    let code = [
        0xc5, 0xf8, 0x92, 0xc3, // KMOVW K0, EBX
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kmovw_edx_k7() {
    let mut emu = emu64();
    // KMOVW EDX, K7
    let code = [
        0xc5, 0xf8, 0x93, 0xd7, // KMOVW EDX, K7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// KMOVD Tests - 32-bit mask register moves
// ============================================================================

#[test]
fn test_kmovd_k0_k1() {
    let mut emu = emu64();
    // KMOVD K0, K1
    let code = [
        0xc5, 0xfb, 0x90, 0xc1, // KMOVD K0, K1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kmovd_k1_k2() {
    let mut emu = emu64();
    // KMOVD K1, K2
    let code = [
        0xc5, 0xfb, 0x90, 0xca, // KMOVD K1, K2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kmovd_k2_k3() {
    let mut emu = emu64();
    // KMOVD K2, K3
    let code = [
        0xc5, 0xfb, 0x90, 0xd3, // KMOVD K2, K3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kmovd_k3_k4() {
    let mut emu = emu64();
    // KMOVD K3, K4
    let code = [
        0xc5, 0xfb, 0x90, 0xdc, // KMOVD K3, K4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kmovd_k4_k5() {
    let mut emu = emu64();
    // KMOVD K4, K5
    let code = [
        0xc5, 0xfb, 0x90, 0xe5, // KMOVD K4, K5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kmovd_k5_k6() {
    let mut emu = emu64();
    // KMOVD K5, K6
    let code = [
        0xc5, 0xfb, 0x90, 0xee, // KMOVD K5, K6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kmovd_k6_k7() {
    let mut emu = emu64();
    // KMOVD K6, K7
    let code = [
        0xc5, 0xfb, 0x90, 0xf7, // KMOVD K6, K7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kmovd_k7_k0() {
    let mut emu = emu64();
    // KMOVD K7, K0
    let code = [
        0xc5, 0xfb, 0x90, 0xf8, // KMOVD K7, K0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// KMOVD to/from GPR tests
#[test]
fn test_kmovd_k1_eax() {
    let mut emu = emu64();
    // KMOVD K1, EAX
    let code = [
        0xc5, 0xfb, 0x92, 0xc8, // KMOVD K1, EAX
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kmovd_eax_k1() {
    let mut emu = emu64();
    // KMOVD EAX, K1
    let code = [
        0xc5, 0xfb, 0x93, 0xc1, // KMOVD EAX, K1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kmovd_k0_esi() {
    let mut emu = emu64();
    // KMOVD K0, ESI
    let code = [
        0xc5, 0xfb, 0x92, 0xc6, // KMOVD K0, ESI
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kmovd_edi_k7() {
    let mut emu = emu64();
    // KMOVD EDI, K7
    let code = [
        0xc5, 0xfb, 0x93, 0xff, // KMOVD EDI, K7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// KMOVQ Tests - 64-bit mask register moves
// ============================================================================

#[test]
fn test_kmovq_k0_k1() {
    let mut emu = emu64();
    // KMOVQ K0, K1
    let code = [
        0xc4, 0xe1, 0xfb, 0x90, 0xc1, // KMOVQ K0, K1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kmovq_k1_k2() {
    let mut emu = emu64();
    // KMOVQ K1, K2
    let code = [
        0xc4, 0xe1, 0xfb, 0x90, 0xca, // KMOVQ K1, K2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kmovq_k2_k3() {
    let mut emu = emu64();
    // KMOVQ K2, K3
    let code = [
        0xc4, 0xe1, 0xfb, 0x90, 0xd3, // KMOVQ K2, K3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kmovq_k3_k4() {
    let mut emu = emu64();
    // KMOVQ K3, K4
    let code = [
        0xc4, 0xe1, 0xfb, 0x90, 0xdc, // KMOVQ K3, K4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kmovq_k4_k5() {
    let mut emu = emu64();
    // KMOVQ K4, K5
    let code = [
        0xc4, 0xe1, 0xfb, 0x90, 0xe5, // KMOVQ K4, K5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kmovq_k5_k6() {
    let mut emu = emu64();
    // KMOVQ K5, K6
    let code = [
        0xc4, 0xe1, 0xfb, 0x90, 0xee, // KMOVQ K5, K6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kmovq_k6_k7() {
    let mut emu = emu64();
    // KMOVQ K6, K7
    let code = [
        0xc4, 0xe1, 0xfb, 0x90, 0xf7, // KMOVQ K6, K7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kmovq_k7_k0() {
    let mut emu = emu64();
    // KMOVQ K7, K0
    let code = [
        0xc4, 0xe1, 0xfb, 0x90, 0xf8, // KMOVQ K7, K0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// KMOVQ to/from GPR tests (requires 64-bit GPRs)
#[test]
fn test_kmovq_k1_rax() {
    let mut emu = emu64();
    // KMOVQ K1, RAX
    let code = [
        0xc4, 0xe1, 0xfb, 0x92, 0xc8, // KMOVQ K1, RAX
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kmovq_rax_k1() {
    let mut emu = emu64();
    // KMOVQ RAX, K1
    let code = [
        0xc4, 0xe1, 0xfb, 0x93, 0xc1, // KMOVQ RAX, K1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kmovq_k0_rbx() {
    let mut emu = emu64();
    // KMOVQ K0, RBX
    let code = [
        0xc4, 0xe1, 0xfb, 0x92, 0xc3, // KMOVQ K0, RBX
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kmovq_rdx_k7() {
    let mut emu = emu64();
    // KMOVQ RDX, K7
    let code = [
        0xc4, 0xe1, 0xfb, 0x93, 0xd7, // KMOVQ RDX, K7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Chain Move Tests
// ============================================================================

#[test]
fn test_kmovw_chain() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf8, 0x90, 0xd1, // KMOVW K2, K1
        0xc5, 0xf8, 0x90, 0xda, // KMOVW K3, K2
        0xc5, 0xf8, 0x90, 0xe3, // KMOVW K4, K3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kmovd_chain_all() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xfb, 0x90, 0xc8, // KMOVD K1, K0
        0xc5, 0xfb, 0x90, 0xd1, // KMOVD K2, K1
        0xc5, 0xfb, 0x90, 0xda, // KMOVD K3, K2
        0xc5, 0xfb, 0x90, 0xe3, // KMOVD K4, K3
        0xc5, 0xfb, 0x90, 0xec, // KMOVD K5, K4
        0xc5, 0xfb, 0x90, 0xf5, // KMOVD K6, K5
        0xc5, 0xfb, 0x90, 0xfe, // KMOVD K7, K6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
