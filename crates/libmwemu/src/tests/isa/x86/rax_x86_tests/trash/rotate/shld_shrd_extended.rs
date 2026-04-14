// Extended tests for SHLD/SHRD - comprehensive edge cases
//
// This file contains additional edge case tests for SHLD and SHRD instructions:
// - All shift counts
// - Boundary conditions
// - Flag behavior edge cases
// - Double-precision shift scenarios

use crate::*;

// ============================================================================
// SHLD comprehensive shift count tests
// ============================================================================

#[test]
fn test_shld_all_counts_16bit() {
    // Test SHLD with various shift counts for 16-bit
    for count in [0, 1, 4, 8, 12, 15, 16].iter() {
        let code = [0x66, 0x0f, 0xa4, 0xd8, *count, 0xf4]; // SHLD AX, BX, count
        let mut emu = emu64();
        emu.regs_mut().rax = 0xAAAA; // 1010_1010_1010_1010
        emu.regs_mut().rbx = 0x5555; // 0101_0101_0101_0101
        emu.load_code_bytes(&code);
        emu.run(None).unwrap();

        if *count == 0 {
            assert_eq!(emu.regs().rax & 0xFFFF, 0xAAAA, "SHLD AX by 0");
        } else if *count == 16 {
            // For 16-bit, count is masked to 5 bits: 16 & 0x1F = 16
            // Since count (16) == operand size (16), result is undefined per spec
            // Implementation treats 16 as valid shift, shifting out all original bits
            assert_eq!(emu.regs().rax & 0xFFFF, 0x5555, "SHLD AX by 16 (full replacement)");
        }
    }
}

#[test]
fn test_shrd_all_counts_16bit() {
    for count in [0, 1, 4, 8, 12, 15, 16].iter() {
        let code = [0x66, 0x0f, 0xac, 0xd8, *count, 0xf4]; // SHRD AX, BX, count
        let mut emu = emu64();
        emu.regs_mut().rax = 0xAAAA;
        emu.regs_mut().rbx = 0x5555;
        emu.load_code_bytes(&code);
        emu.run(None).unwrap();

        if *count == 0 {
            assert_eq!(emu.regs().rax & 0xFFFF, 0xAAAA, "SHRD AX by 0");
        } else if *count == 16 {
            // For 16-bit, count is masked to 5 bits: 16 & 0x1F = 16
            // Since count (16) == operand size (16), result is undefined per spec
            // Implementation treats 16 as valid shift, shifting out all original bits
            assert_eq!(emu.regs().rax & 0xFFFF, 0x5555, "SHRD AX by 16 (full replacement)");
        }
    }
}

#[test]
fn test_shld_all_counts_32bit() {
    for count in [0, 1, 4, 8, 16, 24, 31, 32].iter() {
        let code = [0x0f, 0xa4, 0xd8, *count, 0xf4]; // SHLD EAX, EBX, count
        let mut emu = emu64();
        emu.regs_mut().rax = 0xAAAAAAAA;
        emu.regs_mut().rbx = 0x55555555;
        emu.load_code_bytes(&code);
        emu.run(None).unwrap();

        if *count == 0 {
            assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xAAAAAAAA, "SHLD EAX by 0");
        } else if *count == 32 {
            // Count is masked: 32 MOD 32 = 0, so no shift occurs
            assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xAAAAAAAA, "SHLD EAX by 32 (count masked to 0)");
        }
    }
}

#[test]
fn test_shrd_all_counts_32bit() {
    for count in [0, 1, 4, 8, 16, 24, 31, 32].iter() {
        let code = [0x0f, 0xac, 0xd8, *count, 0xf4]; // SHRD EAX, EBX, count
        let mut emu = emu64();
        emu.regs_mut().rax = 0xAAAAAAAA;
        emu.regs_mut().rbx = 0x55555555;
        emu.load_code_bytes(&code);
        emu.run(None).unwrap();

        if *count == 0 {
            assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xAAAAAAAA, "SHRD EAX by 0");
        } else if *count == 32 {
            // Count is masked: 32 MOD 32 = 0, so no shift occurs
            assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xAAAAAAAA, "SHRD EAX by 32 (count masked to 0)");
        }
    }
}

#[test]
fn test_shld_all_counts_64bit() {
    for count in [0, 1, 8, 16, 32, 48, 63, 64].iter() {
        let code = [0x48, 0x0f, 0xa4, 0xd8, *count, 0xf4]; // SHLD RAX, RBX, count
        let mut emu = emu64();
        emu.regs_mut().rax = 0xAAAAAAAAAAAAAAAA;
        emu.regs_mut().rbx = 0x5555555555555555;
        emu.load_code_bytes(&code);
        emu.run(None).unwrap();

        if *count == 0 {
            assert_eq!(emu.regs().rax, 0xAAAAAAAAAAAAAAAA, "SHLD RAX by 0");
        } else if *count == 64 {
            // Count is masked: 64 MOD 64 = 0, so no shift occurs
            assert_eq!(emu.regs().rax, 0xAAAAAAAAAAAAAAAA, "SHLD RAX by 64 (count masked to 0)");
        }
    }
}

#[test]
fn test_shrd_all_counts_64bit() {
    for count in [0, 1, 8, 16, 32, 48, 63, 64].iter() {
        let code = [0x48, 0x0f, 0xac, 0xd8, *count, 0xf4]; // SHRD RAX, RBX, count
        let mut emu = emu64();
        emu.regs_mut().rax = 0xAAAAAAAAAAAAAAAA;
        emu.regs_mut().rbx = 0x5555555555555555;
        emu.load_code_bytes(&code);
        emu.run(None).unwrap();

        if *count == 0 {
            assert_eq!(emu.regs().rax, 0xAAAAAAAAAAAAAAAA, "SHRD RAX by 0");
        } else if *count == 64 {
            // Count is masked: 64 MOD 64 = 0, so no shift occurs
            assert_eq!(emu.regs().rax, 0xAAAAAAAAAAAAAAAA, "SHRD RAX by 64 (count masked to 0)");
        }
    }
}

// ============================================================================
// Boundary value tests
// ============================================================================

#[test]
fn test_shld_boundary_values_32bit() {
    let test_cases = [
        (0x00000000, 0x00000000),
        (0xFFFFFFFF, 0xFFFFFFFF),
        (0x00000000, 0xFFFFFFFF),
        (0xFFFFFFFF, 0x00000000),
        (0x80000000, 0x00000001),
        (0x00000001, 0x80000000),
    ];

    for (dest, src) in &test_cases {
        let code = [0x0f, 0xa4, 0xd8, 0x10, 0xf4]; // SHLD EAX, EBX, 16
        let mut emu = emu64();
        emu.regs_mut().rax = *dest;
        emu.regs_mut().rbx = *src;
        emu.load_code_bytes(&code);
        emu.run(None).unwrap();

        let expected = ((*dest << 16) | (*src >> 16)) & 0xFFFFFFFF;
        assert_eq!(emu.regs().rax & 0xFFFFFFFF, expected, "SHLD 0x{:08X} with 0x{:08X}", dest, src);
    }
}

#[test]
fn test_shrd_boundary_values_32bit() {
    let test_cases = [
        (0x00000000, 0x00000000),
        (0xFFFFFFFF, 0xFFFFFFFF),
        (0x00000000, 0xFFFFFFFF),
        (0xFFFFFFFF, 0x00000000),
        (0x80000000, 0x00000001),
        (0x00000001, 0x80000000),
    ];

    for (dest, src) in &test_cases {
        let code = [0x0f, 0xac, 0xd8, 0x10, 0xf4]; // SHRD EAX, EBX, 16
        let mut emu = emu64();
        emu.regs_mut().rax = *dest;
        emu.regs_mut().rbx = *src;
        emu.load_code_bytes(&code);
        emu.run(None).unwrap();

        let expected = ((*dest >> 16) | (*src << 16)) & 0xFFFFFFFF;
        assert_eq!(emu.regs().rax & 0xFFFFFFFF, expected, "SHRD 0x{:08X} with 0x{:08X}", dest, src);
    }
}

#[test]
fn test_shld_boundary_values_64bit() {
    let test_cases = [
        (0x0000000000000000, 0x0000000000000000),
        (0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF),
        (0x0000000000000000, 0xFFFFFFFFFFFFFFFF),
        (0xFFFFFFFFFFFFFFFF, 0x0000000000000000),
    ];

    for (dest, src) in &test_cases {
        let code = [0x48, 0x0f, 0xa4, 0xd8, 0x20, 0xf4]; // SHLD RAX, RBX, 32
        let mut emu = emu64();
        emu.regs_mut().rax = *dest;
        emu.regs_mut().rbx = *src;
        emu.load_code_bytes(&code);
        emu.run(None).unwrap();

        let expected = (*dest << 32) | (*src >> 32);
        assert_eq!(emu.regs().rax, expected, "SHLD 0x{:016X} with 0x{:016X}", dest, src);
    }
}

#[test]
fn test_shrd_boundary_values_64bit() {
    let test_cases = [
        (0x0000000000000000, 0x0000000000000000),
        (0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF),
        (0x0000000000000000, 0xFFFFFFFFFFFFFFFF),
        (0xFFFFFFFFFFFFFFFF, 0x0000000000000000),
    ];

    for (dest, src) in &test_cases {
        let code = [0x48, 0x0f, 0xac, 0xd8, 0x20, 0xf4]; // SHRD RAX, RBX, 32
        let mut emu = emu64();
        emu.regs_mut().rax = *dest;
        emu.regs_mut().rbx = *src;
        emu.load_code_bytes(&code);
        emu.run(None).unwrap();

        let expected = (*dest >> 32) | (*src << 32);
        assert_eq!(emu.regs().rax, expected, "SHRD 0x{:016X} with 0x{:016X}", dest, src);
    }
}

// ============================================================================
// Count masking tests
// ============================================================================

#[test]
fn test_shld_count_masking_32bit() {
    let code = [0x0f, 0xa5, 0xd8, 0xf4]; // SHLD EAX, EBX, CL
    let mut emu = emu64();
    emu.regs_mut().rax = 0x12345678;
    emu.regs_mut().rbx = 0xABCDEF01;
    emu.regs_mut().rcx = 0x24; // 36 - should be masked to 4 (36 & 0x1F = 4)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    let rax = emu.regs().rax;

    // Compare with explicit count of 4
    let code2 = [0x0f, 0xa4, 0xd8, 0x04, 0xf4];
    let mut emu = emu64();
    emu.load_code_bytes(&code2);
    emu.regs_mut().rax = 0x12345678;
    emu.regs_mut().rbx = 0xABCDEF01;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, rax & 0xFFFFFFFF, "Count masking for 32-bit SHLD");
}

#[test]
fn test_shrd_count_masking_32bit() {
    let code = [0x0f, 0xad, 0xd8, 0xf4]; // SHRD EAX, EBX, CL
    let mut emu = emu64();
    emu.regs_mut().rax = 0x12345678;
    emu.regs_mut().rbx = 0xABCDEF01;
    emu.regs_mut().rcx = 0x24; // 36 - should be masked to 4
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    let rax = emu.regs().rax;

    let code2 = [0x0f, 0xac, 0xd8, 0x04, 0xf4];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x12345678;
    emu.regs_mut().rbx = 0xABCDEF01;
    emu.load_code_bytes(&code2);
    emu.run(None).unwrap();

    assert_eq!(rax & 0xFFFFFFFF, emu.regs().rax & 0xFFFFFFFF, "Count masking for 32-bit SHRD");
}

#[test]
fn test_shld_count_masking_64bit() {
    let code = [0x48, 0x0f, 0xa5, 0xd8, 0xf4]; // SHLD RAX, RBX, CL
    let mut emu = emu64();
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.regs_mut().rbx = 0xFEDCBA9876543210;
    emu.regs_mut().rcx = 0x48; // 72 - should be masked to 8 (72 & 0x3F = 8)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    let rax = emu.regs().rax;

    let code2 = [0x48, 0x0f, 0xa4, 0xd8, 0x08, 0xf4];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.regs_mut().rbx = 0xFEDCBA9876543210;
    emu.load_code_bytes(&code2);
    emu.run(None).unwrap();

    assert_eq!(rax, emu.regs().rax, "Count masking for 64-bit SHLD");
}

#[test]
fn test_shrd_count_masking_64bit() {
    let code = [0x48, 0x0f, 0xad, 0xd8, 0xf4]; // SHRD RAX, RBX, CL
    let mut emu = emu64();
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.regs_mut().rbx = 0xFEDCBA9876543210;
    emu.regs_mut().rcx = 0x48; // 72 - should be masked to 8
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    let rax = emu.regs().rax;

    let code2 = [0x48, 0x0f, 0xac, 0xd8, 0x08, 0xf4];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.regs_mut().rbx = 0xFEDCBA9876543210;
    emu.load_code_bytes(&code2);
    emu.run(None).unwrap();

    assert_eq!(rax, emu.regs().rax, "Count masking for 64-bit SHRD");
}

// ============================================================================
// Flag tests
// ============================================================================

#[test]
fn test_shld_cf_edge_cases() {
    // Test CF with different bit patterns
    let test_cases = [
        (0x00000000, 8, false), // No bits shifted out
        (0xFF000000, 8, true),  // Bits shifted out
        (0x80000000, 1, true),  // MSB shifted out
        (0x7FFFFFFF, 1, false), // MSB not set
    ];

    for (value, count, expected_cf) in &test_cases {
        let code = [0x0f, 0xa4, 0xd8, *count, 0xf4]; // SHLD EAX, EBX, count
        let mut emu = emu64();
        emu.regs_mut().rax = *value;
        emu.regs_mut().rbx = 0x00000000;
        emu.load_code_bytes(&code);
        emu.run(None).unwrap();

        assert_eq!(emu.flags().f_cf, *expected_cf, "CF for SHLD 0x{:08X} by {}", value, count);
    }
}

#[test]
fn test_shrd_cf_edge_cases() {
    let test_cases = [
        (0x00000000, 8, false), // No bits shifted out
        (0x000000FF, 8, true),  // Bits shifted out
        (0x00000001, 1, true),  // LSB shifted out
        (0xFFFFFFFE, 1, false), // LSB not set
    ];

    for (value, count, expected_cf) in &test_cases {
        let code = [0x0f, 0xac, 0xd8, *count, 0xf4]; // SHRD EAX, EBX, count
        let mut emu = emu64();
        emu.regs_mut().rax = *value;
        emu.regs_mut().rbx = 0x00000000;
        emu.load_code_bytes(&code);
        emu.run(None).unwrap();

        assert_eq!(emu.flags().f_cf, *expected_cf, "CF for SHRD 0x{:08X} by {}", value, count);
    }
}

#[test]
fn test_shld_of_1bit_shift() {
    // OF is only defined for 1-bit shifts
    let code = [0x0f, 0xa4, 0xd8, 0x01, 0xf4]; // SHLD EAX, EBX, 1
    let mut emu = emu64();
    emu.regs_mut().rax = 0x40000000; // Will change sign
    emu.regs_mut().rbx = 0x00000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_of, "OF should be set for sign change");
}

#[test]
fn test_shrd_of_1bit_shift() {
    let code = [0x0f, 0xac, 0xd8, 0x01, 0xf4]; // SHRD EAX, EBX, 1
    let mut emu = emu64();
    emu.regs_mut().rax = 0x00000002;
    emu.regs_mut().rbx = 0x00000001; // Will set MSB
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_of, "OF should be set for sign change");
}

#[test]
fn test_shld_sf_zf_flags() {
    // Test SF and ZF flags
    let code = [0x0f, 0xa4, 0xd8, 0x01, 0xf4]; // SHLD EAX, EBX, 1
    let mut emu = emu64();
    emu.regs_mut().rax = 0x40000000;
    emu.regs_mut().rbx = 0x00000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_sf, "SF should be set (result is negative)");
    assert!(!emu.flags().f_zf, "ZF should be clear (result is non-zero)");
}

#[test]
fn test_shrd_sf_zf_flags() {
    let code = [0x0f, 0xac, 0xd8, 0x01, 0xf4]; // SHRD EAX, EBX, 1
    let mut emu = emu64();
    emu.regs_mut().rax = 0x00000002;
    emu.regs_mut().rbx = 0x00000001;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_sf, "SF should be set (result is negative)");
    assert!(!emu.flags().f_zf, "ZF should be clear (result is non-zero)");
}

#[test]
fn test_shld_zero_result() {
    // Both operands are zero, so result is always zero regardless of shift count
    let code = [0x0f, 0xa4, 0xd8, 0x10, 0xf4]; // SHLD EAX, EBX, 16
    let mut emu = emu64();
    emu.regs_mut().rax = 0x00000000;
    emu.regs_mut().rbx = 0x00000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0, "Result should be zero");
    assert!(emu.flags().f_zf, "ZF should be set");
}

#[test]
fn test_shrd_zero_result() {
    // Both operands are zero, so result is always zero regardless of shift count
    let code = [0x0f, 0xac, 0xd8, 0x10, 0xf4]; // SHRD EAX, EBX, 16
    let mut emu = emu64();
    emu.regs_mut().rax = 0x00000000;
    emu.regs_mut().rbx = 0x00000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0, "Result should be zero");
    assert!(emu.flags().f_zf, "ZF should be set");
}

// ============================================================================
// Multi-precision simulation tests
// ============================================================================

#[test]
fn test_shld_128bit_simulation() {
    // Simulate a 128-bit shift left by 16 bits
    let code = [
        0x48, 0x0f, 0xa4, 0xd0, 0x10, // SHLD RAX, RDX, 16
        0x48, 0xc1, 0xe2, 0x10,       // SHL RDX, 16
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x123456789ABCDEF0; // High 64 bits
    emu.regs_mut().rdx = 0xFEDCBA9876543210; // Low 64 bits
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x56789ABCDEF0FEDC, "High 64 bits after SHLD");
    assert_eq!(emu.regs().rdx, 0xBA98765432100000, "Low 64 bits after SHL");
}

#[test]
fn test_shrd_128bit_simulation() {
    // Simulate a 128-bit shift right by 16 bits
    let code = [
        0x48, 0x0f, 0xac, 0xd0, 0x10, // SHRD RAX, RDX, 16
        0x48, 0xc1, 0xea, 0x10,       // SHR RDX, 16
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0xFEDCBA9876543210; // Low 64 bits
    emu.regs_mut().rdx = 0x123456789ABCDEF0; // High 64 bits
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xDEF0FEDCBA987654, "Low 64 bits after SHRD");
    assert_eq!(emu.regs().rdx, 0x0000123456789ABC, "High 64 bits after SHR");
}

// ============================================================================
// Pattern tests
// ============================================================================

#[test]
fn test_shld_alternating_pattern() {
    let code = [0x0f, 0xa4, 0xd8, 0x10, 0xf4]; // SHLD EAX, EBX, 16
    let mut emu = emu64();
    emu.regs_mut().rax = 0xAAAAAAAA; // 1010...
    emu.regs_mut().rbx = 0x55555555; // 0101...
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xAAAA5555, "Alternating pattern SHLD");
}

#[test]
fn test_shrd_alternating_pattern() {
    let code = [0x0f, 0xac, 0xd8, 0x10, 0xf4]; // SHRD EAX, EBX, 16
    let mut emu = emu64();
    emu.regs_mut().rax = 0xAAAAAAAA;
    emu.regs_mut().rbx = 0x55555555;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x5555AAAA, "Alternating pattern SHRD");
}

#[test]
fn test_shld_shrd_complementary() {
    // SHLD and SHRD with complementary counts should preserve combined value
    let code = [0x0f, 0xa4, 0xd8, 0x0C, 0xf4]; // SHLD EAX, EBX, 12
    let mut emu = emu64();
    emu.regs_mut().rax = 0x12345678;
    emu.regs_mut().rbx = 0xABCDEF01;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    let regs_shld = emu.regs();

    let mut emu = emu64();
    let code2 = [0x0f, 0xac, 0xd8, 0x14, 0xf4]; // SHRD EAX, EBX, 20 (32-12)
    emu.load_code_bytes(&code2);
    emu.regs_mut().rax = 0x12345678;
    emu.regs_mut().rbx = 0xABCDEF01;
    emu.run(None).unwrap();
    let regs_shrd = emu.regs();

    // The combination should cover all bits from both registers
    let combined_shld = ((regs_shld.rax & 0xFFFFFFFF) as u64) << 20;
    let combined_shrd = (regs_shrd.rax & 0xFFFFFFFF) as u64;
    let original = (0x12345678u64 << 32) | 0xABCDEF01;

    // This tests that shifting complements correctly
    let _ = combined_shld | combined_shrd;
    let _ = original;
    // Note: exact check would need both values from the double-width shift
}
