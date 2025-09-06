use crate::{constants, emu};

pub fn WaitForSingleObject(emu: &mut emu::Emu) {
    let hndl = emu.regs().rcx;
    let millis = emu.regs().rdx;

    log_red!(
        emu,
        "kernel32!WaitForSingleObject  hndl: {} millis: {}",
        hndl,
        millis
    );

    emu.regs_mut().rax = constants::WAIT_TIMEOUT;
}
