use crate::*;
const DATA_ADDR: u64 = 0x7000;
use crate::tests::rax_x86_tests::common::*;

// FPU Comparison and Control Instructions

// FCOM/FCOMP/FCOMPP - Compare floating-point

#[test]
fn test_fcom_m32fp() {
    TestCase::from("d8 10").check();
}

#[test]
fn test_fcom_m64fp() {
    TestCase::from("dc 10").check();
}

#[test]
fn test_fcom_st0() {
    TestCase::from("d8 d0").check();
}

#[test]
fn test_fcom_st1() {
    TestCase::from("d8 d1").check();
}

#[test]
fn test_fcom_st2() {
    TestCase::from("d8 d2").check();
}

#[test]
fn test_fcom_st7() {
    TestCase::from("d8 d7").check();
}

#[test]
fn test_fcom() {
    TestCase::from("d8 d1").check();
}

// FCOMP - Compare and pop

#[test]
fn test_fcomp_m32fp() {
    TestCase::from("d8 18").check();
}

#[test]
fn test_fcomp_m64fp() {
    TestCase::from("dc 18").check();
}

#[test]
fn test_fcomp_st0() {
    TestCase::from("d8 d8").check();
}

#[test]
fn test_fcomp_st1() {
    TestCase::from("d8 d9").check();
}

#[test]
fn test_fcomp_st2() {
    TestCase::from("d8 da").check();
}

#[test]
fn test_fcomp_st7() {
    TestCase::from("d8 df").check();
}

#[test]
fn test_fcomp() {
    TestCase::from("d8 d9").check();
}

// FCOMPP - Compare and pop twice

#[test]
fn test_fcompp() {
    TestCase::from("de d9").check();
}

// FCOMI/FCOMIP - Compare and set EFLAGS

#[test]
fn test_fcomi_st0_st0() {
    TestCase::from("db f0").check();
}

#[test]
fn test_fcomi_st0_st1() {
    TestCase::from("db f1").check();
}

#[test]
fn test_fcomi_st0_st2() {
    TestCase::from("db f2").check();
}

#[test]
fn test_fcomi_st0_st3() {
    TestCase::from("db f3").check();
}

#[test]
fn test_fcomi_st0_st7() {
    TestCase::from("db f7").check();
}

// FCOMIP - Compare, set EFLAGS, and pop

#[test]
fn test_fcomip_st0_st0() {
    TestCase::from("df f0").check();
}

#[test]
fn test_fcomip_st0_st1() {
    TestCase::from("df f1").check();
}

#[test]
fn test_fcomip_st0_st2() {
    TestCase::from("df f2").check();
}

#[test]
fn test_fcomip_st0_st3() {
    TestCase::from("df f3").check();
}

#[test]
fn test_fcomip_st0_st7() {
    TestCase::from("df f7").check();
}

// FUCOMI/FUCOMIP - Unordered compare and set EFLAGS

#[test]
fn test_fucomi_st0_st0() {
    TestCase::from("db e8").check();
}

#[test]
fn test_fucomi_st0_st1() {
    TestCase::from("db e9").check();
}

#[test]
fn test_fucomi_st0_st2() {
    TestCase::from("db ea").check();
}

#[test]
fn test_fucomi_st0_st3() {
    TestCase::from("db eb").check();
}

#[test]
fn test_fucomi_st0_st7() {
    TestCase::from("db ef").check();
}

// FUCOMIP - Unordered compare, set EFLAGS, and pop

#[test]
fn test_fucomip_st0_st0() {
    TestCase::from("df e8").check();
}

#[test]
fn test_fucomip_st0_st1() {
    TestCase::from("df e9").check();
}

#[test]
fn test_fucomip_st0_st2() {
    TestCase::from("df ea").check();
}

#[test]
fn test_fucomip_st0_st3() {
    TestCase::from("df eb").check();
}

#[test]
fn test_fucomip_st0_st7() {
    TestCase::from("df ef").check();
}

// FSAVE/FNSAVE - Save FPU state

#[test]
fn test_fsave_m94byte() {
    TestCase::from("9b dd 30").check();
}

#[test]
fn test_fsave_m108byte() {
    TestCase::from("9b dd 30").check();
}

#[test]
fn test_fnsave_m94byte() {
    TestCase::from("dd 30").check();
}

#[test]
fn test_fnsave_m108byte() {
    TestCase::from("dd 30").check();
}

#[test]
fn test_fsave_rax() {
    TestCase::from("9b dd 30").check();
}

#[test]
fn test_fnsave_rcx() {
    TestCase::from("dd 31").check();
}

#[test]
fn test_fsave_mem() {
    TestCase::from("9b dd 30").check();
}

#[test]
fn test_fnsave_mem() {
    TestCase::from("dd 30").check();
}

// FLD1/FLDL2T/FLDL2E/FLDPI/FLDLG2/FLDLN2/FLDZ - Load constants

#[test]
fn test_fld1() {
    TestCase::from("d9 e8").check();
}

#[test]
fn test_fldl2t() {
    TestCase::from("d9 e9").check();
}

#[test]
fn test_fldl2e() {
    TestCase::from("d9 ea").check();
}

#[test]
fn test_fldpi() {
    TestCase::from("d9 eb").check();
}

#[test]
fn test_fldlg2() {
    TestCase::from("d9 ec").check();
}

#[test]
fn test_fldln2() {
    TestCase::from("d9 ed").check();
}

#[test]
fn test_fldz() {
    TestCase::from("d9 ee").check();
}
