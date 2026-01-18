use crate::*;

// VMOVLPS - Move Low Packed Single-Precision

const ALIGNED_ADDR: u64 = 0x3000;

#[test]
fn test_vmovlps_xmm2_xmm0_xmm1() {
    let mut emu = emu64();
    let code = [0xc5, 0xf8, 0x12, 0x15, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovlps_xmm3_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [0xc5, 0xf0, 0x12, 0x1d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovlps_xmm4_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [0xc5, 0xe8, 0x12, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovlps_xmm5_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [0xc5, 0xe0, 0x12, 0x2d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovlps_xmm6_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [0xc5, 0xd8, 0x12, 0x35, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovlps_xmm7_xmm5_xmm6() {
    let mut emu = emu64();
    let code = [0xc5, 0xd0, 0x12, 0x3d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovlps_xmm8_xmm6_xmm7() {
    let mut emu = emu64();
    let code = [0xc5, 0x48, 0x12, 0x05, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovlps_xmm9_xmm7_xmm8() {
    let mut emu = emu64();
    let code = [0xc5, 0x40, 0x12, 0x0d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovlps_xmm10_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [0xc5, 0x38, 0x12, 0x15, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovlps_xmm11_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [0xc5, 0x30, 0x12, 0x1d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovlps_xmm12_xmm10_xmm11() {
    let mut emu = emu64();
    let code = [0xc5, 0x28, 0x12, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovlps_xmm13_xmm11_xmm12() {
    let mut emu = emu64();
    let code = [0xc5, 0x20, 0x12, 0x2d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovlps_xmm14_xmm12_xmm13() {
    let mut emu = emu64();
    let code = [0xc5, 0x18, 0x12, 0x35, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovlps_xmm15_xmm13_xmm14() {
    let mut emu = emu64();
    let code = [0xc5, 0x10, 0x12, 0x3d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovlps_xmm0_xmm14_xmm15() {
    let mut emu = emu64();
    let code = [0xc5, 0x88, 0x12, 0x05, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovlps_xmm1_xmm15_xmm0() {
    let mut emu = emu64();
    let code = [0xc5, 0x80, 0x12, 0x0d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovlps_xmm0_xmm1_mem() {
    let mut emu = emu64();
    let code = [0xc5, 0xf0, 0x12, 0x05, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovlps_xmm1_xmm2_mem() {
    let mut emu = emu64();
    let code = [0xc5, 0xe8, 0x12, 0x0d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovlps_xmm2_xmm3_mem() {
    let mut emu = emu64();
    let code = [0xc5, 0xe0, 0x12, 0x15, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovlps_xmm3_xmm4_mem() {
    let mut emu = emu64();
    let code = [0xc5, 0xd8, 0x12, 0x1d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovlps_xmm4_xmm5_mem() {
    let mut emu = emu64();
    let code = [0xc5, 0xd0, 0x12, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovlps_xmm5_xmm6_mem() {
    let mut emu = emu64();
    let code = [0xc5, 0xc8, 0x12, 0x2d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovlps_xmm6_xmm7_mem() {
    let mut emu = emu64();
    let code = [0xc5, 0xc0, 0x12, 0x35, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovlps_xmm7_xmm8_mem() {
    let mut emu = emu64();
    let code = [0xc5, 0xb8, 0x12, 0x3d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
