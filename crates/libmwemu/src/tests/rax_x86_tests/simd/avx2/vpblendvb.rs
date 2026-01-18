use crate::*;

// VPBLENDVB - Variable Blend Packed Bytes

const ALIGNED_ADDR: u64 = 0x3000;

#[test]
fn test_vpblendvb_xmm2_xmm0_xmm1() {
    let mut emu = emu64();
    let code = [0xc4, 0xe3, 0x79, 0x4c, 0xd1, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpblendvb_xmm3_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [0xc4, 0xe3, 0x71, 0x4c, 0xda, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpblendvb_xmm4_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [0xc4, 0xe3, 0x69, 0x4c, 0xe3, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpblendvb_xmm5_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [0xc4, 0xe3, 0x61, 0x4c, 0xec, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpblendvb_xmm6_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [0xc4, 0xe3, 0x59, 0x4c, 0xf5, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpblendvb_xmm7_xmm5_xmm6() {
    let mut emu = emu64();
    let code = [0xc4, 0xe3, 0x51, 0x4c, 0xfe, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpblendvb_xmm8_xmm6_xmm7() {
    let mut emu = emu64();
    let code = [0xc4, 0x63, 0x49, 0x4c, 0xc7, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpblendvb_xmm9_xmm7_xmm8() {
    let mut emu = emu64();
    let code = [0xc4, 0x43, 0x41, 0x4c, 0xc8, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpblendvb_xmm10_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [0xc4, 0x43, 0x39, 0x4c, 0xd1, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpblendvb_xmm11_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [0xc4, 0x43, 0x31, 0x4c, 0xda, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpblendvb_xmm12_xmm10_xmm11() {
    let mut emu = emu64();
    let code = [0xc4, 0x43, 0x29, 0x4c, 0xe3, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpblendvb_xmm13_xmm11_xmm12() {
    let mut emu = emu64();
    let code = [0xc4, 0x43, 0x21, 0x4c, 0xec, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpblendvb_xmm14_xmm12_xmm13() {
    let mut emu = emu64();
    let code = [0xc4, 0x43, 0x19, 0x4c, 0xf5, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpblendvb_xmm15_xmm13_xmm14() {
    let mut emu = emu64();
    let code = [0xc4, 0x43, 0x11, 0x4c, 0xfe, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpblendvb_xmm0_xmm14_xmm15() {
    let mut emu = emu64();
    let code = [0xc4, 0xc3, 0x09, 0x4c, 0xc7, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpblendvb_xmm1_xmm15_xmm0() {
    let mut emu = emu64();
    let code = [0xc4, 0xe3, 0x01, 0x4c, 0xc8, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpblendvb_xmm0_xmm1_mem() {
    let mut emu = emu64();
    let code = [0xc4, 0xe3, 0x71, 0x4c, 0x05, 0xf6, 0x1f, 0x00, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vpblendvb_xmm1_xmm2_mem() {
    let mut emu = emu64();
    let code = [0xc4, 0xe3, 0x69, 0x4c, 0x0d, 0xf6, 0x1f, 0x00, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vpblendvb_xmm2_xmm3_mem() {
    let mut emu = emu64();
    let code = [0xc4, 0xe3, 0x61, 0x4c, 0x15, 0xf6, 0x1f, 0x00, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vpblendvb_xmm3_xmm4_mem() {
    let mut emu = emu64();
    let code = [0xc4, 0xe3, 0x59, 0x4c, 0x1d, 0xf6, 0x1f, 0x00, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vpblendvb_xmm4_xmm5_mem() {
    let mut emu = emu64();
    let code = [0xc4, 0xe3, 0x51, 0x4c, 0x25, 0xf6, 0x1f, 0x00, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vpblendvb_xmm5_xmm6_mem() {
    let mut emu = emu64();
    let code = [0xc4, 0xe3, 0x49, 0x4c, 0x2d, 0xf6, 0x1f, 0x00, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vpblendvb_xmm6_xmm7_mem() {
    let mut emu = emu64();
    let code = [0xc4, 0xe3, 0x41, 0x4c, 0x35, 0xf6, 0x1f, 0x00, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vpblendvb_xmm7_xmm8_mem() {
    let mut emu = emu64();
    let code = [0xc4, 0xe3, 0x39, 0x4c, 0x3d, 0xf6, 0x1f, 0x00, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vpblendvb_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    let code = [0xc4, 0xe3, 0x75, 0x4c, 0xc2, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpblendvb_ymm1_ymm2_ymm3() {
    let mut emu = emu64();
    let code = [0xc4, 0xe3, 0x6d, 0x4c, 0xcb, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpblendvb_ymm2_ymm3_ymm4() {
    let mut emu = emu64();
    let code = [0xc4, 0xe3, 0x65, 0x4c, 0xd4, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpblendvb_ymm3_ymm4_ymm5() {
    let mut emu = emu64();
    let code = [0xc4, 0xe3, 0x5d, 0x4c, 0xdd, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpblendvb_ymm4_ymm5_ymm6() {
    let mut emu = emu64();
    let code = [0xc4, 0xe3, 0x55, 0x4c, 0xe6, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpblendvb_ymm5_ymm6_ymm7() {
    let mut emu = emu64();
    let code = [0xc4, 0xe3, 0x4d, 0x4c, 0xef, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpblendvb_ymm6_ymm7_ymm0() {
    let mut emu = emu64();
    let code = [0xc4, 0xe3, 0x45, 0x4c, 0xf0, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpblendvb_ymm7_ymm0_ymm1() {
    let mut emu = emu64();
    let code = [0xc4, 0xe3, 0x7d, 0x4c, 0xf9, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpblendvb_ymm0_ymm1_mem256() {
    let mut emu = emu64();
    let code = [0xc4, 0xe3, 0x75, 0x4c, 0x05, 0xf6, 0x1f, 0x00, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 32];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vpblendvb_ymm1_ymm2_mem256() {
    let mut emu = emu64();
    let code = [0xc4, 0xe3, 0x6d, 0x4c, 0x0d, 0xf6, 0x1f, 0x00, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 32];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vpblendvb_ymm2_ymm3_mem256() {
    let mut emu = emu64();
    let code = [0xc4, 0xe3, 0x65, 0x4c, 0x15, 0xf6, 0x1f, 0x00, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 32];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vpblendvb_ymm3_ymm4_mem256() {
    let mut emu = emu64();
    let code = [0xc4, 0xe3, 0x5d, 0x4c, 0x1d, 0xf6, 0x1f, 0x00, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 32];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vpblendvb_ymm4_ymm5_mem256() {
    let mut emu = emu64();
    let code = [0xc4, 0xe3, 0x55, 0x4c, 0x25, 0xf6, 0x1f, 0x00, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 32];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vpblendvb_ymm5_ymm6_mem256() {
    let mut emu = emu64();
    let code = [0xc4, 0xe3, 0x4d, 0x4c, 0x2d, 0xf6, 0x1f, 0x00, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 32];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
