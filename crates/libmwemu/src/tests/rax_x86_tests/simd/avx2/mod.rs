mod vpaddb_vpaddw_vpaddd_vpaddq;
mod vpsubb_vpsubw_vpsubd_vpsubq;
mod vpmullw_vpmulld;
mod vpmulhw_vpmulhuw;
mod vpand_vpor_vpxor;
mod vpcmpeqb_vpcmpeqw_vpcmpeqd_vpcmpeqq;
mod vpshufb;
mod vpsllw_vpslld_vpsllq;
mod vpsrlw_vpsrld_vpsrlq;
mod vpsraw_vpsrad;
mod vpcmpgtb_vpcmpgtw_vpcmpgtd_vpcmpgtq;
mod vpacksswb_vpackssdw;
mod vpackuswb_vpackusdw;
mod vpunpcklbw_vpunpcklwd_vpunpckldq_vpunpcklqdq;
mod vpunpckhbw_vpunpckhwd_vpunpckhdq_vpunpckhqdq;
mod vpabsb_vpabsw_vpabsd;
mod vphaddw_vphaddd;
mod vphsubw_vphsubd;
mod vphaddsw_vphsubsw;
mod vpsignb_vpsignw_vpsignd;
mod vpmulhrsw;
mod vpalignr;
mod vpmovsxbw_vpmovsxbd_vpmovsxbq;
mod vpminsb_vpminsw_vpminsd;
mod vpminub_vpminuw_vpminud;
mod vpmaxsb_vpmaxsw_vpmaxsd;
mod vpmaxub_vpmaxuw_vpmaxud;
mod vpavgb_vpavgw;
mod vpsadbw;
mod vpmaddwd;
mod vpmaddubsw;
mod vpmovzxbw_vpmovzxbd_vpmovzxbq;
mod vpmovzxwd_vpmovzxwq_vpmovzxdq;
mod vpmovsx_variants;
mod vptest;
mod vpsllvd_vpsllvq;
mod vpsrlvd_vpsrlvq;
mod vpsravd;
mod vpmaskmovd_vpmaskmovq;
mod vpshufd;
mod vpshufhw;
mod vpshuflw;
mod vextracti128;
mod vinserti128;
mod vperm2i128;
mod vpermd_vpermq;
mod vpgatherdd_vpgatherdq;
mod vpgatherqd_vpgatherqq;
mod vgatherdps_vgatherdpd;
mod vgatherqps_vgatherqpd;
mod vpbroadcastb_vpbroadcastw;
mod vpbroadcastd_vpbroadcastq;
mod vbroadcasti128;
mod vpblendd;

// AVX2 Permute Instructions
mod vpermps;
mod vpermpd;

// AVX2 Saturating Arithmetic
mod vpaddsb;
mod vpaddsw;
mod vpaddusb;
mod vpaddusw;
mod vpsubsb;
mod vpsubsw;
mod vpsubusb;
mod vpsubusw;

// AVX2 Multiply Operations
mod vpmuldq;
mod vpmuludq;

// AVX2 Logical Operations
mod vpandn;

// AVX2 Shift Byte Operations
mod vpslldq;
mod vpsrldq;

// AVX2 Mask Operations
mod vpmovmskb;

// AVX2 Blend Operations
mod vpblendvb;
mod vpblendw;

// AVX2 Misc Operations
mod vphminposuw;
mod vmpsadbw;
