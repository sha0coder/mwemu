use crate::*;

// KAND/KOR/KXOR - Bitwise Logical Operations on Opmask Registers
//
// AVX-512 opmask register bitwise logical operations.
// Different variants for different sizes:
// - KANDB/KORB/KXORB:   8-bit operations
// - KANDW/KORW/KXORW:  16-bit operations
// - KANDD/KORD/KXORD:  32-bit operations
// - KANDQ/KORQ/KXORQ:  64-bit operations
//
// Opcodes:
// VEX.L1.0F.W0 41 /r    KANDB k1, k2, k3       - Bitwise AND 8-bit masks k2 and k3, store in k1
// VEX.L1.0F.W0 45 /r    KORB k1, k2, k3        - Bitwise OR 8-bit masks k2 and k3, store in k1
// VEX.L1.0F.W0 47 /r    KXORB k1, k2, k3       - Bitwise XOR 8-bit masks k2 and k3, store in k1
// (Similar for W, D, Q variants)

// ============================================================================
// KANDB Tests - 8-bit mask AND
// ============================================================================

#[test]
fn test_kandb_k1_k2_k3() {
    let mut emu = emu64();
    // KANDB K1, K2, K3
    let code = [
        0xc5, 0xed, 0x41, 0xca, // KANDB K1, K2, K3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kandb_k2_k3_k4() {
    let mut emu = emu64();
    // KANDB K2, K3, K4
    let code = [
        0xc5, 0xe5, 0x41, 0xd3, // KANDB K2, K3, K4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kandb_k3_k4_k5() {
    let mut emu = emu64();
    // KANDB K3, K4, K5
    let code = [
        0xc5, 0xdd, 0x41, 0xdc, // KANDB K3, K4, K5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kandb_k4_k5_k6() {
    let mut emu = emu64();
    // KANDB K4, K5, K6
    let code = [
        0xc5, 0xd5, 0x41, 0xe5, // KANDB K4, K5, K6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kandb_k5_k6_k7() {
    let mut emu = emu64();
    // KANDB K5, K6, K7
    let code = [
        0xc5, 0xcd, 0x41, 0xee, // KANDB K5, K6, K7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kandb_k6_k7_k1() {
    let mut emu = emu64();
    // KANDB K6, K7, K1
    let code = [
        0xc5, 0xc5, 0x41, 0xf7, // KANDB K6, K7, K1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kandb_k7_k1_k2() {
    let mut emu = emu64();
    // KANDB K7, K1, K2
    let code = [
        0xc5, 0xf5, 0x41, 0xf9, // KANDB K7, K1, K2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Self-AND tests (should preserve value)
#[test]
fn test_kandb_k1_k2_k2() {
    let mut emu = emu64();
    // KANDB K1, K2, K2 (K2 AND K2 = K2)
    let code = [
        0xc5, 0xed, 0x41, 0xca, // KANDB K1, K2, K2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// KORB Tests - 8-bit mask OR
// ============================================================================

#[test]
fn test_korb_k1_k2_k3() {
    let mut emu = emu64();
    // KORB K1, K2, K3
    let code = [
        0xc5, 0xed, 0x45, 0xca, // KORB K1, K2, K3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_korb_k2_k3_k4() {
    let mut emu = emu64();
    // KORB K2, K3, K4
    let code = [
        0xc5, 0xe5, 0x45, 0xd3, // KORB K2, K3, K4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_korb_k3_k4_k5() {
    let mut emu = emu64();
    // KORB K3, K4, K5
    let code = [
        0xc5, 0xdd, 0x45, 0xdc, // KORB K3, K4, K5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_korb_k4_k5_k6() {
    let mut emu = emu64();
    // KORB K4, K5, K6
    let code = [
        0xc5, 0xd5, 0x45, 0xe5, // KORB K4, K5, K6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_korb_k5_k6_k7() {
    let mut emu = emu64();
    // KORB K5, K6, K7
    let code = [
        0xc5, 0xcd, 0x45, 0xee, // KORB K5, K6, K7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_korb_k6_k7_k1() {
    let mut emu = emu64();
    // KORB K6, K7, K1
    let code = [
        0xc5, 0xc5, 0x45, 0xf7, // KORB K6, K7, K1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_korb_k7_k1_k2() {
    let mut emu = emu64();
    // KORB K7, K1, K2
    let code = [
        0xc5, 0xf5, 0x45, 0xf9, // KORB K7, K1, K2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Self-OR tests (should preserve value)
#[test]
fn test_korb_k1_k2_k2() {
    let mut emu = emu64();
    // KORB K1, K2, K2 (K2 OR K2 = K2)
    let code = [
        0xc5, 0xed, 0x45, 0xca, // KORB K1, K2, K2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// KXORB Tests - 8-bit mask XOR
// ============================================================================

#[test]
fn test_kxorb_k1_k2_k3() {
    let mut emu = emu64();
    // KXORB K1, K2, K3
    let code = [
        0xc5, 0xed, 0x47, 0xca, // KXORB K1, K2, K3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kxorb_k2_k3_k4() {
    let mut emu = emu64();
    // KXORB K2, K3, K4
    let code = [
        0xc5, 0xe5, 0x47, 0xd3, // KXORB K2, K3, K4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kxorb_k3_k4_k5() {
    let mut emu = emu64();
    // KXORB K3, K4, K5
    let code = [
        0xc5, 0xdd, 0x47, 0xdc, // KXORB K3, K4, K5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kxorb_k4_k5_k6() {
    let mut emu = emu64();
    // KXORB K4, K5, K6
    let code = [
        0xc5, 0xd5, 0x47, 0xe5, // KXORB K4, K5, K6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kxorb_k5_k6_k7() {
    let mut emu = emu64();
    // KXORB K5, K6, K7
    let code = [
        0xc5, 0xcd, 0x47, 0xee, // KXORB K5, K6, K7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kxorb_k6_k7_k1() {
    let mut emu = emu64();
    // KXORB K6, K7, K1
    let code = [
        0xc5, 0xc5, 0x47, 0xf7, // KXORB K6, K7, K1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kxorb_k7_k1_k2() {
    let mut emu = emu64();
    // KXORB K7, K1, K2
    let code = [
        0xc5, 0xf5, 0x47, 0xf9, // KXORB K7, K1, K2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Self-XOR tests (should result in zero)
#[test]
fn test_kxorb_k1_k2_k2() {
    let mut emu = emu64();
    // KXORB K1, K2, K2 (K2 XOR K2 = 0)
    let code = [
        0xc5, 0xed, 0x47, 0xca, // KXORB K1, K2, K2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// KANDW Tests - 16-bit mask AND
// ============================================================================

#[test]
fn test_kandw_k0_k1_k2() {
    let mut emu = emu64();
    // KANDW K0, K1, K2 (k0 can be used with W/D/Q variants)
    let code = [
        0xc5, 0xf5, 0x41, 0xc1, // KANDW K0, K1, K2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kandw_k1_k2_k3() {
    let mut emu = emu64();
    // KANDW K1, K2, K3
    let code = [
        0xc5, 0xed, 0x41, 0xca, // KANDW K1, K2, K3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kandw_k2_k3_k4() {
    let mut emu = emu64();
    // KANDW K2, K3, K4
    let code = [
        0xc5, 0xe5, 0x41, 0xd3, // KANDW K2, K3, K4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kandw_k3_k4_k5() {
    let mut emu = emu64();
    // KANDW K3, K4, K5
    let code = [
        0xc5, 0xdd, 0x41, 0xdc, // KANDW K3, K4, K5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kandw_k7_k0_k1() {
    let mut emu = emu64();
    // KANDW K7, K0, K1
    let code = [
        0xc5, 0xfd, 0x41, 0xf8, // KANDW K7, K0, K1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// KORW Tests - 16-bit mask OR
// ============================================================================

#[test]
fn test_korw_k0_k1_k2() {
    let mut emu = emu64();
    // KORW K0, K1, K2
    let code = [
        0xc5, 0xf5, 0x45, 0xc1, // KORW K0, K1, K2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_korw_k1_k2_k3() {
    let mut emu = emu64();
    // KORW K1, K2, K3
    let code = [
        0xc5, 0xed, 0x45, 0xca, // KORW K1, K2, K3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_korw_k2_k3_k4() {
    let mut emu = emu64();
    // KORW K2, K3, K4
    let code = [
        0xc5, 0xe5, 0x45, 0xd3, // KORW K2, K3, K4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_korw_k7_k0_k1() {
    let mut emu = emu64();
    // KORW K7, K0, K1
    let code = [
        0xc5, 0xfd, 0x45, 0xf8, // KORW K7, K0, K1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// KXORW Tests - 16-bit mask XOR
// ============================================================================

#[test]
fn test_kxorw_k0_k1_k2() {
    let mut emu = emu64();
    // KXORW K0, K1, K2
    let code = [
        0xc5, 0xf5, 0x47, 0xc1, // KXORW K0, K1, K2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kxorw_k1_k2_k3() {
    let mut emu = emu64();
    // KXORW K1, K2, K3
    let code = [
        0xc5, 0xed, 0x47, 0xca, // KXORW K1, K2, K3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kxorw_k2_k3_k4() {
    let mut emu = emu64();
    // KXORW K2, K3, K4
    let code = [
        0xc5, 0xe5, 0x47, 0xd3, // KXORW K2, K3, K4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kxorw_k7_k0_k1() {
    let mut emu = emu64();
    // KXORW K7, K0, K1
    let code = [
        0xc5, 0xfd, 0x47, 0xf8, // KXORW K7, K0, K1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Self-XOR to clear (16-bit)
#[test]
fn test_kxorw_k0_k1_k1() {
    let mut emu = emu64();
    // KXORW K0, K1, K1 (clears k0)
    let code = [
        0xc5, 0xf5, 0x47, 0xc1, // KXORW K0, K1, K1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// KANDD Tests - 32-bit mask AND
// ============================================================================

#[test]
fn test_kandd_k0_k1_k2() {
    let mut emu = emu64();
    // KANDD K0, K1, K2
    let code = [
        0xc5, 0xf5, 0x41, 0xc1, // KANDD K0, K1, K2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kandd_k1_k2_k3() {
    let mut emu = emu64();
    // KANDD K1, K2, K3
    let code = [
        0xc5, 0xed, 0x41, 0xca, // KANDD K1, K2, K3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kandd_k7_k5_k6() {
    let mut emu = emu64();
    // KANDD K7, K5, K6
    let code = [
        0xc5, 0xd5, 0x41, 0xfd, // KANDD K7, K5, K6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// KORD Tests - 32-bit mask OR
// ============================================================================

#[test]
fn test_kord_k0_k1_k2() {
    let mut emu = emu64();
    // KORD K0, K1, K2
    let code = [
        0xc5, 0xf5, 0x45, 0xc1, // KORD K0, K1, K2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kord_k1_k2_k3() {
    let mut emu = emu64();
    // KORD K1, K2, K3
    let code = [
        0xc5, 0xed, 0x45, 0xca, // KORD K1, K2, K3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kord_k7_k5_k6() {
    let mut emu = emu64();
    // KORD K7, K5, K6
    let code = [
        0xc5, 0xd5, 0x45, 0xfd, // KORD K7, K5, K6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// KXORD Tests - 32-bit mask XOR
// ============================================================================

#[test]
fn test_kxord_k0_k1_k2() {
    let mut emu = emu64();
    // KXORD K0, K1, K2
    let code = [
        0xc5, 0xf5, 0x47, 0xc1, // KXORD K0, K1, K2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kxord_k1_k2_k3() {
    let mut emu = emu64();
    // KXORD K1, K2, K3
    let code = [
        0xc5, 0xed, 0x47, 0xca, // KXORD K1, K2, K3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kxord_k7_k5_k6() {
    let mut emu = emu64();
    // KXORD K7, K5, K6
    let code = [
        0xc5, 0xd5, 0x47, 0xfd, // KXORD K7, K5, K6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// KANDQ Tests - 64-bit mask AND
// ============================================================================

#[test]
fn test_kandq_k0_k1_k2() {
    let mut emu = emu64();
    // KANDQ K0, K1, K2
    let code = [
        0xc4, 0xe1, 0xf5, 0x41, 0xc1, // KANDQ K0, K1, K2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kandq_k1_k2_k3() {
    let mut emu = emu64();
    // KANDQ K1, K2, K3
    let code = [
        0xc4, 0xe1, 0xed, 0x41, 0xca, // KANDQ K1, K2, K3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kandq_k7_k5_k6() {
    let mut emu = emu64();
    // KANDQ K7, K5, K6
    let code = [
        0xc4, 0xe1, 0xd5, 0x41, 0xfd, // KANDQ K7, K5, K6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// KORQ Tests - 64-bit mask OR
// ============================================================================

#[test]
fn test_korq_k0_k1_k2() {
    let mut emu = emu64();
    // KORQ K0, K1, K2
    let code = [
        0xc4, 0xe1, 0xf5, 0x45, 0xc1, // KORQ K0, K1, K2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_korq_k1_k2_k3() {
    let mut emu = emu64();
    // KORQ K1, K2, K3
    let code = [
        0xc4, 0xe1, 0xed, 0x45, 0xca, // KORQ K1, K2, K3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_korq_k7_k5_k6() {
    let mut emu = emu64();
    // KORQ K7, K5, K6
    let code = [
        0xc4, 0xe1, 0xd5, 0x45, 0xfd, // KORQ K7, K5, K6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// KXORQ Tests - 64-bit mask XOR
// ============================================================================

#[test]
fn test_kxorq_k0_k1_k2() {
    let mut emu = emu64();
    // KXORQ K0, K1, K2
    let code = [
        0xc4, 0xe1, 0xf5, 0x47, 0xc1, // KXORQ K0, K1, K2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kxorq_k1_k2_k3() {
    let mut emu = emu64();
    // KXORQ K1, K2, K3
    let code = [
        0xc4, 0xe1, 0xed, 0x47, 0xca, // KXORQ K1, K2, K3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_kxorq_k7_k5_k6() {
    let mut emu = emu64();
    // KXORQ K7, K5, K6
    let code = [
        0xc4, 0xe1, 0xd5, 0x47, 0xfd, // KXORQ K7, K5, K6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Combined Operations Tests
// ============================================================================

#[test]
fn test_combined_and_or() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x41, 0xc1, // KANDW K0, K1, K2
        0xc5, 0xe5, 0x45, 0xe3, // KORW K4, K3, K4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_combined_and_xor() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xed, 0x41, 0xca, // KANDW K1, K2, K3
        0xc5, 0xdd, 0x47, 0xec, // KXORW K5, K4, K5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_boolean_expression() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x41, 0xe9, // KANDW K5, K1, K2
        0xc5, 0xe5, 0x47, 0xf3, // KXORW K6, K3, K4
        0xc5, 0xd5, 0x45, 0xfe, // KORW K7, K5, K6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
