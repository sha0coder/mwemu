use crate::emu;

pub fn Sleep(emu: &mut emu::Emu) {
    let millis = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!Sleep cannot read millis");

    log_red!(emu, "kernel32!Sleep millis: {}", millis);

    emu.tick += millis as usize;

    emu.stack_pop32(false);
}
