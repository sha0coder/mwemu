use crate::emu;

pub fn GetEnvironmentStrings(emu: &mut emu::Emu) {
    log_red!(
        emu,
        "kernel32!GetEnvironmentStrings"
    );
    let ptr = emu.alloc("environment", 1024);
    emu.maps.write_string(ptr, "PATH=c:\\Windows\\System32");
    emu.regs_mut().rax = ptr;
}