use crate::emu;
use crate::maps::mem64::Permission;
use bitfield::bitfield;
use std::mem::MaybeUninit;
use std::ptr;

const USER_KUSER_SHARED_ADDR: u64 = 0x7FFE0000;

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum AlternativeArchitectureType {
    StandardDesign,
    Nec98x86,
    EndAlternatives,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum NtProductType {
    WinNt = 1,
    LanManNt,
    Server,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KsystemTime {
    pub LowPart: u32,
    pub High1Time: i32,
    pub High2Time: i32,
}

bitfield! {
    /// Represents the `KusdMitigationPoliciesUnion` from C.
    /// Backed by a single `u8` with 4 x 2-bit fields.
    #[derive(Clone, Copy)]
    pub struct KuserSharedData00(u8);
    u8;

    /// Bits 0-1: `NXSupportPolicy`
    nx_support_policy, set_nx_support_policy: 1, 0;

    /// Bits 2-3: `SEHValidationPolicy`
    seh_validation_policy, set_seh_validation_policy: 3, 2;

    /// Bits 4-5: `CurDirDevicesSkippedForDlls`
    cur_dir_devices_skipped, set_cur_dir_devices_skipped: 5, 4;

    /// Bits 6-7: Reserved
    reserved, set_reserved: 7, 6;
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union KusdMitigationPoliciesUnion {
    pub MitigationPolicies: u8,
    pub Anonymous: KuserSharedData00,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union KusdVirtualizationFlagsUnion {
    pub VirtualizationFlags: u8,
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy)]
    pub struct KusdSharedDataFlagsBits(u32);
    u32;

    // Bit fields (total 32 bits)
    pub dbg_error_port_present, set_dbg_error_port_present: 0;
    pub dbg_elevation_enabled, set_dbg_elevation_enabled: 1;
    pub dbg_virt_enabled, set_dbg_virt_enabled: 2;
    pub dbg_installer_detect_enabled, set_dbg_installer_detect_enabled: 3;
    pub dbg_lkg_enabled, set_dbg_lkg_enabled: 4;
    pub dbg_dyn_processor_enabled, set_dbg_dyn_processor_enabled: 5;
    pub dbg_console_broker_enabled, set_dbg_console_broker_enabled: 6;
    pub dbg_secure_boot_enabled, set_dbg_secure_boot_enabled: 7;
    pub dbg_multi_session_sku, set_dbg_multi_session_sku: 8;
    pub dbg_multi_users_in_session_sku, set_dbg_multi_users_in_session_sku: 9;
    pub dbg_state_separation_enabled, set_dbg_state_separation_enabled: 10;
    pub dbg_split_token_enabled, set_dbg_split_token_enabled: 11;
    pub dbg_shadow_admin_enabled, set_dbg_shadow_admin_enabled: 12;
    pub spare_bits, set_spare_bits: 31, 13; // Bits 13..=31 (19 bits)
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union KusdSharedDataFlagsUnion {
    pub SharedDataFlags: u32,
    pub bits: KusdSharedDataFlagsBits,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct OverlayStruct {
    pub ReservedTickCountOverlay: [u32; 3],
    pub TickCountPad: [u32; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union KusdTickCountUnion {
    pub TickCount: KsystemTime,
    pub TickCountQuad: u64,
    pub Overlay: OverlayStruct,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union KusdQpcDataUnion {
    pub QpcData: u16,
    pub anonymous: KusdQpcDataAnon,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct KusdQpcDataAnon {
    pub QpcBypassEnabled: u8,
    pub QpcReserved: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct XstateFeature {
    pub Offset: u32,
    pub Size: u32,
}

// Bitfield helper for ControlFlags
bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy)]
    pub struct ControlFlagsBitfield(u32);
    u32;

    pub OptimizedSave, set_OptimizedSave: 0, 0;
    pub CompactionEnabled, set_CompactionEnabled: 1, 1;
    pub Reserved1, set_Reserved1: 31, 2;
}

// Anonymous union replacement using a wrapper
#[repr(C)]
#[derive(Clone, Copy)]
pub union ControlFlagsUnion {
    pub raw: u32,
    pub bits: ControlFlagsBitfield,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct XstateConfiguration {
    pub EnabledFeatures: u64,
    pub EnabledVolatileFeatures: u64,
    pub Size: u32,
    pub Anonymous: ControlFlagsUnion,
    pub Features: [XstateFeature; 64],
    pub EnabledSupervisorFeatures: u64,
    pub AlignedFeatures: u64,
    pub AllFeatureSize: u32,
    pub AllFeatures: [u32; 64],
    pub EnabledUserVisibleSupervisorFeatures: u64,
    pub ExtendedFeatureDisableFeatures: u64,
    pub AllNonLargeFeatureSize: u32,
    pub MaxSveVectorLength: u16,
    pub Spare: u16,
}

/*
The KuserSharedData is getting from windows 24h2 from  vergilusproject
https://www.vergiliusproject.com/kernels/x64/windows-11/24h2/_KUSER_SHARED_DATA
 */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KuserSharedData {
    pub TickCountLowDeprecated: u32,
    pub TickCountMultiplier: u32,
    pub InterruptTime: KsystemTime,
    pub SystemTime: KsystemTime,
    pub TimeZoneBias: KsystemTime,
    pub ImageNumberLow: u16,
    pub ImageNumberHigh: u16,
    pub NtSystemRoot: [u16; 260],
    pub MaxStackTraceDepth: u32,
    pub CryptoExponent: u32,
    pub TimeZoneId: u32,
    pub LargePageMinimum: u32,
    pub AitSamplingValue: u32,
    pub AppCompatFlag: u32,
    pub RNGSeedVersion: u64,
    pub GlobalValidationRunlevel: u32,
    pub TimeZoneBiasStamp: i32,
    pub NtBuildNumber: u32,
    pub NtProductType: NtProductType,
    pub ProductTypeIsValid: bool,
    pub Reserved0: [bool; 1],
    pub NativeProcessorArchitecture: u16,
    pub NtMajorVersion: u32,
    pub NtMinorVersion: u32,
    pub ProcessorFeatures: [bool; 64],
    pub Reserved1: u32,
    pub Reserved3: u32,
    pub TimeSlip: u32,
    pub AlternativeArchitecture: AlternativeArchitectureType,
    pub BootId: u32,
    pub SystemExpirationDate: i64,
    pub SuiteMask: u32,
    pub KdDebuggerEnabled: bool,
    pub MitigationPolicies: KusdMitigationPoliciesUnion,
    pub CyclesPerYield: u16,
    pub ActiveConsoleId: u32,
    pub DismountCount: u32,
    pub ComPlusPackage: u32,
    pub LastSystemRITEventTickCount: u32,
    pub NumberOfPhysicalPages: u32,
    pub SafeBootMode: bool,
    pub VirtualizationFlags: KusdVirtualizationFlagsUnion,
    pub Reserved12: [u8; 2],
    pub SharedDataFlags: KusdSharedDataFlagsUnion,
    pub DataFlagsPad: [u32; 1],
    pub TestRetInstruction: u64,
    pub QpcFrequency: i64,
    pub SystemCall: u32,
    pub Reserved2: u32,
    pub SystemCallPad: [u64; 2],
    pub TickCount: KusdTickCountUnion,
    pub Cookie: u32,
    pub CookiePad: [u32; 1],
    pub ConsoleSessionForegroundProcessId: i64,
    pub TimeUpdateLock: u64,
    pub BaselineSystemTimeQpc: u64,
    pub BaselineInterruptTimeQpc: u64,
    pub QpcSystemTimeIncrement: u64,
    pub QpcInterruptTimeIncrement: u64,
    pub QpcSystemTimeIncrementShift: u8,
    pub QpcInterruptTimeIncrementShift: u8,
    pub UnparkedProcessorCount: u16,
    pub EnclaveFeatureMask: [u32; 4],
    pub TelemetryCoverageRound: u32,
    pub UserModeGlobalLogger: [u16; 16],
    pub ImageFileExecutionOptions: u32,
    pub LangGenerationCount: u32,
    pub Reserved4: u64,
    pub InterruptTimeBias: u64,
    pub QpcBias: u64,
    pub ActiveProcessorCount: u32,
    pub ActiveGroupCount: u8,
    pub Reserved9: u8,
    pub QpcData: KusdQpcDataUnion,
    pub TimeZoneBiasEffectiveStart: i64,
    pub TimeZoneBiasEffectiveEnd: i64,
    pub XState: XstateConfiguration,
    pub FeatureConfigurationChangeStamp: KsystemTime,
    pub Spare: u32,
    pub UserPointerAuthMask: u64,
    pub Reserved10: [u32; 210],
}

pub fn init_kuser_shared_data(emu: &mut emu::Emu) -> u64 {
    emu.maps
        .create_map(
            "KuserSharedData",
            USER_KUSER_SHARED_ADDR,
            0x1000,
            Permission::READ_WRITE,
        )
        .expect("cannot create KuserSharedData map");

    // The KUSER_SHARED_DATA is getting from: https://github.com/momo5502/sogen/blob/main/src/windows-emulator/kusd_mmio.cpp
    let mut kusd: KuserSharedData = unsafe { MaybeUninit::zeroed().assume_init() };
    kusd.TickCountMultiplier = 0x0fa00000;
    kusd.InterruptTime.LowPart = 0x17bd9547;
    kusd.InterruptTime.High1Time = 0x0000004b;
    kusd.InterruptTime.High2Time = 0x0000004b;
    kusd.SystemTime.LowPart = 0x7af9da99;
    kusd.SystemTime.High1Time = 0x01db27b9;
    kusd.SystemTime.High2Time = 0x01db27b9;
    kusd.TimeZoneBias.LowPart = 0x3c773000;
    kusd.TimeZoneBias.High1Time = -17;
    kusd.TimeZoneBias.High2Time = -17;
    kusd.TimeZoneId = 0x00000002;
    kusd.LargePageMinimum = 0x00200000;
    kusd.RNGSeedVersion = 0x0000000000000013;
    kusd.TimeZoneBiasStamp = 0x00000004;
    kusd.NtBuildNumber = 0x00006c51;
    kusd.NtProductType = NtProductType::WinNt;
    kusd.ProductTypeIsValid = true;
    kusd.NativeProcessorArchitecture = 0x0009;
    kusd.NtMajorVersion = 0x0000000a;
    kusd.BootId = 0x0000000b;
    kusd.SystemExpirationDate = 0x01dc26860a9ff300;
    kusd.SuiteMask = 0x00000110;
    kusd.MitigationPolicies.MitigationPolicies = 0x0a;
    unsafe {
        kusd.MitigationPolicies.Anonymous.set_nx_support_policy(0x2);
        kusd.MitigationPolicies
            .Anonymous
            .set_seh_validation_policy(0x2);
    }
    kusd.CyclesPerYield = 0x0064;
    kusd.DismountCount = 0x00000006;
    kusd.ComPlusPackage = 0x00000001;
    kusd.LastSystemRITEventTickCount = 0x01ec1fd3;
    kusd.NumberOfPhysicalPages = 0x00bf0958;
    kusd.NumberOfPhysicalPages = 0x0000000000bf0958;
    kusd.TickCount.TickCount.LowPart = 0x001f7f05;
    kusd.TickCount.TickCountQuad = 0x00000000001f7f05;
    kusd.Cookie = 0x1c3471da;
    kusd.ConsoleSessionForegroundProcessId = 0x00000000000028f4;
    kusd.TimeUpdateLock = 0x0000000002b28586;
    kusd.BaselineSystemTimeQpc = 0x0000004b17cd596c;
    kusd.BaselineInterruptTimeQpc = 0x0000004b17cd596c;
    kusd.QpcSystemTimeIncrement = 0x8000000000000000;
    kusd.QpcInterruptTimeIncrement = 0x8000000000000000;
    kusd.QpcSystemTimeIncrementShift = 0x01;
    kusd.QpcInterruptTimeIncrementShift = 0x01;
    kusd.UnparkedProcessorCount = 0x000c;
    kusd.TelemetryCoverageRound = 0x00000001;
    kusd.LangGenerationCount = 0x00000003;
    kusd.InterruptTimeBias = 0x00000015a5d56406;
    kusd.ActiveProcessorCount = 0x0000000c;
    kusd.ActiveGroupCount = 0x01;
    kusd.TimeZoneBiasEffectiveStart = 0x01db276e654cb2ff;
    kusd.TimeZoneBiasEffectiveEnd = 0x01db280b8c3b2800;
    kusd.XState.EnabledFeatures = 0x000000000000001f;
    kusd.XState.EnabledVolatileFeatures = 0x000000000000000f;
    kusd.XState.Size = 0x000003c0;
    kusd.QpcData.QpcData = 0x0083;
    kusd.QpcData.anonymous.QpcBypassEnabled = 0x83;
    kusd.QpcBias = 0x000000159530c4af;

    let mut memory: [u8; std::mem::size_of::<KuserSharedData>()] =
        [0; std::mem::size_of::<KuserSharedData>()];

    unsafe {
        // Copy the struct into the allocated memory
        let struct_ptr = &kusd as *const KuserSharedData as *const u8;
        let memory_ptr = memory.as_mut_ptr();
        ptr::copy_nonoverlapping(
            struct_ptr,
            memory_ptr,
            std::mem::size_of::<KuserSharedData>(),
        );
    }

    emu.maps
        .write_bytes(USER_KUSER_SHARED_ADDR, memory.to_vec());

    USER_KUSER_SHARED_ADDR
}
