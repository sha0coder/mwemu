use crate::loaders::pe::lief::DelayLoadDescriptor;

#[test]
fn delay_load_descriptor_from_raw_parses_valid() {
    let mut data = [0u8; 64];

    data[0..4].copy_from_slice(&0x01u32.to_le_bytes());
    data[4..8].copy_from_slice(&0x1000u32.to_le_bytes());
    data[8..12].copy_from_slice(&0u32.to_le_bytes());
    data[12..16].copy_from_slice(&0x2000u32.to_le_bytes());
    data[16..20].copy_from_slice(&0x3000u32.to_le_bytes());
    data[20..24].copy_from_slice(&0x4000u32.to_le_bytes());
    data[24..28].copy_from_slice(&0x5000u32.to_le_bytes());
    data[28..32].copy_from_slice(&0x06000000u32.to_le_bytes());

    let d = DelayLoadDescriptor::from_raw(&data, 0, String::from("user32.dll"));
    assert!(d.is_some());
    let d = d.unwrap();
    assert_eq!(d.attributes, 1);
    assert_eq!(d.dll_name_rva, 0x1000);
    assert_eq!(d.module_handle, 0);
    assert_eq!(d.delay_iat, 0x2000);
    assert_eq!(d.delay_int, 0x3000);
    assert_eq!(d.bound_iat, 0x4000);
    assert_eq!(d.unload_table, 0x5000);
    assert_eq!(d.timestamp, 0x06000000);
    assert_eq!(d.dll_name, "user32.dll");
}

#[test]
fn delay_load_descriptor_empty_when_all_zeros() {
    let data = [0u8; 64];
    let d = DelayLoadDescriptor::from_raw(&data, 0, String::new());
    assert!(d.is_some());
    let d = d.unwrap();
    assert!(d.is_empty());
}

#[test]
fn delay_load_descriptor_not_empty_query() {
    let mut data = [0u8; 32];

    data[0..4].copy_from_slice(&0x03u32.to_le_bytes());
    data[4..8].copy_from_slice(&0x5000u32.to_le_bytes());

    let d = DelayLoadDescriptor::from_raw(&data, 0, String::from("comctl32.dll"));
    assert!(d.is_some());
    let d = d.unwrap();
    assert!(!d.is_empty());
    assert_eq!(d.dll_name_rva, 0x5000);
    assert_eq!(d.attributes, 3);
}
