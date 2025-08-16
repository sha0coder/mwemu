
use crate::{constants, emu};

pub fn UnhandledExceptionFilter(emu: &mut emu::Emu) {
    let exception_info = emu.regs().rcx;

    log::info!(
        "{}** {} kernel32!UnhandledExceptionFilter  exception_info: 0x{:x} {}",
        emu.colors.light_red,
        emu.pos,
        exception_info,
        emu.colors.nc
    );

    emu.regs_mut().rax = constants::EXCEPTION_EXECUTE_HANDLER as u64;
    // a debugger would had answered EXCEPTION_CONTINUE_SEARCH
}