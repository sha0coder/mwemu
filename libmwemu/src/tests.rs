// cargo test -- --nocapture

#[cfg(test)]
mod tests {
    use std::io::Write as _;
    //use log::{info, warn, error, debug};
    use std::sync::Once;

    use crate::emu::Emu;
    use crate::fpu::FPU;
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
    // this tests a linux 64bits raw arithmetic code.
    fn sc32lin_raw() {
        setup();

        let mut emu = emu32();
        emu.cfg.maps_folder = "../maps32/".to_string();
        emu.init(false, false);

        let sample = "../test/sc32lin_raw.bin";
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


        fpu.set_st(1, 0.0);
        assert_eq!(fpu.get_st(1), 0.0);


        // u80 to f64 conversion
        fpu.set_st_u80(1, 0x4000c90fdaa22168c235);
        assert_eq!(fpu.get_st(1), 3.14159265358979323);
        
        assert_eq!(3.141592653589793239, 
                   3.141592653589793);  // true 
                                        //

        // f64 to u80 conversion
        //fpu.set_st(1, 4.141592653589793238);
        //assert_eq!(fpu.get_st_u80(1), 0x4000c90fdaa22168c234);
    }


    #[test]
    #[ignore]
    // this tests the fpu unit.
    fn elf64lin_fpu() {
        setup();

        let mut emu = emu64();
        emu.cfg.maps_folder = "../maps64/".to_string();
        emu.init(false, false);

        let sample = "../test/elf64lin_fpu.bin";
        emu.load_code(sample);
        emu.step(); // fninit
        emu.step(); // fld1
        assert_eq!(emu.fpu.get_st_u80(7), 0);
        emu.step(); // fldpi
        assert_eq!(emu.fpu.get_st_u80(7), 0x3fff8000000000000000);
        assert_eq!(emu.fpu.get_st_u80(6), 0x4000c90fdaa22168c235);
        assert_eq!(emu.fpu.get_st(6), 3.141592653589793239);
        emu.step(); // fadd   st,st(1)
        assert_eq!(emu.fpu.get_st_u80(6), 0x40018487ed5110b4611a);
        assert_eq!(emu.fpu.get_st(6), 4.141592653589793238);
        emu.step(); // fsub   st,st(1)
        assert_eq!(emu.fpu.get_st_u80(6), 0x4000c90fdaa22168c234);
        assert_eq!(emu.fpu.get_st(6), 3.141592653589793238);
        emu.step(); // fsubr  st,st(1)
        assert_eq!(emu.fpu.get_st_u80(6), 0xc000890fdaa22168c234);
        assert_eq!(emu.fpu.get_st(6), -2.141592653589793238);
        emu.step(); // fchs
        assert_eq!(emu.fpu.get_st_u80(6), 0x4000890fdaa22168c234);
        assert_eq!(emu.fpu.get_st(6), 2.141592653589793238);
        emu.step(); // fsqrt
        assert_eq!(emu.fpu.get_st_u80(6), 0x3fffbb51491ea66b6ea4);
        assert_eq!(emu.fpu.get_st(6), 1.463418140378816419);
        emu.step(); //  fxch   st(1)
        assert_eq!(emu.fpu.get_st_u80(7), 0x3fffbb51491ea66b6ea4);
        assert_eq!(emu.fpu.get_st_u80(6), 0x3fff8000000000000000);
        assert_eq!(emu.fpu.get_st(7), 1.463418140378816419);
        emu.step(); //  fptan
        assert_eq!(emu.fpu.get_st_u80(7), 0x3fffbb51491ea66b6ea4);
        assert_eq!(emu.fpu.get_st_u80(6), 0x3fffc75922e5f71d2dc6);
        assert_eq!(emu.fpu.get_st_u80(5), 0x3fff8000000000000000);
        assert_eq!(emu.fpu.get_st(7), 1.463418140378816419);
        assert_eq!(emu.fpu.get_st(6), 1.557407724654902231);
        assert_eq!(emu.fpu.get_st(5), 1.0);
        emu.step(); //  fmulp  st(1),st
        assert_eq!(emu.fpu.get_st_u80(7), 0x3fffbb51491ea66b6ea4);
        assert_eq!(emu.fpu.get_st_u80(6), 0x3fffc75922e5f71d2dc6);
        assert_eq!(emu.fpu.get_st(7), 1.463418140378816419);
        assert_eq!(emu.fpu.get_st(6), 1.557407724654902231);
        emu.step(); // fdivp  st(1),st
        assert_eq!(emu.fpu.get_st_u80(7), 0x3ffef08ce6b636464375);
        assert_eq!(emu.fpu.get_st_u80(6), 0x3fffc75922e5f71d2dc6);
        assert_eq!(emu.fpu.get_st_u80(5), 0x3fff8000000000000000);
        assert_eq!(emu.fpu.get_st(7), 0.9396499819615878249);
        emu.step(); // fsubp  st(1),st
        assert_eq!(emu.fpu.get_st_u80(0), 0xffffc000000000000000);
        emu.step(); // f2xm1
        assert_eq!(emu.fpu.get_st_u80(0), 0xffffc000000000000000);
        emu.step(); // fld1
        assert_eq!(emu.fpu.get_st_u80(7), 0x3fff8000000000000000);
        assert_eq!(emu.fpu.get_st_u80(6), 0x3fffc75922e5f71d2dc6);
        assert_eq!(emu.fpu.get_st_u80(5), 0x3fff8000000000000000);
        assert_eq!(emu.fpu.get_st_u80(0), 0xffffc000000000000000);
        assert_eq!(emu.fpu.get_st(7), 1.0);
        emu.step(); // fldlg2
        assert_eq!(emu.fpu.get_st_u80(6), 0x3ffd9a209a84fbcff799);
        assert_eq!(emu.fpu.get_st(6), 0.3010299956639811952);
        emu.step(); // fyl2x
        assert_eq!(emu.fpu.get_st_u80(7), 0xbfffddb2dbec0456f846);
        assert_eq!(emu.fpu.get_st(7), -1.732020845644619341);
        emu.step(); // fld1
        emu.step(); // fld1
        assert_eq!(emu.fpu.get_st_u80(7), 0xbfffddb2dbec0456f846);
        assert_eq!(emu.fpu.get_st_u80(6), 0x3fff8000000000000000);
        assert_eq!(emu.fpu.get_st_u80(5), 0x3fff8000000000000000);
        assert_eq!(emu.fpu.get_st_u80(0), 0xffffc000000000000000);
        assert_eq!(emu.fpu.get_st(7), -1.732020845644619341);
        emu.step(); // fyl2xp1
        assert_eq!(emu.fpu.get_st_u80(7), 0xbfffddb2dbec0456f846);
        assert_eq!(emu.fpu.get_st_u80(6), 0x3fff8000000000000000);
        assert_eq!(emu.fpu.get_st_u80(5), 0x3fff8000000000000000);
        assert_eq!(emu.fpu.get_st_u80(0), 0xffffc000000000000000);
        assert_eq!(emu.fpu.get_st(7), -1.732020845644619341);
        emu.step(); // fucom  st(1)
        emu.step(); // fcmovnbe st(0), st(1)
        assert_eq!(emu.fpu.get_st_u80(7), 0xbfffddb2dbec0456f846);
        assert_eq!(emu.fpu.get_st_u80(6), 0xbfffddb2dbec0456f846);
        assert_eq!(emu.fpu.get_st(7), -1.732020845644619341);
        assert_eq!(emu.fpu.get_st(6), -1.732020845644619341);
        assert_eq!(emu.fpu.get_st_u80(0), 0xffffc000000000000000);
        emu.step(); // fcmovnu st(0), st(1)
        assert_eq!(emu.fpu.get_st_u80(0), 0xffffc000000000000000);
        emu.step(); // fstp   st(0)
        emu.step(); // fstp   st(0)
        emu.step(); // fstp   st(0)
        assert_eq!(emu.fpu.get_st_u80(7), 0xbfffddb2dbec0456f846);
        assert_eq!(emu.fpu.get_st_u80(6), 0xbfffddb2dbec0456f846);
        assert_eq!(emu.fpu.get_st_u80(0), 0xffffc000000000000000);
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

        emu.run(Some(0x3c0014));
        assert_eq!(emu.regs.rax, 0x57);

        emu.run(Some(0x3c002b));
        assert_eq!(emu.regs.rax, 0x46);

        emu.run(Some(0x3c0042));
        assert_eq!(emu.regs.rax, 0x93);

        emu.run(Some(0x3c0059));
        assert_eq!(emu.regs.rax, 0x56);

        emu.run(Some(0x3c0070));
        assert_eq!(emu.regs.rax, 0x56);

        emu.run(Some(0x3c008c));
        assert_eq!(emu.regs.rax, 0x96);

        emu.run(Some(0x3c00a3));
        assert_eq!(emu.regs.rax, 0x56);

        emu.run(Some(0x3c00bf));
        assert_eq!(emu.regs.rax, 0x896);
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
