use crate::tests::helpers;
use crate::winapi::winapi64;
use crate::*;

#[test]
// test 64bits allocators
pub fn allocator64_test() {
    helpers::setup();

    let mut emu = emu64();
    emu.cfg.maps_folder = "../../maps/maps64/".to_string();
    emu.init(false, false);

    assert_eq!(emu.maps.exists_mapname("shell32.rsrc"), true);
    assert_eq!(emu.maps.get_map_by_name("shell32.rsrc").is_some(), true);
    assert_eq!(emu.maps.exists_mapname("notexist"), false);
    assert_eq!(emu.maps.get_map_by_name("notexist").is_some(), false);

    for _ in 0..700 {
        assert_eq!(emu.maps.alloc(1024).is_some(), true);
        assert_eq!(emu.maps.lib64_alloc(1024).is_some(), true);
    }

    assert_eq!(emu.maps.mem_test(), true);

    emu.maps.clear();

    emu.regs_mut().rcx = 0; // addr
    emu.regs_mut().rdx = 1024; // sz
    emu.regs_mut().r8 = constants::MEM_RESERVE as u64;
    emu.regs_mut().r9 = 0x40; // rwx
    winapi64::kernel32::VirtualAlloc(&mut emu);
    assert_eq!(emu.maps.is_allocated(emu.regs().rax), true);

    emu.regs_mut().rcx = 0x30000000; // addr
    emu.regs_mut().rdx = 1024; // sz
    emu.regs_mut().r8 = (constants::MEM_RESERVE | constants::MEM_COMMIT) as u64;
    emu.regs_mut().r9 = 0x40; // rwx
    winapi64::kernel32::VirtualAlloc(&mut emu);

    emu.regs_mut().rcx = 0x30000000; // addr
    emu.regs_mut().rdx = 1024; // sz
    emu.regs_mut().r8 = constants::MEM_COMMIT as u64;
    emu.regs_mut().r9 = 0x40; // rwx
    winapi64::kernel32::VirtualAlloc(&mut emu);
    assert_eq!(emu.regs().rax, 0x30000000);

    assert_eq!(emu.maps.is_allocated(0x30000000), true);
    assert_eq!(emu.maps.mem_test(), true);
}
