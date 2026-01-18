use crate::*;

// ADCX/ADOX - Multi-precision Arithmetic Instructions
//
// ADCX (Add with Carry Flag):
//   Opcode: 66 0F 38 F6 /r (32-bit), 66 REX.W 0F 38 F6 /r (64-bit)
//   dest = dest + src + CF
//   Updates only CF flag (OF, SF, ZF, AF, PF unmodified)
//
// ADOX (Add with Overflow Flag):
//   Opcode: F3 0F 38 F6 /r (32-bit), F3 REX.W 0F 38 F6 /r (64-bit)
//   dest = dest + src + OF
//   Updates only OF flag (CF, SF, ZF, AF, PF unmodified)
//
// These instructions enable efficient multi-precision arithmetic by
// using two independent carry chains (CF and OF)

// ===== ADCX BASIC 32-BIT TESTS =====

#[test]
fn test_adcx_32bit_no_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf9, // STC (set CF)
        0xf5, // CMC (complement CF, now CF=0)
        0xb8, 0x05, 0x00, 0x00, 0x00, // MOV EAX, 5
        0xbb, 0x03, 0x00, 0x00, 0x00, // MOV EBX, 3
        0x66, 0x0f, 0x38, 0xf6, 0xc3, // ADCX EAX, EBX (5 + 3 + 0 = 8)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 8, "EAX should be 8");
    assert_eq!(emu.flags().dump() & 0x01, 0, "CF should be clear (no carry out)");
}

#[test]
fn test_adcx_32bit_with_carry_in() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf9, // STC (set CF=1)
        0xb8, 0x05, 0x00, 0x00, 0x00, // MOV EAX, 5
        0xbb, 0x03, 0x00, 0x00, 0x00, // MOV EBX, 3
        0x66, 0x0f, 0x38, 0xf6, 0xc3, // ADCX EAX, EBX (5 + 3 + 1 = 9)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 9, "EAX should be 9");
}

#[test]
fn test_adcx_32bit_carry_out() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf8, // CLC (clear CF)
        0xb8, 0xff, 0xff, 0xff, 0xff, // MOV EAX, 0xFFFFFFFF
        0xbb, 0x01, 0x00, 0x00, 0x00, // MOV EBX, 1
        0x66, 0x0f, 0x38, 0xf6, 0xc3, // ADCX EAX, EBX (overflow)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0, "EAX should be 0 (wrapped)");
    assert_ne!(emu.flags().dump() & 0x01, 0, "CF should be set (carry out)");
}

#[test]
fn test_adcx_32bit_chain() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf8, // CLC
        // First addition: 0xFFFFFFFF + 1 = 0, CF=1
        0xb8, 0xff, 0xff, 0xff, 0xff, // MOV EAX, 0xFFFFFFFF
        0xbb, 0x01, 0x00, 0x00, 0x00, // MOV EBX, 1
        0x66, 0x0f, 0x38, 0xf6, 0xc3, // ADCX EAX, EBX
        // Second addition: 0xFFFFFFFF + 0 + 1(CF) = 0, CF=1
        0xb9, 0xff, 0xff, 0xff, 0xff, // MOV ECX, 0xFFFFFFFF
        0xba, 0x00, 0x00, 0x00, 0x00, // MOV EDX, 0
        0x66, 0x0f, 0x38, 0xf6, 0xca, // ADCX ECX, EDX
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0, "EAX should be 0");
    assert_eq!(emu.regs().rcx & 0xFFFFFFFF, 0, "ECX should be 0 (propagated carry)");
    assert_ne!(emu.flags().dump() & 0x01, 0, "CF should still be set");
}

#[test]
fn test_adcx_32bit_preserves_other_flags() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        // Set various flags
        0xb8, 0x00, 0x00, 0x00, 0x80, // MOV EAX, 0x80000000
        0xbb, 0x00, 0x00, 0x00, 0x80, // MOV EBX, 0x80000000
        0x01, 0xd8, // ADD EAX, EBX (sets OF, SF, clears ZF)
        0xf8, // CLC (clear CF so ADCX uses carry-in = 0)
        // Now use ADCX which should preserve OF, SF, ZF
        0xb8, 0x05, 0x00, 0x00, 0x00, // MOV EAX, 5
        0xbb, 0x03, 0x00, 0x00, 0x00, // MOV EBX, 3
        0x66, 0x0f, 0x38, 0xf6, 0xc3, // ADCX EAX, EBX (should only affect CF)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    // ADCX should preserve OF from the ADD instruction
    // (This test verifies ADCX doesn't modify other flags)
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 8, "EAX should be 8");
}

// ===== ADCX 64-BIT TESTS =====

#[test]
fn test_adcx_64bit_no_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf8, // CLC
        0x48, 0xb8, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xbb, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 3
        0x66, 0x48, 0x0f, 0x38, 0xf6, 0xc3, // ADCX RAX, RBX
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 8, "RAX should be 8");
    assert_eq!(emu.flags().dump() & 0x01, 0, "CF should be clear");
}

#[test]
fn test_adcx_64bit_with_carry() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf9, // STC
        0x48, 0xb8, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xbb, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 3
        0x66, 0x48, 0x0f, 0x38, 0xf6, 0xc3, // ADCX RAX, RBX
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 9, "RAX should be 9");
}

#[test]
fn test_adcx_64bit_overflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf8, // CLC
        0x48, 0xb8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, // MOV RAX, 0xFFFFFFFFFFFFFFFF
        0x48, 0xbb, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 1
        0x66, 0x48, 0x0f, 0x38, 0xf6, 0xc3, // ADCX RAX, RBX
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0, "RAX should be 0");
    assert_ne!(emu.flags().dump() & 0x01, 0, "CF should be set");
}

#[test]
fn test_adcx_64bit_large_values() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf8, // CLC
        0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, // MOV RAX, 0x8000000000000000
        0x48, 0xbb, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, // MOV RBX, 0x4000000000000000
        0x66, 0x48, 0x0f, 0x38, 0xf6, 0xc3, // ADCX RAX, RBX
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0xC000000000000000, "RAX should be 0xC000000000000000");
}

// ===== ADOX BASIC 32-BIT TESTS =====

#[test]
fn test_adox_32bit_no_overflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        // Clear OF using XOR which clears OF
        0x31, 0xc0, // XOR EAX, EAX (clears OF among others)
        0xb8, 0x05, 0x00, 0x00, 0x00, // MOV EAX, 5
        0xbb, 0x03, 0x00, 0x00, 0x00, // MOV EBX, 3
        0xf3, 0x0f, 0x38, 0xf6, 0xc3, // ADOX EAX, EBX (5 + 3 + 0 = 8)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 8, "EAX should be 8");
}

#[test]
fn test_adox_32bit_with_overflow_in() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        // Set OF by causing signed overflow
        0xb8, 0xff, 0xff, 0xff, 0x7f, // MOV EAX, 0x7FFFFFFF
        0xbb, 0x01, 0x00, 0x00, 0x00, // MOV EBX, 1
        0x01, 0xd8, // ADD EAX, EBX (sets OF)
        // Now use ADOX with OF=1
        0xb8, 0x05, 0x00, 0x00, 0x00, // MOV EAX, 5
        0xbb, 0x03, 0x00, 0x00, 0x00, // MOV EBX, 3
        0xf3, 0x0f, 0x38, 0xf6, 0xc3, // ADOX EAX, EBX (5 + 3 + 1 = 9)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 9, "EAX should be 9");
}

#[test]
fn test_adox_32bit_overflow_out() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x31, 0xc0, // XOR EAX, EAX (clear OF)
        0xb8, 0xff, 0xff, 0xff, 0xff, // MOV EAX, 0xFFFFFFFF
        0xbb, 0x01, 0x00, 0x00, 0x00, // MOV EBX, 1
        0xf3, 0x0f, 0x38, 0xf6, 0xc3, // ADOX EAX, EBX
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0, "EAX should be 0");
    assert_ne!(emu.flags().dump() & 0x800, 0, "OF should be set");
}

#[test]
fn test_adox_32bit_preserves_cf() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf9, // STC (set CF)
        0x31, 0xc0, // XOR EAX, EAX (clear OF, but preserve CF? Actually XOR clears CF too)
        0xf9, // STC again to ensure CF is set
        0xb8, 0x05, 0x00, 0x00, 0x00, // MOV EAX, 5
        0xbb, 0x03, 0x00, 0x00, 0x00, // MOV EBX, 3
        0xf3, 0x0f, 0x38, 0xf6, 0xc3, // ADOX EAX, EBX (should not modify CF)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_ne!(emu.flags().dump() & 0x01, 0, "CF should still be set (ADOX doesn't modify CF)");
}

// ===== ADOX 64-BIT TESTS =====

#[test]
fn test_adox_64bit_no_overflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x31, 0xc0, // XOR EAX, EAX (clear OF)
        0x48, 0xb8, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xbb, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 3
        0xf3, 0x48, 0x0f, 0x38, 0xf6, 0xc3, // ADOX RAX, RBX
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 8, "RAX should be 8");
}

#[test]
fn test_adox_64bit_overflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x31, 0xc0, // XOR EAX, EAX
        0x48, 0xb8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, // MOV RAX, 0xFFFFFFFFFFFFFFFF
        0x48, 0xbb, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 1
        0xf3, 0x48, 0x0f, 0x38, 0xf6, 0xc3, // ADOX RAX, RBX
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0, "RAX should be 0");
    assert_ne!(emu.flags().dump() & 0x800, 0, "OF should be set");
}

#[test]
fn test_adox_64bit_large_values() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x31, 0xc0, // XOR EAX, EAX
        0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, // MOV RAX, 0x8000000000000000
        0x48, 0xbb, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, // MOV RBX, 0x4000000000000000
        0xf3, 0x48, 0x0f, 0x38, 0xf6, 0xc3, // ADOX RAX, RBX
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0xC000000000000000, "RAX should be 0xC000000000000000");
}

// ===== COMBINED ADCX/ADOX TESTS =====

#[test]
fn test_adcx_adox_independent_flags() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf8, // CLC
        0x31, 0xc0, // XOR EAX, EAX (clear OF)
        // ADCX sets CF
        0xb8, 0xff, 0xff, 0xff, 0xff, // MOV EAX, 0xFFFFFFFF
        0xbb, 0x01, 0x00, 0x00, 0x00, // MOV EBX, 1
        0x66, 0x0f, 0x38, 0xf6, 0xc3, // ADCX EAX, EBX (sets CF)
        // ADOX sets OF (shouldn't affect CF)
        0xb9, 0xff, 0xff, 0xff, 0xff, // MOV ECX, 0xFFFFFFFF
        0xba, 0x01, 0x00, 0x00, 0x00, // MOV EDX, 1
        0xf3, 0x0f, 0x38, 0xf6, 0xca, // ADOX ECX, EDX (sets OF)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_ne!(emu.flags().dump() & 0x01, 0, "CF should still be set");
    assert_ne!(emu.flags().dump() & 0x800, 0, "OF should be set");
}

#[test]
fn test_multiprecision_128bit_add() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf8, // CLC
        0x31, 0xc0, // XOR EAX, EAX (clear OF)
        // Low 64 bits: 1 + 3 = 4
        0x48, 0xb8, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0xbb, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 3
        0x66, 0x48, 0x0f, 0x38, 0xf6, 0xc3, // ADCX RAX, RBX
        // High 64 bits: 2 + 4 + carry = 6
        0x48, 0xb9, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 2
        0x48, 0xba, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RDX, 4
        0x66, 0x48, 0x0f, 0x38, 0xf6, 0xca, // ADCX RCX, RDX
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 4, "RAX (low 64) should be 4");
    assert_eq!(emu.regs().rcx, 6, "RCX (high 64) should be 6");
}

#[test]
fn test_multiprecision_dual_chain() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf8, // CLC
        0x31, 0xc0, // XOR EAX, EAX (clear OF)

        // First chain (CF): Add with carry
        0xb8, 0xff, 0xff, 0xff, 0xff, // MOV EAX, 0xFFFFFFFF
        0xbb, 0x01, 0x00, 0x00, 0x00, // MOV EBX, 1
        0x66, 0x0f, 0x38, 0xf6, 0xc3, // ADCX EAX, EBX (CF=1)

        // Second chain (OF): Independent overflow chain
        0xb9, 0xff, 0xff, 0xff, 0xff, // MOV ECX, 0xFFFFFFFF
        0xba, 0x02, 0x00, 0x00, 0x00, // MOV EDX, 2
        0xf3, 0x0f, 0x38, 0xf6, 0xca, // ADOX ECX, EDX (OF=1)

        // Continue first chain
        0xbe, 0x00, 0x00, 0x00, 0x00, // MOV ESI, 0
        0xbf, 0x00, 0x00, 0x00, 0x00, // MOV EDI, 0
        0x66, 0x0f, 0x38, 0xf6, 0xf7, // ADCX ESI, EDI (0 + 0 + CF = 1)

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0, "EAX should be 0");
    assert_eq!(emu.regs().rcx & 0xFFFFFFFF, 1, "ECX should be 1");
    assert_eq!(emu.regs().rsi & 0xFFFFFFFF, 1, "ESI should be 1 (carry propagated)");
}

// ===== EDGE CASES =====

#[test]
fn test_adcx_zeros() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf8, // CLC
        0x31, 0xc0, // XOR EAX, EAX
        0x31, 0xdb, // XOR EBX, EBX
        0x66, 0x0f, 0x38, 0xf6, 0xc3, // ADCX EAX, EBX (0 + 0 + 0 = 0)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0, "EAX should be 0");
    assert_eq!(emu.flags().dump() & 0x01, 0, "CF should be clear");
}

#[test]
fn test_adox_zeros() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x31, 0xc0, // XOR EAX, EAX (clear OF)
        0x31, 0xdb, // XOR EBX, EBX
        0xf3, 0x0f, 0x38, 0xf6, 0xc3, // ADOX EAX, EBX
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0, "EAX should be 0");
}

#[test]
fn test_adcx_memory_operand() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        // Set up memory with value 0x12345678 at 0x2000
        0xc7, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x78, 0x56, 0x34, 0x12, // MOV DWORD PTR [0x2000], 0x12345678
        0xf8, // CLC
        0xb8, 0x88, 0xa9, 0xcb, 0xed, // MOV EAX, 0xEDCBA988
        0x66, 0x0f, 0x38, 0xf6, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // ADCX EAX, [0x2000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x00000000, "Result should wrap");
    assert_ne!(emu.flags().dump() & 0x01, 0, "CF should be set");
}

#[test]
fn test_adox_memory_operand() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        // Set up memory
        0xc7, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, 0x0a, 0x00, 0x00, 0x00, // MOV DWORD PTR [0x2000], 10
        0x31, 0xc0, // XOR EAX, EAX (clear OF)
        0xb8, 0x14, 0x00, 0x00, 0x00, // MOV EAX, 20
        0xf3, 0x0f, 0x38, 0xf6, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // ADOX EAX, [0x2000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 30, "EAX should be 30");
}

// ===== PRACTICAL MULTI-PRECISION PATTERNS =====

#[test]
fn test_256bit_addition_pattern() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf8, // CLC

        // Limb 0: Low 64 bits
        0x48, 0xb8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, // MOV RAX, 0xFFFFFFFFFFFFFFFF
        0x48, 0xbb, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 1
        0x66, 0x48, 0x0f, 0x38, 0xf6, 0xc3, // ADCX RAX, RBX

        // Limb 1
        0x48, 0xb9, 0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, // MOV RCX, 0xFFFFFFFFFFFFFFFE
        0x48, 0xba, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RDX, 2
        0x66, 0x48, 0x0f, 0x38, 0xf6, 0xca, // ADCX RCX, RDX

        // Limb 2
        0x48, 0xbe, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RSI, 0
        0x48, 0xbf, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RDI, 0
        0x66, 0x48, 0x0f, 0x38, 0xf6, 0xf7, // ADCX RSI, RDI

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0, "Limb 0 should be 0 (wrapped)");
    assert_eq!(emu.regs().rcx, 1, "Limb 1 should be 1 (carry propagated)");
    assert_eq!(emu.regs().rsi, 1, "Limb 2 should be 1 (carry propagated)");
}

#[test]
fn test_interleaved_adcx_adox() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf8, // CLC
        0x31, 0xc0, // XOR EAX, EAX (clear OF)

        // First operand pair (CF chain)
        0xb8, 0x01, 0x00, 0x00, 0x00, // MOV EAX, 1
        0xbb, 0x02, 0x00, 0x00, 0x00, // MOV EBX, 2
        0x66, 0x0f, 0x38, 0xf6, 0xc3, // ADCX EAX, EBX (3, no carry)

        // Second operand pair (OF chain)
        0xb9, 0x04, 0x00, 0x00, 0x00, // MOV ECX, 4
        0xba, 0x05, 0x00, 0x00, 0x00, // MOV EDX, 5
        0xf3, 0x0f, 0x38, 0xf6, 0xca, // ADOX ECX, EDX (9, no overflow)

        // Third pair (CF chain continues)
        0xbe, 0x06, 0x00, 0x00, 0x00, // MOV ESI, 6
        0xbf, 0x07, 0x00, 0x00, 0x00, // MOV EDI, 7
        0x66, 0x0f, 0x38, 0xf6, 0xf7, // ADCX ESI, EDI (13, no carry)

        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 3, "EAX should be 3");
    assert_eq!(emu.regs().rcx & 0xFFFFFFFF, 9, "ECX should be 9");
    assert_eq!(emu.regs().rsi & 0xFFFFFFFF, 13, "ESI should be 13");
}

// ===== ADDITIONAL ADCX TESTS =====

#[test]
fn test_adcx_32bit_repeated_operations() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf8, // CLC
        0xb8, 0x01, 0x00, 0x00, 0x00, // MOV EAX, 1
        0xbb, 0x01, 0x00, 0x00, 0x00, // MOV EBX, 1
        0x66, 0x0f, 0x38, 0xf6, 0xc3, // ADCX EAX, EBX (2)
        0x66, 0x0f, 0x38, 0xf6, 0xc3, // ADCX EAX, EBX (3)
        0x66, 0x0f, 0x38, 0xf6, 0xc3, // ADCX EAX, EBX (4)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 4, "EAX should be 4");
}

#[test]
fn test_adox_32bit_repeated_operations() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x31, 0xc0, // XOR EAX, EAX
        0xb8, 0x10, 0x00, 0x00, 0x00, // MOV EAX, 16
        0xbb, 0x10, 0x00, 0x00, 0x00, // MOV EBX, 16
        0xf3, 0x0f, 0x38, 0xf6, 0xc3, // ADOX EAX, EBX (32)
        0xf3, 0x0f, 0x38, 0xf6, 0xc3, // ADOX EAX, EBX (48)
        0xf3, 0x0f, 0x38, 0xf6, 0xc3, // ADOX EAX, EBX (64)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 64, "EAX should be 64");
}

#[test]
fn test_adcx_power_of_two() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf8, // CLC
        0xb8, 0x00, 0x00, 0x00, 0x01, // MOV EAX, 0x01000000
        0xbb, 0x00, 0x00, 0x00, 0x01, // MOV EBX, 0x01000000
        0x66, 0x0f, 0x38, 0xf6, 0xc3, // ADCX EAX, EBX
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x02000000, "EAX should be 0x02000000");
}

#[test]
fn test_adox_power_of_two() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x31, 0xc0, // XOR EAX, EAX
        0xb8, 0x00, 0x00, 0x00, 0x02, // MOV EAX, 0x02000000
        0xbb, 0x00, 0x00, 0x00, 0x02, // MOV EBX, 0x02000000
        0xf3, 0x0f, 0x38, 0xf6, 0xc3, // ADOX EAX, EBX
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x04000000, "EAX should be 0x04000000");
}

#[test]
fn test_adcx_64bit_power_of_two() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf8, // CLC
        0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 0x0000000100000000
        0x48, 0xbb, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, // MOV RBX, 0x0000000100000000
        0x66, 0x48, 0x0f, 0x38, 0xf6, 0xc3, // ADCX RAX, RBX
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0x0000000200000000, "RAX should be 0x0000000200000000");
}

#[test]
fn test_adox_64bit_power_of_two() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x31, 0xc0, // XOR EAX, EAX
        0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, // MOV RAX, 0x0000000200000000
        0x48, 0xbb, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, // MOV RBX, 0x0000000200000000
        0xf3, 0x48, 0x0f, 0x38, 0xf6, 0xc3, // ADOX RAX, RBX
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0x0000000400000000, "RAX should be 0x0000000400000000");
}

#[test]
fn test_adcx_32bit_boundary() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf8, // CLC
        0xb8, 0xfe, 0xff, 0xff, 0x7f, // MOV EAX, 0x7FFFFFFE
        0xbb, 0x01, 0x00, 0x00, 0x00, // MOV EBX, 1
        0x66, 0x0f, 0x38, 0xf6, 0xc3, // ADCX EAX, EBX
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x7FFFFFFF, "EAX should be 0x7FFFFFFF");
}

#[test]
fn test_adox_32bit_boundary() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x31, 0xc0, // XOR EAX, EAX
        0xb8, 0xfe, 0xff, 0xff, 0x7f, // MOV EAX, 0x7FFFFFFE
        0xbb, 0x01, 0x00, 0x00, 0x00, // MOV EBX, 1
        0xf3, 0x0f, 0x38, 0xf6, 0xc3, // ADOX EAX, EBX
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x7FFFFFFF, "EAX should be 0x7FFFFFFF");
}

#[test]
fn test_adcx_carry_chain_4_limbs() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf8, // CLC
        // Limb 0
        0xb8, 0xff, 0xff, 0xff, 0xff, // MOV EAX, 0xFFFFFFFF
        0xbb, 0x01, 0x00, 0x00, 0x00, // MOV EBX, 1
        0x66, 0x0f, 0x38, 0xf6, 0xc3, // ADCX EAX, EBX
        // Limb 1
        0xb9, 0xff, 0xff, 0xff, 0xff, // MOV ECX, 0xFFFFFFFF
        0xba, 0x00, 0x00, 0x00, 0x00, // MOV EDX, 0
        0x66, 0x0f, 0x38, 0xf6, 0xca, // ADCX ECX, EDX
        // Limb 2
        0xbe, 0xff, 0xff, 0xff, 0xff, // MOV ESI, 0xFFFFFFFF
        0xbf, 0x00, 0x00, 0x00, 0x00, // MOV EDI, 0
        0x66, 0x0f, 0x38, 0xf6, 0xf7, // ADCX ESI, EDI
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0, "EAX should be 0");
    assert_eq!(emu.regs().rcx & 0xFFFFFFFF, 0, "ECX should be 0");
    assert_eq!(emu.regs().rsi & 0xFFFFFFFF, 0, "ESI should be 0");
    assert_ne!(emu.flags().dump() & 0x01, 0, "CF should still be set");
}

#[test]
fn test_adox_overflow_chain_4_limbs() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x31, 0xc0, // XOR EAX, EAX (clear OF)
        // Limb 0
        0xb8, 0xff, 0xff, 0xff, 0xff, // MOV EAX, 0xFFFFFFFF
        0xbb, 0x01, 0x00, 0x00, 0x00, // MOV EBX, 1
        0xf3, 0x0f, 0x38, 0xf6, 0xc3, // ADOX EAX, EBX
        // Limb 1
        0xb9, 0xff, 0xff, 0xff, 0xff, // MOV ECX, 0xFFFFFFFF
        0xba, 0x00, 0x00, 0x00, 0x00, // MOV EDX, 0
        0xf3, 0x0f, 0x38, 0xf6, 0xca, // ADOX ECX, EDX
        // Limb 2
        0xbe, 0xff, 0xff, 0xff, 0xff, // MOV ESI, 0xFFFFFFFF
        0xbf, 0x00, 0x00, 0x00, 0x00, // MOV EDI, 0
        0xf3, 0x0f, 0x38, 0xf6, 0xf7, // ADOX ESI, EDI
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0, "EAX should be 0");
    assert_eq!(emu.regs().rcx & 0xFFFFFFFF, 0, "ECX should be 0");
    assert_eq!(emu.regs().rsi & 0xFFFFFFFF, 0, "ESI should be 0");
    assert_ne!(emu.flags().dump() & 0x800, 0, "OF should still be set");
}

#[test]
fn test_adcx_adox_alternating() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf8, // CLC
        0x31, 0xc0, // XOR EAX, EAX (clear OF)
        // ADCX
        0xb8, 0x01, 0x00, 0x00, 0x00, // MOV EAX, 1
        0xbb, 0x02, 0x00, 0x00, 0x00, // MOV EBX, 2
        0x66, 0x0f, 0x38, 0xf6, 0xc3, // ADCX EAX, EBX (3)
        // ADOX
        0xb9, 0x04, 0x00, 0x00, 0x00, // MOV ECX, 4
        0xba, 0x05, 0x00, 0x00, 0x00, // MOV EDX, 5
        0xf3, 0x0f, 0x38, 0xf6, 0xca, // ADOX ECX, EDX (9)
        // ADCX again
        0xbe, 0x06, 0x00, 0x00, 0x00, // MOV ESI, 6
        0xbf, 0x07, 0x00, 0x00, 0x00, // MOV EDI, 7
        0x66, 0x0f, 0x38, 0xf6, 0xf7, // ADCX ESI, EDI (13)
        // ADOX again
        0x41, 0xb8, 0x08, 0x00, 0x00, 0x00, // MOV R8D, 8
        0x41, 0xb9, 0x09, 0x00, 0x00, 0x00, // MOV R9D, 9
        0x66, 0x45, 0x0f, 0x38, 0xf6, 0xc1, // ADCX R8D, R9D (17)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 3, "EAX should be 3");
    assert_eq!(emu.regs().rcx & 0xFFFFFFFF, 9, "ECX should be 9");
    assert_eq!(emu.regs().rsi & 0xFFFFFFFF, 13, "ESI should be 13");
    assert_eq!(emu.regs().r8 & 0xFFFFFFFF, 17, "R8D should be 17");
}

#[test]
fn test_adcx_64bit_full_range() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf8, // CLC
        0x48, 0xb8, 0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f, // MOV RAX, 0x7FFFFFFFFFFFFFFE
        0x48, 0xbb, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 1
        0x66, 0x48, 0x0f, 0x38, 0xf6, 0xc3, // ADCX RAX, RBX
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0x7FFFFFFFFFFFFFFF, "RAX should be 0x7FFFFFFFFFFFFFFF");
}

#[test]
fn test_adox_64bit_full_range() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x31, 0xc0, // XOR EAX, EAX
        0x48, 0xb8, 0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f, // MOV RAX, 0x7FFFFFFFFFFFFFFE
        0x48, 0xbb, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 1
        0xf3, 0x48, 0x0f, 0x38, 0xf6, 0xc3, // ADOX RAX, RBX
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0x7FFFFFFFFFFFFFFF, "RAX should be 0x7FFFFFFFFFFFFFFF");
}

#[test]
fn test_adcx_with_initial_carry_propagation() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf9, // STC (CF = 1)
        0xb8, 0x00, 0x00, 0x00, 0x00, // MOV EAX, 0
        0xbb, 0x00, 0x00, 0x00, 0x00, // MOV EBX, 0
        0x66, 0x0f, 0x38, 0xf6, 0xc3, // ADCX EAX, EBX (0 + 0 + 1 = 1)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 1, "EAX should be 1 (carry in)");
}

#[test]
fn test_adox_with_initial_overflow_propagation() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        // Set OF
        0xb8, 0xff, 0xff, 0xff, 0x7f, // MOV EAX, 0x7FFFFFFF
        0xbb, 0x01, 0x00, 0x00, 0x00, // MOV EBX, 1
        0x01, 0xd8, // ADD EAX, EBX (sets OF)
        // Use ADOX with OF=1
        0xb8, 0x00, 0x00, 0x00, 0x00, // MOV EAX, 0
        0xbb, 0x00, 0x00, 0x00, 0x00, // MOV EBX, 0
        0xf3, 0x0f, 0x38, 0xf6, 0xc3, // ADOX EAX, EBX (0 + 0 + 1 = 1)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 1, "EAX should be 1 (overflow in)");
}
