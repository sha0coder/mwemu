mod vmovaps_zmm;
mod vmovups_zmm;
mod vaddps_zmm;
mod vsubps_zmm;
mod vmulps_zmm;
mod vdivps_zmm;
mod kmov;
mod kand_kor_kxor;
mod kadd_mask;
mod kandn_knot_mask;
mod ktest_kunpck_kshift;

// AVX-512 FP16 Instructions
mod vaddph_vsubph_vmulph_vdivph;

// AVX-512 Compress/Expand Instructions
mod vcompress_vexpand;

// AVX-512 Bit Manipulation Instructions
mod valign_vprol_vpror_vpternlog;

// AVX-512 Specialized Instructions
mod vdbpsadbw_vplzcnt_vpshld;
