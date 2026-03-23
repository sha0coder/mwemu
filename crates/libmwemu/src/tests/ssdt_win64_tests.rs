use crate::tests::helpers;
use crate::*;

#[test]
fn exe64win_msgbox_ssdt_reaches_cli_trace_window() {
    helpers::setup();

    let mut emu = emu64();
    emu.cfg.maps_folder = "../../maps/maps64/".to_string();
    emu.cfg.emulate_winapi = true; // same behavior as command line --ssdt

    let sample = "../../test/exe64win_msgbox.bin";
    emu.load_code(sample);

    emu.run_to(240).expect("ssdt msgbox should reach ~240 instructions");
    assert!(emu.pos >= 240);
}

#[test]
fn exe64win_mingw_ssdt_reaches_early_execution_window() {
    helpers::setup();

    let mut emu = emu64();
    emu.cfg.maps_folder = "../../maps/maps64/".to_string();
    emu.cfg.emulate_winapi = true; // same behavior as command line --ssdt

    let sample = "../../test/exe64win_mingw.bin";
    emu.load_code(sample);

    emu.run_to(120)
        .expect("ssdt mingw should reach early execution window");
    assert!(emu.pos >= 120);
}
