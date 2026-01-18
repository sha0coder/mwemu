use crate::*;
use std::convert::TryInto;
const DATA_ADDR: u64 = 0x7000;

// Comprehensive tests for PUSHF/POPF/PUSHFQ/POPFQ instructions
//
// PUSHF - Push FLAGS (16-bit) onto stack
// POPF - Pop FLAGS (16-bit) from stack
// PUSHFQ - Push RFLAGS (64-bit) onto stack
// POPFQ - Pop RFLAGS (64-bit) from stack
//
// These instructions save and restore the processor flags

// ============================================================================
// PUSHFQ - Push RFLAGS (64-bit)
// Opcode: 9C (REX.W prefix for 64-bit)
// ============================================================================

#[test]
fn test_pushfq_basic() {
    let mut emu = emu64();
    let code = [
        0x48, 0x9c, // PUSHFQ
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x0FF8, "RSP decremented by 8");

    let mut stack_val = [0u8; 8];
    stack_val = emu.maps.read_bytes(0x0FF8, stack_val.len()).try_into().unwrap();
    let pushed_flags = u64::from_le_bytes(stack_val);
    assert_ne!(pushed_flags, 0, "Flags should be pushed");
}

#[test]
fn test_pushfq_with_carry_set() {
    let mut emu = emu64();
    let code = [
        0xf9, // STC (set carry)
        0x48, 0x9c, // PUSHFQ
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let mut stack_val = [0u8; 8];
    stack_val = emu.maps.read_bytes(0x0FF8, stack_val.len()).try_into().unwrap();
    let pushed_flags = u64::from_le_bytes(stack_val);
    assert_ne!(pushed_flags & 0x01, 0, "CF should be set in pushed flags");
}

#[test]
fn test_pushfq_with_zero_set() {
    let mut emu = emu64();
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX (sets ZF)
        0x48, 0x9c, // PUSHFQ
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let mut stack_val = [0u8; 8];
    stack_val = emu.maps.read_bytes(0x0FF8, stack_val.len()).try_into().unwrap();
    let pushed_flags = u64::from_le_bytes(stack_val);
    assert_ne!(pushed_flags & 0x40, 0, "ZF should be set in pushed flags");
}

#[test]
fn test_pushfq_with_sign_set() {
    let mut emu = emu64();
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x85, 0xc0, // TEST RAX, RAX (sets SF)
        0x48, 0x9c, // PUSHFQ
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let mut stack_val = [0u8; 8];
    stack_val = emu.maps.read_bytes(0x0FF8, stack_val.len()).try_into().unwrap();
    let pushed_flags = u64::from_le_bytes(stack_val);
    assert_ne!(pushed_flags & 0x80, 0, "SF should be set in pushed flags");
}

#[test]
fn test_pushfq_multiple_flags() {
    let mut emu = emu64();
    let code = [
        // XOR clears CF, so do XOR first to set ZF, then STC to set CF
        0x48, 0x31, 0xc0, // XOR RAX, RAX (sets ZF, PF, clears CF/OF)
        0xf9, // STC (set carry)
        0x48, 0x9c, // PUSHFQ
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let mut stack_val = [0u8; 8];
    stack_val = emu.maps.read_bytes(0x0FF8, stack_val.len()).try_into().unwrap();
    let pushed_flags = u64::from_le_bytes(stack_val);
    assert_ne!(pushed_flags & 0x01, 0, "CF should be set");
    assert_ne!(pushed_flags & 0x40, 0, "ZF should be set");
}

#[test]
fn test_pushfq_preserves_registers() {
    let mut emu = emu64();
    let code = [
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0x48, 0xc7, 0xc3, 0x99, 0x00, 0x00, 0x00, // MOV RBX, 0x99
        0x48, 0x9c, // PUSHFQ
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x42, "RAX unchanged");
    assert_eq!(emu.regs().rbx, 0x99, "RBX unchanged");
}

#[test]
fn test_pushfq_multiple_times() {
    let mut emu = emu64();
    let code = [
        0xf9, // STC
        0x48, 0x9c, // PUSHFQ (with CF)
        0xf8, // CLC
        0x48, 0x9c, // PUSHFQ (without CF)
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x1000 - 16, "RSP decremented twice");

    let mut stack_val = [0u8; 8];
    stack_val = emu.maps.read_bytes(0x0FF8, stack_val.len()).try_into().unwrap();
    let first_flags = u64::from_le_bytes(stack_val);
    stack_val = emu.maps.read_bytes(0x0FF0, stack_val.len()).try_into().unwrap();
    let second_flags = u64::from_le_bytes(stack_val);

    assert_ne!(first_flags & 0x01, 0, "First PUSHFQ has CF set");
    assert_eq!(second_flags & 0x01, 0, "Second PUSHFQ has CF clear");
}

// ============================================================================
// POPFQ - Pop RFLAGS (64-bit)
// Opcode: 9D (REX.W prefix for 64-bit)
// ============================================================================

#[test]
fn test_popfq_basic() {
    let mut emu = emu64();
    let code = [
        0x48, 0x9c, // PUSHFQ
        0x48, 0x9d, // POPFQ
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x1000, "RSP restored");
}

#[test]
fn test_popfq_restore_carry() {
    let mut emu = emu64();
    let code = [
        0xf9, // STC (set carry)
        0x48, 0x9c, // PUSHFQ
        0xf8, // CLC (clear carry)
        0x48, 0x9d, // POPFQ (restore carry)
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_cf, "CF should be restored");
}

#[test]
fn test_popfq_restore_zero() {
    let mut emu = emu64();
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX (sets ZF)
        0x48, 0x9c, // PUSHFQ
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0x85, 0xc0, // TEST RAX, RAX (clears ZF)
        0x48, 0x9d, // POPFQ (restore ZF)
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_zf, "ZF should be restored");
}

#[test]
fn test_popfq_restore_sign() {
    let mut emu = emu64();
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x85, 0xc0, // TEST RAX, RAX (sets SF)
        0x48, 0x9c, // PUSHFQ
        0x48, 0x31, 0xc0, // XOR RAX, RAX (clears SF)
        0x48, 0x9d, // POPFQ
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_sf, "SF should be restored");
}

#[test]
fn test_popfq_restore_all_flags() {
    let mut emu = emu64();
    let code = [
        // XOR clears CF, so do XOR first to set ZF, then STC to set CF
        0x48, 0x31, 0xc0, // XOR RAX, RAX (sets ZF, PF, clears CF)
        0xf9, // STC (set CF after XOR)
        0x48, 0x9c, // PUSHFQ
        0xf8, // CLC
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0x85, 0xc0, // TEST RAX, RAX (clears ZF)
        0x48, 0x9d, // POPFQ
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_cf, "CF should be restored");
    assert!(emu.flags().f_zf, "ZF should be restored");
}

#[test]
fn test_popfq_preserves_registers() {
    let mut emu = emu64();
    let code = [
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0x48, 0xc7, 0xc3, 0x99, 0x00, 0x00, 0x00, // MOV RBX, 0x99
        0x48, 0x9c, // PUSHFQ
        0x48, 0x9d, // POPFQ
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x42, "RAX unchanged");
    assert_eq!(emu.regs().rbx, 0x99, "RBX unchanged");
}

// ============================================================================
// PUSHF/POPF Roundtrip Tests
// ============================================================================

#[test]
fn test_pushfq_popfq_roundtrip() {
    let mut emu = emu64();
    let code = [
        0xf9, // STC
        0x48, 0x9c, // PUSHFQ
        0xf8, // CLC
        0x48, 0x9d, // POPFQ
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_cf, "CF should be restored by POPFQ");
    assert_eq!(emu.regs().rsp, 0x1000, "RSP should be balanced");
}

#[test]
fn test_pushfq_popfq_nested() {
    let mut emu = emu64();
    let code = [
        0xf9, // STC
        0x48, 0x9c, // PUSHFQ (save flags with CF)
        0xf8, // CLC
        0x48, 0x9c, // PUSHFQ (save flags without CF)
        0x48, 0x9d, // POPFQ (restore no CF)
        0x48, 0x9d, // POPFQ (restore CF)
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_cf, "CF should be restored from first PUSHFQ");
    assert_eq!(emu.regs().rsp, 0x1000, "Stack should be balanced");
}

#[test]
fn test_pushfq_popfq_with_arithmetic() {
    let mut emu = emu64();
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (sets ZF, CF)
        0x48, 0x9c, // PUSHFQ
        // Do some other arithmetic
        0x48, 0xc7, 0xc0, 0x10, 0x00, 0x00, 0x00, // MOV RAX, 0x10
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (clears ZF, CF)
        0x48, 0x9d, // POPFQ (restore original flags)
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_zf, "ZF should be restored");
    assert!(emu.flags().f_cf, "CF should be restored");
}

#[test]
fn test_pushfq_modify_on_stack() {
    let mut emu = emu64();
    let code = [
        0x48, 0x9c, // PUSHFQ
        // Modify CF bit on stack
        0x48, 0x8b, 0x04, 0x24, // MOV RAX, [RSP]
        0x48, 0x83, 0xc8, 0x01, // OR RAX, 0x01 (set CF bit)
        0x48, 0x89, 0x04, 0x24, // MOV [RSP], RAX
        0x48, 0x9d, // POPFQ
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_cf, "CF should be set from modified stack value");
}

#[test]
fn test_multiple_pushfq_popfq() {
    let mut emu = emu64();
    let code = [
        0xf9, // STC
        0x48, 0x9c, // PUSHFQ
        0x48, 0x9c, // PUSHFQ
        0x48, 0x9c, // PUSHFQ
        0x48, 0x9d, // POPFQ
        0x48, 0x9d, // POPFQ
        0x48, 0x9d, // POPFQ
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x1000, "Stack should be balanced");
    assert!(emu.flags().f_cf, "CF should still be set");
}

// ============================================================================
// 16-bit PUSHF/POPF (without REX.W prefix)
// ============================================================================

#[test]
fn test_pushf_16bit() {
    let mut emu = emu64();
    let code = [
        0x66, 0x9c, // PUSHF with 66H prefix = 16-bit push
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x0FFE, "RSP decremented by 2");
}

#[test]
fn test_popf_16bit() {
    let mut emu = emu64();
    let code = [
        0x66, 0x9c, // PUSHF
        0x66, 0x9d, // POPF
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x1000, "RSP should be balanced");
}

// ============================================================================
// Edge Cases and Special Scenarios
// ============================================================================

#[test]
fn test_pushfq_at_stack_boundary() {
    let mut emu = emu64();
    let code = [
        0x48, 0x9c, // PUSHFQ
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x08; // Near bottom of memory
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x00, "RSP decremented");
}

#[test]
fn test_popfq_from_prepared_stack() {
    let mut emu = emu64();
    let code = [
        0x48, 0x9d, // POPFQ
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x3000;
    emu.load_code_bytes(&code);

    let flags_with_cf = 0x0001u64;
    let flags_bytes = flags_with_cf.to_le_bytes();
    emu.maps.write_bytes_slice(0x3000, &flags_bytes);

    emu.run(None).unwrap();
    assert!(emu.flags().f_cf, "CF should be set from stack");
}

#[test]
fn test_pushfq_popfq_preserves_reserved_bits() {
    let mut emu = emu64();
    let code = [
        0x48, 0x9c, // PUSHFQ
        0x48, 0x9d, // POPFQ
        0x48, 0x9c, // PUSHFQ again
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let mut stack_val = [0u8; 8];
    stack_val = emu.maps.read_bytes(emu.regs().rsp, stack_val.len()).try_into().unwrap();
    let pushed_flags = u64::from_le_bytes(stack_val);

    assert_ne!(pushed_flags & 0x02, 0, "Reserved bit 1 should be set");
}

#[test]
fn test_pushfq_with_overflow() {
    let mut emu = emu64();
    let code = [
        // Use 32-bit operand size to trigger signed overflow
        // 0x7FFFFFFF + 1 = 0x80000000 (positive + positive = negative in 32-bit)
        0xb8, 0xff, 0xff, 0xff, 0x7f, // MOV EAX, 0x7FFFFFFF (32-bit, zero-extends)
        0x83, 0xc0, 0x01, // ADD EAX, 1 (32-bit add, sets OF)
        0x48, 0x9c, // PUSHFQ
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let mut stack_val = [0u8; 8];
    stack_val = emu.maps.read_bytes(0x0FF8, stack_val.len()).try_into().unwrap();
    let pushed_flags = u64::from_le_bytes(stack_val);
    assert_ne!(pushed_flags & 0x800, 0, "OF should be set in pushed flags");
}

#[test]
fn test_popfq_restore_overflow() {
    let mut emu = emu64();
    let code = [
        // Use 32-bit operand size to trigger signed overflow
        0xb8, 0xff, 0xff, 0xff, 0x7f, // MOV EAX, 0x7FFFFFFF (32-bit)
        0x83, 0xc0, 0x01, // ADD EAX, 1 (32-bit add, sets OF)
        0x48, 0x9c, // PUSHFQ
        0x48, 0x31, 0xc0, // XOR RAX, RAX (clears OF)
        0x48, 0x9d, // POPFQ
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_of, "OF should be restored");
}

#[test]
fn test_pushfq_with_direction_flag() {
    let mut emu = emu64();
    let code = [
        0xfd, // STD (set direction flag)
        0x48, 0x9c, // PUSHFQ
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let mut stack_val = [0u8; 8];
    stack_val = emu.maps.read_bytes(0x0FF8, stack_val.len()).try_into().unwrap();
    let pushed_flags = u64::from_le_bytes(stack_val);
    assert_ne!(pushed_flags & 0x400, 0, "DF should be set in pushed flags");
}

#[test]
fn test_popfq_restore_direction_flag() {
    let mut emu = emu64();
    let code = [
        0xfd, // STD
        0x48, 0x9c, // PUSHFQ
        0xfc, // CLD (clear direction flag)
        0x48, 0x9d, // POPFQ
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_df, "DF should be restored");
}

#[test]
fn test_pushfq_popfq_with_all_status_flags() {
    let mut emu = emu64();
    let code = [
        // Set up complex flag state
        // Use 32-bit ADD to set OF (64-bit won't overflow with 0x7FFFFFFF+1)
        0xb8, 0xff, 0xff, 0xff, 0x7f, // MOV EAX, 0x7FFFFFFF
        0x83, 0xc0, 0x01, // ADD EAX, 1 (sets OF, clears CF)
        // Now set CF and DF (STC/STD only modify their specific flags)
        0xf9, // STC
        0xfd, // STD
        0x48, 0x9c, // PUSHFQ
        // Clear all flags
        0xf8, // CLC
        0xfc, // CLD
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        // Restore flags
        0x48, 0x9d, // POPFQ
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_cf, "CF should be restored");
    assert!(emu.flags().f_df, "DF should be restored");
    assert!(emu.flags().f_of, "OF should be restored");
}

#[test]
fn test_pushfq_after_comparison() {
    let mut emu = emu64();
    let code = [
        0x48, 0xc7, 0xc0, 0x0a, 0x00, 0x00, 0x00, // MOV RAX, 10
        0x48, 0xc7, 0xc3, 0x05, 0x00, 0x00, 0x00, // MOV RBX, 5
        0x48, 0x39, 0xd8, // CMP RAX, RBX (10 vs 5, clears CF, ZF, SF)
        0x48, 0x9c, // PUSHFQ
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let mut stack_val = [0u8; 8];
    stack_val = emu.maps.read_bytes(0x0FF8, stack_val.len()).try_into().unwrap();
    let pushed_flags = u64::from_le_bytes(stack_val);
    assert_eq!(pushed_flags & 0x01, 0, "CF should be clear (10 >= 5)");
    assert_eq!(pushed_flags & 0x40, 0, "ZF should be clear (10 != 5)");
    assert_eq!(pushed_flags & 0x80, 0, "SF should be clear (positive result)");
}

#[test]
fn test_popfq_after_comparison() {
    let mut emu = emu64();
    let code = [
        0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00, // MOV RAX, 5
        0x48, 0xc7, 0xc3, 0x0a, 0x00, 0x00, 0x00, // MOV RBX, 10
        0x48, 0x39, 0xd8, // CMP RAX, RBX (5 vs 10, sets CF)
        0x48, 0x9c, // PUSHFQ
        // Do another comparison
        0x48, 0x39, 0xc3, // CMP RBX, RAX (10 vs 5, clears CF)
        // Restore original comparison flags
        0x48, 0x9d, // POPFQ
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_cf, "CF should be restored from first comparison");
}

#[test]
fn test_pushfq_popfq_in_loop_simulation() {
    let mut emu = emu64();
    let code = [
        0xf9, // STC
        // Save flags
        0x48, 0x9c, // PUSHFQ
        // Clear flags
        0xf8, // CLC
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        // Restore flags
        0x48, 0x9d, // POPFQ
        // Save again
        0x48, 0x9c, // PUSHFQ
        // Clear again
        0xf8, // CLC
        // Restore again
        0x48, 0x9d, // POPFQ
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_cf, "CF should be preserved through multiple save/restore");
    assert_eq!(emu.regs().rsp, 0x1000, "Stack should be balanced");
}

#[test]
fn test_pushfq_with_parity_flag() {
    let mut emu = emu64();
    let code = [
        0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3 (0b11, even parity)
        0x48, 0x85, 0xc0, // TEST RAX, RAX (sets PF)
        0x48, 0x9c, // PUSHFQ
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let mut stack_val = [0u8; 8];
    stack_val = emu.maps.read_bytes(0x0FF8, stack_val.len()).try_into().unwrap();
    let pushed_flags = u64::from_le_bytes(stack_val);
    assert_ne!(pushed_flags & 0x04, 0, "PF should be set in pushed flags");
}

#[test]
fn test_popfq_restore_parity() {
    let mut emu = emu64();
    let code = [
        0x48, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x00, // MOV RAX, 3
        0x48, 0x85, 0xc0, // TEST RAX, RAX (sets PF)
        0x48, 0x9c, // PUSHFQ
        0x48, 0xc7, 0xc0, 0x07, 0x00, 0x00, 0x00, // MOV RAX, 7 (odd parity)
        0x48, 0x85, 0xc0, // TEST RAX, RAX (clears PF)
        0x48, 0x9d, // POPFQ
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_pf, "PF should be restored");
}

#[test]
fn test_pushfq_with_auxiliary_carry() {
    let mut emu = emu64();
    let code = [
        0x48, 0xc7, 0xc0, 0x0f, 0x00, 0x00, 0x00, // MOV RAX, 0x0F
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (sets AF)
        0x48, 0x9c, // PUSHFQ
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let mut stack_val = [0u8; 8];
    stack_val = emu.maps.read_bytes(0x0FF8, stack_val.len()).try_into().unwrap();
    let pushed_flags = u64::from_le_bytes(stack_val);
    assert_ne!(pushed_flags & 0x10, 0, "AF should be set in pushed flags");
}

#[test]
fn test_popfq_restore_auxiliary_carry() {
    let mut emu = emu64();
    let code = [
        0x48, 0xc7, 0xc0, 0x0f, 0x00, 0x00, 0x00, // MOV RAX, 0x0F
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (sets AF)
        0x48, 0x9c, // PUSHFQ
        0x48, 0x31, 0xc0, // XOR RAX, RAX (clears AF)
        0x48, 0x9d, // POPFQ
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_af, "AF should be restored");
}
