use crate::emu;
use crate::maps::mem64::Mem64;
use crate::peb64::init_ldr;
use crate::structures::{PEB64, TEB64};
use crate::

pub struct KUSER_SHARED_DATA {
    pub mem_region: Box<Mem64>,

}

pub fn init_kuser_shared_data(emu: &mut emu::Emu) {
    const KUSER_ADDR: u64 = 0x7ffe0000;
    let kuser_shared_addr = emu
        .maps
        .create_map("KUSER_SHARED_DATA", KUSER_ADDR, 0x1000)
        .expect("Cannot allocate KUSER_SHARED_DATA");



    let ldr = init_ldr(emu);

    let peb_addr = emu
        .maps
        .lib64_alloc(PEB64::size() as u64)
        .expect("cannot alloc the PEB64");
    let peb_map = emu
        .maps
        .create_map("peb", peb_addr, PEB64::size() as u64)
        .expect("cannot create peb map");
    // Create KUSER_SHARED_DATA map
    let process_parameters = 0x521e20;
    let peb = PEB64::new(0, ldr, process_parameters);
    peb.save(peb_map);
    emu.maps.write_byte(peb_addr + 2, 0); // not being_debugged

    let teb_addr = emu
        .maps
        .lib64_alloc(TEB64::size() as u64)
        .expect("cannot alloc the TEB64");
    let teb_map = emu
        .maps
        .create_map("teb", teb_addr, TEB64::size() as u64)
        .expect("cannot create teb map");
    let teb = TEB64::new(peb_addr);
    teb.save(teb_map);
}
