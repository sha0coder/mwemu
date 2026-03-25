use std::io::Write as _;
use std::path::PathBuf;
use std::sync::Once;

static INIT: Once = Once::new();

pub fn setup() {
    INIT.call_once(|| {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("error"))
            .format(|buf, record| writeln!(buf, "{}", record.args()))
            .init();
    });
}

/// `rel` is a filename inside the repo top-level `test/` directory (e.g. `exe64win_msgbox.bin`).
/// Resolves via `CARGO_MANIFEST_DIR` so tests work even when the current working directory is not `crates/libmwemu`.
pub fn test_data_path(rel: &str) -> String {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../test")
        .join(rel)
        .to_string_lossy()
        .into_owned()
}

/// Maps folder for 64-bit Windows samples (`maps/maps64/`).
pub fn maps64_folder() -> String {
    let mut s = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../maps/maps64")
        .to_string_lossy()
        .into_owned();
    if !s.ends_with('/') {
        s.push('/');
    }
    s
}

pub fn critical_values(bits: u32) -> Vec<u64> {
    let max = match bits {
        8 => u8::MAX as u64,
        16 => u16::MAX as u64,
        32 => u32::MAX as u64,
        64 => u64::MAX,
        _ => panic!("Unsupported size"),
    };

    let sign_bit = 1u64 << (bits - 1);

    vec![
        0,
        1,
        max,
        sign_bit,
        sign_bit - 1,
        sign_bit + 1,
        0x55,                                 // 01010101
        0xAA,                                 // 10101010
        0xFFFFFFFFFFFFFFFFu64 >> (64 - bits), // all 1s for the width
    ]
}

pub fn shift_counts(bits: u32) -> Vec<u64> {
    vec![
        0,
        1,
        bits as u64 - 1,
        bits as u64,
        bits as u64 + 1,
        63,
        64,
        127,
    ]
}
