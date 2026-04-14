use crate::*;

// VMOVSLDUP - Move and Duplicate Low Packed Single-Precision

const ALIGNED_ADDR: u64 = 0x3000;

#[test]
fn test_vmovsldup_xmm2_xmm0_xmm1() {
    let mut emu = emu64();
    let code = [0xc5, 0xfa, 0x12, 0xd0, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovsldup_xmm3_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [0xc5, 0xfa, 0x12, 0xd9, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovsldup_xmm4_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [0xc5, 0xfa, 0x12, 0xe2, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovsldup_xmm5_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [0xc5, 0xfa, 0x12, 0xeb, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovsldup_xmm6_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [0xc5, 0xfa, 0x12, 0xf4, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovsldup_xmm7_xmm5_xmm6() {
    let mut emu = emu64();
    let code = [0xc5, 0xfa, 0x12, 0xfd, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovsldup_xmm8_xmm6_xmm7() {
    let mut emu = emu64();
    let code = [0xc5, 0x7a, 0x12, 0xc6, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovsldup_xmm9_xmm7_xmm8() {
    let mut emu = emu64();
    let code = [0xc5, 0x7a, 0x12, 0xcf, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovsldup_xmm10_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [0xc4, 0x41, 0x7a, 0x12, 0xd0, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovsldup_xmm11_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [0xc4, 0x41, 0x7a, 0x12, 0xd9, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovsldup_xmm12_xmm10_xmm11() {
    let mut emu = emu64();
    let code = [0xc4, 0x41, 0x7a, 0x12, 0xe2, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovsldup_xmm13_xmm11_xmm12() {
    let mut emu = emu64();
    let code = [0xc4, 0x41, 0x7a, 0x12, 0xeb, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovsldup_xmm14_xmm12_xmm13() {
    let mut emu = emu64();
    let code = [0xc4, 0x41, 0x7a, 0x12, 0xf4, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovsldup_xmm15_xmm13_xmm14() {
    let mut emu = emu64();
    let code = [0xc4, 0x41, 0x7a, 0x12, 0xfd, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovsldup_xmm0_xmm14_xmm15() {
    let mut emu = emu64();
    let code = [0xc4, 0xc1, 0x7a, 0x12, 0xc6, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovsldup_xmm1_xmm15_xmm0() {
    let mut emu = emu64();
    let code = [0xc4, 0xc1, 0x7a, 0x12, 0xcf, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovsldup_xmm0_xmm1_mem() {
    let mut emu = emu64();
    let code = [0xc5, 0xfa, 0x12, 0xc1, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovsldup_xmm1_xmm2_mem() {
    let mut emu = emu64();
    let code = [0xc5, 0xfa, 0x12, 0xca, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovsldup_xmm2_xmm3_mem() {
    let mut emu = emu64();
    let code = [0xc5, 0xfa, 0x12, 0xd3, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovsldup_xmm3_xmm4_mem() {
    let mut emu = emu64();
    let code = [0xc5, 0xfa, 0x12, 0xdc, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovsldup_xmm4_xmm5_mem() {
    let mut emu = emu64();
    let code = [0xc5, 0xfa, 0x12, 0xe5, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovsldup_xmm5_xmm6_mem() {
    let mut emu = emu64();
    let code = [0xc5, 0xfa, 0x12, 0xee, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovsldup_xmm6_xmm7_mem() {
    let mut emu = emu64();
    let code = [0xc5, 0xfa, 0x12, 0xf7, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovsldup_xmm7_xmm8_mem() {
    let mut emu = emu64();
    let code = [0xc4, 0xc1, 0x7a, 0x12, 0xf8, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovsldup_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    let code = [0xc5, 0xfe, 0x12, 0xc1, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovsldup_ymm1_ymm2_ymm3() {
    let mut emu = emu64();
    let code = [0xc5, 0xfe, 0x12, 0xca, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovsldup_ymm2_ymm3_ymm4() {
    let mut emu = emu64();
    let code = [0xc5, 0xfe, 0x12, 0xd3, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovsldup_ymm3_ymm4_ymm5() {
    let mut emu = emu64();
    let code = [0xc5, 0xfe, 0x12, 0xdc, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovsldup_ymm4_ymm5_ymm6() {
    let mut emu = emu64();
    let code = [0xc5, 0xfe, 0x12, 0xe5, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovsldup_ymm5_ymm6_ymm7() {
    let mut emu = emu64();
    let code = [0xc5, 0xfe, 0x12, 0xee, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovsldup_ymm6_ymm7_ymm0() {
    let mut emu = emu64();
    let code = [0xc5, 0xfe, 0x12, 0xf7, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovsldup_ymm7_ymm0_ymm1() {
    let mut emu = emu64();
    let code = [0xc5, 0xfe, 0x12, 0xf8, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovsldup_ymm0_ymm1_mem256() {
    let mut emu = emu64();
    let code = [0xc5, 0xfe, 0x12, 0xc1, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 32];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovsldup_ymm1_ymm2_mem256() {
    let mut emu = emu64();
    let code = [0xc5, 0xfe, 0x12, 0xca, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 32];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovsldup_ymm2_ymm3_mem256() {
    let mut emu = emu64();
    let code = [0xc5, 0xfe, 0x12, 0xd3, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 32];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovsldup_ymm3_ymm4_mem256() {
    let mut emu = emu64();
    let code = [0xc5, 0xfe, 0x12, 0xdc, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 32];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovsldup_ymm4_ymm5_mem256() {
    let mut emu = emu64();
    let code = [0xc5, 0xfe, 0x12, 0xe5, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 32];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vmovsldup_ymm5_ymm6_mem256() {
    let mut emu = emu64();
    let code = [0xc5, 0xfe, 0x12, 0xee, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 32];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
