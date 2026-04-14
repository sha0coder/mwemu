use crate::*;

// VLDMXCSR - Load MXCSR Register
// VSTMXCSR - Store MXCSR Register
//
// VLDMXCSR loads the MXCSR register from a 32-bit memory location.
// VSTMXCSR stores the MXCSR register to a 32-bit memory location.
//
// MXCSR is the SSE control and status register that controls:
// - Rounding mode
// - Exception masks
// - Exception flags
// - Denormals-are-zero and flush-to-zero modes
//
// Opcodes:
// VEX.LZ.0F.WIG AE /2    VLDMXCSR m32    - Load MXCSR from m32
// VEX.LZ.0F.WIG AE /3    VSTMXCSR m32    - Store MXCSR to m32

const ALIGNED_ADDR: u64 = 0x3000; // 32-byte aligned address for testing

// ============================================================================
// VLDMXCSR Tests
// ============================================================================

#[test]
fn test_vldmxcsr_basic() {
    let mut emu = emu64();
    // VLDMXCSR [mem]
    let code = [
        0xc5, 0xf8, 0xae, 0x15, 0x00, 0x40, 0x00, 0x00, // VLDMXCSR [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let mxcsr: [u8; 4] = [0x80, 0x1f, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &mxcsr);

    emu.run(None).unwrap();
}

#[test]
fn test_vldmxcsr_round_to_zero() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf8, 0xae, 0x15, 0x00, 0x40, 0x00, 0x00, // VLDMXCSR [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    // MXCSR with round-to-zero: 0x7F80
    let mxcsr: [u8; 4] = [0x80, 0x7f, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &mxcsr);

    emu.run(None).unwrap();
}

#[test]
fn test_vldmxcsr_round_down() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf8, 0xae, 0x15, 0x00, 0x40, 0x00, 0x00, // VLDMXCSR [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    // MXCSR with round-down: 0x3F80
    let mxcsr: [u8; 4] = [0x80, 0x3f, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &mxcsr);

    emu.run(None).unwrap();
}

#[test]
fn test_vldmxcsr_round_up() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf8, 0xae, 0x15, 0x00, 0x40, 0x00, 0x00, // VLDMXCSR [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    // MXCSR with round-up: 0x5F80
    let mxcsr: [u8; 4] = [0x80, 0x5f, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &mxcsr);

    emu.run(None).unwrap();
}

#[test]
fn test_vldmxcsr_flush_to_zero() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf8, 0xae, 0x15, 0x00, 0x40, 0x00, 0x00, // VLDMXCSR [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    // MXCSR with FTZ: 0x9F80
    let mxcsr: [u8; 4] = [0x80, 0x9f, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &mxcsr);

    emu.run(None).unwrap();
}

#[test]
fn test_vldmxcsr_denormals_are_zero() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf8, 0xae, 0x15, 0x00, 0x40, 0x00, 0x00, // VLDMXCSR [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    // MXCSR with DAZ: 0x1FC0
    let mxcsr: [u8; 4] = [0xc0, 0x1f, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &mxcsr);

    emu.run(None).unwrap();
}

#[test]
fn test_vldmxcsr_all_exceptions_unmasked() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf8, 0xae, 0x15, 0x00, 0x40, 0x00, 0x00, // VLDMXCSR [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    // MXCSR with all exceptions unmasked: 0x1F00
    let mxcsr: [u8; 4] = [0x00, 0x1f, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &mxcsr);

    emu.run(None).unwrap();
}

#[test]
fn test_vldmxcsr_invalid_operation_unmasked() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf8, 0xae, 0x15, 0x00, 0x40, 0x00, 0x00, // VLDMXCSR [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    // MXCSR: 0x1F00 (invalid op unmasked)
    let mxcsr: [u8; 4] = [0x00, 0x1f, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &mxcsr);

    emu.run(None).unwrap();
}

#[test]
fn test_vldmxcsr_divide_by_zero_unmasked() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf8, 0xae, 0x15, 0x00, 0x40, 0x00, 0x00, // VLDMXCSR [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    // MXCSR: 0x1D80 (div-by-zero unmasked)
    let mxcsr: [u8; 4] = [0x80, 0x1d, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &mxcsr);

    emu.run(None).unwrap();
}

#[test]
fn test_vldmxcsr_overflow_unmasked() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf8, 0xae, 0x15, 0x00, 0x40, 0x00, 0x00, // VLDMXCSR [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    // MXCSR: 0x1B80 (overflow unmasked)
    let mxcsr: [u8; 4] = [0x80, 0x1b, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &mxcsr);

    emu.run(None).unwrap();
}

#[test]
fn test_vldmxcsr_underflow_unmasked() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf8, 0xae, 0x15, 0x00, 0x40, 0x00, 0x00, // VLDMXCSR [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    // MXCSR: 0x1780 (underflow unmasked)
    let mxcsr: [u8; 4] = [0x80, 0x17, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &mxcsr);

    emu.run(None).unwrap();
}

#[test]
fn test_vldmxcsr_precision_unmasked() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf8, 0xae, 0x15, 0x00, 0x40, 0x00, 0x00, // VLDMXCSR [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    // MXCSR: 0x0F80 (precision unmasked)
    let mxcsr: [u8; 4] = [0x80, 0x0f, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &mxcsr);

    emu.run(None).unwrap();
}

#[test]
fn test_vldmxcsr_multiple_loads() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf8, 0xae, 0x15, 0x00, 0x40, 0x00, 0x00, // VLDMXCSR [rip + 0x4000]
        0xc5, 0xf8, 0xae, 0x15, 0x04, 0x40, 0x00, 0x00, // VLDMXCSR [rip + 0x4004]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let mxcsr1: [u8; 4] = [0x80, 0x1f, 0x00, 0x00];
    let mxcsr2: [u8; 4] = [0x80, 0x7f, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &mxcsr1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 4, &mxcsr2);

    emu.run(None).unwrap();
}

#[test]
fn test_vldmxcsr_before_arithmetic() {
    let mut emu = emu64();
    // VLDMXCSR followed by arithmetic operation
    let code = [
        0xc5, 0xf8, 0xae, 0x15, 0x00, 0x40, 0x00, 0x00, // VLDMXCSR [rip + 0x4000]
        0xc5, 0xf0, 0x58, 0xc2, // VADDPS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let mxcsr: [u8; 4] = [0x80, 0x1f, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &mxcsr);

    emu.run(None).unwrap();
}

#[test]
fn test_vldmxcsr_rax_indirect() {
    let mut emu = emu64();
    // VLDMXCSR [rax]
    let code = [
        0x48, 0xb8, 0x00, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0x3000
        0xc5, 0xf8, 0xae, 0x10, // VLDMXCSR [rax]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let mxcsr: [u8; 4] = [0x80, 0x1f, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &mxcsr);

    emu.run(None).unwrap();
}

#[test]
fn test_vldmxcsr_rbx_indirect() {
    let mut emu = emu64();
    // VLDMXCSR [rbx]
    let code = [
        0x48, 0xbb, 0x00, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0x3000
        0xc5, 0xf8, 0xae, 0x13, // VLDMXCSR [rbx]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let mxcsr: [u8; 4] = [0x80, 0x1f, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &mxcsr);

    emu.run(None).unwrap();
}

#[test]
fn test_vldmxcsr_rcx_indirect() {
    let mut emu = emu64();
    // VLDMXCSR [rcx]
    let code = [
        0x48, 0xb9, 0x00, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0x3000
        0xc5, 0xf8, 0xae, 0x11, // VLDMXCSR [rcx]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let mxcsr: [u8; 4] = [0x80, 0x1f, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &mxcsr);

    emu.run(None).unwrap();
}

#[test]
fn test_vldmxcsr_offset() {
    let mut emu = emu64();
    // VLDMXCSR [rax + offset]
    let code = [
        0x48, 0xb8, 0xf0, 0x2f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0x2FF0
        0xc5, 0xf8, 0xae, 0x50, 0x10, // VLDMXCSR [rax + 0x10]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let mxcsr: [u8; 4] = [0x80, 0x1f, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &mxcsr);

    emu.run(None).unwrap();
}

// ============================================================================
// VSTMXCSR Tests
// ============================================================================

#[test]
fn test_vstmxcsr_basic() {
    let mut emu = emu64();
    // VSTMXCSR [mem]
    let code = [
        0xc5, 0xf8, 0xae, 0x1d, 0x00, 0x40, 0x00, 0x00, // VSTMXCSR [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vstmxcsr_after_load() {
    let mut emu = emu64();
    // VLDMXCSR followed by VSTMXCSR
    let code = [
        0xc5, 0xf8, 0xae, 0x15, 0x00, 0x40, 0x00, 0x00, // VLDMXCSR [rip + 0x4000]
        0xc5, 0xf8, 0xae, 0x1d, 0x04, 0x40, 0x00, 0x00, // VSTMXCSR [rip + 0x4004]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let mxcsr: [u8; 4] = [0x80, 0x7f, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &mxcsr);

    emu.run(None).unwrap();
}

#[test]
fn test_vstmxcsr_multiple_stores() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf8, 0xae, 0x1d, 0x00, 0x40, 0x00, 0x00, // VSTMXCSR [rip + 0x4000]
        0xc5, 0xf8, 0xae, 0x1d, 0x04, 0x40, 0x00, 0x00, // VSTMXCSR [rip + 0x4004]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vstmxcsr_after_arithmetic() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf0, 0x58, 0xc2, // VADDPS XMM0, XMM1, XMM2
        0xc5, 0xf8, 0xae, 0x1d, 0x00, 0x40, 0x00, 0x00, // VSTMXCSR [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vstmxcsr_rax_indirect() {
    let mut emu = emu64();
    // VSTMXCSR [rax]
    let code = [
        0x48, 0xb8, 0x00, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0x3000
        0xc5, 0xf8, 0xae, 0x18, // VSTMXCSR [rax]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vstmxcsr_rbx_indirect() {
    let mut emu = emu64();
    // VSTMXCSR [rbx]
    let code = [
        0x48, 0xbb, 0x00, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0x3000
        0xc5, 0xf8, 0xae, 0x1b, // VSTMXCSR [rbx]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vstmxcsr_rcx_indirect() {
    let mut emu = emu64();
    // VSTMXCSR [rcx]
    let code = [
        0x48, 0xb9, 0x00, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0x3000
        0xc5, 0xf8, 0xae, 0x19, // VSTMXCSR [rcx]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vstmxcsr_offset() {
    let mut emu = emu64();
    // VSTMXCSR [rax + offset]
    let code = [
        0x48, 0xb8, 0xf0, 0x2f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0x2FF0
        0xc5, 0xf8, 0xae, 0x58, 0x10, // VSTMXCSR [rax + 0x10]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VLDMXCSR/VSTMXCSR Combined Tests
// ============================================================================

#[test]
fn test_ldmxcsr_stmxcsr_roundtrip() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf8, 0xae, 0x15, 0x00, 0x40, 0x00, 0x00, // VLDMXCSR [rip + 0x4000]
        0xc5, 0xf8, 0xae, 0x1d, 0x04, 0x40, 0x00, 0x00, // VSTMXCSR [rip + 0x4004]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let mxcsr: [u8; 4] = [0x80, 0x7f, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &mxcsr);

    emu.run(None).unwrap();
}

#[test]
fn test_ldmxcsr_arithmetic_stmxcsr() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf8, 0xae, 0x15, 0x00, 0x40, 0x00, 0x00, // VLDMXCSR [rip + 0x4000]
        0xc5, 0xf0, 0x58, 0xc2, // VADDPS XMM0, XMM1, XMM2
        0xc5, 0xf8, 0xae, 0x1d, 0x04, 0x40, 0x00, 0x00, // VSTMXCSR [rip + 0x4004]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let mxcsr: [u8; 4] = [0x80, 0x1f, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &mxcsr);

    emu.run(None).unwrap();
}

#[test]
fn test_multiple_ldmxcsr_stmxcsr_pairs() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf8, 0xae, 0x15, 0x00, 0x40, 0x00, 0x00, // VLDMXCSR [rip + 0x4000]
        0xc5, 0xf8, 0xae, 0x1d, 0x08, 0x40, 0x00, 0x00, // VSTMXCSR [rip + 0x4008]
        0xc5, 0xf8, 0xae, 0x15, 0x04, 0x40, 0x00, 0x00, // VLDMXCSR [rip + 0x4004]
        0xc5, 0xf8, 0xae, 0x1d, 0x0c, 0x40, 0x00, 0x00, // VSTMXCSR [rip + 0x400C]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let mxcsr1: [u8; 4] = [0x80, 0x1f, 0x00, 0x00];
    let mxcsr2: [u8; 4] = [0x80, 0x7f, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &mxcsr1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 4, &mxcsr2);

    emu.run(None).unwrap();
}

#[test]
fn test_ldmxcsr_save_restore_pattern() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf8, 0xae, 0x1d, 0x00, 0x40, 0x00, 0x00, // VSTMXCSR [rip + 0x4000] (save)
        0xc5, 0xf8, 0xae, 0x15, 0x04, 0x40, 0x00, 0x00, // VLDMXCSR [rip + 0x4004] (load new)
        0xc5, 0xf0, 0x58, 0xc2, // VADDPS XMM0, XMM1, XMM2 (use new mode)
        0xc5, 0xf8, 0xae, 0x15, 0x00, 0x40, 0x00, 0x00, // VLDMXCSR [rip + 0x4000] (restore)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let mxcsr_new: [u8; 4] = [0x80, 0x7f, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 4, &mxcsr_new);

    emu.run(None).unwrap();
}

#[test]
fn test_vstmxcsr_after_division() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf0, 0x5e, 0xc2, // VDIVPS XMM0, XMM1, XMM2
        0xc5, 0xf8, 0xae, 0x1d, 0x00, 0x40, 0x00, 0x00, // VSTMXCSR [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vstmxcsr_after_sqrt() {
    let mut emu = emu64();
    // VSQRTPS followed by VSTMXCSR
    let code = [
        0xc5, 0xf8, 0x51, 0xc1, // VSQRTPS XMM0, XMM1
        0xc5, 0xf8, 0xae, 0x1d, 0x00, 0x40, 0x00, 0x00, // VSTMXCSR [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_ldmxcsr_context_switch() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf8, 0xae, 0x1d, 0x00, 0x40, 0x00, 0x00, // VSTMXCSR [rip + 0x4000] (save context 1)
        0xc5, 0xf8, 0xae, 0x15, 0x04, 0x40, 0x00, 0x00, // VLDMXCSR [rip + 0x4004] (load context 2)
        0xc5, 0xf0, 0x58, 0xc2, // VADDPS XMM0, XMM1, XMM2 (in context 2)
        0xc5, 0xf8, 0xae, 0x1d, 0x04, 0x40, 0x00, 0x00, // VSTMXCSR [rip + 0x4004] (save context 2)
        0xc5, 0xf8, 0xae, 0x15, 0x00, 0x40, 0x00, 0x00, // VLDMXCSR [rip + 0x4000] (restore context 1)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let mxcsr2: [u8; 4] = [0x80, 0x7f, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 4, &mxcsr2);

    emu.run(None).unwrap();
}
