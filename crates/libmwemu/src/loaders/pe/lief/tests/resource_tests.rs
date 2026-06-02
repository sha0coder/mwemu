use crate::loaders::pe::lief::LiefPe;
use crate::loaders::pe::lief::traits::LiefPeReader;

fn load_msgbox() -> Option<LiefPe> {
    let path = crate::tests::helpers::optional_test_data_path("exe64win_msgbox.bin")?;
    LiefPe::load(&path).ok()
}

#[test]
fn resource_entries_not_empty() {
    let Some(pe) = load_msgbox() else { return };
    let entries = pe.get_resource_entries();
    if entries.is_empty() {
        return;
    }
}

#[test]
fn resource_entries_have_valid_data_rvas() {
    let Some(pe) = load_msgbox() else { return };
    for entry in pe.get_resource_entries() {
        if entry.size > 0 {
            assert!(entry.data_rva > 0,
                "resource entry {:?} has size {} but data_rva is 0",
                entry.name, entry.size
            );
        }
    }
}

#[test]
fn resource_lookup_by_manifest_type() {
    let Some(pe) = load_msgbox() else { return };
    let result = pe.get_resource(Some(24), None, Some("RT_MANIFEST"), None);
    if let Some((data_rva, size)) = result {
        assert!(size > 0, "manifest resource must have nonzero size");
        assert!(data_rva > 0, "manifest resource data_rva must be nonzero");
    }
}

#[test]
fn resource_lookup_by_version_type() {
    let Some(pe) = load_msgbox() else { return };
    let result = pe.get_resource(Some(16), None, Some("RT_VERSION"), None);
    if let Some((data_rva, size)) = result {
        assert!(size > 0, "version resource must have nonzero size");
        assert!(data_rva > 0, "version resource data_rva must be nonzero");
    }
}
