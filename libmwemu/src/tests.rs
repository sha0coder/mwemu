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
    #[ignore]
    // this tests a linux 64bits static binary.
    fn elf64lin_static_helloworld() {
        setup();

        let mut emu = emu64();
        emu.cfg.maps_folder = "../maps64/".to_string();
        emu.init(false, false);

        let sample = "../test/sc32win_peb_ldr_rot.bin";
        emu.load_code(sample);
    }



    #[test]
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

}
