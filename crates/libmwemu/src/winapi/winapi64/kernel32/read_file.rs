use crate::emu;
use crate::winapi::helper;

pub fn ReadFile(emu: &mut emu::Emu) {
    let file_hndl = emu.regs().rcx;
    let buff = emu.regs().rdx;
    let size = emu.regs().r8;
    let bytes_read = emu.regs().r9;
    let overlapped = emu
        .maps
        .read_qword(emu.regs().rsp + 0x20)
        .expect("kernel32!ReadFile cannot read the overlapped");

    log_red!(
        emu,
        "kernel32!ReadFile hndl: 0x{:x} buff: 0x{:x} sz: {} bytes_read: {:x} overlapped: {:x}",
        file_hndl,
        buff,
        size,
        bytes_read,
        overlapped
    );

    if !helper::handler_exist(file_hndl) {
        panic!("\tinvalid handle.")
    }

    let name = helper::handler_get_uri(file_hndl);
    log_red!(
        emu,
        "** {} kernel32!ReadFile name = {name} {}",
        emu.pos,
        emu.colors.nc
    );

    if name == "HaspEmul.dll" {
        let bytes = std::fs::read("/Users/jesus/Downloads/enigma/surprise.dll").unwrap();
        if size as usize > bytes.len() {
            panic!("size is greater than the file size");
        }
        if bytes_read != 0 {
            emu.maps.write_qword(bytes_read, size);
        }
        emu.maps.write_bytes(buff, bytes);
        emu.regs_mut().rax = 1;
    } else {
        panic!("unknown file");
    }
}
