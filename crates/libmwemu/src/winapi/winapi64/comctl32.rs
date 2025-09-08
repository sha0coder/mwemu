use crate::emu;
use crate::serialization;
use crate::winapi::winapi64;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = winapi64::kernel32::guess_api_name(emu, addr);
    match api.as_str() {
        "InitCommonControls" => InitCommonControls(emu),
        "InitCommonControlsEx" => InitCommonControlsEx(emu),
        _ => {
            if emu.cfg.skip_unimplemented == false {
                if emu.cfg.dump_on_exit && emu.cfg.dump_filename.is_some() {
                    serialization::Serialization::dump_to_file(
                        &emu,
                        emu.cfg.dump_filename.as_ref().unwrap(),
                    );
                }

                unimplemented!("atemmpt to call unimplemented API 0x{:x} {}", addr, api);
            }
            log::warn!(
                "calling unimplemented API 0x{:x} {} at 0x{:x}",
                addr,
                api,
                emu.regs().rip
            );
            return api;
        }
    }
    String::new()
}

/*
void InitCommonControls();
*/
fn InitCommonControls(emu: &mut emu::Emu) {
    log_red!(emu, "comctl32!InitCommonControls");
    // TODO: do something
}

/*
BOOL InitCommonControlsEx(
  [in] const INITCOMMONCONTROLSEX *piccs
);
*/
fn InitCommonControlsEx(emu: &mut emu::Emu) {
    log_red!(emu, "comctl32!InitCommonControlsEx");
    // TODO: do something
    emu.regs_mut().rax = 1; // TRUE
}
