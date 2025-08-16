use crate::emu;
use crate::structures;

pub fn GetSystemTime(emu: &mut emu::Emu) {
    let out_time = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!GetSystemTime cannot read out_time param") as u64;

    log::info!(
        "{}** {} kernel32!GetSystemTime ptr: 0x{:x}' {}",
        emu.colors.light_red,
        emu.pos,
        out_time,
        emu.colors.nc
    );
    let systime = structures::SystemTime::now();
    systime.save(out_time, &mut emu.maps);

    emu.stack_pop32(false);
}