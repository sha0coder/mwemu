use crate::*;

// VMOVHPD - Move High Packed Double-Precision

const ALIGNED_ADDR: u64 = 0x3000;

#[test]
fn test_vmovhpd_xmm2_xmm0_xmm1() {
    let mut emu = emu64();
    let code = [0xc5, 0xf9, 0x16, 0x15, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovhpd_xmm3_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [0xc5, 0xf1, 0x16, 0x1d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovhpd_xmm4_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [0xc5, 0xe9, 0x16, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovhpd_xmm5_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [0xc5, 0xe1, 0x16, 0x2d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovhpd_xmm6_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [0xc5, 0xd9, 0x16, 0x35, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovhpd_xmm7_xmm5_xmm6() {
    let mut emu = emu64();
    let code = [0xc5, 0xd1, 0x16, 0x3d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovhpd_xmm8_xmm6_xmm7() {
    let mut emu = emu64();
    let code = [0xc5, 0x49, 0x16, 0x05, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovhpd_xmm9_xmm7_xmm8() {
    let mut emu = emu64();
    let code = [0xc5, 0x41, 0x16, 0x0d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovhpd_xmm10_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [0xc5, 0x39, 0x16, 0x15, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovhpd_xmm11_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [0xc5, 0x31, 0x16, 0x1d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovhpd_xmm12_xmm10_xmm11() {
    let mut emu = emu64();
    let code = [0xc5, 0x29, 0x16, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovhpd_xmm13_xmm11_xmm12() {
    let mut emu = emu64();
    let code = [0xc5, 0x21, 0x16, 0x2d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovhpd_xmm14_xmm12_xmm13() {
    let mut emu = emu64();
    let code = [0xc5, 0x19, 0x16, 0x35, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovhpd_xmm15_xmm13_xmm14() {
    let mut emu = emu64();
    let code = [0xc5, 0x11, 0x16, 0x3d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovhpd_xmm0_xmm14_xmm15() {
    let mut emu = emu64();
    let code = [0xc5, 0x89, 0x16, 0x05, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovhpd_xmm1_xmm15_xmm0() {
    let mut emu = emu64();
    let code = [0xc5, 0x81, 0x16, 0x0d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovhpd_xmm0_xmm1_mem() {
    let mut emu = emu64();
    let code = [0xc5, 0xf1, 0x16, 0x05, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovhpd_xmm1_xmm2_mem() {
    let mut emu = emu64();
    let code = [0xc5, 0xe9, 0x16, 0x0d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovhpd_xmm2_xmm3_mem() {
    let mut emu = emu64();
    let code = [0xc5, 0xe1, 0x16, 0x15, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovhpd_xmm3_xmm4_mem() {
    let mut emu = emu64();
    let code = [0xc5, 0xd9, 0x16, 0x1d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovhpd_xmm4_xmm5_mem() {
    let mut emu = emu64();
    let code = [0xc5, 0xd1, 0x16, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovhpd_xmm5_xmm6_mem() {
    let mut emu = emu64();
    let code = [0xc5, 0xc9, 0x16, 0x2d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovhpd_xmm6_xmm7_mem() {
    let mut emu = emu64();
    let code = [0xc5, 0xc1, 0x16, 0x35, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovhpd_xmm7_xmm8_mem() {
    let mut emu = emu64();
    let code = [0xc5, 0xb9, 0x16, 0x3d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
