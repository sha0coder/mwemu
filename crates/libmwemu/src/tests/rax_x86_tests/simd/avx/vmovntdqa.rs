use crate::*;

// VMOVNTDQA - Load Non-Temporal Packed Integer

const ALIGNED_ADDR: u64 = 0x3000;

#[test]
fn test_vmovntdqa_xmm2_xmm0_xmm1() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0x79, 0x2a, 0x15, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovntdqa_xmm3_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0x79, 0x2a, 0x1d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovntdqa_xmm4_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0x79, 0x2a, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovntdqa_xmm5_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0x79, 0x2a, 0x2d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovntdqa_xmm6_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0x79, 0x2a, 0x35, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovntdqa_xmm7_xmm5_xmm6() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0x79, 0x2a, 0x3d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovntdqa_xmm8_xmm6_xmm7() {
    let mut emu = emu64();
    let code = [0xc4, 0x62, 0x79, 0x2a, 0x05, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovntdqa_xmm9_xmm7_xmm8() {
    let mut emu = emu64();
    let code = [0xc4, 0x62, 0x79, 0x2a, 0x0d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovntdqa_xmm10_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [0xc4, 0x62, 0x79, 0x2a, 0x15, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovntdqa_xmm11_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [0xc4, 0x62, 0x79, 0x2a, 0x1d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovntdqa_xmm12_xmm10_xmm11() {
    let mut emu = emu64();
    let code = [0xc4, 0x62, 0x79, 0x2a, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovntdqa_xmm13_xmm11_xmm12() {
    let mut emu = emu64();
    let code = [0xc4, 0x62, 0x79, 0x2a, 0x2d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovntdqa_xmm14_xmm12_xmm13() {
    let mut emu = emu64();
    let code = [0xc4, 0x62, 0x79, 0x2a, 0x35, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovntdqa_xmm15_xmm13_xmm14() {
    let mut emu = emu64();
    let code = [0xc4, 0x62, 0x79, 0x2a, 0x3d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovntdqa_xmm0_xmm14_xmm15() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0x79, 0x2a, 0x05, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovntdqa_xmm1_xmm15_xmm0() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0x79, 0x2a, 0x0d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovntdqa_xmm0_xmm1_mem() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0x79, 0x2a, 0x05, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovntdqa_xmm1_xmm2_mem() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0x79, 0x2a, 0x0d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovntdqa_xmm2_xmm3_mem() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0x79, 0x2a, 0x15, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovntdqa_xmm3_xmm4_mem() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0x79, 0x2a, 0x1d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovntdqa_xmm4_xmm5_mem() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0x79, 0x2a, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovntdqa_xmm5_xmm6_mem() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0x79, 0x2a, 0x2d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovntdqa_xmm6_xmm7_mem() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0x79, 0x2a, 0x35, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovntdqa_xmm7_xmm8_mem() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0x79, 0x2a, 0x3d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovntdqa_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0x7d, 0x2a, 0x05, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovntdqa_ymm1_ymm2_ymm3() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0x7d, 0x2a, 0x0d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovntdqa_ymm2_ymm3_ymm4() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0x7d, 0x2a, 0x15, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovntdqa_ymm3_ymm4_ymm5() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0x7d, 0x2a, 0x1d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovntdqa_ymm4_ymm5_ymm6() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0x7d, 0x2a, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovntdqa_ymm5_ymm6_ymm7() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0x7d, 0x2a, 0x2d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovntdqa_ymm6_ymm7_ymm0() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0x7d, 0x2a, 0x35, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovntdqa_ymm7_ymm0_ymm1() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0x7d, 0x2a, 0x3d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovntdqa_ymm0_ymm1_mem256() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0x7d, 0x2a, 0x05, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 32];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovntdqa_ymm1_ymm2_mem256() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0x7d, 0x2a, 0x0d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 32];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovntdqa_ymm2_ymm3_mem256() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0x7d, 0x2a, 0x15, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 32];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovntdqa_ymm3_ymm4_mem256() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0x7d, 0x2a, 0x1d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 32];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovntdqa_ymm4_ymm5_mem256() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0x7d, 0x2a, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 32];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovntdqa_ymm5_ymm6_mem256() {
    let mut emu = emu64();
    let code = [0xc4, 0xe2, 0x7d, 0x2a, 0x2d, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 32];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
