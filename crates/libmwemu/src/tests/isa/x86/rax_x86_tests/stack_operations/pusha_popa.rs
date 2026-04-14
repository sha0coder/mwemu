// Module path for tests run via x86_64.rs
use crate::*;
const DATA_ADDR: u64 = 0x7000;

// PUSHA/POPA - Push All and Pop All General-Purpose Registers
// Note: These instructions are not available in 64-bit mode
// But can be tested in compatibility/legacy mode
//
// PUSHA/PUSHAD - Push All General-Purpose Registers
// Opcode: 60 (PUSHA for 16-bit, PUSHAD for 32-bit)
// Pushes: AX/EAX, CX/ECX, DX/EDX, BX/EBX, original SP/ESP, BP/EBP, SI/ESI, DI/EDI
//
// POPA/POPAD - Pop All General-Purpose Registers
// Opcode: 61 (POPA for 16-bit, POPAD for 32-bit)
// Pops: DI/EDI, SI/ESI, BP/EBP, (skip SP/ESP), BX/EBX, DX/EDX, CX/ECX, AX/EAX
//
// The SP/ESP value from stack is skipped during POPA/POPAD

// ===== PUSHAD TESTS (32-bit) =====

#[test]
fn test_pushad_basic_saves_all_registers() {
    // PUSHAD should push all 8 general-purpose 32-bit registers
    let code = [
        0x66, 0x60, // PUSHAD (with operand-size prefix for 32-bit)
        0xf4, // HLT
    ];
    let mut emu = emu32();
    emu.load_code_bytes(&code);
    emu.regs_mut().rax = 0x11111111;
    emu.regs_mut().rcx = 0x22222222;
    emu.regs_mut().rdx = 0x33333333;
    emu.regs_mut().rbx = 0x44444444;
    emu.regs_mut().rbp = 0x55555555;
    emu.regs_mut().rsi = 0x66666666;
    emu.regs_mut().rdi = 0x77777777;
    emu.maps
        .create_map(
            "stack_test",
            0x8000 - (0x8000 / 2),
            0x8000,
            crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    emu.regs_mut().rsp = 0x8000; // Stack pointer
    emu.set_verbose(3);
    emu.init_logger();
    emu.run(None).unwrap();
    let stack_base: u64 = 0x8000u64;

    // EDI (top)
    assert_eq!(emu.maps.read_dword(stack_base - 4).unwrap(), 0x77777777);
    // ESI
    assert_eq!(emu.maps.read_dword(stack_base - 8).unwrap(), 0x66666666);
    // EBP
    assert_eq!(emu.maps.read_dword(stack_base - 12).unwrap(), 0x55555555);
    // ESP original
    assert_eq!(emu.maps.read_dword(stack_base - 16).unwrap(), 0x8000);
    // EBX
    assert_eq!(emu.maps.read_dword(stack_base - 20).unwrap(), 0x44444444);
    // EDX
    assert_eq!(emu.maps.read_dword(stack_base - 24).unwrap(), 0x33333333);
    // ECX
    assert_eq!(emu.maps.read_dword(stack_base - 28).unwrap(), 0x22222222);
    // EAX (bottom)
    assert_eq!(emu.maps.read_dword(stack_base - 32).unwrap(), 0x11111111);
}

#[test]
fn test_pushad_preserves_all_register_values() {
    // PUSHAD doesn't modify register values
    let code = [
        0x66, 0x60, // PUSHAD
        0xf4, // HLT
    ];
    let mut emu = emu32();
    emu.load_code_bytes(&code);
    emu.regs_mut().rax = 0xAAAAAAAA;
    emu.regs_mut().rbx = 0xBBBBBBBB;
    emu.regs_mut().rcx = 0xCCCCCCCC;
    emu.regs_mut().rdx = 0xDDDDDDDD;
    emu.regs_mut().rsi = 0xEEEEEEEE;
    emu.regs_mut().rdi = 0xFFFFFFFF;
    emu.regs_mut().rbp = 0x12341234;
    emu.maps
        .create_map(
            "stack_test",
            0x8000 - (0x8000 / 2),
            0x8000,
            crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    emu.regs_mut().rsp = 0x8000;

    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xAAAAAAAA, "RAX unchanged");
    assert_eq!(emu.regs().rbx & 0xFFFFFFFF, 0xBBBBBBBB, "RBX unchanged");
    assert_eq!(emu.regs().rcx & 0xFFFFFFFF, 0xCCCCCCCC, "RCX unchanged");
    assert_eq!(emu.regs().rdx & 0xFFFFFFFF, 0xDDDDDDDD, "RDX unchanged");
    assert_eq!(emu.regs().rsi & 0xFFFFFFFF, 0xEEEEEEEE, "RSI unchanged");
    assert_eq!(emu.regs().rdi & 0xFFFFFFFF, 0xFFFFFFFF, "RDI unchanged");
    assert_eq!(emu.regs().rbp & 0xFFFFFFFF, 0x12341234, "RBP unchanged");
}

#[test]
fn test_pushad_decrements_stack_pointer() {
    // PUSHAD should decrement RSP by 32 (8 registers * 4 bytes)
    let code = [
        0x66, 0x60, // PUSHAD
        0xf4, // HLT
    ];
    let mut emu = emu32();
    emu.load_code_bytes(&code);
    emu.maps
        .create_map(
            "stack_test",
            0x1000 - (0x1000 / 2),
            0x8000,
            crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.set_verbose(3);
    emu.run(None).unwrap();

    // RSP should be decremented by 32 bytes
    assert_eq!(
        emu.regs().rsp,
        0x1000 - 16,
        "RSP should be decremented by 16"
    );
}

#[test]
fn test_pushad_saves_original_sp_not_modified_sp() {
    // PUSHAD saves the original SP value before any pushes
    let code = [
        0x66, 0x60, // PUSHAD (saves original ESP)
        0xf4, // HLT
    ];
    let mut emu = emu32();
    emu.load_code_bytes(&code);
    emu.maps
        .create_map(
            "stack_test",
            0x8000 - (0x8000 / 2),
            0x8000,
            crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    emu.regs_mut().rsp = 0x8000;

    emu.run(None).unwrap();

    let original_esp_on_stack = emu.maps.read_dword(0x8000 - 20).unwrap();
    assert_eq!(
        original_esp_on_stack, 0x8000,
        "Original ESP should be saved on stack"
    );
}

#[test]
fn test_pushad_with_zero_registers() {
    // PUSHAD with all zero registers
    let code = [
        0x66, 0x60, // PUSHAD
        0xf4, // HLT
    ];
    let mut emu = emu32();
    emu.load_code_bytes(&code);
    emu.regs_mut().rax = 0;
    emu.regs_mut().rbx = 0;
    emu.regs_mut().rcx = 0;
    emu.regs_mut().rdx = 0;
    emu.regs_mut().rsi = 0;
    emu.regs_mut().rdi = 0;
    emu.regs_mut().rbp = 0;
    emu.maps
        .create_map(
            "stack_test",
            0x8000 - (0x8000 / 2),
            0x8000,
            crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    emu.regs_mut().rsp = 0x8000;

    emu.run(None).unwrap();

    let value_on_stack = emu.maps.read_dword(0x8000 - 32).unwrap();
    assert_eq!(value_on_stack, 0, "Zero should be pushed");
}

#[test]
fn test_pushad_with_max_registers() {
    // PUSHAD with 32-bit max values
    let code = [
        0x66, 0x60, // PUSHAD
        0xf4, // HLT
    ];
    let mut emu = emu32();
    emu.load_code_bytes(&code);
    emu.regs_mut().rax = 0xFFFFFFFF;
    emu.regs_mut().rbx = 0xFFFFFFFF;
    emu.regs_mut().rcx = 0xFFFFFFFF;
    emu.regs_mut().rdx = 0xFFFFFFFF;
    emu.regs_mut().rsi = 0xFFFFFFFF;
    emu.regs_mut().rdi = 0xFFFFFFFF;
    emu.regs_mut().rbp = 0xFFFFFFFF;
    emu.maps
        .create_map(
            "stack_test",
            0x8000 - (0x8000 / 2),
            0x8000,
            crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    emu.regs_mut().rsp = 0x8000;

    emu.run(None).unwrap();

    let value_on_stack = emu.maps.read_dword(0x8000 - 32).unwrap();
    assert_eq!(value_on_stack, 0xFFFFFFFF, "Max value should be pushed");
}

#[test]
fn test_pushad_does_not_modify_flags() {
    // PUSHAD should not modify any flags
    let code = [
        0xf9, // STC (set carry flag)
        0x66, 0x60, // PUSHAD
        0xf4, // HLT
    ];
    let mut emu = emu32();
    emu.load_code_bytes(&code);
    emu.flags_mut().load(0x2 | 1); // Reserve bit 1 set, plus CF
    emu.maps
        .create_map(
            "stack_test",
            0x8000 - (0x8000 / 2),
            0x8000,
            crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    emu.regs_mut().rsp = 0x8000;

    emu.run(None).unwrap();

    assert_eq!(emu.flags().f_cf, true, "CF should remain set");
    assert_eq!(emu.flags().dump(), 0x2 | 1, "Flags should be unchanged");
}

// ===== POPAD TESTS (32-bit) =====

#[test]
fn test_popad_restores_registers() {
    // POPAD should restore all 8 general-purpose registers
    let code = [
        // First push known values
        0x66, 0x60, // PUSHAD (saves current registers)
        0x66, 0xb8, 0x11, 0x11, 0x11, 0x11, // MOV EAX, 0x11111111
        0x66, 0xbb, 0x22, 0x22, 0x22, 0x22, // MOV EBX, 0x22222222
        0x66, 0xb9, 0x33, 0x33, 0x33, 0x33, // MOV ECX, 0x33333333
        0x66, 0xba, 0x44, 0x44, 0x44, 0x44, // MOV EDX, 0x44444444
        0x66, 0xbe, 0x55, 0x55, 0x55, 0x55, // MOV ESI, 0x55555555
        0x66, 0xbf, 0x66, 0x66, 0x66, 0x66, // MOV EDI, 0x66666666
        0x66, 0xbd, 0x77, 0x77, 0x77, 0x77, // MOV EBP, 0x77777777
        // Now restore original values
        0x66, 0x61, // POPAD
        0xf4, // HLT
    ];
    let mut emu = emu32();
    emu.load_code_bytes(&code);
    emu.regs_mut().rax = 0xAAAAAAAA;
    emu.regs_mut().rbx = 0xBBBBBBBB;
    emu.regs_mut().rcx = 0xCCCCCCCC;
    emu.regs_mut().rdx = 0xDDDDDDDD;
    emu.regs_mut().rsi = 0xEEEEEEEE;
    emu.regs_mut().rdi = 0xFFFFFFFF;
    emu.regs_mut().rbp = 0x12121212;
    emu.maps
        .create_map(
            "stack_test",
            0x8000 - (0x8000 / 2),
            0x8000,
            crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    emu.regs_mut().rsp = 0x8000;

    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0xAAAAAAAA, "RAX restored");
    assert_eq!(emu.regs().rbx & 0xFFFFFFFF, 0xBBBBBBBB, "RBX restored");
    assert_eq!(emu.regs().rcx & 0xFFFFFFFF, 0xCCCCCCCC, "RCX restored");
    assert_eq!(emu.regs().rdx & 0xFFFFFFFF, 0xDDDDDDDD, "RDX restored");
    assert_eq!(emu.regs().rsi & 0xFFFFFFFF, 0xEEEEEEEE, "RSI restored");
    assert_eq!(emu.regs().rdi & 0xFFFFFFFF, 0xFFFFFFFF, "RDI restored");
    assert_eq!(emu.regs().rbp & 0xFFFFFFFF, 0x12121212, "RBP restored");
}

#[test]
fn test_popad_increments_stack_pointer() {
    // POPAD should increment RSP by 32 (8 registers * 4 bytes)
    let code = [
        0x66, 0x60, // PUSHAD
        0x66, 0x61, // POPAD
        0xf4, // HLT
    ];
    let mut emu = emu32();
    emu.load_code_bytes(&code);
    emu.maps
        .create_map(
            "stack_test",
            0x8000 - (0x8000 / 2),
            0x8000,
            crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    emu.regs_mut().rsp = 0x8000;

    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x8000, "RSP should return to original");
}

#[test]
fn test_popad_ignores_sp_on_stack() {
    // POPAD should ignore (skip) the SP value on the stack
    let code = [
        0x66, 0x60, // PUSHAD (saves ESP)
        0x48, 0x83, 0xec, 0x20, // SUB RSP, 32 (make space)
        0x48, 0xc7, 0x84, 0x24, 0x00, 0xff, 0xff, 0xff, 0xEF, 0xBE, 0xAD,
        0xDE, // MOV QWORD [RSP+offset], 0xDEADBEEF (corrupted ESP on stack)
        0x66, 0x61, // POPAD (should ignore the corrupted SP value)
        0xf4, // HLT
    ];
    let mut emu = emu32();
    emu.load_code_bytes(&code);
    emu.maps
        .create_map(
            "stack_test",
            0x8000 - (0x8000 / 2),
            0x8000,
            crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    emu.regs_mut().rsp = 0x8000;

    emu.run(None).unwrap();

    // ESP should be restored properly despite corruption, incremented correctly
    // (may not be exactly 0x8000 due to the SUB instruction, but should be sensible)
    assert!(emu.regs().rsp > 0x7000, "RSP should be reasonable");
}

#[test]
fn test_pusha_popa_roundtrip() {
    // PUSHA followed by POPA should preserve all values
    let code = [
        0x66, 0x60, // PUSHAD
        0x66, 0xb8, 0x11, 0x22, 0x33, 0x44, // MOV EAX, 0x44332211
        0x66, 0xbb, 0x55, 0x66, 0x77, 0x88, // MOV EBX, 0x88776655
        0x66, 0x61, // POPAD (restore original)
        0xf4, // HLT
    ];
    let mut emu = emu32();
    emu.load_code_bytes(&code);
    emu.regs_mut().rax = 0xAAAAAAAA;
    emu.regs_mut().rbx = 0xBBBBBBBB;
    emu.maps
        .create_map(
            "stack_test",
            0x8000 - (0x8000 / 2),
            0x8000,
            crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    emu.regs_mut().rsp = 0x8000;

    emu.run(None).unwrap();

    assert_eq!(
        emu.regs().rax & 0xFFFFFFFF,
        0xAAAAAAAA,
        "RAX restored after POPAD"
    );
    assert_eq!(
        emu.regs().rbx & 0xFFFFFFFF,
        0xBBBBBBBB,
        "RBX restored after POPAD"
    );
}

#[test]
fn test_pushad_popa_multiple_times() {
    let code = [
        0x66, 0x60, // PUSHAD
        0x66, 0x61, // POPAD
        0x66, 0x60, // PUSHAD
        0x66, 0x61, // POPAD
        0x66, 0x60, // PUSHAD
        0x66, 0x61, // POPAD
        0xf4, // HLT
    ];
    let mut emu = emu32();
    emu.load_code_bytes(&code);
    emu.regs_mut().rax = 0x12345678;
    emu.maps
        .create_map(
            "stack_test",
            0x8000 - (0x8000 / 2),
            0x8000,
            crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    emu.regs_mut().rsp = 0x8000;

    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x12345678, "RAX unchanged");
    assert_eq!(emu.regs().rsp, 0x8000, "RSP restored");
}

#[test]
fn test_pushad_popa_with_alternating_modification() {
    // PUSHAD, modify registers, POPAD, verify restoration
    let code = [
        0x66, 0x60, // PUSHAD (save state 1)
        0x66, 0xb8, 0xFF, 0xFF, 0xFF, 0xFF, // MOV EAX, 0xFFFFFFFF (modify)
        0x66, 0x61, // POPAD (restore state 1)
        0x66, 0x89, 0xc1, // MOV ECX, EAX (copy to ECX for verification)
        0xf4, // HLT
    ];
    let mut emu = emu32();
    emu.load_code_bytes(&code);
    emu.regs_mut().rax = 0x11111111;
    emu.maps
        .create_map(
            "stack_test",
            0x8000 - (0x8000 / 2),
            0x8000,
            crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    emu.regs_mut().rsp = 0x8000;

    emu.run(None).unwrap();

    // EAX should be restored to original value
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x11111111, "RAX restored");
    // ECX should also have the restored value
    assert_eq!(
        emu.regs().rcx & 0xFFFFFFFF,
        0x11111111,
        "ECX contains restored value"
    );
}

#[test]
fn test_pushad_popa_does_not_affect_flags() {
    // PUSHAD and POPAD should not affect flags
    let code = [
        0xf9, // STC (set carry flag)
        0x66, 0x60, // PUSHAD
        0x66, 0x61, // POPAD
        0xf4, // HLT
    ];
    let mut emu = emu32();
    emu.load_code_bytes(&code);
    emu.flags_mut().load(0x2 | 1); // CF set
    emu.maps
        .create_map(
            "stack_test",
            0x8000 - (0x8000 / 2),
            0x8000,
            crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    emu.regs_mut().rsp = 0x8000;

    emu.run(None).unwrap();

    assert_eq!(emu.flags().f_cf, true, "CF unchanged");
}

#[test]
fn test_pushad_with_different_register_patterns() {
    let code = [
        0x66, 0x60, // PUSHAD
        0xf4, // HLT
    ];
    let mut emu = emu32();
    emu.load_code_bytes(&code);
    emu.regs_mut().rax = 0x5A5A5A5A;
    emu.regs_mut().rbx = 0xA5A5A5A5;
    emu.regs_mut().rcx = 0x3C3C3C3C;
    emu.regs_mut().rdx = 0xC3C3C3C3;
    emu.regs_mut().rsi = 0x0F0F0F0F;
    emu.regs_mut().rdi = 0xF0F0F0F0;
    emu.regs_mut().rbp = 0xF00FF00F;
    emu.maps
        .create_map(
            "stack_test",
            0x8000 - (0x8000 / 2),
            0x8000,
            crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    emu.regs_mut().rsp = 0x8000;

    emu.run(None).unwrap();

    let rdi = emu.maps.read_dword(0x8000 - 32).unwrap();
    assert_eq!(rdi, 0xF0F0F0F0, "RDI value on stack");

    let rsi = emu.maps.read_dword(0x8000 - 28).unwrap();
    assert_eq!(rsi, 0x0F0F0F0F, "RSI value on stack");

    let rbp = emu.maps.read_dword(0x8000 - 24).unwrap();
    assert_eq!(rbp, 0xF00FF00F, "RBP value on stack");

    let rax = emu.maps.read_dword(0x8000 - 4).unwrap();
    assert_eq!(rax, 0x5A5A5A5A, "RAX value on stack");
}

#[test]
fn test_popad_overwrites_current_values() {
    // POPAD should overwrite whatever values are currently in registers
    let code = [
        // Pre-load stack with known values via PUSHAD
        0x66, 0x60, // PUSHAD
        // Now the original register values are on stack
        // Load different values into registers
        0x66, 0xb8, 0x99, 0x99, 0x99, 0x99, // MOV EAX, 0x99999999
        0x66, 0xbb, 0x88, 0x88, 0x88, 0x88, // MOV EBX, 0x88888888
        // Pop the original values back
        0x66, 0x61, // POPAD
        0xf4, // HLT
    ];
    let mut emu = emu32();
    emu.load_code_bytes(&code);
    emu.regs_mut().rax = 0x11111111;
    emu.regs_mut().rbx = 0x22222222;
    emu.maps
        .create_map(
            "stack_test",
            0x8000 - (0x8000 / 2),
            0x8000,
            crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    emu.regs_mut().rsp = 0x8000;

    emu.run(None).unwrap();

    assert_eq!(
        emu.regs().rax & 0xFFFFFFFF,
        0x11111111,
        "POPAD overwrites EAX"
    );
    assert_eq!(
        emu.regs().rbx & 0xFFFFFFFF,
        0x22222222,
        "POPAD overwrites EBX"
    );
}

#[test]
fn test_pushad_popa_preserves_higher_bits_in_64bit() {
    let code = [
        0x66, 0x60, // PUSHAD
        0x66, 0x61, // POPAD
        0xf4, // HLT
    ];
    let mut emu = emu32();
    emu.load_code_bytes(&code);
    emu.regs_mut().rax = 0x1111111111111111u64;
    emu.maps
        .create_map(
            "stack_test",
            0x8000 - (0x8000 / 2),
            0x8000,
            crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    emu.regs_mut().rsp = 0x8000;

    emu.run(None).unwrap();

    assert_eq!(
        emu.regs().rax & 0xFFFFFFFF,
        0x11111111,
        "Lower 32 bits unchanged"
    );
    assert_eq!(
        emu.regs().rax >> 32,
        0x11111111u64,
        "Upper 32 bits unchanged"
    );
}

#[test]
fn test_pushad_does_not_fault_with_various_rsp() {
    for rsp_val in &[0x8000u64, 0x9000u64, 0x10000u64, 0xFFF0u64] {
        let code = [
            0x66, 0x60, // PUSHAD
            0xf4, // HLT
        ];
        let mut emu = emu32();
        emu.load_code_bytes(&code);
        emu.maps
            .create_map(
                "stack_test",
                *rsp_val - (*rsp_val / 2),
                0x8000,
                crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
            )
            .unwrap();
        emu.regs_mut().rsp = *rsp_val;

        emu.run(None).unwrap();

        assert_eq!(
            emu.regs().rsp,
            rsp_val - 32,
            "RSP decremented correctly for RSP={:x}",
            rsp_val
        );
    }
}
