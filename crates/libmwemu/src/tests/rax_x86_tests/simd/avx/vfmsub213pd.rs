use crate::*;

// VFMSUB213PD - Fused Multiply-Subtract Packed Double-Precision (213)
//
// FMA (Fused Multiply-Add) instructions perform a*b+c in a single operation
// with only one rounding, providing better performance and precision.

const ALIGNED_ADDR: u64 = 0x3000;

#[test]
fn test_vfmsub213pd_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0xf1, 0xaa, 0xc2, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vfmsub213pd_xmm1_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0xe9, 0xaa, 0xcb, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vfmsub213pd_xmm2_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0xe1, 0xaa, 0xd4, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vfmsub213pd_xmm3_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0xd9, 0xaa, 0xdd, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vfmsub213pd_xmm4_xmm5_xmm6() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0xd1, 0xaa, 0xe6, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vfmsub213pd_xmm5_xmm6_xmm7() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0xc9, 0xaa, 0xef, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vfmsub213pd_xmm6_xmm7_xmm8() {
    let mut emu = emu64();
    let code = [0xc4, 0xc2, 0xc1, 0xaa, 0xf0, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vfmsub213pd_xmm7_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [0xc4, 0xc2, 0xb9, 0xaa, 0xf9, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vfmsub213pd_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [0xc4, 0x42, 0xb1, 0xaa, 0xc2, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vfmsub213pd_xmm9_xmm10_xmm11() {
    let mut emu = emu64();
    let code = [0xc4, 0x42, 0xa9, 0xaa, 0xcb, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vfmsub213pd_xmm10_xmm11_xmm12() {
    let mut emu = emu64();
    let code = [0xc4, 0x42, 0xa1, 0xaa, 0xd4, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vfmsub213pd_xmm11_xmm12_xmm13() {
    let mut emu = emu64();
    let code = [0xc4, 0x42, 0x99, 0xaa, 0xdd, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vfmsub213pd_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0xf5, 0xaa, 0xc2, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vfmsub213pd_ymm1_ymm2_ymm3() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0xed, 0xaa, 0xcb, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vfmsub213pd_ymm2_ymm3_ymm4() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0xe5, 0xaa, 0xd4, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vfmsub213pd_ymm3_ymm4_ymm5() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0xdd, 0xaa, 0xdd, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vfmsub213pd_ymm4_ymm5_ymm6() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0xd5, 0xaa, 0xe6, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vfmsub213pd_ymm5_ymm6_ymm7() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0xcd, 0xaa, 0xef, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vfmsub213pd_ymm6_ymm7_ymm0() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0xc5, 0xaa, 0xf0, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vfmsub213pd_ymm7_ymm0_ymm1() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0xfd, 0xaa, 0xf9, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vfmsub213pd_xmm0_xmm1_mem() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0xf1, 0xaa, 0x05, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vfmsub213pd_xmm1_xmm2_mem() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0xe9, 0xaa, 0x0d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vfmsub213pd_xmm2_xmm3_mem() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0xe1, 0xaa, 0x15, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vfmsub213pd_xmm3_xmm4_mem() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0xd9, 0xaa, 0x1d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vfmsub213pd_xmm4_xmm5_mem() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0xd1, 0xaa, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vfmsub213pd_xmm5_xmm6_mem() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0xc9, 0xaa, 0x2d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vfmsub213pd_ymm0_ymm1_mem256() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0xf5, 0xaa, 0x05, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 32];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vfmsub213pd_ymm1_ymm2_mem256() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0xed, 0xaa, 0x0d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 32];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vfmsub213pd_ymm2_ymm3_mem256() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0xe5, 0xaa, 0x15, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 32];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vfmsub213pd_ymm3_ymm4_mem256() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0xdd, 0xaa, 0x1d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 32];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
