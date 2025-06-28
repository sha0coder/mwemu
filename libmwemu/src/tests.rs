// cargo test -- --nocapture

#[cfg(test)]
mod tests {
    use std::io::Write as _;
    use std::sync::Once;

    use crate::emu::Emu;
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
    // this tests the fpu unit.
    fn elf64lin_fpu() {
        setup();

        let mut emu = emu64();
        emu.cfg.maps_folder = "../maps64/".to_string();
        emu.init(false, false);

        let sample = "../test/elf64lin_fpu.bin";
        emu.load_code(sample);
        emu.run_to(24);

        // for now the test is just emulate that fpu instructions with no crash
        // but calculations are not ok, will test also calculations.

        /*
        assert_eq!(emu.fpu.get_st(0), f640xffffc000000000000000);
        assert_eq!(emu.fpu.get_st(5), 0x3fff8000000000000000);
        assert_eq!(emu.fpu.get_st(6), 0xbfffddb2dbec0456f846);
        assert_eq!(emu.fpu.get_st(7), 0xbfffddb2dbec0456f846);
        */
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
