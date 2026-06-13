//! `--winver`: fetch genuine Windows system DLLs from Microsoft's public
//! symbol server on demand, keyed by the binary's PE `TimeDateStamp` +
//! `SizeOfImage`. This replaces the 8 GB `--iso` download with a few hundred
//! KB per DLL fetched lazily (~30 MB total for a typical process), and lets a
//! user select a build by a friendly name (`win11`) instead of a raw number.
//!
//! Two public services are used:
//!   * winbindex (`winbindex.m417z.com`) — a community index that maps every
//!     Windows binary + version to its PE `timestamp` and `virtualSize`, i.e.
//!     exactly the two fields that form the symbol-server key. We use it to
//!     turn `(kernelbase.dll, 26100.7920)` into that key without owning the
//!     file first.
//!   * msdl (`msdl.microsoft.com`) — Microsoft's symbol server, which serves
//!     the actual binary at `…/<name>/<KEY>/<name>` where
//!     `KEY = format!("{:08X}{:X}", timestamp, virtual_size)`.
//!
//! Cached layout: `maps/winver/<build>/x86_64/<dll>`, mirroring the `--iso`
//! cache so the rest of the emulator is unchanged — it just sees a maps folder.

use std::error::Error;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

/// Serializes symbol-server fetches so concurrent emulator threads don't race
/// on the shared maps cache or burst-hammer the server.
static FETCH_LOCK: Mutex<()> = Mutex::new(());

/// Friendly alias → Windows build (as it appears inside winbindex's `version`
/// string, e.g. "10.0.26100.7920"). A bare build number passed to `--winver`
/// is accepted verbatim, so power users can pin an exact build for repro.
///
/// Only one curated build per family is needed; add rows as new releases ship.
const WINVER_ALIASES: &[(&str, &str)] = &[
    ("win11", "26100.7920"), // 24H2/25H2 line — default
    ("win11@24h2", "26100.7920"),
    ("win11@22h2", "22621.7079"),
    ("win11@21h2", "22000.3260"),
    ("win10", "19041.7291"), // 22H2 (file version stays 19041)
    ("win10@22h2", "19041.7291"),
    ("win2019", "17763.8755"), // Windows Server 2019 / 1809
    // Note: Windows Server 2022 (build 20348) is not indexed by winbindex, so
    // there's no `win2022` alias — use `--iso` for that build.
];

/// Resolve a `--winver` value to a concrete build number. Friendly aliases map
/// through the table; anything else (e.g. "26100.7920") is taken verbatim.
pub fn resolve_build(name: &str) -> String {
    let n = name.trim().to_lowercase();
    WINVER_ALIASES
        .iter()
        .find(|(alias, _)| *alias == n)
        .map(|(_, build)| build.to_string())
        .unwrap_or_else(|| name.trim().to_string())
}

/// Human-readable list of the friendly aliases, for `--list-winvers` / errors.
pub fn known_aliases() -> Vec<(&'static str, &'static str)> {
    WINVER_ALIASES.to_vec()
}

/// Where a build's fetched DLLs are cached.
pub fn cache_folder(build: &str) -> PathBuf {
    PathBuf::from(format!("maps/winver/{}/x86_64/", build))
}

/// Ensure `<cache>/<basename>` exists, fetching it from the symbol server when
/// missing. `basename` is a lowercase DLL filename (e.g. "kernelbase.dll").
/// Returns the host path on success.
pub fn ensure_dll(
    cache: &Path,
    build: &str,
    basename: &str,
) -> Result<PathBuf, Box<dyn Error>> {
    let dest = cache.join(basename);
    // Fast path: already cached, no lock needed.
    if dest.is_file() {
        return Ok(dest);
    }
    // Serialize concurrent fetches: many emulator threads loading PEs at once
    // would otherwise hammer the symbol server and race on the shared cache /
    // index files. Re-check under the lock so we don't re-download what another
    // thread just fetched.
    let _guard = FETCH_LOCK.lock().unwrap();
    if dest.is_file() {
        return Ok(dest);
    }
    fs::create_dir_all(cache)?;

    let key = winbindex_key(basename, build)?;
    let url = format!(
        "https://msdl.microsoft.com/download/symbols/{name}/{key}/{name}",
        name = basename,
        key = key
    );
    log::trace!("--winver: fetching {} (key {}) from symbol server", basename, key);
    let bytes = http_get(&url)?;
    // The symbol server occasionally serves a small HTML/redirect body for an
    // unknown key; guard against caching garbage by requiring a PE header.
    if bytes.len() < 0x40 || &bytes[0..2] != b"MZ" {
        return Err(format!(
            "symbol server did not return a PE for {} (key {}, {} bytes)",
            basename,
            key,
            bytes.len()
        )
        .into());
    }
    // Write atomically-ish: temp then rename, so a killed download never leaves
    // a truncated DLL that later looks "present".
    let tmp = cache.join(format!(".{}.part", basename));
    fs::write(&tmp, &bytes)?;
    fs::rename(&tmp, &dest)?;
    log::trace!("--winver: cached {} ({} KB)", basename, bytes.len() / 1024);
    Ok(dest)
}

/// Look up the symbol-server key (`{timestamp:08X}{virtual_size:X}`) for
/// `basename` at `build` via winbindex. The per-file index is cached under
/// `maps/winver/.index/<basename>.json` so repeated lookups don't re-download.
fn winbindex_key(basename: &str, build: &str) -> Result<String, Box<dyn Error>> {
    let index = load_winbindex_index(basename)?;
    // Top-level object: { "<sha256>": { "fileInfo": { "timestamp", "virtualSize",
    // "version" }, ... }, ... }. Find the entry whose version contains the build.
    let obj = index
        .as_object()
        .ok_or("winbindex: unexpected JSON (not an object)")?;
    // winbindex indexes every architecture of a given filename; we only want
    // the x64 image (IMAGE_FILE_MACHINE_AMD64 = 0x8664 = 34404), otherwise the
    // substring match can pick the 32-bit ntdll of the same build.
    const MACHINE_AMD64: u64 = 0x8664;
    // Apisets (api-ms-win-*) and a few forwarders carry a much lower UBR than the
    // main system DLLs (e.g. 26100.1 vs 26100.7920) because they almost never
    // change. So match the exact build first, then fall back to the same major
    // build (".26100.") picking the highest UBR available.
    let major = build.split('.').next().unwrap_or(build);
    let major_needle = format!(".{}.", major);
    let mut fallback: Option<(u64, u64, u64)> = None; // (ubr, timestamp, virtualSize)

    for entry in obj.values() {
        let fi = match entry.get("fileInfo") {
            Some(v) => v,
            None => continue,
        };
        if fi.get("machineType").and_then(|v| v.as_u64()) != Some(MACHINE_AMD64) {
            continue;
        }
        let version = fi.get("version").and_then(|v| v.as_str()).unwrap_or("");
        let ts = match fi.get("timestamp").and_then(|v| v.as_u64()) {
            Some(v) => v,
            None => continue,
        };
        let vsize = match fi.get("virtualSize").and_then(|v| v.as_u64()) {
            Some(v) => v,
            None => continue,
        };
        if version.contains(build) {
            return Ok(format!("{:08X}{:X}", ts as u32, vsize as u32)); // exact
        }
        if version.contains(&major_needle) {
            let ubr = version
                .split_whitespace()
                .next()
                .and_then(|v| v.rsplit('.').next())
                .and_then(|s| s.parse::<u64>().ok())
                .unwrap_or(0);
            if fallback.map_or(true, |(u, _, _)| ubr > u) {
                fallback = Some((ubr, ts, vsize));
            }
        }
    }
    if let Some((_, ts, vsize)) = fallback {
        return Ok(format!("{:08X}{:X}", ts as u32, vsize as u32));
    }
    Err(format!(
        "no {} variant for build {} (nor major {}) in winbindex",
        basename, build, major
    )
    .into())
}

/// Fetch + decompress the winbindex per-filename index, caching the decoded
/// JSON on disk.
fn load_winbindex_index(basename: &str) -> Result<serde_json::Value, Box<dyn Error>> {
    let idx_dir = PathBuf::from("maps/winver/.index");
    let idx_path = idx_dir.join(format!("{}.json", basename));
    if let Ok(text) = fs::read(&idx_path) {
        if let Ok(v) = serde_json::from_slice::<serde_json::Value>(&text) {
            return Ok(v);
        }
    }
    let url = format!(
        "https://winbindex.m417z.com/data/by_filename_compressed/{}.json.gz",
        basename
    );
    let gz = http_get(&url)?;
    let mut decoder = flate2::read::GzDecoder::new(&gz[..]);
    let mut json = Vec::new();
    decoder.read_to_end(&mut json)?;
    fs::create_dir_all(&idx_dir)?;
    let _ = fs::write(&idx_path, &json); // best-effort cache
    Ok(serde_json::from_slice(&json)?)
}

/// Non-PE data files the loader needs (NLS code-page tables) that are *not* on
/// the symbol server. We seed them from an existing `--iso` cache if the user
/// has one; otherwise the loader runs with zeroed NLS (same as before any iso).
const SEED_DATA_FILES: &[&str] = &["C_1252.NLS", "C_437.NLS", "locale.nls"];

/// DLLs to fetch eagerly so the maps folder passes the loader's validity check
/// and the first steps don't stall on a network round-trip. Everything else is
/// pulled lazily on first `NtOpenFile`.
const SEED_DLLS: &[&str] = &["ntdll.dll", "kernel32.dll", "kernelbase.dll"];

impl crate::emu::Emu {
    /// Ensure a Windows system `dll` is present in the current maps folder,
    /// fetching it from the symbol server when missing. This is the library-wide
    /// safety net: any consumer (CLI, pymwemu, MCP, a test, a third party) that
    /// loads a Windows image/shellcode gets the DLLs auto-provisioned instead of
    /// hitting "required DLL not found". Best-effort — on failure it logs and
    /// returns so the caller's own missing-DLL handling still runs.
    ///
    /// x64 only for now (winver is AMD64-only); 32-bit no-ops until symbol-server
    /// support for `machineType 332` lands.
    pub(crate) fn ensure_maps_dll(&self, dll: &str) {
        if !self.cfg.arch.is_x64() {
            return;
        }
        let filepath = self.cfg.get_maps_folder(dll);
        if std::path::Path::new(&filepath).exists() {
            return;
        }
        // Use the configured build if the consumer picked one (--winver / API),
        // else default to win11.
        let build = self
            .cfg
            .winver
            .clone()
            .unwrap_or_else(|| resolve_build("win11"));
        let folder = self.cfg.maps_folder.clone();
        if let Err(e) = ensure_dll(std::path::Path::new(&folder), &build, &dll.to_lowercase()) {
            log::warn!("winver: could not fetch {} (build {}): {}", dll, build, e);
        }
    }

    /// Point the maps folder at a build's symbol-server cache, fetching the seed
    /// DLLs up front and enabling lazy fetch for the rest. `name` is a friendly
    /// alias (`win11`) or an exact build number (`26100.7920`).
    pub fn set_maps_from_winver(&mut self, name: &str) -> Result<(), Box<dyn Error>> {
        let build = resolve_build(name);
        let cache = cache_folder(&build);
        fs::create_dir_all(&cache)?;
        eprintln!(
            "[mwemu] --winver {}: build {} → fetching system DLLs from the symbol server",
            name, build
        );

        // Seed the non-PE NLS data files from an existing iso cache if present.
        seed_data_files(&cache);

        for dll in SEED_DLLS {
            match ensure_dll(&cache, &build, dll) {
                Ok(_) => {}
                Err(e) => {
                    return Err(format!("--winver: failed to fetch {}: {}", dll, e).into());
                }
            }
        }

        self.cfg.winver = Some(build);
        self.set_maps_folder(cache.to_str().unwrap());
        Ok(())
    }
}

/// Copy the NLS data files into `cache` from any already-extracted iso cache,
/// best-effort. These aren't on the symbol server but barely change between
/// builds, so reusing them is fine.
fn seed_data_files(cache: &Path) {
    let iso_caches = ["maps/iso/x86_64", "../../maps/iso/x86_64"];
    for f in SEED_DATA_FILES {
        let dest = cache.join(f);
        if dest.is_file() {
            continue;
        }
        for src_dir in &iso_caches {
            let src = Path::new(src_dir).join(f);
            if src.is_file() {
                let _ = fs::copy(&src, &dest);
                break;
            }
        }
    }
}

/// Minimal blocking HTTP GET returning the body bytes. Follows redirects (msdl
/// 302s to its blob store).
fn http_get(url: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let resp = ureq::get(url)
        .timeout(std::time::Duration::from_secs(60))
        .call()?;
    if resp.status() != 200 {
        return Err(format!("HTTP {} for {}", resp.status(), url).into());
    }
    let mut bytes = Vec::new();
    resp.into_reader().read_to_end(&mut bytes)?;
    Ok(bytes)
}
