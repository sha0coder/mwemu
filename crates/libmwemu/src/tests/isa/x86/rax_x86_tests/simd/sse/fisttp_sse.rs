use crate::*;

// FISTTP - Store Integer with Truncation (SSE3)
//
// Converts the floating-point value in the ST(0) register to a signed integer
// value using truncation (round toward zero) and stores the result in the
// destination operand.
//
// This is the SSE3 variant that operates on ST(0) and memory.
// Different from FPU FIST in that it always uses truncation.
//
// Opcode:
//   DF /1    FISTTP m16int    - Store ST(0) as word integer (truncate) and pop
//   DB /1    FISTTP m32int    - Store ST(0) as dword integer (truncate) and pop
//   DD /1    FISTTP m64int    - Store ST(0) as qword integer (truncate) and pop

const ALIGNED_ADDR: u64 = 0x3000;

#[test]
fn test_fisttp_m16_basic() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xdf, 0x08, // FISTTP word ptr [RAX]
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_fisttp_m32_basic() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xdb, 0x08, // FISTTP dword ptr [RAX]
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_fisttp_m64_basic() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xdd, 0x08, // FISTTP qword ptr [RAX]
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_fisttp_m16_positive() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xdf, 0x08, // FISTTP word ptr [RAX]
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_fisttp_m16_negative() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xdf, 0x08, // FISTTP word ptr [RAX]
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_fisttp_m32_positive() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xdb, 0x08, // FISTTP dword ptr [RAX]
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_fisttp_m32_negative() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xdb, 0x08, // FISTTP dword ptr [RAX]
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_fisttp_m64_positive() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xdd, 0x08, // FISTTP qword ptr [RAX]
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_fisttp_m64_negative() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xdd, 0x08, // FISTTP qword ptr [RAX]
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_fisttp_m16_truncate_positive() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xdf, 0x08, // FISTTP word ptr [RAX]
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_fisttp_m16_truncate_negative() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xdf, 0x08, // FISTTP word ptr [RAX]
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_fisttp_m32_truncate_positive() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xdb, 0x08, // FISTTP dword ptr [RAX]
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_fisttp_m32_truncate_negative() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xdb, 0x08, // FISTTP dword ptr [RAX]
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_fisttp_m64_truncate_positive() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xdd, 0x08, // FISTTP qword ptr [RAX]
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_fisttp_m64_truncate_negative() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xdd, 0x08, // FISTTP qword ptr [RAX]
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_fisttp_m16_zero() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xdf, 0x08, // FISTTP word ptr [RAX]
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_fisttp_m32_zero() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xdb, 0x08, // FISTTP dword ptr [RAX]
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_fisttp_m64_zero() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xdd, 0x08, // FISTTP qword ptr [RAX]
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_fisttp_m16_max() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xdf, 0x08, // FISTTP word ptr [RAX]
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_fisttp_m16_min() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xdf, 0x08, // FISTTP word ptr [RAX]
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_fisttp_m32_max() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xdb, 0x08, // FISTTP dword ptr [RAX]
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_fisttp_m32_min() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xdb, 0x08, // FISTTP dword ptr [RAX]
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_fisttp_m64_max() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xdd, 0x08, // FISTTP qword ptr [RAX]
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_fisttp_m64_min() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xdd, 0x08, // FISTTP qword ptr [RAX]
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_fisttp_m16_with_displacement() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR - 0x10).to_le_bytes());
    full_code.extend_from_slice(&[
        0xdf, 0x48, 0x10, // FISTTP word ptr [RAX+0x10]
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_fisttp_m32_with_displacement() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR - 0x10).to_le_bytes());
    full_code.extend_from_slice(&[
        0xdb, 0x48, 0x10, // FISTTP dword ptr [RAX+0x10]
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_fisttp_m64_with_displacement() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR - 0x10).to_le_bytes());
    full_code.extend_from_slice(&[
        0xdd, 0x48, 0x10, // FISTTP qword ptr [RAX+0x10]
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_fisttp_m16_rbx() {
    let mut emu = emu64();
    let code = [
        0x48, 0xbb, // MOV RBX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xdf, 0x0b, // FISTTP word ptr [RBX]
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_fisttp_m32_rcx() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb9, // MOV RCX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xdb, 0x09, // FISTTP dword ptr [RCX]
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_fisttp_m64_rdx() {
    let mut emu = emu64();
    let code = [
        0x48, 0xba, // MOV RDX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xdd, 0x0a, // FISTTP qword ptr [RDX]
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_fisttp_m16_rsi() {
    let mut emu = emu64();
    let code = [
        0x48, 0xbe, // MOV RSI, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xdf, 0x0e, // FISTTP word ptr [RSI]
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_fisttp_m32_rdi() {
    let mut emu = emu64();
    let code = [
        0x48, 0xbf, // MOV RDI, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xdb, 0x0f, // FISTTP dword ptr [RDI]
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_fisttp_m64_r8() {
    let mut emu = emu64();
    let code = [
        0x49, 0xb8, // MOV R8, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x41, 0xdd, 0x08, // FISTTP qword ptr [R8]
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_fisttp_m16_r9() {
    let mut emu = emu64();
    let code = [
        0x49, 0xb9, // MOV R9, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x41, 0xdf, 0x09, // FISTTP word ptr [R9]
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_fisttp_m32_r10() {
    let mut emu = emu64();
    let code = [
        0x49, 0xba, // MOV R10, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x41, 0xdb, 0x0a, // FISTTP dword ptr [R10]
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_fisttp_m64_r11() {
    let mut emu = emu64();
    let code = [
        0x49, 0xbb, // MOV R11, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x41, 0xdd, 0x0b, // FISTTP qword ptr [R11]
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_fisttp_m16_sequential() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xdf, 0x08, // FISTTP word ptr [RAX]
        0xdf, 0x48, 0x02, // FISTTP word ptr [RAX+2]
        0xdf, 0x48, 0x04, // FISTTP word ptr [RAX+4]
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_fisttp_m32_sequential() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xdb, 0x08, // FISTTP dword ptr [RAX]
        0xdb, 0x48, 0x04, // FISTTP dword ptr [RAX+4]
        0xdb, 0x48, 0x08, // FISTTP dword ptr [RAX+8]
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_fisttp_m64_sequential() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xdd, 0x08, // FISTTP qword ptr [RAX]
        0xdd, 0x48, 0x08, // FISTTP qword ptr [RAX+8]
        0xdd, 0x48, 0x10, // FISTTP qword ptr [RAX+16]
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_fisttp_mixed_sizes() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xdf, 0x08, // FISTTP word ptr [RAX]
        0xdb, 0x48, 0x04, // FISTTP dword ptr [RAX+4]
        0xdd, 0x48, 0x08, // FISTTP qword ptr [RAX+8]
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_fisttp_m16_fractional_positive() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xdf, 0x08, // FISTTP word ptr [RAX]
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_fisttp_m16_fractional_negative() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xdf, 0x08, // FISTTP word ptr [RAX]
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_fisttp_m32_fractional_positive() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xdb, 0x08, // FISTTP dword ptr [RAX]
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_fisttp_m32_fractional_negative() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xdb, 0x08, // FISTTP dword ptr [RAX]
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_fisttp_m64_fractional_positive() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xdd, 0x08, // FISTTP qword ptr [RAX]
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_fisttp_m64_fractional_negative() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xdd, 0x08, // FISTTP qword ptr [RAX]
        0xf4, // HLT
    ]);
    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}
