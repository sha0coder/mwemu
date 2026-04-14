use crate::*;

// MONITOR/MWAIT - Set Up Monitor Address / Monitor Wait
//
// MONITOR sets up a linear address range to be monitored by hardware
// and prepares the processor to enter an optimized state while waiting
// for an event.
//
// MWAIT causes the processor to enter an optimized state while waiting
// for a write to the address range set up by the MONITOR instruction.
//
// Opcodes:
//   0F 01 C8    MONITOR    - Set up monitor address
//   0F 01 C9    MWAIT      - Monitor wait
//
// These instructions require CPL = 0 (kernel mode) on most processors.

const ALIGNED_ADDR: u64 = 0x3000;

#[test]
fn test_monitor_basic() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x31, 0xc9, // XOR ECX, ECX
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0x01, 0xc8, // MONITOR
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_mwait_basic() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x31, 0xc9, // XOR ECX, ECX
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0x01, 0xc8, // MONITOR
        0x0f, 0x01, 0xc9, // MWAIT
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_monitor_with_rcx_hints() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xb9, 0x01, 0x00, 0x00, 0x00, // MOV ECX, 1
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0x01, 0xc8, // MONITOR
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_monitor_with_rdx_hints() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x31, 0xc9, // XOR ECX, ECX
        0xba, 0x01, 0x00, 0x00, 0x00, // MOV EDX, 1
        0x0f, 0x01, 0xc8, // MONITOR
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_mwait_with_c0_substate() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x31, 0xc9, // XOR ECX, ECX
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0x01, 0xc8, // MONITOR
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0 (C0 state)
        0x0f, 0x01, 0xc9, // MWAIT
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_mwait_with_c1_substate() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x31, 0xc9, // XOR ECX, ECX
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0x01, 0xc8, // MONITOR
        0xb9, 0x10, 0x00, 0x00, 0x00, // MOV ECX, 0x10 (C1 state)
        0x0f, 0x01, 0xc9, // MWAIT
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_mwait_with_interrupt_break() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x31, 0xc9, // XOR ECX, ECX
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0x01, 0xc8, // MONITOR
        0xb9, 0x01, 0x00, 0x00, 0x00, // MOV ECX, 1 (interrupt break enabled)
        0x0f, 0x01, 0xc9, // MWAIT
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_monitor_different_addresses() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x31, 0xc9, // XOR ECX, ECX
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0x01, 0xc8, // MONITOR
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_monitor_aligned_address() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR & !0x3f).to_le_bytes()); // 64-byte aligned
    full_code.extend_from_slice(&[
        0x31, 0xc9, // XOR ECX, ECX
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0x01, 0xc8, // MONITOR
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_monitor_unaligned_address() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR + 7).to_le_bytes()); // Unaligned
    full_code.extend_from_slice(&[
        0x31, 0xc9, // XOR ECX, ECX
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0x01, 0xc8, // MONITOR
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_monitor_sequential() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x31, 0xc9, // XOR ECX, ECX
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0x01, 0xc8, // MONITOR
        0x48, 0x05, 0x40, 0x00, 0x00, 0x00, // ADD RAX, 64
        0x0f, 0x01, 0xc8, // MONITOR
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_mwait_sequential() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x31, 0xc9, // XOR ECX, ECX
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0x01, 0xc8, // MONITOR
        0x0f, 0x01, 0xc9, // MWAIT
        0x0f, 0x01, 0xc8, // MONITOR
        0x0f, 0x01, 0xc9, // MWAIT
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_monitor_with_rbx() {
    let mut emu = emu64();
    let code = [
        0x48, 0xbb, // MOV RBX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0x89, 0xd8, // MOV RAX, RBX
        0x31, 0xc9, // XOR ECX, ECX
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0x01, 0xc8, // MONITOR
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_monitor_with_rcx() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb9, // MOV RCX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0x89, 0xc8, // MOV RAX, RCX
        0x31, 0xc9, // XOR ECX, ECX
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0x01, 0xc8, // MONITOR
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_monitor_with_rdx() {
    let mut emu = emu64();
    let code = [
        0x48, 0xba, // MOV RDX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0x89, 0xd0, // MOV RAX, RDX
        0x31, 0xc9, // XOR ECX, ECX
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0x01, 0xc8, // MONITOR
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_monitor_with_rsi() {
    let mut emu = emu64();
    let code = [
        0x48, 0xbe, // MOV RSI, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0x89, 0xf0, // MOV RAX, RSI
        0x31, 0xc9, // XOR ECX, ECX
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0x01, 0xc8, // MONITOR
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_monitor_with_rdi() {
    let mut emu = emu64();
    let code = [
        0x48, 0xbf, // MOV RDI, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0x89, 0xf8, // MOV RAX, RDI
        0x31, 0xc9, // XOR ECX, ECX
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0x01, 0xc8, // MONITOR
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_monitor_with_r8() {
    let mut emu = emu64();
    let code = [
        0x49, 0xb8, // MOV R8, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x4c, 0x89, 0xc0, // MOV RAX, R8
        0x31, 0xc9, // XOR ECX, ECX
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0x01, 0xc8, // MONITOR
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_monitor_with_r9() {
    let mut emu = emu64();
    let code = [
        0x49, 0xb9, // MOV R9, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x4c, 0x89, 0xc8, // MOV RAX, R9
        0x31, 0xc9, // XOR ECX, ECX
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0x01, 0xc8, // MONITOR
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_mwait_with_ecx_substate_0() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x31, 0xc9, // XOR ECX, ECX
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0x01, 0xc8, // MONITOR
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0
        0x0f, 0x01, 0xc9, // MWAIT
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_mwait_with_ecx_substate_1() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x31, 0xc9, // XOR ECX, ECX
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0x01, 0xc8, // MONITOR
        0xb9, 0x01, 0x00, 0x00, 0x00, // MOV ECX, 1
        0x0f, 0x01, 0xc9, // MWAIT
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_mwait_with_ecx_substate_2() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x31, 0xc9, // XOR ECX, ECX
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0x01, 0xc8, // MONITOR
        0xb9, 0x20, 0x00, 0x00, 0x00, // MOV ECX, 0x20
        0x0f, 0x01, 0xc9, // MWAIT
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_mwait_with_ecx_substate_3() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x31, 0xc9, // XOR ECX, ECX
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0x01, 0xc8, // MONITOR
        0xb9, 0x30, 0x00, 0x00, 0x00, // MOV ECX, 0x30
        0x0f, 0x01, 0xc9, // MWAIT
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_monitor_mwait_pattern_1() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x31, 0xc9, // XOR ECX, ECX
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0x01, 0xc8, // MONITOR
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0x01, 0xc9, // MWAIT
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_monitor_mwait_pattern_2() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0
        0xba, 0x00, 0x00, 0x00, 0x00, // MOV EDX, 0
        0x0f, 0x01, 0xc8, // MONITOR
        0xb9, 0x10, 0x00, 0x00, 0x00, // MOV ECX, 0x10
        0xba, 0x00, 0x00, 0x00, 0x00, // MOV EDX, 0
        0x0f, 0x01, 0xc9, // MWAIT
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_monitor_multiple_granularities() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xb9, 0x01, 0x00, 0x00, 0x00, // MOV ECX, 1
        0xba, 0x01, 0x00, 0x00, 0x00, // MOV EDX, 1
        0x0f, 0x01, 0xc8, // MONITOR
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_monitor_with_max_hints() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xb9, 0xff, 0xff, 0xff, 0xff, // MOV ECX, 0xFFFFFFFF
        0xba, 0xff, 0xff, 0xff, 0xff, // MOV EDX, 0xFFFFFFFF
        0x0f, 0x01, 0xc8, // MONITOR
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_mwait_with_max_hints() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x31, 0xc9, // XOR ECX, ECX
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0x01, 0xc8, // MONITOR
        0xb9, 0xff, 0xff, 0xff, 0xff, // MOV ECX, 0xFFFFFFFF
        0x0f, 0x01, 0xc9, // MWAIT
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_monitor_null_address() {
    let mut emu = emu64();
    let code = [
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        0x31, 0xc9, // XOR ECX, ECX
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0x01, 0xc8, // MONITOR
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_monitor_high_address() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, 0xff, 0xff, 0xff, 0x7f, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0x7FFFFFFF
        0x31, 0xc9, // XOR ECX, ECX
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0x01, 0xc8, // MONITOR
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_monitor_page_boundary() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0x1000 (page boundary)
        0x31, 0xc9, // XOR ECX, ECX
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0x01, 0xc8, // MONITOR
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_monitor_cache_line_boundary() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, 0x00, 0x00, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0x300000 (64-byte aligned)
        0x31, 0xc9, // XOR ECX, ECX
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0x01, 0xc8, // MONITOR
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_mwait_loop() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x31, 0xc9, // XOR ECX, ECX
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0x01, 0xc8, // MONITOR
        0x0f, 0x01, 0xc9, // MWAIT
        0x0f, 0x01, 0xc8, // MONITOR
        0x0f, 0x01, 0xc9, // MWAIT
        0x0f, 0x01, 0xc8, // MONITOR
        0x0f, 0x01, 0xc9, // MWAIT
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_monitor_different_registers_sequence() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x31, 0xc9, // XOR ECX, ECX
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0x01, 0xc8, // MONITOR
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&(ALIGNED_ADDR + 0x40).to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0x89, 0xd8, // MOV RAX, RBX
        0x0f, 0x01, 0xc8, // MONITOR
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_mwait_different_ecx_sequence() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x31, 0xc9, // XOR ECX, ECX
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0x01, 0xc8, // MONITOR
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0
        0x0f, 0x01, 0xc9, // MWAIT
        0x0f, 0x01, 0xc8, // MONITOR
        0xb9, 0x10, 0x00, 0x00, 0x00, // MOV ECX, 0x10
        0x0f, 0x01, 0xc9, // MWAIT
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_monitor_mwait_comprehensive() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0
        0xba, 0x00, 0x00, 0x00, 0x00, // MOV EDX, 0
        0x0f, 0x01, 0xc8, // MONITOR
        0xb9, 0x01, 0x00, 0x00, 0x00, // MOV ECX, 1
        0xba, 0x00, 0x00, 0x00, 0x00, // MOV EDX, 0
        0x0f, 0x01, 0xc9, // MWAIT
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_monitor_with_various_ecx() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xb9, 0x03, 0x00, 0x00, 0x00, // MOV ECX, 3
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0x01, 0xc8, // MONITOR
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_monitor_with_various_edx() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x31, 0xc9, // XOR ECX, ECX
        0xba, 0x03, 0x00, 0x00, 0x00, // MOV EDX, 3
        0x0f, 0x01, 0xc8, // MONITOR
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_mwait_with_various_ecx() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x31, 0xc9, // XOR ECX, ECX
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0x01, 0xc8, // MONITOR
        0xb9, 0x03, 0x00, 0x00, 0x00, // MOV ECX, 3
        0x0f, 0x01, 0xc9, // MWAIT
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_monitor_address_offset_pattern() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x31, 0xc9, // XOR ECX, ECX
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0x01, 0xc8, // MONITOR
        0x48, 0x83, 0xc0, 0x10, // ADD RAX, 16
        0x0f, 0x01, 0xc8, // MONITOR
        0x48, 0x83, 0xc0, 0x10, // ADD RAX, 16
        0x0f, 0x01, 0xc8, // MONITOR
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_mwait_state_transition_pattern() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x31, 0xc9, // XOR ECX, ECX
        0x31, 0xd2, // XOR EDX, EDX
        0x0f, 0x01, 0xc8, // MONITOR
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0 (C0)
        0x0f, 0x01, 0xc9, // MWAIT
        0x0f, 0x01, 0xc8, // MONITOR
        0xb9, 0x10, 0x00, 0x00, 0x00, // MOV ECX, 0x10 (C1)
        0x0f, 0x01, 0xc9, // MWAIT
        0x0f, 0x01, 0xc8, // MONITOR
        0xb9, 0x20, 0x00, 0x00, 0x00, // MOV ECX, 0x20 (C2)
        0x0f, 0x01, 0xc9, // MWAIT
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}
