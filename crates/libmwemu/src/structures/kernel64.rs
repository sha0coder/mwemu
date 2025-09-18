use crate::maps::Maps;
use std::convert::TryInto;
use crate::structures::ListEntry64;


#[derive(Debug)]
pub struct KSystemTime {
    pub low_part: u32,
    pub high1_time: u32,
    pub high2_time: u32,
}

impl Default for KSystemTime {
    fn default() -> Self {
        Self::new()
    }
}

impl KSystemTime {
    pub fn size() -> u32 {
        12
    }

    pub fn new() -> KSystemTime {
        KSystemTime {
            low_part: 0,
            high1_time: 0,
            high2_time: 0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> KSystemTime {
        KSystemTime {
            low_part: maps.read_dword(addr).unwrap(),
            high1_time: maps.read_dword(addr + 4).unwrap(),
            high2_time: maps.read_dword(addr + 8).unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_dword(addr, self.low_part);
        maps.write_dword(addr + 4, self.high1_time);
        maps.write_dword(addr + 8, self.high2_time);
    }
}

#[derive(Debug)]
pub struct SSDT {
    pub p_service_table: u64,
    pub p_counter_table: u64,
    pub number_of_services: u32,
    pub p_argument_table: u64,
}

impl Default for SSDT {
    fn default() -> Self {
        Self::new()
    }
}

impl SSDT {
    pub fn size() -> u32 {
        28
    }

    pub fn new() -> SSDT {
        SSDT {
            p_service_table: 0,
            p_counter_table: 0,
            number_of_services: 0,
            p_argument_table: 0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> SSDT {
        SSDT {
            p_service_table: maps.read_qword(addr).unwrap(),
            p_counter_table: maps.read_qword(addr + 8).unwrap(),
            number_of_services: maps.read_dword(addr + 16).unwrap(),
            p_argument_table: maps.read_qword(addr + 20).unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_qword(addr, self.p_service_table);
        maps.write_qword(addr + 8, self.p_counter_table);
        maps.write_dword(addr + 16, self.number_of_services);
        maps.write_qword(addr + 20, self.p_argument_table);
    }
}

#[derive(Debug)]
pub struct UnicodeString {
    pub length: u16,
    pub maximum_length: u16,
    pub buffer: u64,
}

impl Default for UnicodeString {
    fn default() -> Self {
        Self::new()
    }
}

impl UnicodeString {
    pub fn size() -> u32 {
        12
    }

    pub fn new() -> UnicodeString {
        UnicodeString {
            length: 0,
            maximum_length: 0,
            buffer: 0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> UnicodeString {
        UnicodeString {
            length: maps.read_word(addr).unwrap(),
            maximum_length: maps.read_word(addr + 2).unwrap(),
            buffer: maps.read_qword(addr + 4).unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_word(addr, self.length);
        maps.write_word(addr + 2, self.maximum_length);
        maps.write_qword(addr + 4, self.buffer);
    }
}

#[derive(Debug, Clone)]
pub struct DeviceIoControl {
    pub output_buffer_length: u32,
    pub input_buffer_length: u32,
    pub io_control_code: u32,
    pub type3_input_buffer: u64,
}

impl Default for DeviceIoControl {
    fn default() -> Self {
        Self::new()
    }
}

impl DeviceIoControl {
    pub fn size() -> u32 {
        20
    }

    pub fn new() -> DeviceIoControl {
        DeviceIoControl {
            output_buffer_length: 0,
            input_buffer_length: 0,
            io_control_code: 0,
            type3_input_buffer: 0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> DeviceIoControl {
        DeviceIoControl {
            output_buffer_length: maps.read_dword(addr).unwrap(),
            input_buffer_length: maps.read_dword(addr + 4).unwrap(),
            io_control_code: maps.read_dword(addr + 8).unwrap(),
            type3_input_buffer: maps.read_qword(addr + 12).unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_dword(addr, self.output_buffer_length);
        maps.write_dword(addr + 4, self.input_buffer_length);
        maps.write_dword(addr + 8, self.io_control_code);
        maps.write_qword(addr + 12, self.type3_input_buffer);
    }
}

#[derive(Debug)]
pub struct StringStruct {
    pub length: u16,
    pub maximum_length: u16,
    pub buffer: u64,
}

impl Default for StringStruct {
    fn default() -> Self {
        Self::new()
    }
}

impl StringStruct {
    pub fn size() -> u32 {
        12
    }

    pub fn new() -> StringStruct {
        StringStruct {
            length: 0,
            maximum_length: 0,
            buffer: 0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> StringStruct {
        StringStruct {
            length: maps.read_word(addr).unwrap(),
            maximum_length: maps.read_word(addr + 2).unwrap(),
            buffer: maps.read_qword(addr + 4).unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_word(addr, self.length);
        maps.write_word(addr + 2, self.maximum_length);
        maps.write_qword(addr + 4, self.buffer);
    }
}


#[derive(Debug)]
pub struct SystemModule {
    pub reserved: [u64; 2],
    pub base: u64,
    pub size: u32,
    pub flags: u32,
    pub index: u16,
    pub unknown: u16,
    pub load_count: u16,
    pub module_name_offset: u16,
    pub image_name: [u8; 256],
}

impl Default for SystemModule {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemModule {
    pub fn size() -> u32 {
        288
    }

    pub fn new() -> SystemModule {
        SystemModule {
            reserved: [0; 2],
            base: 0,
            size: 0,
            flags: 0,
            index: 0,
            unknown: 0,
            load_count: 0,
            module_name_offset: 0,
            image_name: [0; 256],
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> SystemModule {
        let mut reserved = [0u64; 2];
        reserved[0] = maps.read_qword(addr).unwrap();
        reserved[1] = maps.read_qword(addr + 8).unwrap();
        SystemModule {
            reserved,
            base: maps.read_qword(addr + 16).unwrap(),
            size: maps.read_dword(addr + 24).unwrap(),
            flags: maps.read_dword(addr + 28).unwrap(),
            index: maps.read_word(addr + 32).unwrap(),
            unknown: maps.read_word(addr + 34).unwrap(),
            load_count: maps.read_word(addr + 36).unwrap(),
            module_name_offset: maps.read_word(addr + 38).unwrap(),
            image_name: maps.read_bytes(addr + 40, 256).try_into().unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_qword(addr, self.reserved[0]);
        maps.write_qword(addr + 8, self.reserved[1]);
        maps.write_qword(addr + 16, self.base);
        maps.write_dword(addr + 24, self.size);
        maps.write_dword(addr + 28, self.flags);
        maps.write_word(addr + 32, self.index);
        maps.write_word(addr + 34, self.unknown);
        maps.write_word(addr + 36, self.load_count);
        maps.write_word(addr + 38, self.module_name_offset);
        maps.write_bytes(addr + 40, self.image_name.to_vec());
    }
}

#[derive(Debug)]
pub struct SystemTimeOfDayInformation {
    pub boot_time: u64,
    pub current_time: u64,
    pub time_zone_bias: u64,
    pub time_zone_id: u32,
    pub reserved: u32,
    pub boot_time_bias: u64,
    pub sleep_time_bias: u64,
}

impl Default for SystemTimeOfDayInformation {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemTimeOfDayInformation {
    pub fn size() -> u32 {
        56
    }

    pub fn new() -> SystemTimeOfDayInformation {
        SystemTimeOfDayInformation {
            boot_time: 0,
            current_time: 0,
            time_zone_bias: 0,
            time_zone_id: 0,
            reserved: 0,
            boot_time_bias: 0,
            sleep_time_bias: 0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> SystemTimeOfDayInformation {
        SystemTimeOfDayInformation {
            boot_time: maps.read_qword(addr).unwrap(),
            current_time: maps.read_qword(addr + 8).unwrap(),
            time_zone_bias: maps.read_qword(addr + 16).unwrap(),
            time_zone_id: maps.read_dword(addr + 24).unwrap(),
            reserved: maps.read_dword(addr + 28).unwrap(),
            boot_time_bias: maps.read_qword(addr + 32).unwrap(),
            sleep_time_bias: maps.read_qword(addr + 40).unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_qword(addr, self.boot_time);
        maps.write_qword(addr + 8, self.current_time);
        maps.write_qword(addr + 16, self.time_zone_bias);
        maps.write_dword(addr + 24, self.time_zone_id);
        maps.write_dword(addr + 28, self.reserved);
        maps.write_qword(addr + 32, self.boot_time_bias);
        maps.write_qword(addr + 40, self.sleep_time_bias);
    }
}

#[derive(Debug)]
pub struct SystemProcessInformation {
    pub next_entry_offset: u32,
    pub number_of_threads: u32,
    pub reserved1: [u8; 48],
    pub image_name: UnicodeString,
    pub base_priority: u32,
    pub unique_process_id: u64,
    pub inherited_from_unique_process_id: u64,
    pub handle_count: u32,
    pub session_id: u32,
    pub unique_process_key: u64,
    pub peak_virtual_size: u64,
    pub virtual_size: u64,
    pub page_fault_count: u32,
    pub peak_working_set_size: u64,
    pub working_set_size: u64,
    pub quota_peak_paged_pool_usage: u64,
    pub quota_paged_pool_usage: u64,
    pub quota_peak_non_paged_pool_usage: u64,
    pub quota_non_paged_pool_usage: u64,
    pub pagefile_usage: u64,
    pub peak_pagefile_usage: u64,
    pub private_page_count: u64,
    pub reserved7: [u64; 6], //  LARGE_INTEGER = u64?
}

impl Default for SystemProcessInformation {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemProcessInformation {
    pub fn size() -> u32 {
        224 // depends on UnicodeString.size()
    }

    pub fn new() -> SystemProcessInformation {
        SystemProcessInformation {
            next_entry_offset: 0,
            number_of_threads: 0,
            reserved1: [0; 48],
            image_name: UnicodeString::new(),
            base_priority: 0,
            unique_process_id: 0,
            inherited_from_unique_process_id: 0,
            handle_count: 0,
            session_id: 0,
            unique_process_key: 0,
            peak_virtual_size: 0,
            virtual_size: 0,
            page_fault_count: 0,
            peak_working_set_size: 0,
            working_set_size: 0,
            quota_peak_paged_pool_usage: 0,
            quota_paged_pool_usage: 0,
            quota_peak_non_paged_pool_usage: 0,
            quota_non_paged_pool_usage: 0,
            pagefile_usage: 0,
            peak_pagefile_usage: 0,
            private_page_count: 0,
            reserved7: [0; 6],
        }
    }
}

#[derive(Debug)]
pub struct ClientId {
    pub unique_process: u32,
    pub unique_thread: u32,
}

impl Default for ClientId {
    fn default() -> Self {
        Self::new()
    }
}

impl ClientId {
    pub fn size() -> u32 {
        8
    }

    pub fn new() -> ClientId {
        ClientId {
            unique_process: 0,
            unique_thread: 0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> ClientId {
        ClientId {
            unique_process: maps.read_dword(addr).unwrap(),
            unique_thread: maps.read_dword(addr + 4).unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_dword(addr, self.unique_process);
        maps.write_dword(addr + 4, self.unique_thread);
    }
}

#[derive(Debug)]
pub struct SystemThreadInformation {
    pub reserved1: [u64; 3], // LARGE_INTEGER = u64
    pub reserved2: u32,
    pub start_address: u64,
    pub client_id: ClientId,
    pub priority: u32,
    pub base_priority: u32,
    pub context_switches: u32,
    pub thread_state: u32,
    pub wait_reason: u32,
}

impl Default for SystemThreadInformation {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemThreadInformation {
    pub fn size() -> u32 {
        60 // 3*8 + 4 + 8 + 8 + 5*4 = 60 
    }

    pub fn new() -> SystemThreadInformation {
        SystemThreadInformation {
            reserved1: [0; 3],
            reserved2: 0,
            start_address: 0,
            client_id: ClientId::new(),
            priority: 0,
            base_priority: 0,
            context_switches: 0,
            thread_state: 0,
            wait_reason: 0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> SystemThreadInformation {
        SystemThreadInformation {
            reserved1: [
                maps.read_qword(addr).unwrap(),
                maps.read_qword(addr + 8).unwrap(),
                maps.read_qword(addr + 16).unwrap(),
            ],
            reserved2: maps.read_dword(addr + 24).unwrap(),
            start_address: maps.read_qword(addr + 32).unwrap(),
            client_id: ClientId::load(addr + 40, maps),
            priority: maps.read_dword(addr + 48).unwrap(),
            base_priority: maps.read_dword(addr + 52).unwrap(),
            context_switches: maps.read_dword(addr + 56).unwrap(),
            thread_state: maps.read_dword(addr + 60).unwrap(),
            wait_reason: maps.read_dword(addr + 64).unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_qword(addr, self.reserved1[0]);
        maps.write_qword(addr + 8, self.reserved1[1]);
        maps.write_qword(addr + 16, self.reserved1[2]);
        maps.write_dword(addr + 24, self.reserved2);
        maps.write_qword(addr + 32, self.start_address);
        self.client_id.save(addr + 40, maps);
        maps.write_dword(addr + 48, self.priority);
        maps.write_dword(addr + 52, self.base_priority);
        maps.write_dword(addr + 56, self.context_switches);
        maps.write_dword(addr + 60, self.thread_state);
        maps.write_dword(addr + 64, self.wait_reason);
    }
}

#[derive(Debug)]
pub struct MDL {
    pub next: u64,
    pub size: u16,
    pub mdl_flags: u16,
    pub process: u64,
    pub mapped_system_va: u64,
    pub start_va: u64,
    pub byte_count: u32,
    pub byte_offset: u32,
}

impl Default for MDL {
    fn default() -> Self {
        Self::new()
    }
}

impl MDL {
    pub fn size() -> u32 {
        40 // 8+2+2+8+8+8+4+4
    }

    pub fn new() -> MDL {
        MDL {
            next: 0,
            size: 0,
            mdl_flags: 0,
            process: 0,
            mapped_system_va: 0,
            start_va: 0,
            byte_count: 0,
            byte_offset: 0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> MDL {
        MDL {
            next: maps.read_qword(addr).unwrap(),
            size: maps.read_word(addr + 8).unwrap(),
            mdl_flags: maps.read_word(addr + 10).unwrap(),
            process: maps.read_qword(addr + 16).unwrap(),
            mapped_system_va: maps.read_qword(addr + 24).unwrap(),
            start_va: maps.read_qword(addr + 32).unwrap(),
            byte_count: maps.read_dword(addr + 40).unwrap(),
            byte_offset: maps.read_dword(addr + 44).unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_qword(addr, self.next);
        maps.write_word(addr + 8, self.size);
        maps.write_word(addr + 10, self.mdl_flags);
        maps.write_qword(addr + 16, self.process);
        maps.write_qword(addr + 24, self.mapped_system_va);
        maps.write_qword(addr + 32, self.start_va);
        maps.write_dword(addr + 40, self.byte_count);
        maps.write_dword(addr + 44, self.byte_offset);
    }
}

#[derive(Debug, Copy, Clone)]
pub struct KIDTEntry {
    pub offset_low: u16,
    pub selector: u16,
    pub base: u32,
}

impl Default for KIDTEntry {
    fn default() -> Self {
        Self::new()
    }
}

impl KIDTEntry {
    pub fn size() -> u32 {
        8
    }

    pub fn new() -> KIDTEntry {
        KIDTEntry {
            offset_low: 0,
            selector: 0,
            base: 0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> KIDTEntry {
        KIDTEntry {
            offset_low: maps.read_word(addr).unwrap(),
            selector: maps.read_word(addr + 2).unwrap(),
            base: maps.read_dword(addr + 4).unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_word(addr, self.offset_low);
        maps.write_word(addr + 2, self.selector);
        maps.write_dword(addr + 4, self.base);
    }
}

#[derive(Debug, Copy, Clone)]
pub struct KIDTEntry64 {
    pub offset_low: u16,
    pub selector: u16,
    pub reserved0: u16,
    pub offset_middle: u16,
    pub offset_high: u32,
    pub reserved1: u32,
}

impl Default for KIDTEntry64 {
    fn default() -> Self {
        Self::new()
    }
}

impl KIDTEntry64 {
    pub fn size() -> u32 {
        16
    }

    pub fn new() -> KIDTEntry64 {
        KIDTEntry64 {
            offset_low: 0,
            selector: 0,
            reserved0: 0,
            offset_middle: 0,
            offset_high: 0,
            reserved1: 0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> KIDTEntry64 {
        KIDTEntry64 {
            offset_low: maps.read_word(addr).unwrap(),
            selector: maps.read_word(addr + 2).unwrap(),
            reserved0: maps.read_word(addr + 4).unwrap(),
            offset_middle: maps.read_word(addr + 6).unwrap(),
            offset_high: maps.read_dword(addr + 8).unwrap(),
            reserved1: maps.read_dword(addr + 12).unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_word(addr, self.offset_low);
        maps.write_word(addr + 2, self.selector);
        maps.write_word(addr + 4, self.reserved0);
        maps.write_word(addr + 6, self.offset_middle);
        maps.write_dword(addr + 8, self.offset_high);
        maps.write_dword(addr + 12, self.reserved1);
    }
}

#[derive(Debug)]
pub struct EThread {
    pub data: [u8; 4096],
}

impl Default for EThread { fn default() -> Self { Self::new() } }

impl EThread {
    pub fn size() -> u32 { 4096 }
    pub fn new() -> EThread { EThread { data: [0; 4096] } }

    pub fn load(addr: u64, maps: &Maps) -> EThread {
        let buf = maps.read_bytes(addr, 4096).try_into().unwrap();
        EThread { data: buf }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_bytes(addr, self.data.to_vec());
    }
}

#[derive(Debug)]
pub struct EProcess {
    pub data: [u8; 4096],
}

impl Default for EProcess { fn default() -> Self { Self::new() } }

impl EProcess {
    pub fn size() -> u32 { 4096 }
    pub fn new() -> EProcess { EProcess { data: [0; 4096] } }

    pub fn load(addr: u64, maps: &Maps) -> EProcess {
        let buf = maps.read_bytes(addr, 4096).try_into().unwrap();
        EProcess { data: buf }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_bytes(addr, self.data.to_vec());
    }
}

#[derive(Debug)]
pub struct KEVent {
    pub data: [u8; 4096],
}

impl Default for KEVent { fn default() -> Self { Self::new() } }

impl KEVent {
    pub fn size() -> u32 { 4096 }
    pub fn new() -> KEVent { KEVent { data: [0; 4096] } }

    pub fn load(addr: u64, maps: &Maps) -> KEVent {
        let buf = maps.read_bytes(addr, 4096).try_into().unwrap();
        KEVent { data: buf }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_bytes(addr, self.data.to_vec());
    }
}

#[derive(Debug)]
pub struct Mutant {
    pub data: [u8; 4096],
}

impl Default for Mutant { fn default() -> Self { Self::new() } }

impl Mutant {
    pub fn size() -> u32 { 4096 }
    pub fn new() -> Mutant { Mutant { data: [0; 4096] } }

    pub fn load(addr: u64, maps: &Maps) -> Mutant {
        let buf = maps.read_bytes(addr, 4096).try_into().unwrap();
        Mutant { data: buf }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_bytes(addr, self.data.to_vec());
    }
}



#[derive(Debug)]
pub struct RtlOsVersionInfoW {
    pub dw_os_version_info_size: u32,
    pub dw_major_version: u32,
    pub dw_minor_version: u32,
    pub dw_build_number: u32,
    pub dw_platform_id: u32,
    pub sz_csd_version: [u8; 256],
}

impl Default for RtlOsVersionInfoW {
    fn default() -> Self { Self::new() }
}

impl RtlOsVersionInfoW {
    pub fn size() -> u32 { 276 } // 5*4 + 256
    pub fn new() -> RtlOsVersionInfoW {
        RtlOsVersionInfoW {
            dw_os_version_info_size: 0,
            dw_major_version: 0,
            dw_minor_version: 0,
            dw_build_number: 0,
            dw_platform_id: 0,
            sz_csd_version: [0; 256],
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> RtlOsVersionInfoW {
        let buf = maps.read_bytes(addr + 20, 256).try_into().unwrap();
        RtlOsVersionInfoW {
            dw_os_version_info_size: maps.read_dword(addr).unwrap(),
            dw_major_version: maps.read_dword(addr + 4).unwrap(),
            dw_minor_version: maps.read_dword(addr + 8).unwrap(),
            dw_build_number: maps.read_dword(addr + 12).unwrap(),
            dw_platform_id: maps.read_dword(addr + 16).unwrap(),
            sz_csd_version: buf,
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_dword(addr, self.dw_os_version_info_size);
        maps.write_dword(addr + 4, self.dw_major_version);
        maps.write_dword(addr + 8, self.dw_minor_version);
        maps.write_dword(addr + 12, self.dw_build_number);
        maps.write_dword(addr + 16, self.dw_platform_id);
        maps.write_bytes(addr + 20, self.sz_csd_version.to_vec());
    }
}

#[derive(Debug)]
pub struct RtlOsVersionInfoExW {
    pub dw_os_version_info_size: u32,
    pub dw_major_version: u32,
    pub dw_minor_version: u32,
    pub dw_build_number: u32,
    pub dw_platform_id: u32,
    pub sz_csd_version: [u8; 256],
    pub w_service_pack_major: u16,
    pub w_service_pack_minor: u16,
    pub w_suite_mask: u16,
    pub w_product_type: u8,
    pub w_reserved: u8,
}

impl Default for RtlOsVersionInfoExW {
    fn default() -> Self { Self::new() }
}

impl RtlOsVersionInfoExW {
    pub fn size() -> u32 { 284 } // 276 + 2 + 2 + 2 + 1 + 1
    pub fn new() -> RtlOsVersionInfoExW {
        RtlOsVersionInfoExW {
            dw_os_version_info_size: 0,
            dw_major_version: 0,
            dw_minor_version: 0,
            dw_build_number: 0,
            dw_platform_id: 0,
            sz_csd_version: [0; 256],
            w_service_pack_major: 0,
            w_service_pack_minor: 0,
            w_suite_mask: 0,
            w_product_type: 0,
            w_reserved: 0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> RtlOsVersionInfoExW {
        let buf = maps.read_bytes(addr + 20, 256).try_into().unwrap();
        RtlOsVersionInfoExW {
            dw_os_version_info_size: maps.read_dword(addr).unwrap(),
            dw_major_version: maps.read_dword(addr + 4).unwrap(),
            dw_minor_version: maps.read_dword(addr + 8).unwrap(),
            dw_build_number: maps.read_dword(addr + 12).unwrap(),
            dw_platform_id: maps.read_dword(addr + 16).unwrap(),
            sz_csd_version: buf,
            w_service_pack_major: maps.read_word(addr + 276).unwrap(),
            w_service_pack_minor: maps.read_word(addr + 278).unwrap(),
            w_suite_mask: maps.read_word(addr + 280).unwrap(),
            w_product_type: maps.read_byte(addr + 282).unwrap(),
            w_reserved: maps.read_byte(addr + 283).unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_dword(addr, self.dw_os_version_info_size);
        maps.write_dword(addr + 4, self.dw_major_version);
        maps.write_dword(addr + 8, self.dw_minor_version);
        maps.write_dword(addr + 12, self.dw_build_number);
        maps.write_dword(addr + 16, self.dw_platform_id);
        maps.write_bytes(addr + 20, self.sz_csd_version.to_vec());
        maps.write_word(addr + 276, self.w_service_pack_major);
        maps.write_word(addr + 278, self.w_service_pack_minor);
        maps.write_word(addr + 280, self.w_suite_mask);
        maps.write_byte(addr + 282, self.w_product_type);
        maps.write_byte(addr + 283, self.w_reserved);
    }
}

#[derive(Debug)]
pub struct IDT {
    pub limit: u16,
    pub descriptors: u64,
}

impl Default for IDT { fn default() -> Self { Self::new() } }

impl IDT {
    pub fn size() -> u32 { 10 } // 2 + 8
    pub fn new() -> IDT { IDT { limit: 0, descriptors: 0 } }

    pub fn load(addr: u64, maps: &Maps) -> IDT {
        IDT {
            limit: maps.read_word(addr).unwrap(),
            descriptors: maps.read_qword(addr + 2).unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_word(addr, self.limit);
        maps.write_qword(addr + 2, self.descriptors);
    }
}

#[derive(Debug)]
pub struct KAPC {
    pub type_: u8,
    pub spare_byte0: u8,
    pub size: u8,
    pub spare_byte1: u8,
    pub spare_long0: u32,
    pub thread: u64,
    pub apc_list_entry: ListEntry64,
    pub kernel_routine: u64,
    pub rundown_routine: u64,
    pub normal_routine: u64,
    pub normal_context: u64,
    pub system_argument1: u64,
    pub system_argument2: u64,
    pub apc_state_index: u8,
    pub apc_mode: u8,
    pub inserted: u8,
}

impl Default for KAPC { fn default() -> Self { Self::new() } }

impl KAPC {
    pub fn size() -> u32 { 72 } // aprox
    pub fn new() -> KAPC {
        KAPC {
            type_: 0,
            spare_byte0: 0,
            size: 0,
            spare_byte1: 0,
            spare_long0: 0,
            thread: 0,
            apc_list_entry: ListEntry64::new(),
            kernel_routine: 0,
            rundown_routine: 0,
            normal_routine: 0,
            normal_context: 0,
            system_argument1: 0,
            system_argument2: 0,
            apc_state_index: 0,
            apc_mode: 0,
            inserted: 0,
        }
    }
}

#[derive(Debug)]
pub struct ObjectAttributes {
    pub length: u32,
    pub root_directory: u64,
    pub object_name: u64,
    pub attributes: u32,
    pub security_descriptor: u64,
    pub security_quality_of_service: u64,
}

impl Default for ObjectAttributes { fn default() -> Self { Self::new() } }

impl ObjectAttributes {
    pub fn size() -> u32 { 40 } // 4 + 8 + 8 + 4 + 8 + 8
    pub fn new() -> ObjectAttributes {
        ObjectAttributes {
            length: 0,
            root_directory: 0,
            object_name: 0,
            attributes: 0,
            security_descriptor: 0,
            security_quality_of_service: 0,
        }
    }
}

#[derive(Debug)]
pub struct FileStandardInformation {
    pub allocation_size: u64, // LARGE_INTEGER
    pub end_of_file: u64,     // LARGE_INTEGER
    pub number_of_links: u32,
    pub delete_pending: u8,
    pub directory: u8,
}

impl Default for FileStandardInformation { fn default() -> Self { Self::new() } }

impl FileStandardInformation {
    pub fn size() -> u32 { 18 } // 8 + 8 + 4 + 1 + 1
    pub fn new() -> FileStandardInformation {
        FileStandardInformation {
            allocation_size: 0,
            end_of_file: 0,
            number_of_links: 0,
            delete_pending: 0,
            directory: 0,
        }
    }
}

#[derive(Debug)]
pub struct DescriptorTable {
    pub table: [KIDTEntry64; 256],
}

impl Default for DescriptorTable { fn default() -> Self { Self::new() } }

impl DescriptorTable {
    pub fn size() -> u32 { 16 * 256 } // each KIDTEntry64 = 16 bytes
    pub fn new() -> DescriptorTable {
        DescriptorTable { table: [KIDTEntry64::new(); 256] }
    }
}

#[derive(Debug)]
pub struct DriverObject {
    pub type_: u16,
    pub size: u16,
    pub device_object: u64,
    pub flags: u32,
    pub driver_start: u64,
    pub driver_size: u32,
    pub driver_section: u64,
    pub driver_extension: u64,
    pub driver_name: UnicodeString,
    pub hardware_database: u64,
    pub fast_io_dispatch: u64,
    pub driver_init: u64,
    pub driver_start_io: u64,
    pub driver_unload: u64,
    pub major_function: [u64; 28],
}

impl Default for DriverObject { fn default() -> Self { Self::new() } }

impl DriverObject {
    pub fn size() -> u32 { 0 } 
    pub fn new() -> DriverObject {
        DriverObject {
            type_: 0,
            size: 0,
            device_object: 0,
            flags: 0,
            driver_start: 0,
            driver_size: 0,
            driver_section: 0,
            driver_extension: 0,
            driver_name: UnicodeString::new(),
            hardware_database: 0,
            fast_io_dispatch: 0,
            driver_init: 0,
            driver_start_io: 0,
            driver_unload: 0,
            major_function: [0; 28],
        }
    }
}

#[derive(Debug)]
pub struct KDeviceQueue {
    pub type_: u16,
    pub size: u16,
    pub device_list_head: ListEntry64,
    pub lock: u64,
    pub busy: u8,
}

impl Default for KDeviceQueue { fn default() -> Self { Self::new() } }

impl KDeviceQueue {
    pub fn size() -> u32 { 
        2+2+8+1+0x10
    } 
    pub fn new() -> KDeviceQueue {
        KDeviceQueue {
            type_: 0,
            size: 0,
            device_list_head: ListEntry64::new(),
            lock: 0,
            busy: 0,
        }
    }
}

#[derive(Debug)]
pub struct KDPC {
    pub type_: u8,
    pub importance: u8,
    pub number: u16,
    pub dpc_list_entry: ListEntry64,
    pub deferred_routine: u64,
    pub deferred_context: u64,
    pub system_argument1: u64,
    pub system_argument2: u64,
    pub dpc_data: u64,
}

impl Default for KDPC { fn default() -> Self { Self::new() } }

impl KDPC {
    pub fn size() -> u32 { 1+1+2+0x10+8+8+8+8+8 } 
    pub fn new() -> KDPC {
        KDPC {
            type_: 0,
            importance: 0,
            number: 0,
            dpc_list_entry: ListEntry64::new(),
            deferred_routine: 0,
            deferred_context: 0,
            system_argument1: 0,
            system_argument2: 0,
            dpc_data: 0,
        }
    }
}

#[derive(Debug)]
pub struct DeviceObject {
    pub type_: u16,
    pub size: u16,
    pub reference_count: u32,
    pub driver_object: u64,
    pub next_device: u64,
    pub attached_device: u64,
    pub current_irp: u64,
    pub timer: u64,
    pub flags: u32,
    pub characteristics: u32,
    pub vpb: u64,
    pub device_extension: u64,
    pub device_type: u32,
    pub stack_size: u8,
    pub queue: ListEntry64,
    pub alignment_requirement: u32,
    pub device_queue: KDeviceQueue,
    pub dpc: KDPC,
    pub active_thread_count: u32,
    pub security_descriptor: u64,
    pub device_lock: KEVent,
    pub sector_size: u16,
    pub spare1: u16,
    pub device_object_extension: u64,
    pub reserved: u64,
}

impl Default for DeviceObject { fn default() -> Self { Self::new() } }

impl DeviceObject {
    pub fn new() -> DeviceObject {
        DeviceObject {
            type_: 0,
            size: 0,
            reference_count: 0,
            driver_object: 0,
            next_device: 0,
            attached_device: 0,
            current_irp: 0,
            timer: 0,
            flags: 0,
            characteristics: 0,
            vpb: 0,
            device_extension: 0,
            device_type: 0,
            stack_size: 0,
            queue: ListEntry64::new(),
            alignment_requirement: 0,
            device_queue: KDeviceQueue::new(),
            dpc: KDPC::new(),
            active_thread_count: 0,
            security_descriptor: 0,
            device_lock: KEVent::new(),
            sector_size: 0,
            spare1: 0,
            device_object_extension: 0,
            reserved: 0,
        }
    }
}

#[derive(Debug)]
pub struct FileObject {
    pub type_: u16,
    pub size: u16,
    pub device_object: u64,
    pub vpb: u64,
    pub fs_context: u64,
    pub fs_context2: u64,
    pub section_object_pointer: u64,
    pub private_cache_map: u64,
    pub final_status: u32,
    pub related_file_object: u64,
    pub lock_operation: u8,
    pub delete_pending: u8,
    pub read_access: u8,
    pub write_access: u8,
    pub delete_access: u8,
    pub shared_read: u8,
    pub shared_write: u8,
    pub shared_delete: u8,
    pub flags: u32,
    pub file_name: UnicodeString,
    pub current_byte_offset: u64,
    pub waiters: u32,
    pub busy: u32,
    pub last_lock: u64,
    pub lock: KEVent,
    pub event: KEVent,
    pub completion_context: u64,
    pub irp_list_lock: u32,
    pub irp_list: ListEntry64,
    pub file_object_extension: u64,
}

impl Default for FileObject { fn default() -> Self { Self::new() } }

impl FileObject {
    pub fn new() -> FileObject {
        FileObject {
            type_: 0,
            size: 0,
            device_object: 0,
            vpb: 0,
            fs_context: 0,
            fs_context2: 0,
            section_object_pointer: 0,
            private_cache_map: 0,
            final_status: 0,
            related_file_object: 0,
            lock_operation: 0,
            delete_pending: 0,
            read_access: 0,
            write_access: 0,
            delete_access: 0,
            shared_read: 0,
            shared_write: 0,
            shared_delete: 0,
            flags: 0,
            file_name: UnicodeString::new(),
            current_byte_offset: 0,
            waiters: 0,
            busy: 0,
            last_lock: 0,
            lock: KEVent::new(),
            event: KEVent::new(),
            completion_context: 0,
            irp_list_lock: 0,
            irp_list: ListEntry64::new(),
            file_object_extension: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IoParameters {
    pub device_io_control: DeviceIoControl,
}

impl IoParameters {
    pub fn new() -> IoParameters {
        IoParameters {
            device_io_control: DeviceIoControl::new(),
        }
    }
}

#[derive(Debug)]
pub struct IoStackLocation {
    pub major_function: u8,
    pub minor_function: u8,
    pub flags: u8,
    pub control: u8,
    pub _padding: [u8; 8],
    pub parameters: IoParameters,
    pub device_object: u64,
    pub file_object: u64,
    pub completion_routine: u64,
    pub context: u64,
}

impl Default for IoStackLocation { fn default() -> Self { Self::new() } }

impl IoStackLocation {
    pub fn new() -> IoStackLocation {
        IoStackLocation {
            major_function: 0,
            minor_function: 0,
            flags: 0,
            control: 0,
            _padding: [0; 8],
            parameters: IoParameters::new(),
            device_object: 0,
            file_object: 0,
            completion_routine: 0,
            context: 0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> IoStackLocation {
        IoStackLocation {
            major_function: maps.read_byte(addr).unwrap(),
            minor_function: maps.read_byte(addr + 1).unwrap(),
            flags: maps.read_byte(addr + 2).unwrap(),
            control: maps.read_byte(addr + 3).unwrap(),
            _padding: maps.read_bytes(addr + 4, 8).try_into().unwrap(),
            parameters: IoParameters {
                device_io_control: DeviceIoControl::load(addr + 12, maps),
            },
            device_object: maps.read_qword(addr + 12 + DeviceIoControl::size() as u64).unwrap(),
            file_object: maps.read_qword(addr + 12 + DeviceIoControl::size() as u64 + 8).unwrap(),
            completion_routine: maps.read_qword(addr + 12 + DeviceIoControl::size() as u64 + 16).unwrap(),
            context: maps.read_qword(addr + 12 + DeviceIoControl::size() as u64 + 24).unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_byte(addr, self.major_function);
        maps.write_byte(addr + 1, self.minor_function);
        maps.write_byte(addr + 2, self.flags);
        maps.write_byte(addr + 3, self.control);
        maps.write_bytes(addr + 4, self._padding.to_vec());
        self.parameters.device_io_control.save(addr + 12, maps);
        maps.write_qword(addr + 12 + DeviceIoControl::size() as u64, self.device_object);
        maps.write_qword(addr + 12 + DeviceIoControl::size() as u64 + 8, self.file_object);
        maps.write_qword(addr + 12 + DeviceIoControl::size() as u64 + 16, self.completion_routine);
        maps.write_qword(addr + 12 + DeviceIoControl::size() as u64 + 24, self.context);
    }
}

#[derive(Debug)]
pub struct IrpOverlay {
    pub user_apc_routine: u64,
    pub user_apc_context: u64,
}

impl Default for IrpOverlay { fn default() -> Self { Self::new() } }

impl IrpOverlay {
    pub fn new() -> IrpOverlay {
        IrpOverlay {
            user_apc_routine: 0,
            user_apc_context: 0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> IrpOverlay {
        IrpOverlay {
            user_apc_routine: maps.read_qword(addr).unwrap(),
            user_apc_context: maps.read_qword(addr + 8).unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_qword(addr, self.user_apc_routine);
        maps.write_qword(addr + 8, self.user_apc_context);
    }
}

#[derive(Debug)]
pub struct IoStatusBlock {
    pub status: u64,
    pub information: u64,
}

impl Default for IoStatusBlock { fn default() -> Self { Self::new() } }

impl IoStatusBlock {
    pub fn new() -> IoStatusBlock {
        IoStatusBlock { status: 0, information: 0 }
    }

    pub fn load(addr: u64, maps: &Maps) -> IoStatusBlock {
        IoStatusBlock {
            status: maps.read_qword(addr).unwrap(),
            information: maps.read_qword(addr + 8).unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_qword(addr, self.status);
        maps.write_qword(addr + 8, self.information);
    }
}

#[derive(Debug)]
pub struct KDeviceQueueEntry {
    pub device_list_entry: ListEntry64,
    pub sort_key: u32,
    pub inserted: u8,
    pub _padding: [u8; 3],
}

impl Default for KDeviceQueueEntry { fn default() -> Self { Self::new() } }

impl KDeviceQueueEntry {
    pub fn new() -> KDeviceQueueEntry {
        KDeviceQueueEntry {
            device_list_entry: ListEntry64::new(),
            sort_key: 0,
            inserted: 0,
            _padding: [0; 3],
        }
    }
}

#[derive(Debug)]
pub struct TailOverlay {
    pub device_queue_entry: KDeviceQueueEntry,
    pub padding: [u8; 8],
    pub reserved1: [u64; 2],
    pub list_entry: ListEntry64,
    pub current_stack_location: u64,
    pub reserved2: u64,
}

impl Default for TailOverlay { fn default() -> Self { Self::new() } }

impl TailOverlay {
    pub fn new() -> TailOverlay {
        TailOverlay {
            device_queue_entry: KDeviceQueueEntry::new(),
            padding: [0; 8],
            reserved1: [0; 2],
            list_entry: ListEntry64::new(),
            current_stack_location: 0,
            reserved2: 0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> TailOverlay {
        TailOverlay {
            device_queue_entry: KDeviceQueueEntry::new(),
            padding: maps.read_bytes(addr + 24, 8).try_into().unwrap(),
            reserved1: [
                maps.read_qword(addr + 32).unwrap(),
                maps.read_qword(addr + 40).unwrap(),
            ],
            list_entry: ListEntry64::new(), 
            current_stack_location: maps.read_qword(addr + 56).unwrap(),
            reserved2: maps.read_qword(addr + 64).unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_bytes(addr + 24, self.padding.to_vec());
        maps.write_qword(addr + 32, self.reserved1[0]);
        maps.write_qword(addr + 40, self.reserved1[1]);
        self.list_entry.save(addr + 48, maps);
        maps.write_qword(addr + 56, self.current_stack_location);
        maps.write_qword(addr + 64, self.reserved2);
    }
}

#[derive(Debug)]
pub struct IrpTail {
    pub overlay: TailOverlay,
}

impl Default for IrpTail { fn default() -> Self { Self::new() } }

impl IrpTail {
    pub fn new() -> IrpTail {
        IrpTail { overlay: TailOverlay::new() }
    }

    pub fn load(addr: u64, maps: &Maps) -> IrpTail {
        IrpTail { overlay: TailOverlay::load(addr, maps) }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        self.overlay.save(addr, maps)
    }
}

#[derive(Debug)]
pub struct Irp {
    pub type_: u16,
    pub size: u16,
    pub mdl_address: u64,
    pub flags: u32,
    pub associated_irp: u64,
    pub thread_list_entry: ListEntry64,
    pub io_status: IoStatusBlock,
    pub requestor_mode: u8,
    pub pending_returned: u8,
    pub stack_count: u8,
    pub current_location: u8,
    pub cancel: u8,
    pub cancel_irql: u8,
    pub apc_environment: u8,
    pub allocation_flags: u8,
    pub user_iosb: u64,
    pub user_event: u64,
    pub overlay: IrpOverlay,
    pub cancel_routine: u64,
    pub user_buffer: u64,
    pub tail: IrpTail,
}

impl Default for Irp { fn default() -> Self { Self::new() } }

impl Irp {
    pub fn new() -> Irp {
        Irp {
            type_: 0,
            size: 0,
            mdl_address: 0,
            flags: 0,
            associated_irp: 0,
            thread_list_entry: ListEntry64::new(),
            io_status: IoStatusBlock::new(),
            requestor_mode: 0,
            pending_returned: 0,
            stack_count: 0,
            current_location: 0,
            cancel: 0,
            cancel_irql: 0,
            apc_environment: 0,
            allocation_flags: 0,
            user_iosb: 0,
            user_event: 0,
            overlay: IrpOverlay::new(),
            cancel_routine: 0,
            user_buffer: 0,
            tail: IrpTail::new(),
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> Irp {
        Irp {
            type_: maps.read_word(addr).unwrap(),
            size: maps.read_word(addr + 2).unwrap(),
            mdl_address: maps.read_qword(addr + 4).unwrap(),
            flags: maps.read_dword(addr + 12).unwrap(),
            associated_irp: maps.read_qword(addr + 16).unwrap(),
            thread_list_entry: ListEntry64::new(),
            io_status: IoStatusBlock::load(addr + 32, maps),
            requestor_mode: maps.read_byte(addr + 48).unwrap(),
            pending_returned: maps.read_byte(addr + 49).unwrap(),
            stack_count: maps.read_byte(addr + 50).unwrap(),
            current_location: maps.read_byte(addr + 51).unwrap(),
            cancel: maps.read_byte(addr + 52).unwrap(),
            cancel_irql: maps.read_byte(addr + 53).unwrap(),
            apc_environment: maps.read_byte(addr + 54).unwrap(),
            allocation_flags: maps.read_byte(addr + 55).unwrap(),
            user_iosb: maps.read_qword(addr + 56).unwrap(),
            user_event: maps.read_qword(addr + 64).unwrap(),
            overlay: IrpOverlay::load(addr + 72, maps),
            cancel_routine: maps.read_qword(addr + 88).unwrap(),
            user_buffer: maps.read_qword(addr + 96).unwrap(),
            tail: IrpTail::load(addr + 104, maps),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_word(addr, self.type_);
        maps.write_word(addr + 2, self.size);
        maps.write_qword(addr + 4, self.mdl_address);
        maps.write_dword(addr + 12, self.flags);
        maps.write_qword(addr + 16, self.associated_irp);
        self.thread_list_entry.save(addr + 24, maps);
        self.io_status.save(addr + 32, maps);
        maps.write_byte(addr + 48, self.requestor_mode);
        maps.write_byte(addr + 49, self.pending_returned);
        maps.write_byte(addr + 50, self.stack_count);
        maps.write_byte(addr + 51, self.current_location);
        maps.write_byte(addr + 52, self.cancel);
        maps.write_byte(addr + 53, self.cancel_irql);
        maps.write_byte(addr + 54, self.apc_environment);
        maps.write_byte(addr + 55, self.allocation_flags);
        maps.write_qword(addr + 56, self.user_iosb);
        maps.write_qword(addr + 64, self.user_event);
        self.overlay.save(addr + 72, maps);
        maps.write_qword(addr + 88, self.cancel_routine);
        maps.write_qword(addr + 96, self.user_buffer);
        self.tail.save(addr + 104, maps);
    }
}

#[derive(Debug)]
pub struct NtTib {
    pub exception_list: u64,
    pub stack_base: u64,
    pub stack_limit: u64,
    pub reserved1: u64,
    pub reserved2: u64,
    pub reserved3: u64,
    pub self_ptr: u64,
}

impl Default for NtTib { fn default() -> Self { Self::new() } }

impl NtTib {
    pub fn new() -> NtTib {
        NtTib {
            exception_list: 0,
            stack_base: 0,
            stack_limit: 0,
            reserved1: 0,
            reserved2: 0,
            reserved3: 0,
            self_ptr: 0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> NtTib {
        NtTib {
            exception_list: maps.read_qword(addr).unwrap(),
            stack_base: maps.read_qword(addr + 8).unwrap(),
            stack_limit: maps.read_qword(addr + 16).unwrap(),
            reserved1: maps.read_qword(addr + 24).unwrap(),
            reserved2: maps.read_qword(addr + 32).unwrap(),
            reserved3: maps.read_qword(addr + 40).unwrap(),
            self_ptr: maps.read_qword(addr + 48).unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_qword(addr, self.exception_list);
        maps.write_qword(addr + 8, self.stack_base);
        maps.write_qword(addr + 16, self.stack_limit);
        maps.write_qword(addr + 24, self.reserved1);
        maps.write_qword(addr + 32, self.reserved2);
        maps.write_qword(addr + 40, self.reserved3);
        maps.write_qword(addr + 48, self.self_ptr);
    }
}

#[derive(Debug)]
pub struct Teb {
    pub nt_tib: NtTib,
    pub environment_pointer: u64,
    pub client_id: ClientId,
    pub pad0: u64,
    pub active_rpc_handle: u64,
    pub thread_local_storage_pointer: u64,
    pub process_environment_block: u64,
    pub last_error_value: u32,
    pub count_of_owned_critical_sections: u32,
    pub csr_client_thread: u64,
    pub win32_thread_info: u64,
    pub user32_reserved: [u32; 26],
    pub user_reserved: [u32; 5],
    pub wow32_reserved: u64,
    pub current_locale: u32,
}

impl Default for Teb { fn default() -> Self { Self::new() } }

impl Teb {
    pub fn new() -> Teb {
        Teb {
            nt_tib: NtTib::new(),
            environment_pointer: 0,
            client_id: ClientId::new(),
            pad0: 0,
            active_rpc_handle: 0,
            thread_local_storage_pointer: 0,
            process_environment_block: 0,
            last_error_value: 0,
            count_of_owned_critical_sections: 0,
            csr_client_thread: 0,
            win32_thread_info: 0,
            user32_reserved: [0; 26],
            user_reserved: [0; 5],
            wow32_reserved: 0,
            current_locale: 0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> Teb {
        Teb {
            nt_tib: NtTib::load(addr, maps),
            environment_pointer: maps.read_qword(addr + 56).unwrap(),
            client_id: ClientId::load(addr + 64, maps),
            pad0: maps.read_qword(addr + 80).unwrap(),
            active_rpc_handle: maps.read_qword(addr + 88).unwrap(),
            thread_local_storage_pointer: maps.read_qword(addr + 96).unwrap(),
            process_environment_block: maps.read_qword(addr + 104).unwrap(),
            last_error_value: maps.read_dword(addr + 112).unwrap(),
            count_of_owned_critical_sections: maps.read_dword(addr + 116).unwrap(),
            csr_client_thread: maps.read_qword(addr + 120).unwrap(),
            win32_thread_info: maps.read_qword(addr + 128).unwrap(),
            user32_reserved: [0;26],
            user_reserved: [0;5],
            //user32_reserved: maps.read_bytes(addr + 136, 26 * 4).try_into().unwrap(),
            //user_reserved: maps.read_bytes(addr + 240, 5 * 4).try_into().unwrap(),
            wow32_reserved: maps.read_qword(addr + 260).unwrap(),
            current_locale: maps.read_dword(addr + 268).unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        self.nt_tib.save(addr, maps);
        maps.write_qword(addr + 56, self.environment_pointer);
        self.client_id.save(addr + 64, maps);
        maps.write_qword(addr + 80, self.pad0);
        maps.write_qword(addr + 88, self.active_rpc_handle);
        maps.write_qword(addr + 96, self.thread_local_storage_pointer);
        maps.write_qword(addr + 104, self.process_environment_block);
        maps.write_dword(addr + 112, self.last_error_value);
        maps.write_dword(addr + 116, self.count_of_owned_critical_sections);
        maps.write_qword(addr + 120, self.csr_client_thread);
        maps.write_qword(addr + 128, self.win32_thread_info);
        maps.write_bytes(addr + 136, self.user32_reserved.iter().copied().flat_map(|v| v.to_le_bytes()).collect());
        maps.write_bytes(addr + 240, self.user_reserved.iter().copied().flat_map(|v| v.to_le_bytes()).collect());
        maps.write_qword(addr + 260, self.wow32_reserved);
        maps.write_dword(addr + 268, self.current_locale);
    }
}

#[derive(Debug)]
pub struct Peb {
    pub inherited_address_space: u8,
    pub read_image_file_exec_options: u8,
    pub being_debugged: u8,
    pub bit_field: u8,
    pub mutant: u64,
    pub image_base_address: u64,
    pub ldr: u64,
    pub process_parameters: u64,
    pub sub_system_data: u64,
    pub process_heap: u64,
    pub fast_peb_lock: u64,
    pub atl_thunk_slist_ptr: u64,
    pub ifeo_key: u64,
    pub cross_process_flags: u64,
    pub user_shared_info_ptr: u64,
    pub system_reserved: u32,
    pub atl_thunk_slist_ptr32: u32,
    pub api_set_map: u64,
    pub tls_expansion_counter: u64,
    pub tls_bitmap: u64,
    pub tls_bitmap_bits: [u32; 2],
    pub read_only_shared_memory_base: u64,
    pub shared_data: u64,
    pub read_only_static_server_data: u64,
    pub ansi_code_page_data: u64,
    pub oem_code_page_data: u64,
    pub unicode_case_table_data: u64,
    pub number_of_processors: u32,
    pub nt_global_flag: u32,
    pub critical_section_timeout: i64,
    pub heap_segment_reserve: u64,
    pub heap_segment_commit: u64,
    pub heap_decommit_total_free_threshold: u64,
    pub heap_decommit_free_block_threshold: u64,
    pub number_of_heaps: u32,
    pub maximum_number_of_heaps: u32,
    pub process_heaps: u64,
    pub gdi_shared_handle_table: u64,
    pub process_starter_helper: u64,
    pub gdi_dc_attribute_list: u64,
    pub loader_lock: u64,
    pub os_major_version: u32,
    pub os_minor_version: u32,
    pub os_build_number: u16,
    pub os_csd_version: u16,
    pub os_platform_id: u32,
    pub image_subsystem: u32,
    pub image_subsystem_major_version: u32,
    pub image_subsystem_minor_version: u64,
    pub gdi_handle_buffer: [u32; 60],
    pub post_process_init_routine: u64,
    pub tls_expansion_bitmap: u64,
    pub tls_expansion_bitmap_bits: [u32; 32],
    pub session_id: u64,
    pub app_compat_flags: u64,
    pub app_compat_flags_user: u64,
    pub p_shim_data: u64,
    pub app_compat_info: u64,
    pub csd_version: UnicodeString,
    pub activation_context_data: u64,
    pub process_assembly_storage_map: u64,
    pub system_default_activation_context_data: u64,
    pub system_assembly_storage_map: u64,
    pub minimum_stack_commit: u64,
    pub fls_callback: u64,
    pub fls_list_head: ListEntry64,
    pub fls_bitmap: u64,
    pub fls_bitmap_bits: [u32; 4],
    pub fls_high_index: u64,
    pub wer_registration_data: u64,
    pub wer_ship_assert_ptr: u64,
    pub p_unused: u64,
    pub p_image_header_hash: u64,
    pub tracing_flags: u64,
    pub csr_server_read_only_shared_memory_base: u64,
    pub tpp_workerp_list_lock: u64,
    pub tpp_workerp_list: ListEntry64,
    pub wait_on_address_hash_table: [u64; 128],
}

impl Default for Peb { fn default() -> Self { Self::new() } }

impl Peb {
    pub fn new() -> Peb {
        Peb {
            inherited_address_space: 0,
            read_image_file_exec_options: 0,
            being_debugged: 0,
            bit_field: 0,
            mutant: 0,
            image_base_address: 0,
            ldr: 0,
            process_parameters: 0,
            sub_system_data: 0,
            process_heap: 0,
            fast_peb_lock: 0,
            atl_thunk_slist_ptr: 0,
            ifeo_key: 0,
            cross_process_flags: 0,
            user_shared_info_ptr: 0,
            system_reserved: 0,
            atl_thunk_slist_ptr32: 0,
            api_set_map: 0,
            tls_expansion_counter: 0,
            tls_bitmap: 0,
            tls_bitmap_bits: [0; 2],
            read_only_shared_memory_base: 0,
            shared_data: 0,
            read_only_static_server_data: 0,
            ansi_code_page_data: 0,
            oem_code_page_data: 0,
            unicode_case_table_data: 0,
            number_of_processors: 0,
            nt_global_flag: 0,
            critical_section_timeout: 0,
            heap_segment_reserve: 0,
            heap_segment_commit: 0,
            heap_decommit_total_free_threshold: 0,
            heap_decommit_free_block_threshold: 0,
            number_of_heaps: 0,
            maximum_number_of_heaps: 0,
            process_heaps: 0,
            gdi_shared_handle_table: 0,
            process_starter_helper: 0,
            gdi_dc_attribute_list: 0,
            loader_lock: 0,
            os_major_version: 0,
            os_minor_version: 0,
            os_build_number: 0,
            os_csd_version: 0,
            os_platform_id: 0,
            image_subsystem: 0,
            image_subsystem_major_version: 0,
            image_subsystem_minor_version: 0,
            gdi_handle_buffer: [0; 60],
            post_process_init_routine: 0,
            tls_expansion_bitmap: 0,
            tls_expansion_bitmap_bits: [0; 32],
            session_id: 0,
            app_compat_flags: 0,
            app_compat_flags_user: 0,
            p_shim_data: 0,
            app_compat_info: 0,
            csd_version: UnicodeString::new(),
            activation_context_data: 0,
            process_assembly_storage_map: 0,
            system_default_activation_context_data: 0,
            system_assembly_storage_map: 0,
            minimum_stack_commit: 0,
            fls_callback: 0,
            fls_list_head: ListEntry64::new(),
            fls_bitmap: 0,
            fls_bitmap_bits: [0; 4],
            fls_high_index: 0,
            wer_registration_data: 0,
            wer_ship_assert_ptr: 0,
            p_unused: 0,
            p_image_header_hash: 0,
            tracing_flags: 0,
            csr_server_read_only_shared_memory_base: 0,
            tpp_workerp_list_lock: 0,
            tpp_workerp_list: ListEntry64::new(),
            wait_on_address_hash_table: [0; 128],
        }
    }
}

#[derive(Debug)]
pub struct LargeInteger {
    pub low_part: u32,
    pub high_part: u32,
}

impl Default for LargeInteger { fn default() -> Self { Self::new() } }

impl LargeInteger {
    pub fn new() -> LargeInteger {
        LargeInteger { low_part: 0, high_part: 0 }
    }

    pub fn load(addr: u64, maps: &Maps) -> LargeInteger {
        LargeInteger {
            low_part: maps.read_dword(addr).unwrap(),
            high_part: maps.read_dword(addr + 4).unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_dword(addr, self.low_part);
        maps.write_dword(addr + 4, self.high_part);
    }
}

#[derive(Debug)]
pub struct RtlUserProcessParameters {
    pub reserved1: [u8; 16],
    pub reserved2: [u32; 10],
    pub image_path_name: UnicodeString,
    pub command_line: UnicodeString,
}

impl Default for RtlUserProcessParameters { fn default() -> Self { Self::new() } }

impl RtlUserProcessParameters {
    pub fn new() -> RtlUserProcessParameters {
        RtlUserProcessParameters {
            reserved1: [0; 16],
            reserved2: [0; 10],
            image_path_name: UnicodeString::new(),
            command_line: UnicodeString::new(),
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> RtlUserProcessParameters {
        RtlUserProcessParameters {
            reserved1: maps.read_bytes(addr, 16).try_into().unwrap(),
            reserved2: maps.read_bytes(addr + 16, 10 * 4)
                .chunks_exact(4)
                .map(|c| u32::from_le_bytes(c.try_into().unwrap()))
                .collect::<Vec<u32>>()
                .try_into()
                .unwrap(),
            image_path_name: UnicodeString::load(addr + 56, maps),
            command_line: UnicodeString::load(addr + 64, maps),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_bytes(addr, self.reserved1.to_vec());
        maps.write_bytes(addr + 16, self.reserved2.iter().flat_map(|v| v.to_le_bytes()).collect());
        self.image_path_name.save(addr + 56, maps);
        self.command_line.save(addr + 64, maps);
    }
}


