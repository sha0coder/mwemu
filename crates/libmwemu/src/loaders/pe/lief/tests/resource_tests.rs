use crate::loaders::pe::lief::LiefPe;
use crate::loaders::pe::lief::traits::LiefPeReader;
use crate::tests::helpers;

/// Load the 32-bit Windows PE32 fixture that is known to contain
/// resources (it ships with 12 RT_ICON entries — type id 3).
///
/// This fixture is required for the regression tests in this file,
/// so a missing/unreadable file is a hard failure rather than a
/// silent skip — otherwise the resource-lookup-after-release
/// regression could pass trivially by not running.
fn load_minecraft_fixture() -> LiefPe {
    let path = helpers::test_data_path("exe32win_minecraft.bin");
    LiefPe::load(&path)
        .unwrap_or_else(|e| panic!("failed to load required fixture {}: {}", path, e))
}

#[test]
fn resource_entries_not_empty() {
    let pe = load_minecraft_fixture();
    let entries = pe.get_resource_entries();
    assert!(
        !entries.is_empty(),
        "minecraft fixture must report at least one resource entry"
    );
}

#[test]
fn resource_entries_have_valid_data_rvas() {
    let pe = load_minecraft_fixture();
    let mut checked = 0;
    for entry in pe.get_resource_entries() {
        if entry.size > 0 {
            assert!(
                entry.data_rva > 0,
                "resource entry {:?} has size {} but data_rva is 0",
                entry.name,
                entry.size
            );
            checked += 1;
        }
    }
    assert!(
        checked > 0,
        "fixture should have at least one nonzero-size resource entry"
    );
}

#[test]
fn resource_lookup_by_icon_type() {
    // RT_ICON is resource type id 3. The minecraft fixture has 12
    // RT_ICON entries.
    let pe = load_minecraft_fixture();
    let result = pe.get_resource(Some(3), Some(1), None, None);
    let (data_rva, size) = result.expect("RT_ICON/1 must be locatable in minecraft fixture");
    assert!(size > 0, "icon resource must have nonzero size");
    assert!(data_rva > 0, "icon resource data_rva must be nonzero");
}

/// Regression test: PE resource lookup must keep working after the
/// loader has called `release_mmap()`. The previous implementation
/// pulled `.rsrc` bytes from `LiefSectionManager`, which clears its
/// mmap, section cache, and LIEF binary on release. The fix routes
/// resource lookup through the retained `persistent_raw` bytes and a
/// small cached `.rsrc` layout, so lookups no longer depend on the
/// section manager's released state.
#[test]
fn resource_lookup_works_after_release_mmap() {
    let mut pe = load_minecraft_fixture();

    // Sanity check: the fixture must report the icon resource before
    // release.
    let pre_release = pe.get_resource(Some(3), Some(1), None, None);
    let (pre_rva, pre_size) = pre_release
        .expect("RT_ICON/1 must be locatable before release_mmap() on the minecraft fixture");

    // Drop the section manager's mmap, section cache, and LIEF binary —
    // this is the state the normal loader reaches right before
    // `Emu.pe64 = Some(pe64)`.
    pe.release_mmap();

    // Resource lookup must still succeed and report the same RVA/size.
    let post_release = pe.get_resource(Some(3), Some(1), None, None);
    let (post_rva, post_size) = post_release
        .expect("RT_ICON/1 must remain locatable after release_mmap() on the minecraft fixture");
    assert_eq!(
        post_rva, pre_rva,
        "icon RVA must match before and after release_mmap()"
    );
    assert_eq!(
        post_size, pre_size,
        "icon size must match before and after release_mmap()"
    );

    // Spot-check a second icon entry (id 5) to make sure the fix isn't
    // accidentally hard-coded to a single entry.
    let other = pe
        .get_resource(Some(3), Some(5), None, None)
        .expect("RT_ICON/5 must remain locatable after release_mmap()");
    assert!(other.0 > 0, "icon data_rva must be nonzero");
    assert!(other.1 > 0, "icon size must be nonzero");
}
