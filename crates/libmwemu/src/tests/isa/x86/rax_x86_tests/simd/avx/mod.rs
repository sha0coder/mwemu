mod vmovaps_vmovapd;
mod vmovups_vmovupd;
mod vmovdqa_vmovdqu;
mod vbroadcastss_vbroadcastsd;
mod vextractf128_vinsertf128;
mod vshufps_vshufpd;
mod vblendps_vblendpd;
mod vperm2f128;
mod vunpcklps_vunpcklpd;
mod vunpckhps_vunpckhpd;
mod vextractf128;
mod vinsertf128;
mod vcmpps_vcmppd;
mod vandps_vandpd;
mod vorps_vorpd;
mod vxorps_vxorpd;
mod vandnps_vandnpd;
mod vhaddps_vhaddpd;
mod vhsubps_vhsubpd;
mod vaddps_vaddpd;
mod vsubps_vsubpd;
mod vmulps_vmulpd;
mod vdivps_vdivpd;
mod vsqrtps_vsqrtpd;
mod vminps_vminpd;
mod vmaxps_vmaxpd;
mod vzeroupper_vzeroall;
mod vldmxcsr_vstmxcsr;
mod vmovmskps_vmovmskpd;
mod vmaskmovps_vmaskmovpd;
mod vtestps_vtestpd;
mod vrcpps;
mod vrsqrtps;
mod vdpps;
mod vcvtps2pd_vcvtpd2ps;
mod vcvtdq2ps_vcvtps2dq;
mod vcvtdq2pd_vcvtpd2dq;
mod vcvttps2dq_vcvttpd2dq;
mod vcvtss2sd_vcvtsd2ss;
mod vcvtsi2ss_vcvtsi2sd;
mod vcvtss2si_vcvtsd2si;
mod vcvttss2si_vcvttsd2si;

// AVX Permute Instructions
mod vpermilps;
mod vpermilpd;

// AVX Blend Instructions
mod vblendvps;
mod vblendvpd;

// AVX Dot Product
mod vdppd;

// AVX Scalar Arithmetic
mod vaddss_vaddsd;
mod vsubss_vsubsd;
mod vmulss_vmulsd;
mod vdivss_vdivsd;

// AVX Add/Sub Packed
mod vaddsubps_vaddsubpd;

// AVX Rounding
mod vroundps;
mod vroundpd;
mod vroundss;
mod vroundsd;

// AVX Comparison
mod vcomisd;
mod vcomiss;
mod vucomisd;
mod vucomiss;

// AVX Scalar Move
mod vmovss;
mod vmovsd;

// AVX Move Variants
mod vmovhlps;
mod vmovlhps;
mod vmovlps;
mod vmovhps;
mod vmovlpd;
mod vmovhpd;

// AVX Duplicate Move
mod vmovshdup;
mod vmovsldup;
mod vmovddup;

// AVX Non-Temporal Move
mod vmovntps;
mod vmovntpd;
mod vmovntdq;
mod vmovntdqa;

// AVX FMA (Fused Multiply-Add) Instructions
mod vfmadd132ps;
mod vfmadd213ps;
mod vfmadd231ps;
mod vfmadd132pd;
mod vfmadd213pd;
mod vfmadd231pd;
mod vfmsub132ps;
mod vfmsub213ps;
mod vfmsub231ps;
mod vfmsub132pd;
mod vfmsub213pd;
mod vfmsub231pd;
mod vfnmadd132ps;
mod vfnmadd213ps;
mod vfnmadd231ps;
mod vfnmadd132pd;
mod vfnmadd213pd;
mod vfnmadd231pd;
mod vfnmsub132ps;
mod vfnmsub213ps;
mod vfnmsub231ps;
mod vfnmsub132pd;
mod vfnmsub213pd;
mod vfnmsub231pd;
