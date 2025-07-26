// cargo test -- --nocapture

#[cfg(test)]
mod tests {
    use std::io::Write as _;
    use iced_x86::Register;
    use crate::regs64::U256;
    use crate::maps::mem64::Mem64;
    use std::sync::Once;
    use crate::constants;
    use crate::winapi64;
    use crate::winapi32;
    use crate::emu::Emu;
    use crate::fpu::FPU;
    use crate::fpu::f80::F80;
    use crate::engine::logic;
    use crate::emu64;
    use crate::emu32;
    use crate::serialization::Serialization;
    use crate::structures;
    use std::process::Command;

    static INIT: Once = Once::new();

    fn setup() {
        INIT.call_once(|| {
            env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("error"))
                .format(|buf, record| writeln!(buf, "{}", record.args()))
                .init();
        });
    }

    #[test]
    fn regsisters_test() {
        setup();

        let mut emu = emu64();
        let regs = &mut emu.regs;

        // ====== 1. Direct 64 bits registers ======
        regs.rax = 0x123456789ABCDEF0;
        assert_eq!(regs.rax, 0x123456789ABCDEF0);

        regs.rbx = 0xCAFEBABECAFEBABE;
        assert_eq!(regs.rbx, 0xCAFEBABECAFEBABE);

        regs.rsp = 0x7FFF_FFFF_FFFF;
        regs.rbp = 0x5555_5555_5555;
        assert_eq!(regs.rsp, 0x7FFF_FFFF_FFFF);
        assert_eq!(regs.rbp, 0x5555_5555_5555);

        regs.cr0 = 0x80000011;
        assert_eq!(regs.cr0, 0x80000011);

        regs.msr = 0xDEADBEEF;
        assert_eq!(regs.msr, 0xDEADBEEF);

        regs.tr3 = 0x1234;
        assert_eq!(regs.tr3, 0x1234);

        regs.gs = 0xABCD;
        regs.fs = 0xDCBA;
        assert_eq!(regs.gs, 0xABCD);
        assert_eq!(regs.fs, 0xDCBA);

        // ====== 2. subregisters using methods ======
        regs.set_ax(0xBEEF);
        assert_eq!(regs.get_ax(), 0xBEEF);
        assert_eq!(regs.get_al(), 0xEF);
        assert_eq!(regs.get_ah(), 0xBE);

        regs.set_al(0x44);
        assert_eq!(regs.get_ax(), 0xBE44);
        assert_eq!(regs.get_al(), 0x44);

        regs.set_ah(0x22);
        assert_eq!(regs.get_ax(), 0x2244);
        assert_eq!(regs.get_ah(), 0x22);

        regs.set_eax(0x11223344);
        assert_eq!(regs.get_eax(), 0x11223344);
        assert_eq!(regs.get_ax(), 0x3344);
        assert_eq!(regs.get_al(), 0x44);

        regs.set_r8d(0x55667788);
        assert_eq!(regs.get_r8d(), 0x55667788);
        regs.set_r8w(0x99AA);
        assert_eq!(regs.get_r8w(), 0x99AA);
        regs.set_r8l(0xBB);
        assert_eq!(regs.get_r8l(), 0xBB);
        regs.set_r8h(0xCC);
        assert_eq!(regs.get_r8h(), 0xCC);

        // ====== 3. access by register name ======
        regs.set_by_name("eax", 0xAABBCCDD);
        assert_eq!(regs.get_by_name("eax"), 0xAABBCCDD);
        regs.set_by_name("al", 0xEE);
        assert_eq!(regs.get_by_name("al"), 0xEE);
        assert_eq!(regs.get_eax() & 0xFF, 0xEE);

        // ====== 4. XMM ======
        let xmm_val: u128 = 0x112233445566778899AABBCCDDEEFF00;
        assert!(regs.is_xmm(Register::XMM1));
        regs.set_xmm_reg(Register::XMM1, xmm_val);
        assert_eq!(regs.get_xmm_reg(Register::XMM1), xmm_val);
        regs.set_xmm_by_name("xmm1", xmm_val);
        assert_eq!(regs.get_xmm_by_name("xmm1"), xmm_val);

        // ====== 5. YMM ======
        let ymm_val = U256::from_big_endian(&[
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77,
            0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF,
            0xFF, 0xEE, 0xDD, 0xCC, 0xBB, 0xAA, 0x99, 0x88,
            0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11, 0x00,
        ]);
        assert!(regs.is_ymm(Register::YMM1));
        regs.set_ymm_reg(Register::YMM1, ymm_val);
        assert_eq!(regs.get_ymm_reg(Register::YMM1), ymm_val);
        regs.set_ymm_by_name("ymm1", ymm_val);
        assert_eq!(regs.get_ymm_by_name("ymm1"), ymm_val);

        // ====== 6. Auxiliar methods ======
        regs.set_reg(Register::RAX, 0xCAFED00DDEADBEEF);
        assert_eq!(regs.get_reg(Register::RAX), 0xCAFED00DDEADBEEF);

        assert!(regs.is_reg("rax"));
        assert!(!regs.is_reg("xyz"));

        assert!(regs.is_xmm_by_name("xmm2"));
        assert!(regs.is_ymm_by_name("ymm3"));

        assert_eq!(regs.get_size(Register::RAX), 64);
        assert_eq!(regs.get_size(Register::AL), 8);

        // ====== 7. sanitize32 (should remove high part) ======
        regs.rax = 0xAABBCCDDEEFF7788;
        regs.sanitize32();
        assert_eq!(regs.rax, 0x00000000EEFF7788);

        // ====== 8. rand & clear ======
        regs.rand();
        let r1 = regs.rax;
        regs.rand();
        let r2 = regs.rax;
        assert_ne!(r1, r2); // rand should change something

        regs.clear::<64>();
        assert_eq!(regs.rax, 0);
        assert_eq!(regs.rbx, 0);
        assert_eq!(regs.rcx, 0);

    }

    #[test]
    fn fpu_f80_emulation() {
        let mut f80 = F80::new();

        f80.st = (16383u128 << 64) | (1u128 << 63);
        assert_eq!(f80.to_integer_u128(), 1);

        f80.set_f64(1.0);
        assert_eq!(f80.get(), 0x3fff8000000000000000);


        // Test zero
        f80.set_f64(0.0);
        assert!(f80.is_zero());
        assert_eq!(f80.get_f64(), 0.0);

        f80.set_f64(-0.0);
        assert!(f80.is_zero());
        assert_eq!(f80.get_f64(), -0.0);

        // Test infinity
        f80.set_f64(f64::INFINITY);
        assert!(f80.is_infinite());
        assert_eq!(f80.get_f64(), f64::INFINITY);

        f80.set_f64(f64::NEG_INFINITY);
        assert!(f80.is_infinite());
        assert_eq!(f80.get_f64(), f64::NEG_INFINITY);

        // Test NaN
        f80.set_f64(f64::NAN);
        assert!(f80.is_nan());
        assert!(f80.get_f64().is_nan());

        // Test normal numbers roundtrip with tolerance
        let test_values = [1.0, -1.0, 3.141592653589793, -2.718281828459045, 1e10, -1e-10];

        for &val in &test_values {
            f80.set_f64(val);
            let back = f80.get_f64();
            let diff = (val - back).abs();
            assert!(diff < 1e-12, "val: {}, got: {}", val, back);
        }

        // Test flags negative checks
        f80.set_f64(42.0);
        assert!(!f80.is_nan());
        assert!(!f80.is_infinite());
        assert!(!f80.is_zero());

        let test_values = [
            0u128,
            1,
            9,
            10,
            42,
            12345,
            99999999,
            12345678901234567890u128, // big num
        ];

        for &val in &test_values {
            f80.set(val);

            // Conver to BCD packed and reconstruct
            let bcd = f80.to_bcd_packed();
            let mut f80_2 = F80::new();
            f80_2.from_bcd_packed(&bcd);

            assert_eq!(
                f80.to_integer_u128(),
                f80_2.to_integer_u128(),
                "BCD roundtrip: valor entero no coincide para valor {}",
                val
            );
            assert!((f80.get_f64() - f80_2.get_f64()).abs() < 1e-10, "BCD roundtrip no coincide para valor {}", val);
        }

        f80.set_f64(259.0);
        let bcd = f80.to_bcd_packed();

        assert_eq!(bcd.len(), 10);
        assert_eq!(bcd[0], 0x59);
        assert_eq!(bcd[1], 0x02);

        f80.st = F80::encode_from_u128(259, false);
        let bcd = f80.to_bcd_packed();

        assert_eq!(bcd[0], 0x59);
        assert_eq!(bcd[1], 0x02);

        let mut f80 = F80::new();
        let val:u128 = 256;
        f80.set(val);

        let bytes = f80.get_bytes();
        let mut f80_2 = F80::new();
        f80_2.set_bytes(&bytes);

        assert_eq!(f80.get(), f80_2.get(), "Error en get() para valor {}", val);
        assert_eq!(f80.to_integer_u128(), f80_2.to_integer_u128(), "Error en to_integer_u128 para valor {}", val);

        let bcd1 = f80.to_bcd_packed();
        let bcd2 = f80_2.to_bcd_packed();
        assert_eq!(bcd1, bcd2, "Error en BCD packed para valor {}", val);


        // test a.add(b)
        
        let mut b:F80 = F80::new();
        f80.set_f64(-1.1);
        b.set_f64(1.9);
        f80.add(b);
        assert_eq!(f80.get_f64(), 0.7999999999999998);
        assert_eq!(f80.get_round_f64(4), 0.8);
        assert_eq!(f80.get(), 0x3ffeccccccccccccc000);

        f80.set_f64(1.0);
        b.set_f64(2.0);
        f80.sub(b);
        assert_eq!(f80.get_f64(), -1.0);
    }

    #[test]
    // this tests windows 32bits shellcodes, and fetching apis and doing some api calls, pointing
    // to strings etc.
    fn sc32win_peb_ldr_rot() {
        setup();

        let mut emu = emu32();
        emu.cfg.maps_folder = "../maps32/".to_string();

        let sample = "../test/sc32win_peb_ldr_rot.bin";
        emu.load_code(sample);
        emu.run(Some(0x3c0116));

        let ptr = emu.regs.get_ebx();
        assert_eq!(ptr, 0x3c01b8);
        let s: String = emu.maps.read_string(ptr);
        assert!(s.starts_with("Host: msn.com"));
    }

    #[test]
    // this tests the arithmetics of an obfuscated windos 32bits shellcode.
    // also tests reading string from memory.
    fn sc32win_veryobfus() {
        setup();

        let mut emu = emu32();
        emu.cfg.maps_folder = "../maps32/".to_string();

        let sample = "../test/sc32win_veryobfus.bin";
        emu.load_code(sample);
        emu.run(Some(0x3cfaa5));

        let ptr_ntdll_str = emu.regs.get_edi();
        let ntdll_str = emu.maps.read_string(ptr_ntdll_str);

        assert!(ntdll_str.starts_with("ntdll"));

        let eax = emu.regs.get_eax(); // ptr to ntdll.text

        let name = match emu.maps.get_addr_name(eax) {
            Some(n) => n,
            None => {
                return assert_eq!(1,2);
            }
        };

        assert_eq!(name, "ntdll.text");
    }

    #[test]
    // this tests a windows 64bits shellcode, and pointing o sockaddr structure.
    // also tests steps.
    fn sc64win_metasploit() {
        setup();

        let mut emu = emu64();
        emu.cfg.maps_folder = "../maps64/".to_string();

        let sample = "../test/sc64win_metasploit.bin";
        emu.load_code(sample);
        //emu.set_verbose(3);
        emu.run(Some(0x3c00c8));
        emu.step();
        emu.run(Some(0x3c00c8));
        emu.step();
        emu.run(Some(0x3c00c8));
        emu.step();
        emu.run(Some(0x3c00c8));
        //emu.spawn_console();

        let stack = emu.regs.rsp;
        let sockaddr_ptr = emu.maps.read_qword(stack + 8).unwrap();
        let sockaddr = emu.maps.read_qword(sockaddr_ptr).unwrap();

        assert_eq!(sockaddr,  0x12c190a5c110002);
    }

    #[test]
    // this test a windows 64bits executable that calculates apis like shellcodes and does basic api calls.
    // aso read strings and patch string.
    fn exe64win_msgbox() {
        setup();

        let mut emu = emu64();
        emu.cfg.maps_folder = "../maps64/".to_string();

        let sample = "../test/exe64win_msgbox.bin";
        emu.load_code(sample);
        emu.run(Some(0x14000123f));

        let message = emu.maps.read_string(emu.regs.rdx);
        let title = emu.maps.read_string(emu.regs.rdi);

        assert_eq!(message, "message");
        assert_eq!(title, "title");

        emu.maps.write_string(emu.regs.rdx, "inject");

        // launch the msgbox
        emu.step();
    }

    #[test]
    // this tests a windows 32bits executable, that require iat binding of multiple libs.
    fn exe32win_minecraft() {
        setup();

        let mut emu = emu32();
        emu.cfg.maps_folder = "../maps32/".to_string();

        let sample = "../test/exe32win_minecraft.bin";
        emu.load_code(sample);
        emu.run(Some(0x403740));

        assert_eq!(emu.regs.get_ebx(), 2);
    }


    #[test]
    // enigma packer should be emulated at least 102,302,404 insturctions.
    // this test is few seconds slow but will verify many cpu instructions.
    fn exe64win_enigma() {
        setup();

        let mut emu = emu64();
        emu.cfg.maps_folder = "../maps64/".to_string();

        let sample = "../test/exe64win_enigma.bin";
        emu.load_code(sample);
        emu.run(Some(0x140578ad3));

        assert!(emu.pos > 102302239);
    }

    #[test]
    // this tests a linux 64bits static ELF binary.
    fn elf64lin_static_helloworld() {
        setup();

        let mut emu = emu64();

        let sample = "../test/elf64lin_static_helloworld.bin";
        emu.load_code(sample);
        emu.run(Some(0x44ab87));

        assert_eq!(emu.regs.rcx, 0x4cc2d0);
        assert_eq!(emu.pos, 11111); 
    }

    #[test]
    // this tests a linux 64bits raw arithmetic code.
    fn sc64lin_arith_100iter() {
        setup();

        let mut emu = emu64();
        emu.cfg.maps_folder = "../maps64/".to_string();
        

        let sample = "../test/sc64lin_arith_100iter.bin";
        emu.load_code(sample);
        emu.run(Some(0x3c0040));

        assert_eq!(emu.regs.rax, 0x4d9364d94bc0001e);
    }

    #[test]
    // this tests a metasploit rshell of 32bits linux, the tests verify the sockaddr and shell.
    fn sc32lin_rshell() {
        setup();

        let mut emu = emu32();
        emu.cfg.maps_folder = "../maps32/".to_string();
        

        let sample = "../test/sc32lin_rshell.bin";
        emu.load_code(sample);
        emu.run_to(31);
        let sockaddr = emu.maps.read_bytes(emu.regs.get_ecx(), 9);
        assert_eq!(sockaddr, [0x02,0x00,0x05,0x39,0x01,0x03,0x03,0x07,0x01]);

        emu.run_to(42);
        assert_eq!(emu.maps.read_string(emu.regs.get_ebx()), "//bin/sh");
    }


    #[test]
    // basic tests of some fpu functionst.
    fn fpu_conversions() {
        setup();

        let mut fpu = FPU::new();
        assert_eq!(fpu.get_top(), 0);
        assert_eq!(fpu.get_depth(), 0);

        fpu.push_f64(0.0);
        fpu.push_f64(1.0);
        assert_eq!(fpu.peek_st_logical_f64(0), 1.0);
        assert_eq!(fpu.peek_st_logical_f64(1), 0.0);


        // u80 to f64 conversion
        fpu.set_st_u80(1, 0x4000c90fdaa22168c235);
        fpu.st.print();
        assert_eq!(fpu.peek_st_logical_f64(1), 3.14159265358979323);
        assert_eq!(fpu.peek_st_logical_u80(1), 0x4000c90fdaa22168c235);
       
        /*
        assert_eq!(3.141592653589793239, 
                   3.141592653589793);  // true cuts to 64bits
                                        //
        */

        // f64 to u80 conversion
        //fpu.set_st(1, 4.141592653589793238);
        //assert_eq!(fpu.peek_st_u80(1), 0x4000c90fdaa22168c234);
        //

        
    }


    #[test]
    // this tests the fpu unit.
    fn elf64lin_fpu() {
        setup();

        let mut emu = emu64();

        emu.cfg.maps_folder = "../maps64/".to_string();
        

        let sample = "../test/elf64lin_fpu.bin";
        emu.load_code(sample);
        emu.fpu.clear();
        emu.fpu.trace = true;
        assert_eq!(emu.fpu.peek_st_u80(7), 0);
        emu.step(); // 1 fninit
        assert_eq!(emu.fpu.peek_st_u80(7), 0);
        emu.step(); // 2 fld1
        assert_eq!(emu.fpu.peek_st_u80(7), 0x3fff8000000000000000);
        assert_eq!(emu.fpu.peek_st_f64(7), 1.0);
        emu.step(); // 3 fldpi
        assert_eq!(emu.fpu.peek_st_u80(7), 0x3fff8000000000000000);
        assert_eq!(emu.fpu.peek_st_u80(6), 0x4000c90fdaa22168c234); // should end in 235
        assert_eq!(emu.fpu.peek_st_f64(6), 3.141592653589793);
        emu.step(); // 4 fadd   st,st(1)
        assert_eq!(emu.fpu.peek_st_u80(6), 0x40018487ed5110b4611a);
        assert_eq!(emu.fpu.peek_st_f64(6), 4.141592653589793);
        emu.step(); // 5 fsub   st,st(1)
        assert_eq!(emu.fpu.peek_st_u80(7), 0x3fff8000000000000000);
        assert_eq!(emu.fpu.peek_st_u80(6), 0x4000c90fdaa22168c234);
        assert_eq!(emu.fpu.peek_st_f64(6), 3.141592653589793);
        emu.step(); // 6 fsubr  st,st(1)
        assert_eq!(emu.fpu.peek_st_u80(6), 0xc000890fdaa22168c234);
        assert_eq!(emu.fpu.peek_st_f64(6), -2.141592653589793238);
        emu.step(); // 7 fchs
        assert_eq!(emu.fpu.peek_st_u80(6), 0x4000890fdaa22168c234);
        assert_eq!(emu.fpu.peek_st_f64(6), 2.141592653589793);
        emu.step(); // 8 fsqrt
        assert_eq!(emu.fpu.peek_st_u80(6), 0x3fffbb51491ea66b7000); // should end in 6ea4,
                                                                    // its comupted as f64
        assert_eq!(emu.fpu.peek_st_f64(6), 1.4634181403788165);

        emu.step(); //  9 fxch   st(1) 
        assert_eq!(emu.fpu.peek_st_u80(7), 0x3fffbb51491ea66b7000); // should end in 6ea4
        assert_eq!(emu.fpu.peek_st_u80(6), 0x3fff8000000000000000);
        assert_eq!(emu.fpu.peek_st_f64(7), 1.4634181403788165);
        emu.step(); //  10 fptan
        assert_eq!(emu.fpu.peek_st_u80(7), 0x3fffbb51491ea66b7000); // should end in 6ea4
        assert_eq!(emu.fpu.peek_st_u80(6), 0x3fffc75922e5f71d3000); // should end in 3000
        assert_eq!(emu.fpu.peek_st_u80(5), 0x3fff8000000000000000);
        assert_eq!(emu.fpu.peek_st_f64(7), 1.4634181403788165);
        assert_eq!(emu.fpu.peek_st_f64(6), 1.5574077246549023);
        assert_eq!(emu.fpu.peek_st_f64(5), 1.0);
        emu.step(); //  11 fmulp  st(1),st
        assert_eq!(emu.fpu.peek_st_u80(7), 0x3fffbb51491ea66b7000); // should end in 6ea4
        assert_eq!(emu.fpu.peek_st_u80(6), 0x3fffc75922e5f71d3000); // should end in 2dc6
        assert_eq!(emu.fpu.peek_st_f64(7), 1.4634181403788165);
        assert_eq!(emu.fpu.peek_st_f64(6), 1.5574077246549023);
        emu.step(); // 12 fdivp  st(1),st
        assert_eq!(emu.fpu.peek_st_u80(7), 0x3ffef08ce6b636464000); // should end in 375
        assert_eq!(emu.fpu.peek_st_u80(6), 0x3fffc75922e5f71d3000); // should end in 2dc6
        assert_eq!(emu.fpu.peek_st_u80(5), 0x3fff8000000000000000);
        assert_eq!(emu.fpu.peek_st_f64(7), 0.9396499819615878);
        assert_eq!(emu.fpu.peek_st_f64(6), 1.5574077246549023); 
        emu.step(); // 13 fsubp  st(1),st
        assert_eq!(emu.fpu.peek_st_u80(0), 0xffffc000000000000000);
        emu.step(); // 14 f2xm1
        assert_eq!(emu.fpu.peek_st_u80(0), 0xffffc000000000000000);
        emu.step(); // 15 fld1
        assert_eq!(emu.fpu.peek_st_u80(7), 0x3fff8000000000000000); // should end in 375
        assert_eq!(emu.fpu.peek_st_u80(6), 0x3fffc75922e5f71d3000); // should end in 2dc6
        assert_eq!(emu.fpu.peek_st_u80(5), 0x3fff8000000000000000);
        assert_eq!(emu.fpu.peek_st_u80(0), 0xffffc000000000000000);
        emu.step(); // 16 fldlg2
        assert_eq!(emu.fpu.st.get_top(), 6);
        assert_eq!(emu.fpu.st.get_depth(), 2);
        assert_eq!(emu.fpu.peek_st_u80(6), 0x3ffd9a209a84fbcff800); // 799);
        assert_eq!(emu.fpu.peek_st_f64(6), 0.3010299956639812); 
        emu.step(); // 17 fyl2x
        assert_eq!(emu.fpu.peek_st_u80(7), 0xbfffddb2dbec0456f800); //46);
        assert_eq!(emu.fpu.peek_st_f64(7), -1.7320208456446193);
        emu.step(); // 18 fld1
        emu.step(); // 19 fld1
        assert_eq!(emu.fpu.peek_st_u80(7), 0xbfffddb2dbec0456f800); //46);
        assert_eq!(emu.fpu.peek_st_u80(6), 0x3fff8000000000000000);
        assert_eq!(emu.fpu.peek_st_u80(5), 0x3fff8000000000000000);
        assert_eq!(emu.fpu.peek_st_u80(0), 0xffffc000000000000000);
        assert_eq!(emu.fpu.peek_st_f64(7), -1.7320208456446193);
        emu.step(); // 20 fyl2xp1
        assert_eq!(emu.fpu.peek_st_u80(7), 0xbfffddb2dbec0456f800); //46);
        assert_eq!(emu.fpu.peek_st_u80(6), 0x3fff8000000000000000);
        assert_eq!(emu.fpu.peek_st_u80(5), 0x3fff8000000000000000);
        assert_eq!(emu.fpu.peek_st_u80(0), 0xffffc000000000000000);
        assert_eq!(emu.fpu.peek_st_f64(7), -1.7320208456446193);
        emu.step(); // 21 fucom  st(1)
        emu.step(); // 22 fcmovnbe st(0), st(1)
        assert_eq!(emu.fpu.peek_st_u80(7), 0xbfffddb2dbec0456f800); //46);
        assert_eq!(emu.fpu.peek_st_u80(6), 0xbfffddb2dbec0456f800); //46);
        assert_eq!(emu.fpu.peek_st_f64(7), -1.7320208456446193);
        assert_eq!(emu.fpu.peek_st_f64(6), -1.7320208456446193);
        assert_eq!(emu.fpu.peek_st_u80(0), 0xffffc000000000000000);
        emu.step(); // 23 fcmovnu st(0), st(1)
        assert_eq!(emu.fpu.peek_st_u80(0), 0xffffc000000000000000);
        emu.step(); // fstp   st(0)
        emu.step(); // fstp   st(0)
        emu.step(); // fstp   st(0)
        assert_eq!(emu.fpu.peek_st_u80(7), 0xbfffddb2dbec0456f800); //46);
        assert_eq!(emu.fpu.peek_st_u80(6), 0xbfffddb2dbec0456f800); //46);
        assert_eq!(emu.fpu.peek_st_u80(0), 0xffffc000000000000000);
    }


    #[test]
    // this tests a linux 64bits flags
    fn elf64lin_flags() {
        setup();

        let mut emu = emu64();
        emu.cfg.maps_folder = "../maps64/".to_string();
        

        let sample = "../test/elf64lin_flags.bin";
        emu.load_code(sample);

        // test instruction add
        emu.run(Some(0x401014));
        assert_eq!(emu.flags.f_cf, true);
        assert_eq!(emu.flags.f_of, false);
        assert_eq!(emu.flags.f_zf, true);
        assert_eq!(emu.flags.f_sf, false);
        assert_eq!(emu.flags.f_pf, true);

        // test instruction sub
        emu.run(Some(0x40102a));
        assert_eq!(emu.flags.f_cf, false);
        assert_eq!(emu.flags.f_of, false);
        assert_eq!(emu.flags.f_zf, true);
        assert_eq!(emu.flags.f_sf, false);
        assert_eq!(emu.flags.f_pf, true);

        // test instruction cmp
        emu.run(Some(0x401040));
        assert_eq!(emu.flags.f_cf, true);
        assert_eq!(emu.flags.f_of, false);
        assert_eq!(emu.flags.f_zf, false);
        assert_eq!(emu.flags.f_sf, true);
        assert_eq!(emu.flags.f_pf, false);


        // test instruction test
        emu.run(Some(0x401056));
        assert_eq!(emu.flags.f_cf, false);
        assert_eq!(emu.flags.f_of, false);
        assert_eq!(emu.flags.f_zf, true);
        assert_eq!(emu.flags.f_sf, false);
        assert_eq!(emu.flags.f_pf, true);

        // test and
        emu.run(Some(0x40106c));
        assert_eq!(emu.flags.f_cf, false);
        assert_eq!(emu.flags.f_of, false);
        assert_eq!(emu.flags.f_zf, true);
        assert_eq!(emu.flags.f_sf, false);
        assert_eq!(emu.flags.f_pf, true);


        // test or with 0x0
        emu.run(Some(0x401087));
        assert_eq!(emu.flags.f_cf, false);
        assert_eq!(emu.flags.f_of, false);
        assert_eq!(emu.flags.f_zf, false);
        assert_eq!(emu.flags.f_sf, true);
        assert_eq!(emu.flags.f_pf, true);

        // test shl
        emu.run(Some(0x40109d));
        assert_eq!(emu.flags.f_cf, true);
        assert_eq!(emu.flags.f_of, true);
        assert_eq!(emu.flags.f_zf, true);
        assert_eq!(emu.flags.f_sf, false);
        assert_eq!(emu.flags.f_pf, true);

        // test add
        emu.run(Some(0x4010b8));
        assert_eq!(emu.flags.f_cf, false);
        assert_eq!(emu.flags.f_of, true);
        assert_eq!(emu.flags.f_zf, false);
        assert_eq!(emu.flags.f_sf, true);
        assert_eq!(emu.flags.f_pf, true);
    }

    
    #[test]
    // test serialization
    fn should_serialize() {
        setup();

        // init
        let mut emu = emu64();

        // load maps
        emu.cfg.maps_folder = "../maps64/".to_string();
        

        // load binary
        emu.load_code("../test/exe64win_msgbox.bin");

        // set registers
        emu.regs.rdx = 0x1;

        // serialize
        let serialized = Serialization::serialize(&emu);

        // deserialize
        let emu: Emu = Serialization::deserialize(&serialized);

        // assert
        assert_eq!(emu.regs.rdx, 0x1);
    }


    #[test]
    // the donut shellcode generator, with a 32bits truncated payload, emulate 30_862_819
    // instructions and check.
    fn sc32win_donut() {
        setup();

        let mut emu = emu32();
        emu.cfg.maps_folder = "../maps32/".to_string();
        

        let sample = "../test/sc32win_donut.bin";
        emu.load_code(sample);
        emu.run_to(30_862_819);

        assert_eq!(emu.regs.get_eax(), 0x7f937230);
        assert_eq!(emu.regs.get_ebx(), 0xc);
    }

    #[test]
    // test memory management operations
    fn maps_memory_operations() {
        setup();

        let mut emu = emu64();
        // with no init call
        
        // Test memory allocation
        let base = 0x10000;
        let size = 0x1000;
        let result = emu.maps.create_map("test_map", base, size);
        assert!(result.is_ok());
        
        // Test memory exists
        assert!(emu.maps.is_allocated(base));
        assert!(emu.maps.exists_mapname("test_map"));
        
        // Test memory read/write operations
        assert!(emu.maps.write_dword(base, 0xDEADBEEF));
        assert_eq!(emu.maps.read_dword(base).unwrap(), 0xDEADBEEF);
        
        // Test qword operations
        assert!(emu.maps.write_qword(base + 8, 0x123456789ABCDEF0));
        assert_eq!(emu.maps.read_qword(base + 8).unwrap(), 0x123456789ABCDEF0);
        
        // Test byte operations
        assert!(emu.maps.write_byte(base + 16, 0xAB));
        assert_eq!(emu.maps.read_byte(base + 16).unwrap(), 0xAB);
        
        // Test word operations
        assert!(emu.maps.write_word(base + 18, 0x1234));
        assert_eq!(emu.maps.read_word(base + 18).unwrap(), 0x1234);
        
        // Test boundary conditions - should fail with banzai mode
        emu.maps.set_banzai(true);
        assert!(!emu.maps.write_dword(base + size, 0x12345678));
        assert!(emu.maps.read_dword(base + size).is_none());
        
        // Test string operations
        let test_str = "Hello World";
        emu.maps.write_string(base + 32, test_str);
        assert_eq!(emu.maps.read_string(base + 32), test_str);
        
        // Test duplicate map creation should fail
        let result2 = emu.maps.create_map("test_map", base, size);
        assert!(result2.is_err());
        
        // Test overlapping memory should fail
        let result3 = emu.maps.create_map("test_map2", base + 0x500, size);
        assert!(result3.is_err());
    }

    #[test]
    // test breakpoint functionality, improve this with a running sample.
    fn breakpoint_functionality() {
        setup();

        let mut bp = crate::breakpoint::Breakpoint::new();
        
        // Test initial state
        assert_eq!(bp.get_bp(), 0);
        
        // Test basic breakpoint operations
        bp.set_bp(0x401000);
        assert_eq!(bp.get_bp(), 0x401000);
        
        // Test memory breakpoints
        bp.set_mem_read(0x402000);
        bp.set_mem_write(0x403000);
        
        // Test instruction breakpoints
        bp.set_instruction(100);
        
        // Test clearing breakpoints
        bp.clear_bp();
        assert_eq!(bp.get_bp(), 0);
        
        // Test multiple breakpoint operations
        bp.set_bp(0x500000);
        bp.set_mem_read(0x600000);
        bp.set_mem_write(0x700000);
        bp.set_instruction(200);
        
        assert_eq!(bp.get_bp(), 0); // only one type of bt at once, the setters clear all the
                                    // breakpointts.
        
        bp.clear_bp();
        assert_eq!(bp.get_bp(), 0);

        let mut emu = emu64();
        emu.cfg.maps_folder = "../maps64/".to_string();
        
        emu.load_code("../test/exe64win_msgbox.bin");
        assert!(!emu.maps.is_allocated(0));
        emu.bp.clear_bp();
        emu.bp.set_bp(0x1400011d6);
        emu.run(None);
        assert!(!emu.maps.is_allocated(0));
        assert_eq!(emu.pos, 4);

        /*
        emu.bp.set_mem_write(0x329f70);
        emu.run(None);
        assert_eq!(emu.pos, 15);

    14 0x1400010df: mov   [rbp-10h],rdi
	mem_trace: pos = 14 rip = 1400010df op = write bits = 64 address = 0x329f70 value = 0x1400011df name = 'stack'

        */

        emu.bp.set_instruction(100);
        assert_eq!(emu.bp.get_instruction(), 100);
        emu.run(None);
        assert_eq!(emu.pos, 100);

        /* is not matching
        emu.bp.set_mem_read(0x329eb8);
        emu.run(None);
        assert_eq!(emu.pos, 102);

	mem_trace: pos = 102 rip = 1400010c4 op = read bits = 64 address = 0x329eb8 value = 0xc name = 'stack'
	mem_trace: pos = 102 rip = 1400010c4 op = write bits = 64 address = 0x329eb8 value = 0xc name = 'register'
    102 0x1400010c4: pop   rcx ;0xc
        */


    }

    #[test]
    // test flag calculations and parity table
    fn flag_calculations() {
        setup();

        // Test parity flag calculation
        assert_eq!(crate::flags::PARITY_LOOKUP_TABLE[0], true);   // 0 has even parity (0 ones)
        assert_eq!(crate::flags::PARITY_LOOKUP_TABLE[1], false);  // 1 has odd parity (1 one)
        assert_eq!(crate::flags::PARITY_LOOKUP_TABLE[3], true);   // 3 (11b) has even parity (2 ones)
        assert_eq!(crate::flags::PARITY_LOOKUP_TABLE[7], false);  // 7 (111b) has odd parity (3 ones)
        assert_eq!(crate::flags::PARITY_LOOKUP_TABLE[15], true);  // 15 (1111b) has even parity (4 ones)
        assert_eq!(crate::flags::PARITY_LOOKUP_TABLE[255], true); // 255 (11111111b) has even parity (8 ones)
        
        // Test flag constants
        assert_eq!(crate::flags::MIN_U8, 0);
        assert_eq!(crate::flags::MAX_U8, 0xff);
        assert_eq!(crate::flags::MIN_U16, 0);
        assert_eq!(crate::flags::MAX_U16, 0xffff);
        assert_eq!(crate::flags::MIN_U32, 0);
        assert_eq!(crate::flags::MAX_U32, 0xffffffff);
        assert_eq!(crate::flags::MIN_U64, 0);
        assert_eq!(crate::flags::MAX_U64, 0xffffffffffffffff);
        
        // Test signed constants
        assert_eq!(crate::flags::MIN_I8, -128);
        assert_eq!(crate::flags::MAX_I8, 0x7f);
        assert_eq!(crate::flags::MIN_I16, -32768);
        assert_eq!(crate::flags::MAX_I16, 0x7fff);
        assert_eq!(crate::flags::MIN_I32, -2147483648);
        assert_eq!(crate::flags::MAX_I32, 0x7fffffff);
        assert_eq!(crate::flags::MIN_I64, -9223372036854775808);
        assert_eq!(crate::flags::MAX_I64, 0x7fffffffffffffff);
    }

    #[test]
    // test configuration management
    fn config_management() {
        setup();

        let mut cfg = crate::config::Config::new();
        
        // Test default values
        assert!(!cfg.is_64bits); // should default to 32-bit
        
        // Test 32/64-bit mode switching
        cfg.is_64bits = true;
        assert!(cfg.is_64bits);
        
        cfg.is_64bits = false;
        assert!(!cfg.is_64bits);
        
        // Test maps folder configuration
        cfg.maps_folder = "/test/path".to_string();
        assert_eq!(cfg.maps_folder, "/test/path");
        
        // Test other configuration options
        cfg.verbose = 3;
        assert_eq!(cfg.verbose, 3);
        
        // Test emulator with different configs
        let emu32 = emu32();
        assert!(!emu32.cfg.is_64bits);
        
        let emu64 = emu64();
        assert!(emu64.cfg.is_64bits);
    }

    #[test]
    // test hooks system basic functionality
    fn hooks_system() {
        setup();

        let mut hooks = crate::hooks::Hooks::new();
        
        // Test initial state - all hooks should be None
        assert!(hooks.hook_on_interrupt.is_none());
        assert!(hooks.hook_on_exception.is_none());
        assert!(hooks.hook_on_memory_read.is_none());
        assert!(hooks.hook_on_memory_write.is_none());
        assert!(hooks.hook_on_pre_instruction.is_none());
        assert!(hooks.hook_on_post_instruction.is_none());
        assert!(hooks.hook_on_winapi_call.is_none());
        
        // Test setting hooks
        hooks.hook_on_interrupt = Some(|_emu, _addr, _interrupt| true);
        assert!(hooks.hook_on_interrupt.is_some());
        
        hooks.hook_on_exception = Some(|_emu, _addr, _ex_type| true);
        assert!(hooks.hook_on_exception.is_some());
        
        hooks.hook_on_memory_read = Some(|_emu, _ip, _addr, _sz| {});
        assert!(hooks.hook_on_memory_read.is_some());
        
        hooks.hook_on_memory_write = Some(|_emu, _ip, _addr, _sz, value| value);
        assert!(hooks.hook_on_memory_write.is_some());
        
        hooks.hook_on_pre_instruction = Some(|_emu, _addr, _ins, _sz| true);
        assert!(hooks.hook_on_pre_instruction.is_some());
        
        hooks.hook_on_post_instruction = Some(|_emu, _addr, _ins, _sz, _ok| {});
        assert!(hooks.hook_on_post_instruction.is_some());
        
        hooks.hook_on_winapi_call = Some(|_emu, _addr, _called_addr| true);
        assert!(hooks.hook_on_winapi_call.is_some());

        // Test if all hooks are set
        assert!(!hooks.hook_on_interrupt.is_none());
        assert!(!hooks.hook_on_exception.is_none());
        assert!(!hooks.hook_on_memory_read.is_none());
        assert!(!hooks.hook_on_memory_write.is_none());
        assert!(!hooks.hook_on_pre_instruction.is_none());
        assert!(!hooks.hook_on_post_instruction.is_none());
        assert!(!hooks.hook_on_winapi_call.is_none());
    }

    #[test]
    // test error conditions and edge cases
    fn error_conditions() {
        setup();

        let mut emu = emu64();
        // Don't call init to avoid DLL loading issues
        
        // Test invalid memory access with banzai mode
        emu.maps.set_banzai(true);
        assert!(emu.maps.read_dword(0x999999).is_none());
        assert!(!emu.maps.write_dword(0x999999, 0x12345678));
        
        // Test reading from unallocated memory
        assert!(emu.maps.read_qword(0x123456789).is_none());
        assert!(!emu.maps.write_qword(0x123456789, 0xDEADBEEF));
        
        // Test zero-sized memory operations
        let base = 0x20000;
        emu.maps.create_map("zero_test", base, 0x1000).unwrap();
        
        // Test reading/writing at exact boundaries
        assert!(emu.maps.write_dword(base + 0x1000 - 4, 0x12345678));
        assert!(emu.maps.read_dword(base + 0x1000 - 4).is_some());
        
        // Test one byte past boundary should fail with banzai mode
        assert!(!emu.maps.write_dword(base + 0x1000 - 3, 0x12345678));
        
        // Test string operations with boundaries
        let long_string = "A".repeat(100);
        emu.maps.write_string(base, &long_string);
        let read_string = emu.maps.read_string(base);
        assert_eq!(read_string, long_string);
    }

    #[test]
    // test emulator initialization and basic operations
    fn emulator_initialization() {
        setup();

        // Test 64-bit emulator
        let mut emu64 = emu64();
        assert!(emu64.cfg.is_64bits);
        assert_eq!(emu64.pos, 0);
        assert!(!emu64.force_break);
        assert!(!emu64.force_reload);
        
        // Don't call init to avoid DLL loading issues
        
        // Test 32-bit emulator
        let mut emu32 = emu32();
        assert!(!emu32.cfg.is_64bits);
        assert_eq!(emu32.pos, 0);
        
        // Don't call init to avoid DLL loading issues
        
        // Test emulator state after initialization
        assert_eq!(emu64.regs.rip, 0);
        assert_eq!(emu32.regs.rip, 0);
        
        // Test register clearing
        emu64.regs.rax = 0x123456789ABCDEF0;
        emu64.regs.clear::<64>();
        assert_eq!(emu64.regs.rax, 0);
        
        emu32.regs.rax = 0x123456789ABCDEF0;
        emu32.regs.sanitize32();
        assert_eq!(emu32.regs.rax & 0xFFFFFFFF00000000, 0);
    }

    #[test]
    // test memory map operations and edge cases
    fn memory_map_operations() {
        setup();

        let mut emu = emu64();
        // Don't call init to avoid DLL loading issues
        
        // Test multiple memory maps
        emu.maps.create_map("map1", 0x10000, 0x1000).unwrap();
        emu.maps.create_map("map2", 0x20000, 0x2000).unwrap();
        emu.maps.create_map("map3", 0x30000, 0x1000).unwrap();
        
        // Test map existence
        assert!(emu.maps.exists_mapname("map1"));
        assert!(emu.maps.exists_mapname("map2"));
        assert!(emu.maps.exists_mapname("map3"));
        assert!(!emu.maps.exists_mapname("nonexistent"));
        
        // Test memory allocation checks
        assert!(emu.maps.is_allocated(0x10000));
        assert!(emu.maps.is_allocated(0x10500));
        assert!(emu.maps.is_allocated(0x10FFF));
        assert!(!emu.maps.is_allocated(0x11000));
        
        assert!(emu.maps.is_allocated(0x20000));
        assert!(emu.maps.is_allocated(0x21FFF));
        assert!(!emu.maps.is_allocated(0x22000));
        
        // Test getting map by name
        let map1 = emu.maps.get_map_by_name("map1");
        assert!(map1.is_some());
        assert_eq!(map1.unwrap().get_base(), 0x10000);
        
        let nonexistent = emu.maps.get_map_by_name("nonexistent");
        assert!(nonexistent.is_none());
        
        // Test memory size queries
        let size1 = emu.maps.get_mem_size(0x10000);
        assert!(size1.is_some());
        
        let size_invalid = emu.maps.get_mem_size(0x99999);
        assert!(size_invalid.is_none());
        
        // Test cross-map operations
        assert!(emu.maps.write_dword(0x10000, 0x11111111));
        assert!(emu.maps.write_dword(0x20000, 0x22222222));
        assert!(emu.maps.write_dword(0x30000, 0x33333333));
        
        assert_eq!(emu.maps.read_dword(0x10000).unwrap(), 0x11111111);
        assert_eq!(emu.maps.read_dword(0x20000).unwrap(), 0x22222222);
        assert_eq!(emu.maps.read_dword(0x30000).unwrap(), 0x33333333);
    }

    #[test]
    // test FPU stack operations beyond basic F80 tests
    fn fpu_stack_operations() {
        setup();

        let mut fpu = FPU::new();
        
        // Test initial stack state
        assert_eq!(fpu.get_top(), 0);
        assert_eq!(fpu.get_depth(), 0);
        
        // Test stack push operations
        fpu.push_f64(1.0);
        assert_eq!(fpu.get_depth(), 1);
        assert_eq!(fpu.peek_st_logical_f64(0), 1.0);
        
        fpu.push_f64(2.0);
        assert_eq!(fpu.get_depth(), 2);
        assert_eq!(fpu.peek_st_logical_f64(0), 2.0);
        assert_eq!(fpu.peek_st_logical_f64(1), 1.0);
        
        fpu.push_f64(3.0);
        assert_eq!(fpu.get_depth(), 3);
        assert_eq!(fpu.peek_st_logical_f64(0), 3.0);
        assert_eq!(fpu.peek_st_logical_f64(1), 2.0);
        assert_eq!(fpu.peek_st_logical_f64(2), 1.0);
        
        // Test stack pop operations
        let val = fpu.pop_f64();
        assert_eq!(val, 3.0);
        assert_eq!(fpu.get_depth(), 2);
        assert_eq!(fpu.peek_st_logical_f64(0), 2.0);
        
        // Test stack overflow protection (push 5 more values to reach 8 total)
        for i in 3..9 {
            fpu.push_f64(i as f64);
        }
        
        // Stack should be full now, test behavior
        assert_eq!(fpu.get_depth(), 8);
        
        // Test clearing stack
        fpu.clear();
        assert_eq!(fpu.get_depth(), 0);
        assert_eq!(fpu.get_top(), 0);
        
        // Test mixed operations
        fpu.push_f64(10.5);
        fpu.push_f64(20.25);
        fpu.push_f64(30.125);
        fpu.st.print(); 
        assert_eq!(fpu.peek_st_logical_f64(0), 30.125);
        assert_eq!(fpu.peek_st_logical_f64(1), 20.25);
        assert_eq!(fpu.peek_st_logical_f64(2), 10.5);
    }

    #[test]
    // test mem64
    fn mem64_test() {
        setup();

        let mut mem = Mem64::new();
        mem.set_name("memtest");
        assert_eq!(mem.get_name(), "memtest");

        mem.set_base(0x400000);
        mem.set_size(1024);
        assert_eq!(mem.get_base(), 0x400000);
        assert_eq!(mem.size(), 1024);

        mem.write_bytes(0x400010, &[1, 2, 3, 4]);
        assert_eq!(mem.read_bytes(0x400010, 4), &[1, 2, 3, 4]);

        mem.write_byte(0x400010, 0x12);
        assert_eq!(mem.read_byte(0x400010), 0x12);

        mem.write_word(0x400010, 0x1234);
        assert_eq!(mem.read_word(0x400010), 0x1234);

        mem.write_dword(0x400010, 0x12345678);
        assert_eq!(mem.read_dword(0x400010), 0x12345678);

        mem.write_qword(0x400010, 0x123456789ABCDEF0);
        assert_eq!(mem.read_qword(0x400010), 0x123456789ABCDEF0);

        mem.write_oword(0x400010, 0x123456789ABCDEF0123456789ABCDEF0);
        assert_eq!(mem.read_oword(0x400010), 0x123456789ABCDEF0123456789ABCDEF0);

        mem.write_wide_string(0x400010, "Hello, world!");
        assert_eq!(mem.read_wide_string(0x400010), "Hello, world!");

        mem.write_string(0x400010, "Hello, world!");
        assert_eq!(mem.read_string(0x400010), "Hello, world!");

        mem.write_string(0x400010, "Hello, ");
        mem.write_string(0x400010 + 7, "world!");
        assert_eq!(mem.read_string(0x400010), "Hello, world!");


        assert_eq!(mem.inside(0x4000ab), true);
        assert_eq!(mem.inside(0x400000+1024), false);

        mem.clear();

        let mut mem2 = Mem64::new();
        mem2.set_base(0x400000);
        mem2.set_size(16);
        mem2.load("../test/sc32win_donut.bin");
        let md5 = format!("{:x}", mem2.md5());
        assert_eq!(md5, "66d6376c2dd0b8d4d35461844e5b0e6c");
    }

    #[test]
    // test 64bits allocators
    fn allocator64_test() {
        setup();

        let mut emu = emu64();
        emu.cfg.maps_folder = "../maps64/".to_string();
        emu.init(false, false);

        assert_eq!(emu.maps.exists_mapname("shell32.rsrc"), true);
        assert_eq!(emu.maps.get_map_by_name("shell32.rsrc").is_some(), true);
        assert_eq!(emu.maps.exists_mapname("notexist"), false);
        assert_eq!(emu.maps.get_map_by_name("notexist").is_some(), false);

        for _ in 0..1000 {
            assert_eq!(emu.maps.alloc(1024).is_some(), true);
            assert_eq!(emu.maps.lib64_alloc(1024).is_some(), true);
        }

        assert_eq!(emu.maps.mem_test(), true);

        emu.maps.clear();


        emu.regs.rcx = 0; // addr
        emu.regs.rdx = 1024; // sz
        emu.regs.r8 = constants::MEM_RESERVE as u64;
        emu.regs.r9 = 0x40; // rwx
        winapi64::kernel32::VirtualAlloc(&mut emu);
        assert_eq!(emu.maps.is_allocated(emu.regs.rax), true);

        emu.regs.rcx = 0x30000000; // addr
        emu.regs.rdx = 1024; // sz
        emu.regs.r8 = (constants::MEM_RESERVE | constants::MEM_COMMIT) as u64;
        emu.regs.r9 = 0x40; // rwx
        winapi64::kernel32::VirtualAlloc(&mut emu);

        emu.regs.rcx = 0x30000000; // addr
        emu.regs.rdx = 1024; // sz
        emu.regs.r8 = constants::MEM_COMMIT as u64;
        emu.regs.r9 = 0x40; // rwx
        winapi64::kernel32::VirtualAlloc(&mut emu);
        assert_eq!(emu.regs.rax, 0x30000000);

        assert_eq!(emu.maps.is_allocated(0x30000000), true);
        assert_eq!(emu.maps.mem_test(), true);
    }

    #[test]
    // test 32bits allocators
    fn allocator32_test() {
        setup();

        let mut emu = emu32();
        emu.cfg.maps_folder = "../maps32/".to_string();
        emu.maps.clear();
        emu.init(false, false);

        assert_eq!(emu.maps.exists_mapname("shell32.rsrc"), true);
        assert_eq!(emu.maps.get_map_by_name("shell32.rsrc").is_some(), true);
        assert_eq!(emu.maps.exists_mapname("notexist"), false);
        assert_eq!(emu.maps.get_map_by_name("notexist").is_some(), false);

        for _ in 0..1000 {
            assert_eq!(emu.maps.alloc(1024).is_some(), true);
            assert_eq!(emu.maps.lib32_alloc(1024).is_some(), true);
        }

        assert_eq!(emu.maps.mem_test(), true);

        emu.stack_push32(0x40); // rwx
        emu.stack_push32(constants::MEM_RESERVE);
        emu.stack_push32(1024); // sz
        emu.stack_push32(0); // addr
        winapi32::kernel32::VirtualAlloc(&mut emu);
        assert_eq!(emu.maps.is_allocated(emu.regs.rax), true);

        emu.stack_push32(0x40); // rwx
        emu.stack_push32(constants::MEM_RESERVE | constants::MEM_COMMIT);
        emu.stack_push32(1024); // sz
        emu.stack_push32(0x30000000); // addr
        winapi32::kernel32::VirtualAlloc(&mut emu);

        emu.stack_push32(0x40); // rwx
        emu.stack_push32(constants::MEM_COMMIT);
        emu.stack_push32(1024); // sz
        emu.stack_push32(0x30000000); // addr
        winapi32::kernel32::VirtualAlloc(&mut emu);
        assert_eq!(emu.regs.rax, 0x30000000);

        assert!(emu.maps.is_allocated(0x30000000));
        assert!(emu.maps.mem_test());
    }

    #[test]
    // stack32 tests
    fn stack32_test() {
        setup();

        let mut emu = emu32();
        emu.cfg.maps_folder = "../maps32/".to_string();
        emu.init(false, false);
        
        let stack_check = emu.maps.get_map_by_name("stack");
        assert!(stack_check.is_some());
        let stack = stack_check.unwrap();
        let base = stack.get_base();

        assert!(emu.regs.get_esp() < emu.regs.get_ebp());
        assert!(emu.regs.get_esp() > stack.get_base());
        assert!(emu.regs.get_esp() < stack.get_bottom());
        assert!(emu.regs.get_ebp() > stack.get_base());
        assert!(emu.regs.get_ebp() < stack.get_bottom());
        assert!(stack.inside(emu.regs.get_esp()));
        assert!(stack.inside(emu.regs.get_ebp()));

        for i in 0..5000 {
            emu.stack_push32(i as u32);
        }
        emu.stack_pop32(false);

        assert!(emu.regs.get_esp() > base);
    }

    #[test]
    // stack64 tests
    fn stack64_test() {
        setup();

        let mut emu = emu64();
        emu.cfg.maps_folder = "../maps64/".to_string();
        emu.init(false, false);

        let stack_check = emu.maps.get_map_by_name("stack");
        assert!(stack_check.is_some());
        let stack = stack_check.unwrap();
        let base = stack.get_base();

        assert!(emu.regs.rsp < emu.regs.rbp);
        assert!(emu.regs.rsp > stack.get_base());
        assert!(emu.regs.rsp < stack.get_bottom());
        assert!(emu.regs.rbp > stack.get_base());
        assert!(emu.regs.rbp < stack.get_bottom());
        assert!(stack.inside(emu.regs.rsp));
        assert!(stack.inside(emu.regs.rbp));

        for i in 0..5000 {
            emu.stack_push64(i as u64);
        }
        emu.stack_pop64(false);

        assert!(emu.regs.rsp > base);
    }

    #[test]
    // logic tests
    fn logic_test() {
        setup();

        let mut emu = emu64();
        emu.cfg.maps_folder = "../maps64/".to_string();
        

        let num: u64 = 0x1234_5678_9ABC_DEF0;
        let shift:u64 = 12;
        let size:u32 = 32;
        let src: u64 = num >> (size as u64 - shift);
        let num2 = logic::shld(&mut emu, num, src, shift, size).0;
        assert_eq!(
            logic::shrd(&mut emu, num2, src, shift, size),
            (num, false)
        );

        let mut r: u64;
        (r, _) = logic::shrd(&mut emu, 0x9fd88893, 0x1b, 0x6, 32);
        assert!(r == 0x6e7f6222);
        (r, _) = logic::shrd(&mut emu, 0x6fdcb03, 0x0, 0x6, 32);
        assert!(r == 0x1bf72c);
        (r, _) = logic::shrd(&mut emu, 0x91545f1d, 0x6fe2, 0x6, 32);
        assert!(r == 0x8a45517c);
        (r, _) = logic::shld(&mut emu, 0x1b, 0xf1a7eb1d, 0xa, 32);
        assert!(r == 0x6fc6);
        (r, _) = logic::shld(&mut emu, 0x1, 0xffffffff, 4, 32);
        assert!(r == 0x1f);
        (r, _) = logic::shld(&mut emu, 0x1, 0xffffffff, 33, 32);
        assert!(r == 0x3);
        (r, _) = logic::shld(&mut emu, 0x144e471f8, 0x14F498, 0x3e, 64);
        assert!(r == 0x53d26);
    }

    #[test]
    // avoid c code, try to be 100% rust
    fn pure_rust_check() {
        let output = Command::new("cargo")
            .args(&["metadata", "--format-version", "1"])
            .output();

        assert!(output.is_ok()); // cargo executed well

        let out = output.unwrap();
        let stdout = String::from_utf8(out.stdout);
        
        assert!(stdout.is_ok()); // not utf8 errors
        let stoud2 = stdout.unwrap();

        assert!(stoud2.contains("libc"));
    }

    #[test]
    fn elf64lin_cpu_arithmetics() {
        setup();

        let mut emu = emu64();
        emu.load_code("../test/elf64lin_cpu_arithmetics.bin");

        assert_eq!(emu.flags.dump(), 0x202); // initial flags (match with gdb linux)

        emu.run_to(5); // position 5 is emulated
        assert_eq!(emu.regs.rax, 3);
        assert_eq!(emu.flags.dump(), 0x206);

        emu.run_to(6);  // dec ax
        assert_eq!(emu.regs.rax, 2);
        assert_eq!(emu.flags.dump(), 0x202);

        emu.run_to(8); // last dec rax zero reached
        assert_eq!(emu.regs.rax, 0);
        assert_eq!(emu.flags.dump(), 0x246);

        emu.run_to(11); // neg ax 
        assert_eq!(emu.regs.rax, 0x1122334455668888);
        assert_eq!(emu.flags.dump(), 0x297); // [ CF PF AF SF IF ]

        emu.run_to(14); // sar al, 1
        assert_eq!(emu.regs.rax, 0xffffffff556688c4);
        assert_eq!(emu.flags.dump(), 0x292);

        emu.run_to(23); // shl ax, 1
        assert_eq!(emu.regs.rax, 0x15596260);
        assert_eq!(emu.flags.dump(), 0xa17);

        emu.run_to(29); // shl rax, cl
        assert_eq!(emu.regs.rax, 0x55658980);
        assert_eq!(emu.flags.dump(), 0x212);
        
        emu.run_to(30); // shr al, 1
        assert_eq!(emu.regs.rax, 0x55658940);                
        assert_eq!(emu.flags.dump(), 0xa12);
                       
        emu.run_to(31); // shr ax, 1
        assert_eq!(emu.regs.rax, 0x556544a0);                
        assert_eq!(emu.flags.dump(), 0xa16);

        emu.run_to(40); // imul eax
        assert_eq!(emu.regs.rax, 0x21000000);
        assert_eq!(emu.flags.dump(), 0xa17); // [ CF PF AF IF OF ]

        emu.run_to(41); // imul rax
        assert_eq!(emu.regs.rax, 0x441000000000000);
        assert_eq!(emu.flags.dump(), 0x216); // [ PF AF IF ]
                                             
        emu.run_to(43); // imul eax, eax
        assert_eq!(emu.regs.rax, 0);
        assert_eq!(emu.flags.dump(), 0x216); // [ PF AF IF ]

        emu.run_to(45); // imul rax, rax
        assert_eq!(emu.regs.rax, 0x1eace4a3c82fb840);
        assert_eq!(emu.flags.dump(), 0xa17); // [ CF PF AF IF OF ]

        emu.run_to(48); // imul  rax,2
        assert_eq!(emu.regs.rax, 0x120bdc200);
        assert_eq!(emu.flags.dump(), 0x216); // [ PF AF IF ]  

        emu.run_to(49); // rcl al, 1
        assert_eq!(emu.regs.rax, 0x120bdc200); 
        assert_eq!(emu.flags.dump(), 0x216); // [ PF AF IF ]

        emu.run_to(50); // rcl ax, 1
        assert_eq!(emu.regs.rax, 0x120bd8400);
        assert_eq!(emu.flags.dump(), 0x217); // [ CF PF AF IF ]
    
        emu.run_to(52); // rcl   rax,1
        assert_eq!(emu.regs.rax, 0x82f61002); // ERROR
        assert_eq!(emu.flags.dump(), 0x216); // [ PF AF IF ]


        emu.run_to(58); // rcr   ax,1
        assert_eq!(emu.regs.rax, 0x82f60800);
        assert_eq!(emu.flags.dump(), 0x217); // [ CF PF AF IF ]

        emu.run_to(65);
        assert_eq!(emu.regs.rax, 0x60bd8200);
        assert_eq!(emu.flags.dump(), 0x216); // [ PF AF IF ]
    }


    #[test]
    // peb/teb/ldr basic tests
    fn peb_teb_ldr_structures_test() {
        setup();

        let mut emu = emu32();
        emu.cfg.maps_folder = "../maps32/".to_string();
        emu.load_code("../test/exe32win_minecraft.bin");

        let peb = emu.maps.get_mem("peb");
        let peb_addr = peb.get_base();
        assert!(peb_addr > 0x1000);
        assert!(emu.maps.is_allocated(peb_addr));
        let teb = emu.maps.get_mem("teb");
        let teb_addr = teb.get_base();
        assert!(teb_addr > 0x1000);
        assert!(emu.maps.is_allocated(teb_addr));
        let ldr = emu.maps.get_mem("ldr");
        let ldr_addr = ldr.get_base();
        assert!(ldr_addr > 0x1000);
        assert!(emu.maps.is_allocated(ldr_addr));

        let peb_struct = structures::PEB::load(peb_addr, &mut emu.maps);
        let teb_struct = structures::TEB::load(teb_addr, &mut emu.maps);
        let ldr_struct = structures::PebLdrData::load(ldr_addr, &mut emu.maps);


        assert_eq!(ldr_struct.in_load_order_module_list.flink, ldr_struct.in_memory_order_module_list.flink - 0x8);
        assert_eq!(ldr_struct.in_initialization_order_module_list.flink, ldr_struct.in_memory_order_module_list.flink + 0x8);
        assert_eq!(ldr_addr, peb_struct.ldr as u64);

        let mut ldr_entry = structures::LdrDataTableEntry::load(ldr_struct.in_load_order_module_list.flink as u64, &mut emu.maps);
        let ntdll_addr = emu.maps.get_mem("ntdll.pe").get_base();
        
        assert_eq!(peb_struct.image_base_addr, ntdll_addr as u32);
        assert_eq!(peb_struct.ldr, ldr_addr as u32);
        assert_eq!(peb_struct.being_debugged, 0);

        assert!(teb_struct.process_id > 0);
        assert!(teb_struct.thread_id > 0);

        assert_eq!(teb_struct.process_environment_block, peb_addr as u32);
        assert_eq!(teb_struct.last_error_value, 0);
        //assert!(teb_struct.environment_pointer > 0);
        
        let main_pe_w = emu.maps.get_addr_name(ldr_entry.dll_base as u64);
        assert!(main_pe_w.is_some());
        let main_pe = main_pe_w.unwrap();
        assert_eq!(main_pe, "exe32win_minecraft.pe");


        assert_eq!(ldr_entry.in_memory_order_links.flink, ldr_entry.in_load_order_links.flink + 0x8);
        assert_eq!(ldr_entry.in_initialization_order_links.flink, ldr_entry.in_memory_order_links.flink + 0x8);

        assert_eq!(ldr_entry.in_memory_order_links.blink, ldr_entry.in_load_order_links.blink + 0x8);
        assert_eq!(ldr_entry.in_initialization_order_links.blink, ldr_entry.in_memory_order_links.blink + 0x8);

        let sample_w = emu.maps.get_addr_name(ldr_entry.dll_base as u64);
        assert!(sample_w.is_some());
        let sample = sample_w.unwrap();
        assert_eq!(sample, "exe32win_minecraft.pe");

        // follow to next flink
        ldr_entry = structures::LdrDataTableEntry::load(ldr_entry.in_load_order_links.flink as u64, &mut emu.maps);

        assert_eq!(ldr_entry.in_memory_order_links.flink, ldr_entry.in_load_order_links.flink + 0x8);
        assert_eq!(ldr_entry.in_initialization_order_links.flink, ldr_entry.in_memory_order_links.flink + 0x8);

        assert_eq!(ldr_entry.in_memory_order_links.blink, ldr_entry.in_load_order_links.blink + 0x8);
        assert_eq!(ldr_entry.in_initialization_order_links.blink, ldr_entry.in_memory_order_links.blink + 0x8);

        let sample_w = emu.maps.get_addr_name(ldr_entry.dll_base as u64);
        assert!(sample_w.is_some());
        let sample = sample_w.unwrap();
        assert_eq!(sample, "netapi32.pe");

        let ntdll_str_ptr = ldr_entry.base_dll_name.buffer as u64;
        assert!(ntdll_str_ptr > 0);
        let ntdll_str = emu.maps.read_wide_string(ntdll_str_ptr);
        assert_eq!(ntdll_str, "netapi32.dll");

        let ntdll_str_ptr = ldr_entry.full_dll_name.buffer as u64;
        assert!(ntdll_str_ptr > 0);
        let ntdll_str = emu.maps.read_wide_string(ntdll_str_ptr);
        assert_eq!(ntdll_str, "C:\\Windows\\System32\\netapi32.dll");




        // 64BITS //

        let mut emu = emu64();
        emu.cfg.maps_folder = "../maps64/".to_string();
        emu.load_code("../test/exe64win_msgbox.bin");

        let ntdll_addr = emu.maps.get_mem("ntdll.pe").get_base();

        let peb = emu.maps.get_mem("peb");
        let peb_addr = peb.get_base();
        assert!(peb_addr > 0x1000);
        assert!(emu.maps.is_allocated(peb_addr));
        let teb = emu.maps.get_mem("teb");
        let teb_addr = teb.get_base();
        assert!(teb_addr > 0x1000);
        assert!(emu.maps.is_allocated(teb_addr));
        let ldr = emu.maps.get_mem("ldr");
        let ldr_addr = ldr.get_base();
        assert!(ldr_addr > 0x1000);
        assert!(emu.maps.is_allocated(ldr_addr));


        let peb_struct = structures::PEB64::load(peb_addr, &mut emu.maps);
        let teb_struct = structures::TEB64::load(teb_addr, &mut emu.maps);

        assert_eq!(peb_struct.image_base_addr, ntdll_addr);
        assert_eq!(peb_struct.ldr, ldr_addr);
        assert_eq!(peb_struct.being_debugged, 0);

        assert!(teb_struct.process_id > 0);
        assert!(teb_struct.thread_id > 0);

        assert_eq!(teb_struct.process_environment_block, peb_addr);
        assert_eq!(teb_struct.last_error_value, 0);
        //assert!(teb_struct.environment_pointer > 0);

        let ldr_struct = structures::PebLdrData64::load(ldr_addr, &mut emu.maps);
        let entry_addr = ldr_struct.in_load_order_module_list.flink;
        assert!(entry_addr >= 0x1000);
        let mut ldr_entry = structures::LdrDataTableEntry64::load(entry_addr, &mut emu.maps);

        //let ntdll_addr = emu.maps.get_mem("ntdll.pe").get_base();
        


        assert_eq!(ldr_entry.in_memory_order_links.flink, ldr_entry.in_load_order_links.flink + 0x10);
        assert_eq!(ldr_entry.in_initialization_order_links.flink, ldr_entry.in_memory_order_links.flink + 0x10);

        assert_eq!(ldr_entry.in_memory_order_links.blink, ldr_entry.in_load_order_links.blink + 0x10);
        assert_eq!(ldr_entry.in_initialization_order_links.blink, ldr_entry.in_memory_order_links.blink + 0x10);

        let sample_w = emu.maps.get_addr_name(ldr_entry.dll_base);
        assert!(sample_w.is_some());
        let sample = sample_w.unwrap();
        assert_eq!(sample, "exe64win_msgbox.pe");

        // follow to next flink
        ldr_entry = structures::LdrDataTableEntry64::load(ldr_entry.in_load_order_links.flink, &mut emu.maps);

        assert_eq!(ldr_entry.in_memory_order_links.flink, ldr_entry.in_load_order_links.flink + 0x10);
        assert_eq!(ldr_entry.in_initialization_order_links.flink, ldr_entry.in_memory_order_links.flink + 0x10);

        assert_eq!(ldr_entry.in_memory_order_links.blink, ldr_entry.in_load_order_links.blink + 0x10);
        assert_eq!(ldr_entry.in_initialization_order_links.blink, ldr_entry.in_memory_order_links.blink + 0x10);

        let sample_w = emu.maps.get_addr_name(ldr_entry.dll_base);
        assert!(sample_w.is_some());
        let sample = sample_w.unwrap();
        assert_eq!(sample, "ntdll.pe");

        let ntdll_str_ptr = ldr_entry.base_dll_name.buffer as u64;
        assert!(ntdll_str_ptr > 0);
        let ntdll_str = emu.maps.read_wide_string(ntdll_str_ptr);
        assert_eq!(ntdll_str, "ntdll.dll");

        let ntdll_str_ptr = ldr_entry.full_dll_name.buffer as u64;
        assert!(ntdll_str_ptr > 0);
        let ntdll_str = emu.maps.read_wide_string(ntdll_str_ptr);
        assert_eq!(ntdll_str, "C:\\Windows\\System32\\ntdll.dll");


    }
}
