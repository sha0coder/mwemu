use crate::*;

// VPANDN - Bitwise Logical AND NOT

const ALIGNED_ADDR: u64 = 0x3000;

#[test]
fn test_vpandn_xmm2_xmm0_xmm1() {
    let mut emu = emu64();
    let code = [0xc5, 0xf9, 0xdf, 0xd1, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpandn_xmm3_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [0xc5, 0xf1, 0xdf, 0xda, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpandn_xmm4_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [0xc5, 0xe9, 0xdf, 0xe3, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpandn_xmm5_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [0xc5, 0xe1, 0xdf, 0xec, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpandn_xmm6_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [0xc5, 0xd9, 0xdf, 0xf5, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpandn_xmm7_xmm5_xmm6() {
    let mut emu = emu64();
    let code = [0xc5, 0xd1, 0xdf, 0xfe, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpandn_xmm8_xmm6_xmm7() {
    let mut emu = emu64();
    let code = [0xc5, 0x49, 0xdf, 0xc7, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpandn_xmm9_xmm7_xmm8() {
    let mut emu = emu64();
    let code = [0xc4, 0x41, 0x41, 0xdf, 0xc8, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpandn_xmm10_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [0xc4, 0x41, 0x39, 0xdf, 0xd1, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpandn_xmm11_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [0xc4, 0x41, 0x31, 0xdf, 0xda, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpandn_xmm12_xmm10_xmm11() {
    let mut emu = emu64();
    let code = [0xc4, 0x41, 0x29, 0xdf, 0xe3, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpandn_xmm13_xmm11_xmm12() {
    let mut emu = emu64();
    let code = [0xc4, 0x41, 0x21, 0xdf, 0xec, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpandn_xmm14_xmm12_xmm13() {
    let mut emu = emu64();
    let code = [0xc4, 0x41, 0x19, 0xdf, 0xf5, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpandn_xmm15_xmm13_xmm14() {
    let mut emu = emu64();
    let code = [0xc4, 0x41, 0x11, 0xdf, 0xfe, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpandn_xmm0_xmm14_xmm15() {
    let mut emu = emu64();
    let code = [0xc4, 0xc1, 0x09, 0xdf, 0xc7, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpandn_xmm1_xmm15_xmm0() {
    let mut emu = emu64();
    let code = [0xc5, 0x81, 0xdf, 0xc8, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpandn_xmm0_xmm1_mem() {
    let mut emu = emu64();
    let code = [0xc5, 0xf1, 0xdf, 0x05, 0xf8, 0x1f, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vpandn_xmm1_xmm2_mem() {
    let mut emu = emu64();
    let code = [0xc5, 0xe9, 0xdf, 0x0d, 0xf8, 0x1f, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vpandn_xmm2_xmm3_mem() {
    let mut emu = emu64();
    let code = [0xc5, 0xe1, 0xdf, 0x15, 0xf8, 0x1f, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vpandn_xmm3_xmm4_mem() {
    let mut emu = emu64();
    let code = [0xc5, 0xd9, 0xdf, 0x1d, 0xf8, 0x1f, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vpandn_xmm4_xmm5_mem() {
    let mut emu = emu64();
    let code = [0xc5, 0xd1, 0xdf, 0x25, 0xf8, 0x1f, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vpandn_xmm5_xmm6_mem() {
    let mut emu = emu64();
    let code = [0xc5, 0xc9, 0xdf, 0x2d, 0xf8, 0x1f, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vpandn_xmm6_xmm7_mem() {
    let mut emu = emu64();
    let code = [0xc5, 0xc1, 0xdf, 0x35, 0xf8, 0x1f, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vpandn_xmm7_xmm8_mem() {
    let mut emu = emu64();
    let code = [0xc5, 0xb9, 0xdf, 0x3d, 0xf8, 0x1f, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vpandn_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    let code = [0xc5, 0xf5, 0xdf, 0xc2, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpandn_ymm1_ymm2_ymm3() {
    let mut emu = emu64();
    let code = [0xc5, 0xed, 0xdf, 0xcb, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpandn_ymm2_ymm3_ymm4() {
    let mut emu = emu64();
    let code = [0xc5, 0xe5, 0xdf, 0xd4, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpandn_ymm3_ymm4_ymm5() {
    let mut emu = emu64();
    let code = [0xc5, 0xdd, 0xdf, 0xdd, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpandn_ymm4_ymm5_ymm6() {
    let mut emu = emu64();
    let code = [0xc5, 0xd5, 0xdf, 0xe6, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpandn_ymm5_ymm6_ymm7() {
    let mut emu = emu64();
    let code = [0xc5, 0xcd, 0xdf, 0xef, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpandn_ymm6_ymm7_ymm0() {
    let mut emu = emu64();
    let code = [0xc5, 0xc5, 0xdf, 0xf0, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpandn_ymm7_ymm0_ymm1() {
    let mut emu = emu64();
    let code = [0xc5, 0xfd, 0xdf, 0xf9, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpandn_ymm0_ymm1_mem256() {
    let mut emu = emu64();
    let code = [0xc5, 0xf5, 0xdf, 0x05, 0xf8, 0x1f, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 32];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vpandn_ymm1_ymm2_mem256() {
    let mut emu = emu64();
    let code = [0xc5, 0xed, 0xdf, 0x0d, 0xf8, 0x1f, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 32];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vpandn_ymm2_ymm3_mem256() {
    let mut emu = emu64();
    let code = [0xc5, 0xe5, 0xdf, 0x15, 0xf8, 0x1f, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 32];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vpandn_ymm3_ymm4_mem256() {
    let mut emu = emu64();
    let code = [0xc5, 0xdd, 0xdf, 0x1d, 0xf8, 0x1f, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 32];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vpandn_ymm4_ymm5_mem256() {
    let mut emu = emu64();
    let code = [0xc5, 0xd5, 0xdf, 0x25, 0xf8, 0x1f, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 32];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vpandn_ymm5_ymm6_mem256() {
    let mut emu = emu64();
    let code = [0xc5, 0xcd, 0xdf, 0x2d, 0xf8, 0x1f, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 32];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
