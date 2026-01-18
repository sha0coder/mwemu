use crate::*;

// VPHMINPOSUW - Horizontal Minimum of Packed Unsigned Words

const ALIGNED_ADDR: u64 = 0x3000;

#[test]
fn test_vphminposuw_xmm2_xmm0_xmm1() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0x79, 0x41, 0xd0, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vphminposuw_xmm3_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0x79, 0x41, 0xd9, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vphminposuw_xmm4_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0x79, 0x41, 0xe2, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vphminposuw_xmm5_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0x79, 0x41, 0xeb, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vphminposuw_xmm6_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0x79, 0x41, 0xf4, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vphminposuw_xmm7_xmm5_xmm6() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0x79, 0x41, 0xfd, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vphminposuw_xmm8_xmm6_xmm7() {
    let mut emu = emu64();
    let code = [0xc4, 0x62, 0x79, 0x41, 0xc6, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vphminposuw_xmm9_xmm7_xmm8() {
    let mut emu = emu64();
    let code = [0xc4, 0x62, 0x79, 0x41, 0xcf, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vphminposuw_xmm10_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [0xc4, 0x42, 0x79, 0x41, 0xd0, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vphminposuw_xmm11_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [0xc4, 0x42, 0x79, 0x41, 0xd9, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vphminposuw_xmm12_xmm10_xmm11() {
    let mut emu = emu64();
    let code = [0xc4, 0x42, 0x79, 0x41, 0xe2, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vphminposuw_xmm13_xmm11_xmm12() {
    let mut emu = emu64();
    let code = [0xc4, 0x42, 0x79, 0x41, 0xeb, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vphminposuw_xmm14_xmm12_xmm13() {
    let mut emu = emu64();
    let code = [0xc4, 0x42, 0x79, 0x41, 0xf4, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vphminposuw_xmm15_xmm13_xmm14() {
    let mut emu = emu64();
    let code = [0xc4, 0x42, 0x79, 0x41, 0xfd, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vphminposuw_xmm0_xmm14_xmm15() {
    let mut emu = emu64();
    let code = [0xc4, 0xc2, 0x79, 0x41, 0xc6, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vphminposuw_xmm1_xmm15_xmm0() {
    let mut emu = emu64();
    let code = [0xc4, 0xc2, 0x79, 0x41, 0xcf, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vphminposuw_xmm0_xmm1_mem() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0x79, 0x41, 0x05, 0xf7, 0x1f, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vphminposuw_xmm1_xmm2_mem() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0x79, 0x41, 0x0d, 0xf7, 0x1f, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vphminposuw_xmm2_xmm3_mem() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0x79, 0x41, 0x15, 0xf7, 0x1f, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vphminposuw_xmm3_xmm4_mem() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0x79, 0x41, 0x1d, 0xf7, 0x1f, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vphminposuw_xmm4_xmm5_mem() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0x79, 0x41, 0x25, 0xf7, 0x1f, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vphminposuw_xmm5_xmm6_mem() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0x79, 0x41, 0x2d, 0xf7, 0x1f, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vphminposuw_xmm6_xmm7_mem() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0x79, 0x41, 0x35, 0xf7, 0x1f, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vphminposuw_xmm7_xmm8_mem() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0x79, 0x41, 0x3d, 0xf7, 0x1f, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
