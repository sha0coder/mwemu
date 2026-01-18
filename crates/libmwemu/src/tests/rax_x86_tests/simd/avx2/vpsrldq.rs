use crate::*;

// VPSRLDQ - Shift Packed Data Right Logical by Bytes

const ALIGNED_ADDR: u64 = 0x3000;

#[test]
fn test_vpsrldq_xmm2_xmm0_xmm1() {
    let mut emu = emu64();
    let code = [0xc5, 0xe9, 0x73, 0xd8, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpsrldq_xmm3_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [0xc5, 0xe1, 0x73, 0xd9, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpsrldq_xmm4_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [0xc5, 0xd9, 0x73, 0xda, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpsrldq_xmm5_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [0xc5, 0xd1, 0x73, 0xdb, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpsrldq_xmm6_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [0xc5, 0xc9, 0x73, 0xdc, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpsrldq_xmm7_xmm5_xmm6() {
    let mut emu = emu64();
    let code = [0xc5, 0xc1, 0x73, 0xdd, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpsrldq_xmm8_xmm6_xmm7() {
    let mut emu = emu64();
    let code = [0xc5, 0xb9, 0x73, 0xde, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpsrldq_xmm9_xmm7_xmm8() {
    let mut emu = emu64();
    let code = [0xc5, 0xb1, 0x73, 0xdf, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpsrldq_xmm10_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [0xc4, 0xc1, 0x29, 0x73, 0xd8, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpsrldq_xmm11_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [0xc4, 0xc1, 0x21, 0x73, 0xd9, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpsrldq_xmm12_xmm10_xmm11() {
    let mut emu = emu64();
    let code = [0xc4, 0xc1, 0x19, 0x73, 0xda, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpsrldq_xmm13_xmm11_xmm12() {
    let mut emu = emu64();
    let code = [0xc4, 0xc1, 0x11, 0x73, 0xdb, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpsrldq_xmm14_xmm12_xmm13() {
    let mut emu = emu64();
    let code = [0xc4, 0xc1, 0x09, 0x73, 0xdc, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpsrldq_xmm15_xmm13_xmm14() {
    let mut emu = emu64();
    let code = [0xc4, 0xc1, 0x01, 0x73, 0xdd, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpsrldq_xmm0_xmm14_xmm15() {
    let mut emu = emu64();
    let code = [0xc4, 0xc1, 0x79, 0x73, 0xde, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpsrldq_xmm1_xmm15_xmm0() {
    let mut emu = emu64();
    let code = [0xc4, 0xc1, 0x71, 0x73, 0xdf, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpsrldq_xmm0_xmm1_mem() {
    let mut emu = emu64();
    let code = [0xc5, 0xf9, 0x73, 0x1d, 0xf7, 0x1f, 0x00, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vpsrldq_xmm1_xmm2_mem() {
    let mut emu = emu64();
    let code = [0xc5, 0xf1, 0x73, 0x1d, 0xf7, 0x1f, 0x00, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vpsrldq_xmm2_xmm3_mem() {
    let mut emu = emu64();
    let code = [0xc5, 0xe9, 0x73, 0x1d, 0xf7, 0x1f, 0x00, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vpsrldq_xmm3_xmm4_mem() {
    let mut emu = emu64();
    let code = [0xc5, 0xe1, 0x73, 0x1d, 0xf7, 0x1f, 0x00, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vpsrldq_xmm4_xmm5_mem() {
    let mut emu = emu64();
    let code = [0xc5, 0xd9, 0x73, 0x1d, 0xf7, 0x1f, 0x00, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vpsrldq_xmm5_xmm6_mem() {
    let mut emu = emu64();
    let code = [0xc5, 0xd1, 0x73, 0x1d, 0xf7, 0x1f, 0x00, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vpsrldq_xmm6_xmm7_mem() {
    let mut emu = emu64();
    let code = [0xc5, 0xc9, 0x73, 0x1d, 0xf7, 0x1f, 0x00, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vpsrldq_xmm7_xmm8_mem() {
    let mut emu = emu64();
    let code = [0xc5, 0xc1, 0x73, 0x1d, 0xf7, 0x1f, 0x00, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vpsrldq_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    let code = [0xc5, 0xfd, 0x73, 0xd9, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpsrldq_ymm1_ymm2_ymm3() {
    let mut emu = emu64();
    let code = [0xc5, 0xf5, 0x73, 0xda, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpsrldq_ymm2_ymm3_ymm4() {
    let mut emu = emu64();
    let code = [0xc5, 0xed, 0x73, 0xdb, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpsrldq_ymm3_ymm4_ymm5() {
    let mut emu = emu64();
    let code = [0xc5, 0xe5, 0x73, 0xdc, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpsrldq_ymm4_ymm5_ymm6() {
    let mut emu = emu64();
    let code = [0xc5, 0xdd, 0x73, 0xdd, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpsrldq_ymm5_ymm6_ymm7() {
    let mut emu = emu64();
    let code = [0xc5, 0xd5, 0x73, 0xde, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpsrldq_ymm6_ymm7_ymm0() {
    let mut emu = emu64();
    let code = [0xc5, 0xcd, 0x73, 0xdf, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpsrldq_ymm7_ymm0_ymm1() {
    let mut emu = emu64();
    let code = [0xc5, 0xc5, 0x73, 0xd8, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
#[test]
fn test_vpsrldq_ymm0_ymm1_mem256() {
    let mut emu = emu64();
    let code = [0xc5, 0xfd, 0x73, 0x1d, 0xf7, 0x1f, 0x00, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 32];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vpsrldq_ymm1_ymm2_mem256() {
    let mut emu = emu64();
    let code = [0xc5, 0xf5, 0x73, 0x1d, 0xf7, 0x1f, 0x00, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 32];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vpsrldq_ymm2_ymm3_mem256() {
    let mut emu = emu64();
    let code = [0xc5, 0xed, 0x73, 0x1d, 0xf7, 0x1f, 0x00, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 32];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vpsrldq_ymm3_ymm4_mem256() {
    let mut emu = emu64();
    let code = [0xc5, 0xe5, 0x73, 0x1d, 0xf7, 0x1f, 0x00, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 32];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vpsrldq_ymm4_ymm5_mem256() {
    let mut emu = emu64();
    let code = [0xc5, 0xdd, 0x73, 0x1d, 0xf7, 0x1f, 0x00, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 32];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
#[test]
fn test_vpsrldq_ymm5_ymm6_mem256() {
    let mut emu = emu64();
    let code = [0xc5, 0xd5, 0x73, 0x1d, 0xf7, 0x1f, 0x00, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 32];
    emu.maps.write_bytes_slice(0x3000, &test_data);
    emu.run(None).unwrap();
}
