use crate::*;
use crate::tests::rax_x86_tests::common::*;

// SIMD Packing and Unpacking Operations

// PABSB/PABSW/PABSD/PABSQ - Packed absolute value

#[test]
fn test_pabsb_mm_mm() {
    let mut emu = emu64();
    TestCase::from("0f 38 1c c1").check();
}

#[test]
fn test_pabsb_xmm_xmm() {
    let mut emu = emu64();
    TestCase::from("66 0f 38 1c c1").check();
}

#[test]
fn test_pabsb_xmm_m128() {
    let mut emu = emu64();
    TestCase::from("66 0f 38 1c 00").check();
}

#[test]
fn test_pabsw_mm_mm() {
    let mut emu = emu64();
    TestCase::from("0f 38 1d c1").check();
}

#[test]
fn test_pabsw_xmm_xmm() {
    let mut emu = emu64();
    TestCase::from("66 0f 38 1d c1").check();
}

#[test]
fn test_pabsw_xmm_m128() {
    let mut emu = emu64();
    TestCase::from("66 0f 38 1d 00").check();
}

#[test]
fn test_pabsd_mm_mm() {
    let mut emu = emu64();
    TestCase::from("0f 38 1e c1").check();
}

#[test]
fn test_pabsd_xmm_xmm() {
    let mut emu = emu64();
    TestCase::from("66 0f 38 1e c1").check();
}

#[test]
fn test_pabsd_xmm_m128() {
    let mut emu = emu64();
    TestCase::from("66 0f 38 1e 00").check();
}

#[test]
fn test_pabsq_xmm_xmm() {
    let mut emu = emu64();
    TestCase::from("66 0f 38 1f c1").check();
}

#[test]
fn test_pabsq_xmm_m128() {
    let mut emu = emu64();
    TestCase::from("66 0f 38 1f 00").check();
}

// PMAXSB/PMAXSW/PMAXSD/PMAXSQ - Packed maximum (signed)

#[test]
fn test_pmaxsb_xmm_xmm() {
    let mut emu = emu64();
    TestCase::from("66 0f 38 3c c1").check();
}

#[test]
fn test_pmaxsb_xmm_m128() {
    let mut emu = emu64();
    TestCase::from("66 0f 38 3c 00").check();
}

#[test]
fn test_pmaxsw_mm_mm() {
    let mut emu = emu64();
    TestCase::from("0f ee c1").check();
}

#[test]
fn test_pmaxsw_xmm_xmm() {
    let mut emu = emu64();
    TestCase::from("66 0f ee c1").check();
}

#[test]
fn test_pmaxsw_xmm_m128() {
    let mut emu = emu64();
    TestCase::from("66 0f ee 00").check();
}

#[test]
fn test_pmaxsd_xmm_xmm() {
    let mut emu = emu64();
    TestCase::from("66 0f 38 3d c1").check();
}

#[test]
fn test_pmaxsd_xmm_m128() {
    let mut emu = emu64();
    TestCase::from("66 0f 38 3d 00").check();
}

#[test]
fn test_pmaxsq_xmm_xmm() {
    let mut emu = emu64();
    TestCase::from("66 0f 38 3d c1").check();
}

#[test]
fn test_pmaxsq_xmm_m128() {
    let mut emu = emu64();
    TestCase::from("66 0f 38 3d 00").check();
}

// PMAXUD/PMAXUQ - Packed maximum (unsigned dword/qword)

#[test]
fn test_pmaxud_xmm_xmm() {
    let mut emu = emu64();
    TestCase::from("66 0f 38 3f c1").check();
}

#[test]
fn test_pmaxud_xmm_m128() {
    let mut emu = emu64();
    TestCase::from("66 0f 38 3f 00").check();
}

#[test]
fn test_pmaxuq_xmm_xmm() {
    let mut emu = emu64();
    TestCase::from("66 0f 38 3f c1").check();
}

#[test]
fn test_pmaxuq_xmm_m128() {
    let mut emu = emu64();
    TestCase::from("66 0f 38 3f 00").check();
}

// PMINSD/PMINSQ - Packed minimum (signed dword/qword)

#[test]
fn test_pminsd_xmm_xmm() {
    let mut emu = emu64();
    TestCase::from("66 0f 38 39 c1").check();
}

#[test]
fn test_pminsd_xmm_m128() {
    let mut emu = emu64();
    TestCase::from("66 0f 38 39 00").check();
}

#[test]
fn test_pminsq_xmm_xmm() {
    let mut emu = emu64();
    TestCase::from("66 0f 38 39 c1").check();
}

#[test]
fn test_pminsq_xmm_m128() {
    let mut emu = emu64();
    TestCase::from("66 0f 38 39 00").check();
}

// PMINUD/PMINUQ - Packed minimum (unsigned dword/qword)

#[test]
fn test_pminud_xmm_xmm() {
    let mut emu = emu64();
    TestCase::from("66 0f 38 3b c1").check();
}

#[test]
fn test_pminud_xmm_m128() {
    let mut emu = emu64();
    TestCase::from("66 0f 38 3b 00").check();
}

#[test]
fn test_pminuq_xmm_xmm() {
    let mut emu = emu64();
    TestCase::from("66 0f 38 3b c1").check();
}

#[test]
fn test_pminuq_xmm_m128() {
    let mut emu = emu64();
    TestCase::from("66 0f 38 3b 00").check();
}

// PMULLD/PMULLQ - Packed multiply low (dword/qword)

#[test]
fn test_pmulld_xmm_xmm() {
    let mut emu = emu64();
    TestCase::from("66 0f 38 40 c1").check();
}

#[test]
fn test_pmulld_xmm_m128() {
    let mut emu = emu64();
    TestCase::from("66 0f 38 40 00").check();
}

#[test]
fn test_pmullq_xmm_xmm() {
    let mut emu = emu64();
    TestCase::from("66 0f 38 40 c1").check();
}

#[test]
fn test_pmullq_xmm_m128() {
    let mut emu = emu64();
    TestCase::from("66 0f 38 40 00").check();
}

// PSRAW/PSRAD/PSRAQ - Packed shift right arithmetic

#[test]
fn test_psraw_mm_mm() {
    let mut emu = emu64();
    TestCase::from("0f e1 c1").check();
}

#[test]
fn test_psraw_mm_m64() {
    let mut emu = emu64();
    TestCase::from("0f e1 00").check();
}

#[test]
fn test_psraw_mm_imm8() {
    let mut emu = emu64();
    TestCase::from("0f 71 e0 04").check();
}

#[test]
fn test_psraw_xmm_xmm() {
    let mut emu = emu64();
    TestCase::from("66 0f e1 c1").check();
}

#[test]
fn test_psraw_xmm_m128() {
    let mut emu = emu64();
    TestCase::from("66 0f e1 00").check();
}

#[test]
fn test_psraw_xmm_imm8() {
    let mut emu = emu64();
    TestCase::from("66 0f 71 e0 04").check();
}

#[test]
fn test_psrad_mm_mm() {
    let mut emu = emu64();
    TestCase::from("0f e2 c1").check();
}

#[test]
fn test_psrad_mm_m64() {
    let mut emu = emu64();
    TestCase::from("0f e2 00").check();
}

#[test]
fn test_psrad_mm_imm8() {
    let mut emu = emu64();
    TestCase::from("0f 72 e0 04").check();
}

#[test]
fn test_psrad_xmm_xmm() {
    let mut emu = emu64();
    TestCase::from("66 0f e2 c1").check();
}

#[test]
fn test_psrad_xmm_m128() {
    let mut emu = emu64();
    TestCase::from("66 0f e2 00").check();
}

#[test]
fn test_psrad_xmm_imm8() {
    let mut emu = emu64();
    TestCase::from("66 0f 72 e0 04").check();
}

#[test]
fn test_psraq_xmm_xmm() {
    let mut emu = emu64();
    TestCase::from("66 0f e2 c1").check();
}

#[test]
fn test_psraq_xmm_m128() {
    let mut emu = emu64();
    TestCase::from("66 0f e2 00").check();
}

#[test]
fn test_psraq_xmm_imm8() {
    let mut emu = emu64();
    TestCase::from("66 0f 72 e0 04").check();
}

// MOVDQA/VMOVDQA32/VMOVDQA64 - Move aligned packed integers

#[test]
fn test_movdqa_xmm_xmm() {
    let mut emu = emu64();
    TestCase::from("66 0f 6f c1").check();
}

#[test]
fn test_movdqa_xmm_m128() {
    let mut emu = emu64();
    TestCase::from("66 0f 6f 00").check();
}

#[test]
fn test_movdqa_m128_xmm() {
    let mut emu = emu64();
    TestCase::from("66 0f 7f 00").check();
}

#[test]
fn test_vmovdqa32_xmm_xmm() {
    let mut emu = emu64();
    TestCase::from("c5 f9 6f c1").check();
}

#[test]
fn test_vmovdqa32_ymm_ymm() {
    let mut emu = emu64();
    TestCase::from("c5 fd 6f c1").check();
}

#[test]
fn test_vmovdqa64_xmm_xmm() {
    let mut emu = emu64();
    TestCase::from("c5 f9 6f c1").check();
}

#[test]
fn test_vmovdqa64_ymm_ymm() {
    let mut emu = emu64();
    TestCase::from("c5 fd 6f c1").check();
}

// MOVDQU/VMOVDQU8/VMOVDQU16/VMOVDQU32/VMOVDQU64 - Move unaligned packed integers

#[test]
fn test_movdqu_xmm_xmm() {
    let mut emu = emu64();
    TestCase::from("f3 0f 6f c1").check();
}

#[test]
fn test_movdqu_xmm_m128() {
    let mut emu = emu64();
    TestCase::from("f3 0f 6f 00").check();
}

#[test]
fn test_movdqu_m128_xmm() {
    let mut emu = emu64();
    TestCase::from("f3 0f 7f 00").check();
}

#[test]
fn test_vmovdqu8_xmm_xmm() {
    let mut emu = emu64();
    TestCase::from("c5 fa 6f c1").check();
}

#[test]
fn test_vmovdqu8_ymm_ymm() {
    let mut emu = emu64();
    TestCase::from("c5 fe 6f c1").check();
}

#[test]
fn test_vmovdqu16_xmm_xmm() {
    let mut emu = emu64();
    TestCase::from("c5 fa 6f c1").check();
}

#[test]
fn test_vmovdqu16_ymm_ymm() {
    let mut emu = emu64();
    TestCase::from("c5 fe 6f c1").check();
}

#[test]
fn test_vmovdqu32_xmm_xmm() {
    let mut emu = emu64();
    TestCase::from("c5 fa 6f c1").check();
}

#[test]
fn test_vmovdqu32_ymm_ymm() {
    let mut emu = emu64();
    TestCase::from("c5 fe 6f c1").check();
}

#[test]
fn test_vmovdqu64_xmm_xmm() {
    let mut emu = emu64();
    TestCase::from("c5 fa 6f c1").check();
}

#[test]
fn test_vmovdqu64_ymm_ymm() {
    let mut emu = emu64();
    TestCase::from("c5 fe 6f c1").check();
}
