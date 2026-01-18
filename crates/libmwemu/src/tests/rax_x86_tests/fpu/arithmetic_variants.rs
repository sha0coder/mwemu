use crate::*;
const DATA_ADDR: u64 = 0x7000;
use crate::tests::rax_x86_tests::common::*;

// FPU Arithmetic Variants - FADD/FADDP/FIADD family

// FADD - Add floating-point

#[test]
fn test_fadd_m32fp() {
    TestCase::from("d8 00").check();
}

#[test]
fn test_fadd_m64fp() {
    TestCase::from("dc 00").check();
}

#[test]
fn test_fadd_st0_st0() {
    TestCase::from("d8 c0").check();
}

#[test]
fn test_fadd_st0_st1() {
    TestCase::from("d8 c1").check();
}

#[test]
fn test_fadd_st0_st2() {
    TestCase::from("d8 c2").check();
}

#[test]
fn test_fadd_st0_st3() {
    TestCase::from("d8 c3").check();
}

#[test]
fn test_fadd_st0_st7() {
    TestCase::from("d8 c7").check();
}

#[test]
fn test_fadd_st1_st0() {
    TestCase::from("dc c1").check();
}

#[test]
fn test_fadd_st2_st0() {
    TestCase::from("dc c2").check();
}

#[test]
fn test_fadd_st7_st0() {
    TestCase::from("dc c7").check();
}

// FADDP - Add and pop

#[test]
fn test_faddp_st1_st0() {
    TestCase::from("de c1").check();
}

#[test]
fn test_faddp_st2_st0() {
    TestCase::from("de c2").check();
}

#[test]
fn test_faddp_st3_st0() {
    TestCase::from("de c3").check();
}

#[test]
fn test_faddp_st7_st0() {
    TestCase::from("de c7").check();
}

#[test]
fn test_faddp() {
    TestCase::from("de c1").check();
}

// FIADD - Add integer

#[test]
fn test_fiadd_m16int() {
    TestCase::from("de 00").check();
}

#[test]
fn test_fiadd_m32int() {
    TestCase::from("da 00").check();
}

#[test]
fn test_fiadd_m16int_rax() {
    TestCase::from("de 00").check();
}

#[test]
fn test_fiadd_m32int_rcx() {
    TestCase::from("da 01").check();
}

// FMUL/FMULP/FIMUL - Multiply

#[test]
fn test_fmul_m32fp() {
    TestCase::from("d8 08").check();
}

#[test]
fn test_fmul_m64fp() {
    TestCase::from("dc 08").check();
}

#[test]
fn test_fmul_st0_st0() {
    TestCase::from("d8 c8").check();
}

#[test]
fn test_fmul_st0_st1() {
    TestCase::from("d8 c9").check();
}

#[test]
fn test_fmul_st0_st2() {
    TestCase::from("d8 ca").check();
}

#[test]
fn test_fmul_st0_st3() {
    TestCase::from("d8 cb").check();
}

#[test]
fn test_fmul_st0_st7() {
    TestCase::from("d8 cf").check();
}

#[test]
fn test_fmul_st1_st0() {
    TestCase::from("dc c9").check();
}

#[test]
fn test_fmul_st2_st0() {
    TestCase::from("dc ca").check();
}

#[test]
fn test_fmul_st7_st0() {
    TestCase::from("dc cf").check();
}

// FMULP - Multiply and pop

#[test]
fn test_fmulp_st1_st0() {
    TestCase::from("de c9").check();
}

#[test]
fn test_fmulp_st2_st0() {
    TestCase::from("de ca").check();
}

#[test]
fn test_fmulp_st3_st0() {
    TestCase::from("de cb").check();
}

#[test]
fn test_fmulp_st7_st0() {
    TestCase::from("de cf").check();
}

#[test]
fn test_fmulp() {
    TestCase::from("de c9").check();
}

// FIMUL - Multiply by integer

#[test]
fn test_fimul_m16int() {
    TestCase::from("de 08").check();
}

#[test]
fn test_fimul_m32int() {
    TestCase::from("da 08").check();
}

#[test]
fn test_fimul_m16int_rax() {
    TestCase::from("de 08").check();
}

#[test]
fn test_fimul_m32int_rcx() {
    TestCase::from("da 09").check();
}

// FDIV/FDIVP/FIDIV - Divide

#[test]
fn test_fdiv_m32fp() {
    TestCase::from("d8 30").check();
}

#[test]
fn test_fdiv_m64fp() {
    TestCase::from("dc 30").check();
}

#[test]
fn test_fdiv_st0_st0() {
    TestCase::from("d8 f0").check();
}

#[test]
fn test_fdiv_st0_st1() {
    TestCase::from("d8 f1").check();
}

#[test]
fn test_fdiv_st0_st2() {
    TestCase::from("d8 f2").check();
}

#[test]
fn test_fdiv_st0_st7() {
    TestCase::from("d8 f7").check();
}

#[test]
fn test_fdiv_st1_st0() {
    TestCase::from("dc f9").check();
}

#[test]
fn test_fdiv_st2_st0() {
    TestCase::from("dc fa").check();
}

#[test]
fn test_fdiv_st7_st0() {
    TestCase::from("dc ff").check();
}

// FDIVP - Divide and pop

#[test]
fn test_fdivp_st1_st0() {
    TestCase::from("de f9").check();
}

#[test]
fn test_fdivp_st2_st0() {
    TestCase::from("de fa").check();
}

#[test]
fn test_fdivp_st3_st0() {
    TestCase::from("de fb").check();
}

#[test]
fn test_fdivp_st7_st0() {
    TestCase::from("de ff").check();
}

#[test]
fn test_fdivp() {
    TestCase::from("de f9").check();
}

// FIDIV - Divide by integer

#[test]
fn test_fidiv_m16int() {
    TestCase::from("de 30").check();
}

#[test]
fn test_fidiv_m32int() {
    TestCase::from("da 30").check();
}

#[test]
fn test_fidiv_m16int_rax() {
    TestCase::from("de 30").check();
}

#[test]
fn test_fidiv_m32int_rcx() {
    TestCase::from("da 31").check();
}

// FDIVR/FDIVRP/FIDIVR - Reverse divide

#[test]
fn test_fdivr_m32fp() {
    TestCase::from("d8 38").check();
}

#[test]
fn test_fdivr_m64fp() {
    TestCase::from("dc 38").check();
}

#[test]
fn test_fdivr_st0_st0() {
    TestCase::from("d8 f8").check();
}

#[test]
fn test_fdivr_st0_st1() {
    TestCase::from("d8 f9").check();
}

#[test]
fn test_fdivr_st0_st2() {
    TestCase::from("d8 fa").check();
}

#[test]
fn test_fdivr_st0_st7() {
    TestCase::from("d8 ff").check();
}

#[test]
fn test_fdivr_st1_st0() {
    TestCase::from("dc f1").check();
}

#[test]
fn test_fdivr_st2_st0() {
    TestCase::from("dc f2").check();
}

#[test]
fn test_fdivr_st7_st0() {
    TestCase::from("dc f7").check();
}

// FDIVRP - Reverse divide and pop

#[test]
fn test_fdivrp_st1_st0() {
    TestCase::from("de f1").check();
}

#[test]
fn test_fdivrp_st2_st0() {
    TestCase::from("de f2").check();
}

#[test]
fn test_fdivrp_st3_st0() {
    TestCase::from("de f3").check();
}

#[test]
fn test_fdivrp_st7_st0() {
    TestCase::from("de f7").check();
}

#[test]
fn test_fdivrp() {
    TestCase::from("de f1").check();
}

// FIDIVR - Reverse divide by integer

#[test]
fn test_fidivr_m16int() {
    TestCase::from("de 38").check();
}

#[test]
fn test_fidivr_m32int() {
    TestCase::from("da 38").check();
}

#[test]
fn test_fidivr_m16int_rax() {
    TestCase::from("de 38").check();
}

#[test]
fn test_fidivr_m32int_rcx() {
    TestCase::from("da 39").check();
}

// FSUB/FSUBP/FISUB - Subtract

#[test]
fn test_fsub_m32fp() {
    TestCase::from("d8 20").check();
}

#[test]
fn test_fsub_m64fp() {
    TestCase::from("dc 20").check();
}

#[test]
fn test_fsub_st0_st0() {
    TestCase::from("d8 e0").check();
}

#[test]
fn test_fsub_st0_st1() {
    TestCase::from("d8 e1").check();
}

#[test]
fn test_fsub_st0_st2() {
    TestCase::from("d8 e2").check();
}

#[test]
fn test_fsub_st0_st7() {
    TestCase::from("d8 e7").check();
}

#[test]
fn test_fsub_st1_st0() {
    TestCase::from("dc e9").check();
}

#[test]
fn test_fsub_st2_st0() {
    TestCase::from("dc ea").check();
}

#[test]
fn test_fsub_st7_st0() {
    TestCase::from("dc ef").check();
}

// FSUBP - Subtract and pop

#[test]
fn test_fsubp_st1_st0() {
    TestCase::from("de e9").check();
}

#[test]
fn test_fsubp_st2_st0() {
    TestCase::from("de ea").check();
}

#[test]
fn test_fsubp_st3_st0() {
    TestCase::from("de eb").check();
}

#[test]
fn test_fsubp_st7_st0() {
    TestCase::from("de ef").check();
}

#[test]
fn test_fsubp() {
    TestCase::from("de e9").check();
}

// FISUB - Subtract integer

#[test]
fn test_fisub_m16int() {
    TestCase::from("de 20").check();
}

#[test]
fn test_fisub_m32int() {
    TestCase::from("da 20").check();
}

#[test]
fn test_fisub_m16int_rax() {
    TestCase::from("de 20").check();
}

#[test]
fn test_fisub_m32int_rcx() {
    TestCase::from("da 21").check();
}

// FSUBR/FSUBRP/FISUBR - Reverse subtract

#[test]
fn test_fsubr_m32fp() {
    TestCase::from("d8 28").check();
}

#[test]
fn test_fsubr_m64fp() {
    TestCase::from("dc 28").check();
}

#[test]
fn test_fsubr_st0_st0() {
    TestCase::from("d8 e8").check();
}

#[test]
fn test_fsubr_st0_st1() {
    TestCase::from("d8 e9").check();
}

#[test]
fn test_fsubr_st0_st2() {
    TestCase::from("d8 ea").check();
}

#[test]
fn test_fsubr_st0_st7() {
    TestCase::from("d8 ef").check();
}

#[test]
fn test_fsubr_st1_st0() {
    TestCase::from("dc e1").check();
}

#[test]
fn test_fsubr_st2_st0() {
    TestCase::from("dc e2").check();
}

#[test]
fn test_fsubr_st7_st0() {
    TestCase::from("dc e7").check();
}

// FSUBRP - Reverse subtract and pop

#[test]
fn test_fsubrp_st1_st0() {
    TestCase::from("de e1").check();
}

#[test]
fn test_fsubrp_st2_st0() {
    TestCase::from("de e2").check();
}

#[test]
fn test_fsubrp_st3_st0() {
    TestCase::from("de e3").check();
}

#[test]
fn test_fsubrp_st7_st0() {
    TestCase::from("de e7").check();
}

#[test]
fn test_fsubrp() {
    TestCase::from("de e1").check();
}

// FISUBR - Reverse subtract integer

#[test]
fn test_fisubr_m16int() {
    TestCase::from("de 28").check();
}

#[test]
fn test_fisubr_m32int() {
    TestCase::from("da 28").check();
}

#[test]
fn test_fisubr_m16int_rax() {
    TestCase::from("de 28").check();
}

#[test]
fn test_fisubr_m32int_rcx() {
    TestCase::from("da 29").check();
}
