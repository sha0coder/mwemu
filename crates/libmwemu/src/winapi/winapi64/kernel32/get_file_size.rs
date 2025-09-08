use crate::emu;
use crate::winapi::helper;

pub fn GetFileSize(emu: &mut emu::Emu) {
    let h_file = emu.regs().rcx;
    let lp_file_size_high = emu.regs().rdx as usize;
    log_red!(
        emu,
        "** {} kernel32!GetFileSize {:x} {:x}",
        emu.pos,
        h_file,
        lp_file_size_high
    );
    // TODO: Implement this

    let name = helper::handler_get_uri(h_file);
    if name == "HaspEmul.dll" {
        let size = std::fs::metadata("/Users/jesus/Downloads/enigma/surprise.dll")
            .unwrap()
            .len();
        log::info!(
            "** {} kernel32!GetFileSize {:x} {:x} size: {}",
            emu.pos,
            h_file,
            lp_file_size_high,
            size
        );
        emu.regs_mut().rax = size as u64;
    } else {
        panic!("unknown file");
    }
}
