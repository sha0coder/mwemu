// cargo test -- --nocapture

#[cfg(test)]
mod tests {
    use std::io::Write as _;
    //use log::{info, warn, error, debug};
    use std::sync::Once;

    use crate::emu::Emu;
    use crate::fpu::FPU;
    use crate::fpu::f80::F80;
    use crate::emu64;
    use crate::emu32;
    use crate::serialization::Serialization;

    static INIT: Once = Once::new();

    fn setup() {
        INIT.call_once(|| {
            env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("error"))
                .format(|buf, record| writeln!(buf, "{}", record.args()))
                .init();
        });
    }

    #[test]
    // test F80 emulation
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
        emu.init(false, false);

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
        emu.init(false, false);

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
        emu.init(false, false);

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
        emu.init(false, false);

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
        emu.init(false, false);

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
        emu.init(false, false);

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
        emu.cfg.maps_folder = "../maps64/".to_string();
        emu.init(false, false);

        let sample = "../test/elf64lin_static_helloworld.bin";
        emu.load_code(sample);
        emu.run(Some(0x40425f));

        assert_eq!(emu.regs.rax, 0xd80);
    }

    #[test]
    // this tests a linux 64bits raw arithmetic code.
    fn sc64lin_arith_100iter() {
        setup();

        let mut emu = emu64();
        emu.cfg.maps_folder = "../maps64/".to_string();
        emu.init(false, false);

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
        emu.init(false, false);

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
        emu.init(false, false);

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
    fn sc64lin_flags() {
        setup();

        let mut emu = emu64();
        emu.cfg.maps_folder = "../maps64/".to_string();
        emu.init(false, false);

        let sample = "../test/sc64lin_flags.bin";
        emu.load_code(sample);

        // test instruction add
        emu.run(Some(0x3c0014));
        assert_eq!(emu.flags.f_cf, true);
        assert_eq!(emu.flags.f_of, false);
        assert_eq!(emu.flags.f_zf, true);
        assert_eq!(emu.flags.f_sf, false);
        assert_eq!(emu.flags.f_pf, true);

        // test instruction sub
        emu.run(Some(0x3c002b));
        assert_eq!(emu.flags.f_cf, false);
        assert_eq!(emu.flags.f_of, false);
        assert_eq!(emu.flags.f_zf, true);
        assert_eq!(emu.flags.f_sf, false);
        assert_eq!(emu.flags.f_pf, true);

        // test instruction cmp
        emu.run(Some(0x3c0042));
        assert_eq!(emu.flags.f_cf, true);
        assert_eq!(emu.flags.f_of, false);
        assert_eq!(emu.flags.f_zf, false);
        assert_eq!(emu.flags.f_sf, true);
        assert_eq!(emu.flags.f_pf, false);


        // test instruction test
        emu.run(Some(0x3c0059));
        assert_eq!(emu.flags.f_cf, false);
        assert_eq!(emu.flags.f_of, false);
        assert_eq!(emu.flags.f_zf, true);
        assert_eq!(emu.flags.f_sf, false);
        assert_eq!(emu.flags.f_pf, true);

        // test and
        emu.run(Some(0x3c0070));
        assert_eq!(emu.flags.f_cf, false);
        assert_eq!(emu.flags.f_of, false);
        assert_eq!(emu.flags.f_zf, true);
        assert_eq!(emu.flags.f_sf, false);
        assert_eq!(emu.flags.f_pf, true);


        // test or with 0x0
        emu.run(Some(0x3c008c));
        assert_eq!(emu.flags.f_cf, false);
        assert_eq!(emu.flags.f_of, false);
        assert_eq!(emu.flags.f_zf, false);
        assert_eq!(emu.flags.f_sf, true);
        assert_eq!(emu.flags.f_pf, true);

        // test shl
        emu.run(Some(0x3c00a3));
        assert_eq!(emu.flags.f_cf, true);
        assert_eq!(emu.flags.f_of, true);
        assert_eq!(emu.flags.f_zf, true);
        assert_eq!(emu.flags.f_sf, false);
        assert_eq!(emu.flags.f_pf, true);

        // test add
        emu.run(Some(0x3c00bf));
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
        emu.init(false, false);

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
        emu.init(false, false);

        let sample = "../test/sc32win_donut.bin";
        emu.load_code(sample);
        emu.run_to(30_862_819);

        assert_eq!(emu.regs.get_eax(), 0x7f937230);
        assert_eq!(emu.regs.get_ebx(), 0xc);
    }

}
