use crate::maps::mem64::Permission;
use crate::{tests::helpers, *};

#[test]
pub fn test_unified_step_and_run_methods() {
    helpers::setup();

    // Test 1: Single-threaded mode (default)
    let mut emu = emu64();
    assert_eq!(
        emu.is_threading_enabled(),
        false,
        "Threading should be disabled by default"
    );

    // Load some simple code - NOP instructions
    let code = vec![0x90, 0x90, 0x90]; // 3 NOP instructions
    emu.maps
        .create_map("code", 0x1000, 0x1000, Permission::READ_WRITE_EXECUTE);
    emu.maps.write_bytes(0x1000, code);
    emu.regs_mut().rip = 0x1000;

    // Test step() in single-threaded mode
    let result = emu.step();
    assert!(result, "Step should succeed in single-threaded mode");
    assert_eq!(emu.regs().rip, 0x1001, "RIP should advance after NOP");

    // Test 2: Enable threading and verify it's set
    emu.enable_threading(true);
    assert_eq!(
        emu.is_threading_enabled(),
        true,
        "Threading should be enabled"
    );

    // Step again with threading enabled (but still only 1 thread)
    let result = emu.step();
    assert!(result, "Step should succeed with threading enabled");
    assert_eq!(
        emu.regs().rip,
        0x1002,
        "RIP should advance after second NOP"
    );

    // Test 3: Verify run() method works
    let mut emu2 = emu32();
    emu2.maps
        .create_map("code", 0x1000, 0x1000, Permission::READ_WRITE_EXECUTE);
    let code32 = vec![0x90, 0x90, 0xC3]; // 2 NOPs and RET
    emu2.maps.write_bytes(0x1000, code32);
    emu2.regs_mut().set_eip(0x1000);

    // Create a minimal stack for the RET instruction
    emu2.maps
        .create_map("stack", 0x2000, 0x1000, Permission::READ_WRITE);
    emu2.regs_mut().set_esp(0x2500);
    emu2.maps.write_dword(0x2500, 0x3000); // Return address

    // Run until RET
    let result = emu2.run(Some(0x1002));
    assert!(result.is_ok(), "Run should succeed");

    // Test 4: Verify threading can be toggled
    let mut cfg = Config::new();
    cfg.enable_threading = false;
    assert_eq!(cfg.enable_threading, false);
    cfg.enable_threading = true;
    assert_eq!(cfg.enable_threading, true);
}
