use crate::tests::helpers;
use crate::winapi::winapi32;
use crate::*;

#[test]
// test 32bits allocators
pub fn allocator32_test() {
    helpers::setup();

    let mut emu = emu32();
    emu.cfg.maps_folder = "../../maps/maps32/".to_string();
    emu.maps.clear();
    emu.init(false, false);

    assert_eq!(emu.maps.exists_mapname("shell32.rsrc"), true);
    assert_eq!(emu.maps.get_map_by_name("shell32.rsrc").is_some(), true);
    assert_eq!(emu.maps.exists_mapname("notexist"), false);
    assert_eq!(emu.maps.get_map_by_name("notexist").is_some(), false);

    for _ in 0..700 {
        assert_eq!(emu.maps.alloc(1024).is_some(), true);
        assert_eq!(emu.maps.lib32_alloc(1024).is_some(), true);
    }

    assert_eq!(emu.maps.mem_test(), true);

    emu.stack_push32(0x40); // rwx
    emu.stack_push32(constants::MEM_RESERVE);
    emu.stack_push32(1024); // sz
    emu.stack_push32(0); // addr
    winapi32::kernel32::VirtualAlloc(&mut emu);
    assert_eq!(emu.maps.is_allocated(emu.regs().rax), true);

    emu.stack_push32(0x40); // rwx
    emu.stack_push32(constants::MEM_RESERVE | constants::MEM_COMMIT);
    emu.stack_push32(1024); // sz
    emu.stack_push32(0x30000000); // addr
    winapi32::kernel32::VirtualAlloc(&mut emu);

    emu.stack_push32(0x40); // rwx
    emu.stack_push32(constants::MEM_COMMIT);
    emu.stack_push32(1024); // sz
    emu.stack_push32(0x30000000); // addr
    winapi32::kernel32::VirtualAlloc(&mut emu);
    assert_eq!(emu.regs().rax, 0x30000000);

    assert!(emu.maps.is_allocated(0x30000000));
    assert!(emu.maps.mem_test());
}
