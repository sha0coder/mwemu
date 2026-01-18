use crate::*;
use crate::tests::rax_x86_tests::common::*;

// AVX-512 Mask Register Operations
// These operate on k0-k7 mask registers

// KMOVW/KMOVB/KMOVQ/KMOVD - Move mask values

#[test]
fn test_kmovw_k1_k2() {
    let mut emu = emu64();
    TestCase::from("c5 f8 90 ca").check();
}

#[test]
fn test_kmovw_k3_k4() {
    let mut emu = emu64();
    TestCase::from("c5 f8 90 dc").check();
}

#[test]
fn test_kmovw_k5_k6() {
    let mut emu = emu64();
    TestCase::from("c5 f8 90 ee").check();
}

#[test]
fn test_kmovw_k1_m16() {
    let mut emu = emu64();
    TestCase::from("c5 f8 90 08").check();
}

#[test]
fn test_kmovw_m16_k1() {
    let mut emu = emu64();
    TestCase::from("c5 f8 91 08").check();
}

#[test]
fn test_kmovw_k1_eax() {
    let mut emu = emu64();
    TestCase::from("c5 f8 92 c8").check();
}

#[test]
fn test_kmovw_eax_k1() {
    let mut emu = emu64();
    TestCase::from("c5 f8 93 c1").check();
}

#[test]
fn test_kmovb_k1_k2() {
    let mut emu = emu64();
    TestCase::from("c5 f9 90 ca").check();
}

#[test]
fn test_kmovb_k3_k4() {
    let mut emu = emu64();
    TestCase::from("c5 f9 90 dc").check();
}

#[test]
fn test_kmovb_k1_m8() {
    let mut emu = emu64();
    TestCase::from("c5 f9 90 08").check();
}

#[test]
fn test_kmovb_m8_k1() {
    let mut emu = emu64();
    TestCase::from("c5 f9 91 08").check();
}

#[test]
fn test_kmovb_k1_eax() {
    let mut emu = emu64();
    TestCase::from("c5 f9 92 c8").check();
}

#[test]
fn test_kmovb_eax_k1() {
    let mut emu = emu64();
    TestCase::from("c5 f9 93 c1").check();
}

#[test]
fn test_kmovq_k1_k2() {
    let mut emu = emu64();
    TestCase::from("c4 e1 f8 90 ca").check();
}

#[test]
fn test_kmovq_k1_m64() {
    let mut emu = emu64();
    TestCase::from("c4 e1 f8 90 08").check();
}

#[test]
fn test_kmovq_m64_k1() {
    let mut emu = emu64();
    TestCase::from("c4 e1 f8 91 08").check();
}

#[test]
fn test_kmovq_k1_rax() {
    let mut emu = emu64();
    TestCase::from("c4 e1 fb 92 c8").check();
}

#[test]
fn test_kmovq_rax_k1() {
    let mut emu = emu64();
    TestCase::from("c4 e1 fb 93 c1").check();
}

#[test]
fn test_kmovd_k1_k2() {
    let mut emu = emu64();
    TestCase::from("c5 fb 90 ca").check();
}

#[test]
fn test_kmovd_k1_m32() {
    let mut emu = emu64();
    TestCase::from("c5 fb 90 08").check();
}

#[test]
fn test_kmovd_m32_k1() {
    let mut emu = emu64();
    TestCase::from("c5 fb 91 08").check();
}

#[test]
fn test_kmovd_k1_eax() {
    let mut emu = emu64();
    TestCase::from("c5 fb 92 c8").check();
}

#[test]
fn test_kmovd_eax_k1() {
    let mut emu = emu64();
    TestCase::from("c5 fb 93 c1").check();
}

// KANDW/KANDB/KANDQ/KANDD - Bitwise AND

#[test]
fn test_kandw_k1_k2_k3() {
    let mut emu = emu64();
    TestCase::from("c5 ec 41 cb").check();
}

#[test]
fn test_kandw_k4_k5_k6() {
    let mut emu = emu64();
    TestCase::from("c5 d4 41 e6").check();
}

#[test]
fn test_kandw_k0_k1_k2() {
    let mut emu = emu64();
    TestCase::from("c5 f4 41 c2").check();
}

#[test]
fn test_kandb_k1_k2_k3() {
    let mut emu = emu64();
    TestCase::from("c5 ed 41 cb").check();
}

#[test]
fn test_kandb_k4_k5_k6() {
    let mut emu = emu64();
    TestCase::from("c5 d5 41 e6").check();
}

#[test]
fn test_kandq_k1_k2_k3() {
    let mut emu = emu64();
    TestCase::from("c4 e1 ec 41 cb").check();
}

#[test]
fn test_kandq_k4_k5_k6() {
    let mut emu = emu64();
    TestCase::from("c4 e1 d4 41 e6").check();
}

#[test]
fn test_kandd_k1_k2_k3() {
    let mut emu = emu64();
    TestCase::from("c5 ed 41 cb").check();
}

#[test]
fn test_kandd_k4_k5_k6() {
    let mut emu = emu64();
    TestCase::from("c5 d5 41 e6").check();
}

// KANDNW/KANDNB/KANDNQ/KANDND - Bitwise AND NOT

#[test]
fn test_kandnw_k1_k2_k3() {
    let mut emu = emu64();
    TestCase::from("c5 ec 42 cb").check();
}

#[test]
fn test_kandnw_k4_k5_k6() {
    let mut emu = emu64();
    TestCase::from("c5 d4 42 e6").check();
}

#[test]
fn test_kandnb_k1_k2_k3() {
    let mut emu = emu64();
    TestCase::from("c5 ed 42 cb").check();
}

#[test]
fn test_kandnb_k4_k5_k6() {
    let mut emu = emu64();
    TestCase::from("c5 d5 42 e6").check();
}

#[test]
fn test_kandnq_k1_k2_k3() {
    let mut emu = emu64();
    TestCase::from("c4 e1 ec 42 cb").check();
}

#[test]
fn test_kandnq_k4_k5_k6() {
    let mut emu = emu64();
    TestCase::from("c4 e1 d4 42 e6").check();
}

#[test]
fn test_kandnd_k1_k2_k3() {
    let mut emu = emu64();
    TestCase::from("c5 ed 42 cb").check();
}

#[test]
fn test_kandnd_k4_k5_k6() {
    let mut emu = emu64();
    TestCase::from("c5 d5 42 e6").check();
}

// KORW/KORB/KORQ/KORD - Bitwise OR

#[test]
fn test_korw_k1_k2_k3() {
    let mut emu = emu64();
    TestCase::from("c5 ec 45 cb").check();
}

#[test]
fn test_korw_k4_k5_k6() {
    let mut emu = emu64();
    TestCase::from("c5 d4 45 e6").check();
}

#[test]
fn test_korb_k1_k2_k3() {
    let mut emu = emu64();
    TestCase::from("c5 ed 45 cb").check();
}

#[test]
fn test_korb_k4_k5_k6() {
    let mut emu = emu64();
    TestCase::from("c5 d5 45 e6").check();
}

#[test]
fn test_korq_k1_k2_k3() {
    let mut emu = emu64();
    TestCase::from("c4 e1 ec 45 cb").check();
}

#[test]
fn test_korq_k4_k5_k6() {
    let mut emu = emu64();
    TestCase::from("c4 e1 d4 45 e6").check();
}

#[test]
fn test_kord_k1_k2_k3() {
    let mut emu = emu64();
    TestCase::from("c5 ed 45 cb").check();
}

#[test]
fn test_kord_k4_k5_k6() {
    let mut emu = emu64();
    TestCase::from("c5 d5 45 e6").check();
}

// KXORW/KXORB/KXORQ/KXORD - Bitwise XOR

#[test]
fn test_kxorw_k1_k2_k3() {
    let mut emu = emu64();
    TestCase::from("c5 ec 47 cb").check();
}

#[test]
fn test_kxorw_k4_k5_k6() {
    let mut emu = emu64();
    TestCase::from("c5 d4 47 e6").check();
}

#[test]
fn test_kxorb_k1_k2_k3() {
    let mut emu = emu64();
    TestCase::from("c5 ed 47 cb").check();
}

#[test]
fn test_kxorb_k4_k5_k6() {
    let mut emu = emu64();
    TestCase::from("c5 d5 47 e6").check();
}

#[test]
fn test_kxorq_k1_k2_k3() {
    let mut emu = emu64();
    TestCase::from("c4 e1 ec 47 cb").check();
}

#[test]
fn test_kxorq_k4_k5_k6() {
    let mut emu = emu64();
    TestCase::from("c4 e1 d4 47 e6").check();
}

#[test]
fn test_kxord_k1_k2_k3() {
    let mut emu = emu64();
    TestCase::from("c5 ed 47 cb").check();
}

#[test]
fn test_kxord_k4_k5_k6() {
    let mut emu = emu64();
    TestCase::from("c5 d5 47 e6").check();
}

// KXNORW/KXNORB/KXNORQ/KXNORD - Bitwise XNOR

#[test]
fn test_kxnorw_k1_k2_k3() {
    let mut emu = emu64();
    TestCase::from("c5 ec 46 cb").check();
}

#[test]
fn test_kxnorw_k4_k5_k6() {
    let mut emu = emu64();
    TestCase::from("c5 d4 46 e6").check();
}

#[test]
fn test_kxnorb_k1_k2_k3() {
    let mut emu = emu64();
    TestCase::from("c5 ed 46 cb").check();
}

#[test]
fn test_kxnorb_k4_k5_k6() {
    let mut emu = emu64();
    TestCase::from("c5 d5 46 e6").check();
}

#[test]
fn test_kxnorq_k1_k2_k3() {
    let mut emu = emu64();
    TestCase::from("c4 e1 ec 46 cb").check();
}

#[test]
fn test_kxnorq_k4_k5_k6() {
    let mut emu = emu64();
    TestCase::from("c4 e1 d4 46 e6").check();
}

#[test]
fn test_kxnord_k1_k2_k3() {
    let mut emu = emu64();
    TestCase::from("c5 ed 46 cb").check();
}

#[test]
fn test_kxnord_k4_k5_k6() {
    let mut emu = emu64();
    TestCase::from("c5 d5 46 e6").check();
}

// KNOTW/KNOTB/KNOTQ/KNOTD - Bitwise NOT

#[test]
fn test_knotw_k1_k2() {
    let mut emu = emu64();
    TestCase::from("c5 f8 44 ca").check();
}

#[test]
fn test_knotw_k3_k4() {
    let mut emu = emu64();
    TestCase::from("c5 f8 44 dc").check();
}

#[test]
fn test_knotb_k1_k2() {
    let mut emu = emu64();
    TestCase::from("c5 f9 44 ca").check();
}

#[test]
fn test_knotb_k3_k4() {
    let mut emu = emu64();
    TestCase::from("c5 f9 44 dc").check();
}

#[test]
fn test_knotq_k1_k2() {
    let mut emu = emu64();
    TestCase::from("c4 e1 f8 44 ca").check();
}

#[test]
fn test_knotq_k3_k4() {
    let mut emu = emu64();
    TestCase::from("c4 e1 f8 44 dc").check();
}

#[test]
fn test_knotd_k1_k2() {
    let mut emu = emu64();
    TestCase::from("c5 fb 44 ca").check();
}

#[test]
fn test_knotd_k3_k4() {
    let mut emu = emu64();
    TestCase::from("c5 fb 44 dc").check();
}

// KORTESTW/KORTESTB/KORTESTQ/KORTESTD - Bitwise OR and Set Flags

#[test]
fn test_kortestw_k1_k2() {
    let mut emu = emu64();
    TestCase::from("c5 f8 98 ca").check();
}

#[test]
fn test_kortestw_k3_k4() {
    let mut emu = emu64();
    TestCase::from("c5 f8 98 dc").check();
}

#[test]
fn test_kortestb_k1_k2() {
    let mut emu = emu64();
    TestCase::from("c5 f9 98 ca").check();
}

#[test]
fn test_kortestb_k3_k4() {
    let mut emu = emu64();
    TestCase::from("c5 f9 98 dc").check();
}

#[test]
fn test_kortestq_k1_k2() {
    let mut emu = emu64();
    TestCase::from("c4 e1 f8 98 ca").check();
}

#[test]
fn test_kortestq_k3_k4() {
    let mut emu = emu64();
    TestCase::from("c4 e1 f8 98 dc").check();
}

#[test]
fn test_kortestd_k1_k2() {
    let mut emu = emu64();
    TestCase::from("c5 fb 98 ca").check();
}

#[test]
fn test_kortestd_k3_k4() {
    let mut emu = emu64();
    TestCase::from("c5 fb 98 dc").check();
}

// KTESTW/KTESTB/KTESTQ/KTESTD - Packed Bit Test and Set Flags

#[test]
fn test_ktestw_k1_k2() {
    let mut emu = emu64();
    TestCase::from("c5 f8 99 ca").check();
}

#[test]
fn test_ktestw_k3_k4() {
    let mut emu = emu64();
    TestCase::from("c5 f8 99 dc").check();
}

#[test]
fn test_ktestb_k1_k2() {
    let mut emu = emu64();
    TestCase::from("c5 f9 99 ca").check();
}

#[test]
fn test_ktestb_k3_k4() {
    let mut emu = emu64();
    TestCase::from("c5 f9 99 dc").check();
}

#[test]
fn test_ktestq_k1_k2() {
    let mut emu = emu64();
    TestCase::from("c4 e1 f8 99 ca").check();
}

#[test]
fn test_ktestq_k3_k4() {
    let mut emu = emu64();
    TestCase::from("c4 e1 f8 99 dc").check();
}

#[test]
fn test_ktestd_k1_k2() {
    let mut emu = emu64();
    TestCase::from("c5 fb 99 ca").check();
}

#[test]
fn test_ktestd_k3_k4() {
    let mut emu = emu64();
    TestCase::from("c5 fb 99 dc").check();
}

// KADDW/KADDB/KADDQ/KADDD - Add mask values

#[test]
fn test_kaddw_k1_k2_k3() {
    let mut emu = emu64();
    TestCase::from("c5 ec 4a cb").check();
}

#[test]
fn test_kaddw_k4_k5_k6() {
    let mut emu = emu64();
    TestCase::from("c5 d4 4a e6").check();
}

#[test]
fn test_kaddb_k1_k2_k3() {
    let mut emu = emu64();
    TestCase::from("c5 ed 4a cb").check();
}

#[test]
fn test_kaddb_k4_k5_k6() {
    let mut emu = emu64();
    TestCase::from("c5 d5 4a e6").check();
}

#[test]
fn test_kaddq_k1_k2_k3() {
    let mut emu = emu64();
    TestCase::from("c4 e1 ec 4a cb").check();
}

#[test]
fn test_kaddq_k4_k5_k6() {
    let mut emu = emu64();
    TestCase::from("c4 e1 d4 4a e6").check();
}

#[test]
fn test_kaddd_k1_k2_k3() {
    let mut emu = emu64();
    TestCase::from("c5 ed 4a cb").check();
}

#[test]
fn test_kaddd_k4_k5_k6() {
    let mut emu = emu64();
    TestCase::from("c5 d5 4a e6").check();
}

// KSHIFTLW/KSHIFTLB/KSHIFTLQ/KSHIFTLD - Shift left

#[test]
fn test_kshiftlw_k1_k2_imm() {
    let mut emu = emu64();
    TestCase::from("c4 e3 f9 32 ca 04").check();
}

#[test]
fn test_kshiftlw_k3_k4_imm() {
    let mut emu = emu64();
    TestCase::from("c4 e3 f9 32 dc 08").check();
}

#[test]
fn test_kshiftlb_k1_k2_imm() {
    let mut emu = emu64();
    TestCase::from("c4 e3 79 32 ca 02").check();
}

#[test]
fn test_kshiftlb_k3_k4_imm() {
    let mut emu = emu64();
    TestCase::from("c4 e3 79 32 dc 04").check();
}

#[test]
fn test_kshiftlq_k1_k2_imm() {
    let mut emu = emu64();
    TestCase::from("c4 e3 f9 33 ca 10").check();
}

#[test]
fn test_kshiftlq_k3_k4_imm() {
    let mut emu = emu64();
    TestCase::from("c4 e3 f9 33 dc 20").check();
}

#[test]
fn test_kshiftld_k1_k2_imm() {
    let mut emu = emu64();
    TestCase::from("c4 e3 79 33 ca 08").check();
}

#[test]
fn test_kshiftld_k3_k4_imm() {
    let mut emu = emu64();
    TestCase::from("c4 e3 79 33 dc 10").check();
}

// KSHIFTRW/KSHIFTRB/KSHIFTRQ/KSHIFTRD - Shift right

#[test]
fn test_kshiftrw_k1_k2_imm() {
    let mut emu = emu64();
    TestCase::from("c4 e3 f9 30 ca 04").check();
}

#[test]
fn test_kshiftrw_k3_k4_imm() {
    let mut emu = emu64();
    TestCase::from("c4 e3 f9 30 dc 08").check();
}

#[test]
fn test_kshiftrb_k1_k2_imm() {
    let mut emu = emu64();
    TestCase::from("c4 e3 79 30 ca 02").check();
}

#[test]
fn test_kshiftrb_k3_k4_imm() {
    let mut emu = emu64();
    TestCase::from("c4 e3 79 30 dc 04").check();
}

#[test]
fn test_kshiftrq_k1_k2_imm() {
    let mut emu = emu64();
    TestCase::from("c4 e3 f9 31 ca 10").check();
}

#[test]
fn test_kshiftrq_k3_k4_imm() {
    let mut emu = emu64();
    TestCase::from("c4 e3 f9 31 dc 20").check();
}

#[test]
fn test_kshiftrd_k1_k2_imm() {
    let mut emu = emu64();
    TestCase::from("c4 e3 79 31 ca 08").check();
}

#[test]
fn test_kshiftrd_k3_k4_imm() {
    let mut emu = emu64();
    TestCase::from("c4 e3 79 31 dc 10").check();
}

// KUNPCKBW/KUNPCKWD/KUNPCKDQ - Unpack mask values

#[test]
fn test_kunpckbw_k1_k2_k3() {
    let mut emu = emu64();
    TestCase::from("c5 ec 4b cb").check();
}

#[test]
fn test_kunpckbw_k4_k5_k6() {
    let mut emu = emu64();
    TestCase::from("c5 d4 4b e6").check();
}

#[test]
fn test_kunpckwd_k1_k2_k3() {
    let mut emu = emu64();
    TestCase::from("c5 ec 4b cb").check();
}

#[test]
fn test_kunpckwd_k4_k5_k6() {
    let mut emu = emu64();
    TestCase::from("c5 d4 4b e6").check();
}

#[test]
fn test_kunpckdq_k1_k2_k3() {
    let mut emu = emu64();
    TestCase::from("c4 e1 ec 4b cb").check();
}

#[test]
fn test_kunpckdq_k4_k5_k6() {
    let mut emu = emu64();
    TestCase::from("c4 e1 d4 4b e6").check();
}
