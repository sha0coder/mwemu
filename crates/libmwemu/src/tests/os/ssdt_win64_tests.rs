use crate::tests::helpers;
use crate::*;
use iced_x86::Mnemonic;
use std::cell::RefCell;
use std::rc::Rc;

/// Step size for absolute `run_to` targets while searching for the first `syscall`.
const MSG_SYSCALL_STEP: u64 = 25_000;
/// Upper bound on instructions scanned for the first Windows `syscall` (ntdll stub).
const MSG_SYSCALL_SCAN_CAP: u64 = 1_000_000;

#[test]
fn exe64win_msgbox_ssdt_reaches_cli_trace_window() {
    helpers::setup();

    let mut emu = emu64();
    emu.cfg.maps_folder = helpers::win64_maps_folder();
    emu.cfg.emulate_winapi = true; // same behavior as command line --ssdt

    let sample = helpers::test_data_path("exe64win_msgbox.bin");
    assert!(
        std::path::Path::new(&sample).is_file(),
        "missing test sample (run from mwemu repo): {}",
        sample
    );
    emu.load_code(&sample);

    // Fast smoke: SS/--ssdt still runs forward.
    let target = 120u64;
    emu.run_to(target).unwrap_or_else(|e| {
        panic!(
            "run_to({}) failed: {} (pos={} rip=0x{:x})",
            target,
            e,
            emu.pos,
            emu.regs().rip
        );
    });
    assert!(emu.pos >= target);
}

/// Runs the msgbox PE until the first **`syscall`** in Windows mode (`!linux`), or until
/// [`MSG_SYSCALL_SCAN_CAP`] instructions, whichever comes first.
///
/// Stops as soon as the hook sees `Mnemonic::Syscall` so we do not keep emulating after the
/// syscall (avoids extra work and reduces the chance of hitting unrelated emulator bugs later).
///
/// **Ignored by default**: scanning can take a long time in `debug` builds; long runs may also
/// hit unimplemented operand paths before any syscall. Run locally with:
/// `cargo test -p libmwemu exe64win_msgbox_ssdt_hits_first_windows_syscall --release -- --ignored --nocapture`
#[test]
#[ignore = "slow: run with --release -- --ignored; may need MSG_SYSCALL_SCAN_CAP raised"]
fn exe64win_msgbox_ssdt_hits_first_windows_syscall() {
    helpers::setup();

    let mut emu = emu64();
    emu.cfg.maps_folder = helpers::win64_maps_folder();
    emu.cfg.emulate_winapi = true;

    let sample = helpers::test_data_path("exe64win_msgbox.bin");
    assert!(
        std::path::Path::new(&sample).is_file(),
        "missing {}",
        sample
    );

    let hit = Rc::new(RefCell::new(false));
    let hit_flag = Rc::clone(&hit);
    emu.hooks
        .on_post_instruction(move |emu, _rip, ins, _sz, _ok| {
            if emu.os.is_windows() && ins.is_x86() && ins.as_x86().mnemonic() == Mnemonic::Syscall {
                *hit_flag.borrow_mut() = true;
            }
        });

    emu.load_code(&sample);

    let mut next_goal = MSG_SYSCALL_STEP;
    while !*hit.borrow() && emu.pos < MSG_SYSCALL_SCAN_CAP {
        let goal = next_goal.min(MSG_SYSCALL_SCAN_CAP);
        if goal <= emu.pos {
            break;
        }
        emu.run_to(goal).unwrap_or_else(|e| {
            panic!(
                "run_to({}) failed: {} (pos={} rip=0x{:x})",
                goal,
                e,
                emu.pos,
                emu.regs().rip
            );
        });
        if *hit.borrow() {
            break;
        }
        next_goal += MSG_SYSCALL_STEP;
    }

    assert!(
        *hit.borrow(),
        "no Windows `syscall` before pos {} (rip=0x{:x}). Raise MSG_SYSCALL_SCAN_CAP if the CRT path grew.",
        MSG_SYSCALL_SCAN_CAP,
        emu.regs().rip
    );
}

#[test]
fn exe64win_mingw_ssdt_reaches_early_execution_window() {
    helpers::setup();

    let mut emu = emu64();
    emu.cfg.maps_folder = helpers::win64_maps_folder();
    emu.cfg.emulate_winapi = true; // same behavior as command line --ssdt

    let sample = helpers::test_data_path("exe64win_mingw.bin");
    assert!(
        std::path::Path::new(&sample).is_file(),
        "missing {}",
        sample
    );
    emu.load_code(&sample);

    emu.run_to(120)
        .expect("ssdt mingw should reach early execution window");
    assert!(emu.pos >= 120);
}

/// Verifies that `ntdll!LdrInitializeThunk` can be emulated for more than 15 000 instructions
/// under `--ssdt` mode, equivalent to:
///
/// ```text
/// cargo run --release -- -f test/exe64win_msgbox.bin -6 --ssdt
/// ```
///
/// `load_code` with `emulate_winapi = true` calls `LdrInitializeThunk` internally via
/// `call64`; `emu.pos` after `load_code` reflects instructions executed inside that call.
/// We use banzai mode so unimplemented instructions are skipped rather than aborting early.
///
/// Run with:
/// ```text
/// cargo test -p libmwemu ssdt_ldr_initialize_thunk_executes_over_15000_instructions \
///     --release -- --ignored --nocapture
/// ```
#[test]
#[ignore = "slow: run with --release -- --ignored --nocapture"]
fn ssdt_ldr_initialize_thunk() {
    helpers::setup();

    let mut emu = emu64();
    emu.cfg.maps_folder = helpers::win64_maps_folder();
    emu.cfg.emulate_winapi = true; // --ssdt
    emu.cfg.skip_unimplemented = true; // --banzai: skip unimplemented, keep going
    emu.maps.set_banzai(true);

    let sample = helpers::test_data_path("exe64win_msgbox.bin");
    assert!(
        std::path::Path::new(&sample).is_file(),
        "missing test sample: {}",
        sample
    );

    // load_code triggers ntdll!LdrInitializeThunk via call64 when emulate_winapi is set.
    // emu.pos counts every instruction executed during that internal call.
    emu.load_code(&sample);

    assert!(
        emu.pos > 15_000,
        "LdrInitializeThunk emulation only reached {} instructions (need > 15 000). \
         rip=0x{:x}  — check for early crash or unimplemented syscall.",
        emu.pos,
        emu.regs().rip
    );
}
