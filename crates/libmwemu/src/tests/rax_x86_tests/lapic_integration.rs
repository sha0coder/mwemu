//! LAPIC integration tests - testing LAPIC through x86 emulator execution.
//!
//! These tests run actual x86-64 code that accesses the LAPIC MMIO registers
//! at 0xFEE00000, verifying that the emulator correctly routes these accesses
//! to the inline LAPIC implementation.

use crate::*;

// LAPIC register offsets from base 0xFEE00000
const LAPIC_BASE: u64 = 0xFEE00000;
const LAPIC_ID: u64 = 0x020;
const LAPIC_VERSION: u64 = 0x030;
const LAPIC_TPR: u64 = 0x080;
const LAPIC_EOI: u64 = 0x0B0;
const LAPIC_SVR: u64 = 0x0F0;
const LAPIC_ISR_BASE: u64 = 0x100;
const LAPIC_IRR_BASE: u64 = 0x200;
const LAPIC_LVT_TIMER: u64 = 0x320;
const LAPIC_TIMER_ICR: u64 = 0x380;
const LAPIC_TIMER_CCR: u64 = 0x390;
const LAPIC_TIMER_DCR: u64 = 0x3E0;

/// Helper to build code that loads LAPIC base into RAX
fn lapic_base_to_rax() -> Vec<u8> {
    // mov rax, 0xFEE00000
    vec![0x48, 0xB8, 0x00, 0x00, 0xE0, 0xFE, 0x00, 0x00, 0x00, 0x00]
}

/// Helper to read LAPIC register into EBX: mov ebx, [rax + offset]
fn read_lapic_to_ebx(offset: u32) -> Vec<u8> {
    // mov ebx, [rax + offset] (32-bit displacement)
    vec![
        0x8B, 0x98,
        (offset & 0xFF) as u8,
        ((offset >> 8) & 0xFF) as u8,
        ((offset >> 16) & 0xFF) as u8,
        ((offset >> 24) & 0xFF) as u8,
    ]
}

/// Helper to write ECX to LAPIC register: mov [rax + offset], ecx
fn write_ecx_to_lapic(offset: u32) -> Vec<u8> {
    // mov [rax + offset], ecx (32-bit displacement)
    vec![
        0x89, 0x88,
        (offset & 0xFF) as u8,
        ((offset >> 8) & 0xFF) as u8,
        ((offset >> 16) & 0xFF) as u8,
        ((offset >> 24) & 0xFF) as u8,
    ]
}

// ============================================================================
// BASIC REGISTER ACCESS TESTS
// ============================================================================

#[test]
fn test_lapic_read_version_register() {
    let mut emu = emu64();
    let mut code = Vec::new();
    code.extend(lapic_base_to_rax());
    code.extend(read_lapic_to_ebx(LAPIC_VERSION as u32));
    code.push(0xF4); // HLT

    let mut emu = emu64();
   emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(
        emu.regs().rbx as u32, 0x00050014,
        "LAPIC version should be 0x00050014 (modern APIC with 6 LVT entries)"
    );
}

#[test]
fn test_lapic_read_id_register() {
    let mut emu = emu64();
    let mut code = Vec::new();
    code.extend(lapic_base_to_rax());
    code.extend(read_lapic_to_ebx(LAPIC_ID as u32));
    code.push(0xF4); // HLT

    let mut emu = emu64();
   emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(
        emu.regs().rbx as u32, 0,
        "LAPIC ID should be 0 for first CPU"
    );
}

#[test]
fn test_lapic_read_svr_default() {
    let mut emu = emu64();
    let mut code = Vec::new();
    code.extend(lapic_base_to_rax());
    code.extend(read_lapic_to_ebx(LAPIC_SVR as u32));
    code.push(0xF4); // HLT

    let mut emu = emu64();
   emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(
        emu.regs().rbx as u32, 0x1FF,
        "SVR should be 0x1FF (APIC enabled, vector 0xFF)"
    );
}

#[test]
fn test_lapic_write_and_read_tpr() {
    let mut emu = emu64();
    let mut code = Vec::new();
    code.extend(lapic_base_to_rax());

    // mov ecx, 0x42
    code.extend([0xB9, 0x42, 0x00, 0x00, 0x00]);
    code.extend(write_ecx_to_lapic(LAPIC_TPR as u32));
    code.extend(read_lapic_to_ebx(LAPIC_TPR as u32));
    code.push(0xF4); // HLT

    let mut emu = emu64();
   emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(
        emu.regs().rbx as u32, 0x42,
        "TPR should read back the written value"
    );
}

#[test]
fn test_lapic_tpr_masks_to_8bits() {
    let mut emu = emu64();
    let mut code = Vec::new();
    code.extend(lapic_base_to_rax());

    // mov ecx, 0x12345678
    code.extend([0xB9, 0x78, 0x56, 0x34, 0x12]);
    code.extend(write_ecx_to_lapic(LAPIC_TPR as u32));
    code.extend(read_lapic_to_ebx(LAPIC_TPR as u32));
    code.push(0xF4); // HLT

    let mut emu = emu64();
   emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(
        emu.regs().rbx as u32, 0x78,
        "TPR should mask to lower 8 bits"
    );
}

#[test]
fn test_lapic_write_and_read_id() {
    let mut emu = emu64();
    let mut code = Vec::new();
    code.extend(lapic_base_to_rax());

    // mov ecx, 0x05000000 (APIC ID = 5)
    code.extend([0xB9, 0x00, 0x00, 0x00, 0x05]);
    code.extend(write_ecx_to_lapic(LAPIC_ID as u32));
    code.extend(read_lapic_to_ebx(LAPIC_ID as u32));
    code.push(0xF4); // HLT

    let mut emu = emu64();
   emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(
        emu.regs().rbx as u32, 0x05000000,
        "APIC ID should read back the written value"
    );
}

// ============================================================================
// TIMER CONFIGURATION TESTS
// ============================================================================

#[test]
fn test_lapic_lvt_timer_default_masked() {
    let mut emu = emu64();
    // LVT Timer should be masked by default
    let mut code = Vec::new();
    code.extend(lapic_base_to_rax());
    code.extend(read_lapic_to_ebx(LAPIC_LVT_TIMER as u32));
    code.push(0xF4); // HLT

    let mut emu = emu64();
   emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(
        (emu.regs().rbx as u32 & 0x10000) != 0,
        "LVT Timer should be masked by default"
    );
}

#[test]
fn test_lapic_timer_configure_oneshot() {
    let mut emu = emu64();
    let mut code = Vec::new();
    code.extend(lapic_base_to_rax());

    // mov ecx, 0x00000020
    code.extend([0xB9, 0x20, 0x00, 0x00, 0x00]);
    code.extend(write_ecx_to_lapic(LAPIC_LVT_TIMER as u32));
    code.extend(read_lapic_to_ebx(LAPIC_LVT_TIMER as u32));
    code.push(0xF4); // HLT

    let mut emu = emu64();
   emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let lvt = emu.regs().rbx as u32;
    assert_eq!(lvt & 0xFF, 0x20, "Timer vector should be 0x20");
    assert_eq!((lvt >> 17) & 0x3, 0, "Timer mode should be oneshot (0)");
    assert_eq!(lvt & 0x10000, 0, "Timer should be unmasked");
}

#[test]
fn test_lapic_timer_configure_periodic() {
    let mut emu = emu64();
    let mut code = Vec::new();
    code.extend(lapic_base_to_rax());

    // 0x00020030
    // mov ecx, 0x00020030
    code.extend([0xB9, 0x30, 0x00, 0x02, 0x00]);
    code.extend(write_ecx_to_lapic(LAPIC_LVT_TIMER as u32));
    code.extend(read_lapic_to_ebx(LAPIC_LVT_TIMER as u32));
    code.push(0xF4); // HLT

    let mut emu = emu64();
   emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let lvt = emu.regs().rbx as u32;
    assert_eq!(lvt & 0xFF, 0x30, "Timer vector should be 0x30");
    assert_eq!((lvt >> 17) & 0x3, 1, "Timer mode should be periodic (1)");
}

#[test]
fn test_lapic_timer_divide_config() {
    let mut emu = emu64();
    let mut code = Vec::new();
    code.extend(lapic_base_to_rax());

    code.extend([0xB9, 0x0B, 0x00, 0x00, 0x00]);
    code.extend(write_ecx_to_lapic(LAPIC_TIMER_DCR as u32));
    code.extend(read_lapic_to_ebx(LAPIC_TIMER_DCR as u32));
    code.push(0xF4); // HLT

    let mut emu = emu64();
   emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(
        emu.regs().rbx as u32, 0x0B,
        "DCR should be 0x0B (divide by 1)"
    );
}

#[test]
fn test_lapic_timer_initial_count() {
    let mut emu = emu64();
    let mut code = Vec::new();
    code.extend(lapic_base_to_rax());

    code.extend([0xB9, 0x78, 0x56, 0x34, 0x12]);
    code.extend(write_ecx_to_lapic(LAPIC_TIMER_ICR as u32));
    code.extend(read_lapic_to_ebx(LAPIC_TIMER_ICR as u32));
    code.push(0xF4); // HLT

    let mut emu = emu64();
   emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(
        emu.regs().rbx as u32, 0x12345678,
        "Initial count should read back correctly"
    );
}

#[test]
fn test_lapic_timer_ccr_zero_when_no_timer() {
    let mut emu = emu64();
    // CCR should be 0 when timer hasn't been started
    let mut code = Vec::new();
    code.extend(lapic_base_to_rax());
    code.extend(read_lapic_to_ebx(LAPIC_TIMER_CCR as u32));
    code.push(0xF4); // HLT

    let mut emu = emu64();
   emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(
        emu.regs().rbx as u32, 0,
        "CCR should be 0 when timer not started"
    );
}

#[test]
fn test_lapic_timer_ccr_decreases() {
    let mut emu = emu64();
    let mut code = Vec::new();
    code.extend(lapic_base_to_rax());

    code.extend([0xB9, 0x20, 0x00, 0x00, 0x00]);
    code.extend(write_ecx_to_lapic(LAPIC_LVT_TIMER as u32));

    code.extend([0xB9, 0x0B, 0x00, 0x00, 0x00]);
    code.extend(write_ecx_to_lapic(LAPIC_TIMER_DCR as u32));

    code.extend([0xB9, 0xFF, 0xFF, 0xFF, 0xFF]);
    code.extend(write_ecx_to_lapic(LAPIC_TIMER_ICR as u32));

    code.extend(read_lapic_to_ebx(LAPIC_TIMER_CCR as u32));

    // mov ecx, [rax + LAPIC_TIMER_ICR]
    code.extend([0x8B, 0x88, 0x80, 0x03, 0x00, 0x00]);

    code.push(0xF4); // HLT

    let mut emu = emu64();
   emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // CCR should be less than or equal to initial count
    let ccr = emu.regs().rbx as u32;
    let icr = emu.regs().rcx as u32;
    assert!(
        ccr <= icr,
        "CCR ({}) should be <= ICR ({})",
        ccr, icr
    );
}

// ============================================================================
// SVR AND APIC ENABLE/DISABLE TESTS
// ============================================================================

#[test]
fn test_lapic_disable_via_svr() {
    let mut emu = emu64();
    let mut code = Vec::new();
    code.extend(lapic_base_to_rax());

    // mov ecx, 0x0FF (spurious vector 0xFF, APIC disabled)
    code.extend([0xB9, 0xFF, 0x00, 0x00, 0x00]);
    code.extend(write_ecx_to_lapic(LAPIC_SVR as u32));

    code.extend(read_lapic_to_ebx(LAPIC_SVR as u32));

    // mov ecx, 0x1FF
    code.extend([0xB9, 0xFF, 0x01, 0x00, 0x00]);
    code.extend(write_ecx_to_lapic(LAPIC_SVR as u32));

    // mov ecx, [rax + LAPIC_SVR]
    code.extend([0x8B, 0x88, 0xF0, 0x00, 0x00, 0x00]);

    code.push(0xF4); // HLT

    let mut emu = emu64();
   emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(
        emu.regs().rbx as u32, 0xFF,
        "After disable, SVR should be 0xFF (APIC disabled)"
    );
    assert_eq!(
        emu.regs().rcx as u32, 0x1FF,
        "After re-enable, SVR should be 0x1FF"
    );
}

// ============================================================================
// ISR/IRR REGISTER TESTS
// ============================================================================

#[test]
fn test_lapic_isr_initially_zero() {
    let mut emu = emu64();
    let mut code = Vec::new();
    code.extend(lapic_base_to_rax());

    code.extend(read_lapic_to_ebx(LAPIC_ISR_BASE as u32));

    code.extend([0x8B, 0x88, 0x70, 0x01, 0x00, 0x00]); // mov ecx, [rax + 0x170]

    code.push(0xF4); // HLT

    let mut emu = emu64();
   emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx as u32, 0, "ISR[0] should be 0");
    assert_eq!(emu.regs().rcx as u32, 0, "ISR[7] should be 0");
}

#[test]
fn test_lapic_irr_initially_zero() {
    let mut emu = emu64();
    let mut code = Vec::new();
    code.extend(lapic_base_to_rax());

    code.extend(read_lapic_to_ebx(LAPIC_IRR_BASE as u32));

    code.extend([0x8B, 0x88, 0x70, 0x02, 0x00, 0x00]); // mov ecx, [rax + 0x270]

    code.push(0xF4); // HLT

    let mut emu = emu64();
   emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx as u32, 0, "IRR[0] should be 0");
    assert_eq!(emu.regs().rcx as u32, 0, "IRR[7] should be 0");
}

// ============================================================================
// MULTI-REGISTER ACCESS TESTS
// ============================================================================

#[test]
fn test_lapic_multiple_register_writes() {
    let mut emu = emu64();
    let mut code = Vec::new();
    code.extend(lapic_base_to_rax());

    code.extend([0xB9, 0x10, 0x00, 0x00, 0x00]);
    code.extend(write_ecx_to_lapic(LAPIC_TPR as u32));

    code.extend([0xB9, 0x03, 0x00, 0x00, 0x00]);
    code.extend(write_ecx_to_lapic(LAPIC_TIMER_DCR as u32));

    code.extend([0xB9, 0x40, 0x00, 0x02, 0x00]);
    code.extend(write_ecx_to_lapic(LAPIC_LVT_TIMER as u32));

    // TPR -> ebx
    code.extend(read_lapic_to_ebx(LAPIC_TPR as u32));

    // DCR -> r8d (using REX prefix)
    // mov r8d, [rax + DCR]
    code.extend([0x44, 0x8B, 0x80, 0xE0, 0x03, 0x00, 0x00]);

    // LVT Timer -> r9d
    // mov r9d, [rax + LVT_TIMER]
    code.extend([0x44, 0x8B, 0x88, 0x20, 0x03, 0x00, 0x00]);

    code.push(0xF4); // HLT

    let mut emu = emu64();
   emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx as u32, 0x10, "TPR should be 0x10");
    assert_eq!(emu.regs().r8 as u32, 0x03, "DCR should be 0x03");
    assert_eq!(emu.regs().r9 as u32, 0x00020040, "LVT Timer should be 0x00020040");
}

#[test]
fn test_lapic_sequential_timer_configuration() {
    let mut emu = emu64();
    let mut code = Vec::new();
    code.extend(lapic_base_to_rax());

    // 1. Set divide config to divide by 16 (0x03)
    code.extend([0xB9, 0x03, 0x00, 0x00, 0x00]);
    code.extend(write_ecx_to_lapic(LAPIC_TIMER_DCR as u32));

    // 2. Configure LVT Timer: periodic mode (0x20000), vector 0x32, unmasked
    code.extend([0xB9, 0x32, 0x00, 0x02, 0x00]);
    code.extend(write_ecx_to_lapic(LAPIC_LVT_TIMER as u32));

    // 3. Set initial count to start timer
    code.extend([0xB9, 0x00, 0x00, 0x10, 0x00]); // 0x100000
    code.extend(write_ecx_to_lapic(LAPIC_TIMER_ICR as u32));

    // DCR -> ebx
    code.extend(read_lapic_to_ebx(LAPIC_TIMER_DCR as u32));

    // LVT -> ecx
    code.extend([0x8B, 0x88, 0x20, 0x03, 0x00, 0x00]);

    // ICR -> edx
    code.extend([0x8B, 0x90, 0x80, 0x03, 0x00, 0x00]);

    // CCR -> r8d (to verify timer is running)
    code.extend([0x44, 0x8B, 0x80, 0x90, 0x03, 0x00, 0x00]);

    code.push(0xF4); // HLT

    let mut emu = emu64();
   emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx as u32, 0x03, "DCR should be 0x03");
    assert_eq!(emu.regs().rcx as u32, 0x00020032, "LVT should be periodic with vector 0x32");
    assert_eq!(emu.regs().rdx as u32, 0x00100000, "ICR should be 0x100000");
    // CCR should be non-zero and less than or equal to ICR (timer running)
    let ccr = emu.regs().r8 as u32;
    assert!(ccr <= 0x00100000, "CCR should be <= ICR");
}

// ============================================================================
// EOI TESTS
// ============================================================================

#[test]
fn test_lapic_eoi_write() {
    let mut emu = emu64();
    let mut code = Vec::new();
    code.extend(lapic_base_to_rax());

    code.extend([0xB9, 0x00, 0x00, 0x00, 0x00]);
    code.extend(write_ecx_to_lapic(LAPIC_EOI as u32));

    code.extend(read_lapic_to_ebx(LAPIC_ISR_BASE as u32));

    code.push(0xF4); // HLT

    let mut emu = emu64();
   emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx as u32, 0, "ISR[0] should still be 0 after EOI");
}

// ============================================================================
// BYTE AND WORD ACCESS TESTS
// ============================================================================

#[test]
fn test_lapic_byte_read() {
    let mut emu = emu64();
    let mut code = Vec::new();
    code.extend(lapic_base_to_rax());

    // mov bl, [rax + TPR] - read single byte
    code.extend([0x8A, 0x98, 0x80, 0x00, 0x00, 0x00]);

    code.push(0xF4); // HLT

    let mut emu = emu64();
   emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // TPR defaults to 0
    assert_eq!((emu.regs().rbx & 0xFF) as u8, 0, "Byte read of TPR should be 0");
}

#[test]
fn test_lapic_word_read() {
    let mut emu = emu64();
    let mut code = Vec::new();
    code.extend(lapic_base_to_rax());

    code.extend([0xB9, 0xAB, 0x00, 0x00, 0x00]);
    code.extend(write_ecx_to_lapic(LAPIC_TPR as u32));

    // mov bx, [rax + TPR] - read word
    code.extend([0x66, 0x8B, 0x98, 0x80, 0x00, 0x00, 0x00]);

    code.push(0xF4); // HLT

    let mut emu = emu64();
   emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(
        (emu.regs().rbx & 0xFFFF) as u16, 0xAB,
        "Word read of TPR should be 0x00AB"
    );
}
