#![no_main]
use libfuzzer_sys::fuzz_target;
use libmwemu::emu::Emu;
use std::panic;

fuzz_target!(|data: &[u8]| {
    if data.len() < 2 {
        return;
    }
    let _ = panic::catch_unwind(panic::AssertUnwindSafe(|| {
        let mut emu = Emu::new(libmwemu::arch::Arch::X86);
        let _ = emu.load_code_bytes(data);
    }));
});
