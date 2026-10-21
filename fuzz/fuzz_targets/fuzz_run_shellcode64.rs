#![no_main]
use libfuzzer_sys::fuzz_target;
use libmwemu::emu::Emu;

fuzz_target!(|data: &[u8]| {
    if data.len() < 4 || data.len() > 4096 {
        return;
    }
    let mut emu = Emu::new(libmwemu::arch::Arch::X86_64);
    if emu.load_code_bytes(data).is_ok() {
        emu.cfg.verbose = 0;
        emu.cfg.max_instructions = Some(1000);
        let _ = emu.run(None);
    }
});
