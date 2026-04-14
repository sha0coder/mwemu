use crate::*;

// CRC32 - Accumulate CRC32 Value
//
// SSE4.2 instruction that performs a single CRC32 (Cyclic Redundancy Check)
// computation using the polynomial 0x11EDC6F41 (Castagnoli).
//
// Computes an accumulated CRC32 value:
//   dest = CRC32(dest, src)
//
// The instruction operates on different operand sizes:
//   - 8-bit: CRC32 r32, r/m8
//   - 16-bit: CRC32 r32, r/m16
//   - 32-bit: CRC32 r32, r/m32
//   - 64-bit: CRC32 r64, r/m64 (REX.W prefix)
//
// The destination is always a 32-bit or 64-bit register.
//
// Opcodes:
//   F2 0F 38 F0 /r       CRC32 r32, r/m8
//   F2 0F 38 F1 /r       CRC32 r32, r/m16 (with 66h prefix)
//   F2 0F 38 F1 /r       CRC32 r32, r/m32
//   F2 REX.W 0F 38 F0 /r CRC32 r64, r/m8
//   F2 REX.W 0F 38 F1 /r CRC32 r64, r/m64

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// 8-bit Operand Tests (r32, r/m8)
// ============================================================================

#[test]
fn test_crc32_r32_r8_basic() {
    let mut emu = emu64();
    let code = [
        0xb8, 0x00, 0x00, 0x00, 0x00, // MOV EAX, 0 (initial CRC)
        0xb1, 0x41, // MOV CL, 'A' (0x41)
        0xf2, 0x0f, 0x38, 0xf0, 0xc1, // CRC32 EAX, CL
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_crc32_r32_r8_accumulate() {
    let mut emu = emu64();
    let code = [
        0xb8, 0x00, 0x00, 0x00, 0x00, // MOV EAX, 0
        0xb1, 0x61, // MOV CL, 'a'
        0xf2, 0x0f, 0x38, 0xf0, 0xc1, // CRC32 EAX, CL
        0xb1, 0x62, // MOV CL, 'b'
        0xf2, 0x0f, 0x38, 0xf0, 0xc1, // CRC32 EAX, CL
        0xb1, 0x63, // MOV CL, 'c'
        0xf2, 0x0f, 0x38, 0xf0, 0xc1, // CRC32 EAX, CL
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_crc32_r32_r8_zero() {
    let mut emu = emu64();
    let code = [
        0xb8, 0x00, 0x00, 0x00, 0x00, // MOV EAX, 0
        0xb1, 0x00, // MOV CL, 0
        0xf2, 0x0f, 0x38, 0xf0, 0xc1, // CRC32 EAX, CL
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_crc32_r32_r8_ff() {
    let mut emu = emu64();
    let code = [
        0xb8, 0x00, 0x00, 0x00, 0x00, // MOV EAX, 0
        0xb1, 0xff, // MOV CL, 0xFF
        0xf2, 0x0f, 0x38, 0xf0, 0xc1, // CRC32 EAX, CL
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_crc32_r32_mem8() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0
        0xf2, 0x0f, 0x38, 0xf0, 0x08, // CRC32 ECX, BYTE PTR [RAX]
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 1] = [0x42];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_crc32_different_registers_r8() {
    let mut emu = emu64();
    let code = [
        0xbb, 0x00, 0x00, 0x00, 0x00, // MOV EBX, 0
        0xb2, 0x48, // MOV DL, 'H'
        0xf2, 0x0f, 0x38, 0xf0, 0xda, // CRC32 EBX, DL
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_crc32_r32_r8_sequence() {
    let mut emu = emu64();
    let code = [
        0xb8, 0x00, 0x00, 0x00, 0x00, // MOV EAX, 0
        0xb1, 0x48, // MOV CL, 'H'
        0xf2, 0x0f, 0x38, 0xf0, 0xc1, // CRC32 EAX, CL
        0xb1, 0x65, // MOV CL, 'e'
        0xf2, 0x0f, 0x38, 0xf0, 0xc1, // CRC32 EAX, CL
        0xb1, 0x6c, // MOV CL, 'l'
        0xf2, 0x0f, 0x38, 0xf0, 0xc1, // CRC32 EAX, CL
        0xf2, 0x0f, 0x38, 0xf0, 0xc1, // CRC32 EAX, CL (second 'l')
        0xb1, 0x6f, // MOV CL, 'o'
        0xf2, 0x0f, 0x38, 0xf0, 0xc1, // CRC32 EAX, CL
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// 16-bit Operand Tests (r32, r/m16)
// ============================================================================

#[test]
fn test_crc32_r32_r16_basic() {
    let mut emu = emu64();
    let code = [
        0xb8, 0x00, 0x00, 0x00, 0x00, // MOV EAX, 0
        0x66, 0xb9, 0x34, 0x12, // MOV CX, 0x1234
        0x66, 0xf2, 0x0f, 0x38, 0xf1, 0xc1, // CRC32 EAX, CX
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_crc32_r32_r16_accumulate() {
    let mut emu = emu64();
    let code = [
        0xb8, 0x00, 0x00, 0x00, 0x00, // MOV EAX, 0
        0x66, 0xb9, 0x11, 0x11, // MOV CX, 0x1111
        0x66, 0xf2, 0x0f, 0x38, 0xf1, 0xc1, // CRC32 EAX, CX
        0x66, 0xb9, 0x22, 0x22, // MOV CX, 0x2222
        0x66, 0xf2, 0x0f, 0x38, 0xf1, 0xc1, // CRC32 EAX, CX
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_crc32_r32_r16_zero() {
    let mut emu = emu64();
    let code = [
        0xb8, 0x00, 0x00, 0x00, 0x00, // MOV EAX, 0
        0x66, 0xb9, 0x00, 0x00, // MOV CX, 0
        0x66, 0xf2, 0x0f, 0x38, 0xf1, 0xc1, // CRC32 EAX, CX
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_crc32_r32_r16_ffff() {
    let mut emu = emu64();
    let code = [
        0xb8, 0x00, 0x00, 0x00, 0x00, // MOV EAX, 0
        0x66, 0xb9, 0xff, 0xff, // MOV CX, 0xFFFF
        0x66, 0xf2, 0x0f, 0x38, 0xf1, 0xc1, // CRC32 EAX, CX
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_crc32_r32_mem16() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0
        0x66, 0xf2, 0x0f, 0x38, 0xf1, 0x08, // CRC32 ECX, WORD PTR [RAX]
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 2] = [0x78, 0x56];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_crc32_different_registers_r16() {
    let mut emu = emu64();
    let code = [
        0xba, 0x00, 0x00, 0x00, 0x00, // MOV EDX, 0
        0x66, 0xbb, 0xcd, 0xab, // MOV BX, 0xABCD
        0x66, 0xf2, 0x0f, 0x38, 0xf1, 0xd3, // CRC32 EDX, BX
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// 32-bit Operand Tests (r32, r/m32)
// ============================================================================

#[test]
fn test_crc32_r32_r32_basic() {
    let mut emu = emu64();
    let code = [
        0xb8, 0x00, 0x00, 0x00, 0x00, // MOV EAX, 0
        0xb9, 0x78, 0x56, 0x34, 0x12, // MOV ECX, 0x12345678
        0xf2, 0x0f, 0x38, 0xf1, 0xc1, // CRC32 EAX, ECX
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_crc32_r32_r32_accumulate() {
    let mut emu = emu64();
    let code = [
        0xb8, 0x00, 0x00, 0x00, 0x00, // MOV EAX, 0
        0xb9, 0x11, 0x11, 0x11, 0x11, // MOV ECX, 0x11111111
        0xf2, 0x0f, 0x38, 0xf1, 0xc1, // CRC32 EAX, ECX
        0xb9, 0x22, 0x22, 0x22, 0x22, // MOV ECX, 0x22222222
        0xf2, 0x0f, 0x38, 0xf1, 0xc1, // CRC32 EAX, ECX
        0xb9, 0x33, 0x33, 0x33, 0x33, // MOV ECX, 0x33333333
        0xf2, 0x0f, 0x38, 0xf1, 0xc1, // CRC32 EAX, ECX
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_crc32_r32_r32_zero() {
    let mut emu = emu64();
    let code = [
        0xb8, 0x00, 0x00, 0x00, 0x00, // MOV EAX, 0
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0
        0xf2, 0x0f, 0x38, 0xf1, 0xc1, // CRC32 EAX, ECX
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_crc32_r32_r32_ffffffff() {
    let mut emu = emu64();
    let code = [
        0xb8, 0x00, 0x00, 0x00, 0x00, // MOV EAX, 0
        0xb9, 0xff, 0xff, 0xff, 0xff, // MOV ECX, 0xFFFFFFFF
        0xf2, 0x0f, 0x38, 0xf1, 0xc1, // CRC32 EAX, ECX
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_crc32_r32_mem32() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0
        0xf2, 0x0f, 0x38, 0xf1, 0x08, // CRC32 ECX, DWORD PTR [RAX]
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 4] = [0xef, 0xbe, 0xad, 0xde];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_crc32_different_registers_r32() {
    let mut emu = emu64();
    let code = [
        0xbb, 0x00, 0x00, 0x00, 0x00, // MOV EBX, 0
        0xba, 0x12, 0x34, 0x56, 0x78, // MOV EDX, 0x78563412
        0xf2, 0x0f, 0x38, 0xf1, 0xda, // CRC32 EBX, EDX
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_crc32_same_register() {
    let mut emu = emu64();
    // CRC32 of register with itself
    let code = [
        0xb8, 0x11, 0x11, 0x11, 0x11, // MOV EAX, 0x11111111
        0xf2, 0x0f, 0x38, 0xf1, 0xc0, // CRC32 EAX, EAX
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// 64-bit Operand Tests (r64, r/m8 with REX.W)
// ============================================================================

#[test]
fn test_crc32_r64_r8_basic() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0xb1, 0x41, // MOV CL, 'A'
        0xf2, 0x48, 0x0f, 0x38, 0xf0, 0xc1, // CRC32 RAX, CL
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_crc32_r64_r8_accumulate() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0xb1, 0x31, // MOV CL, '1'
        0xf2, 0x48, 0x0f, 0x38, 0xf0, 0xc1, // CRC32 RAX, CL
        0xb1, 0x32, // MOV CL, '2'
        0xf2, 0x48, 0x0f, 0x38, 0xf0, 0xc1, // CRC32 RAX, CL
        0xb1, 0x33, // MOV CL, '3'
        0xf2, 0x48, 0x0f, 0x38, 0xf0, 0xc1, // CRC32 RAX, CL
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_crc32_r64_mem8() {
    let mut emu = emu64();
    let code = [0x48, 0xbb];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0xf2, 0x48, 0x0f, 0x38, 0xf0, 0x03, // CRC32 RAX, BYTE PTR [RBX]
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 1] = [0x5A];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// 64-bit Operand Tests (r64, r/m64 with REX.W)
// ============================================================================

#[test]
fn test_crc32_r64_r64_basic() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x48, 0xb9, 0x78, 0x56, 0x34, 0x12, 0xef, 0xcd, 0xab, 0x90, // MOV RCX, 0x90ABCDEF12345678
        0xf2, 0x48, 0x0f, 0x38, 0xf1, 0xc1, // CRC32 RAX, RCX
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_crc32_r64_r64_accumulate() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x48, 0xb9, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, // MOV RCX, 0x1111111111111111
        0xf2, 0x48, 0x0f, 0x38, 0xf1, 0xc1, // CRC32 RAX, RCX
        0x48, 0xb9, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, // MOV RCX, 0x2222222222222222
        0xf2, 0x48, 0x0f, 0x38, 0xf1, 0xc1, // CRC32 RAX, RCX
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_crc32_r64_r64_zero() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x48, 0xb9, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RCX, 0
        0xf2, 0x48, 0x0f, 0x38, 0xf1, 0xc1, // CRC32 RAX, RCX
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_crc32_r64_r64_ffffffffffffffff() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0x48, 0xb9, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, // MOV RCX, 0xFFFFFFFFFFFFFFFF
        0xf2, 0x48, 0x0f, 0x38, 0xf1, 0xc1, // CRC32 RAX, RCX
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_crc32_r64_mem64() {
    let mut emu = emu64();
    let code = [0x48, 0xbb];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0xf2, 0x48, 0x0f, 0x38, 0xf1, 0x03, // CRC32 RAX, QWORD PTR [RBX]
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 8] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_crc32_r64_different_registers() {
    let mut emu = emu64();
    let code = [
        0x48, 0xbb, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RBX, 0
        0x48, 0xba, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x00, 0x11, // MOV RDX, 0x1100FFEEDDCCBBAA
        0xf2, 0x48, 0x0f, 0x38, 0xf1, 0xda, // CRC32 RBX, RDX
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_crc32_r64_same_register() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, // MOV RAX, 0xF0DEBC9A78563412
        0xf2, 0x48, 0x0f, 0x38, 0xf1, 0xc0, // CRC32 RAX, RAX
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Mixed Size Accumulation Tests
// ============================================================================

#[test]
fn test_crc32_mixed_sizes() {
    let mut emu = emu64();
    let code = [
        0xb8, 0x00, 0x00, 0x00, 0x00, // MOV EAX, 0
        0xb1, 0x41, // MOV CL, 'A'
        0xf2, 0x0f, 0x38, 0xf0, 0xc1, // CRC32 EAX, CL (8-bit)
        0x66, 0xb9, 0x42, 0x43, // MOV CX, 0x4342 ('BC')
        0x66, 0xf2, 0x0f, 0x38, 0xf1, 0xc1, // CRC32 EAX, CX (16-bit)
        0xb9, 0x44, 0x45, 0x46, 0x47, // MOV ECX, 0x47464544 ('DEFG')
        0xf2, 0x0f, 0x38, 0xf1, 0xc1, // CRC32 EAX, ECX (32-bit)
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_crc32_mixed_64bit_sizes() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0
        0xb1, 0x58, // MOV CL, 'X'
        0xf2, 0x48, 0x0f, 0x38, 0xf0, 0xc1, // CRC32 RAX, CL (8-bit to 64-bit)
        0x48, 0xb9, 0x59, 0x5a, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, // MOV RCX, data
        0xf2, 0x48, 0x0f, 0x38, 0xf1, 0xc1, // CRC32 RAX, RCX (64-bit)
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Memory Addressing Modes
// ============================================================================

#[test]
fn test_crc32_mem_displacement() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR - 0x10).to_le_bytes());
    full_code.extend_from_slice(&[
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0
        0xf2, 0x0f, 0x38, 0xf1, 0x48, 0x10, // CRC32 ECX, DWORD PTR [RAX+0x10]
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 4] = [0x12, 0x34, 0x56, 0x78];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_crc32_mem_sib() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xbb, 0x08, 0x00, 0x00, 0x00, // MOV EBX, 8
        0xb9, 0x00, 0x00, 0x00, 0x00, // MOV ECX, 0
        0xf2, 0x0f, 0x38, 0xf1, 0x0c, 0x18, // CRC32 ECX, DWORD PTR [RAX+RBX]
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    let actual_data: [u8; 4] = [0xaa, 0xbb, 0xcc, 0xdd];
    emu.maps.write_bytes_slice(ALIGNED_ADDR + 8, &actual_data);
    emu.run(None).unwrap();
}

// ============================================================================
// Initial CRC Value Tests
// ============================================================================

#[test]
fn test_crc32_nonzero_initial() {
    let mut emu = emu64();
    let code = [
        0xb8, 0xff, 0xff, 0xff, 0xff, // MOV EAX, 0xFFFFFFFF
        0xb1, 0x41, // MOV CL, 'A'
        0xf2, 0x0f, 0x38, 0xf0, 0xc1, // CRC32 EAX, CL
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_crc32_specific_initial_value() {
    let mut emu = emu64();
    let code = [
        0xb8, 0x12, 0x34, 0x56, 0x78, // MOV EAX, 0x78563412
        0xb9, 0xab, 0xcd, 0xef, 0x90, // MOV ECX, 0x90EFCDAB
        0xf2, 0x0f, 0x38, 0xf1, 0xc1, // CRC32 EAX, ECX
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Extended Register Tests
// ============================================================================

#[test]
fn test_crc32_r32_r8l() {
    let mut emu = emu64();
    let code = [
        0xb8, 0x00, 0x00, 0x00, 0x00, // MOV EAX, 0
        0x40, 0xb6, 0x5a, // MOV SIL, 0x5A
        0xf2, 0x40, 0x0f, 0x38, 0xf0, 0xc6, // CRC32 EAX, SIL
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_crc32_with_r8_r15() {
    let mut emu = emu64();
    let code = [
        0x49, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R8, 0
        0x41, 0xb1, 0x7f, // MOV R9B, 0x7F
        0xf2, 0x45, 0x0f, 0x38, 0xf0, 0xc1, // CRC32 R8D, R9B
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_crc32_r32_with_r8d_r9d() {
    let mut emu = emu64();
    let code = [
        0x45, 0xb8, 0x00, 0x00, 0x00, 0x00, // MOV R8D, 0
        0x41, 0xb9, 0x11, 0x22, 0x33, 0x44, // MOV R9D, 0x44332211
        0xf2, 0x45, 0x0f, 0x38, 0xf1, 0xc1, // CRC32 R8D, R9D
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_crc32_r64_with_r10_r11() {
    let mut emu = emu64();
    let code = [
        0x49, 0xba, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // MOV R10, 0
        0x49, 0xbb, 0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x10, // MOV R11, 0x1032547698BADCFE
        0xf2, 0x4d, 0x0f, 0x38, 0xf1, 0xd3, // CRC32 R10, R11
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Sequential CRC Computation Tests
// ============================================================================

#[test]
fn test_crc32_string_abc() {
    let mut emu = emu64();
    let code = [
        0xb8, 0x00, 0x00, 0x00, 0x00, // MOV EAX, 0
        0xb1, 0x61, // MOV CL, 'a'
        0xf2, 0x0f, 0x38, 0xf0, 0xc1, // CRC32 EAX, CL
        0xb1, 0x62, // MOV CL, 'b'
        0xf2, 0x0f, 0x38, 0xf0, 0xc1, // CRC32 EAX, CL
        0xb1, 0x63, // MOV CL, 'c'
        0xf2, 0x0f, 0x38, 0xf0, 0xc1, // CRC32 EAX, CL
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_crc32_string_test123() {
    let mut emu = emu64();
    let code = [
        0xb8, 0x00, 0x00, 0x00, 0x00, // MOV EAX, 0
        0xb1, 0x74, // 't'
        0xf2, 0x0f, 0x38, 0xf0, 0xc1,
        0xb1, 0x65, // 'e'
        0xf2, 0x0f, 0x38, 0xf0, 0xc1,
        0xb1, 0x73, // 's'
        0xf2, 0x0f, 0x38, 0xf0, 0xc1,
        0xb1, 0x74, // 't'
        0xf2, 0x0f, 0x38, 0xf0, 0xc1,
        0xb1, 0x31, // '1'
        0xf2, 0x0f, 0x38, 0xf0, 0xc1,
        0xb1, 0x32, // '2'
        0xf2, 0x0f, 0x38, 0xf0, 0xc1,
        0xb1, 0x33, // '3'
        0xf2, 0x0f, 0x38, 0xf0, 0xc1,
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_crc32_multiple_sequences() {
    let mut emu = emu64();
    let code = [
        // First CRC
        0xb8, 0x00, 0x00, 0x00, 0x00, // MOV EAX, 0
        0xb1, 0x41, // MOV CL, 'A'
        0xf2, 0x0f, 0x38, 0xf0, 0xc1, // CRC32 EAX, CL
        // Second CRC (independent)
        0xbb, 0x00, 0x00, 0x00, 0x00, // MOV EBX, 0
        0xb1, 0x42, // MOV CL, 'B'
        0xf2, 0x0f, 0x38, 0xf0, 0xd9, // CRC32 EBX, CL
        // Third CRC (independent)
        0xba, 0x00, 0x00, 0x00, 0x00, // MOV EDX, 0
        0xb1, 0x43, // MOV CL, 'C'
        0xf2, 0x0f, 0x38, 0xf0, 0xd1, // CRC32 EDX, CL
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
