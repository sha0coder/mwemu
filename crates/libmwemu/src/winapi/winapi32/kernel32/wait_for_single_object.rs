use crate::emu;
use crate::constants;

pub fn WaitForSingleObject(emu: &mut emu::Emu) {
    let handle = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!WaitForSingleObject error reading handle") as u64;
    let millis = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!WaitForSingleObject error reading millis");

    log::info!(
        "{}** {} kernel32!WaitForSingleObject  hndl: {} millis: {} {}",
        emu.colors.light_red,
        emu.pos,
        handle,
        millis,
        emu.colors.nc
    );

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.regs_mut().rax = constants::WAIT_TIMEOUT;
}