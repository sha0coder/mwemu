use crate::peb::peb32;
use crate::{emu, serialization};

use lazy_static::lazy_static;
use std::sync::Mutex;

mod add_vectored_exception_handler;
mod are_file_apis_ansi;
mod close_handle;
mod connect_named_pipe;
mod copy_file_a;
mod copy_file_w;
mod create_event_a;
mod create_file_mapping_a;
mod create_file_mapping_w;
mod create_file_w;
mod create_mutex_a;
mod create_mutex_w;
mod create_named_pipe_a;
mod create_process_a;
mod create_remote_thread;
mod create_thread;
mod create_toolhelp32_snapshot;
mod crypt_create_hash;
mod decode_pointer;
mod delete_file_a;
mod disconnect_named_pipe;
mod encode_pointer;
mod enter_critical_section;
mod exit_process;
mod expand_environment_strings_a;
mod expand_environment_strings_w;
mod file_time_to_dos_date_time;
mod file_time_to_local_file_time;
mod file_time_to_system_time;
mod find_close;
mod find_first_file_a;
mod find_first_file_w;
mod find_next_file_a;
mod find_next_file_w;
mod find_resource_a;
mod find_resource_w;
mod fls_alloc;
mod fls_get_value;
mod fls_set_value;
mod free_library;
mod free_resource;
mod get_acp;
mod get_command_line_a;
mod get_command_line_w;
mod get_computer_name_a;
mod get_cp_info;
mod get_current_directory_a;
mod get_current_directory_w;
mod get_current_process;
mod get_current_process_id;
mod get_current_thread_id;
mod get_disk_free_space_a;
mod get_environment_strings;
mod get_environment_strings_w;
mod get_file_attributes_a;
mod get_file_attributes_w;
mod get_file_type;
mod get_full_path_name_a;
mod get_full_path_name_w;
mod get_last_error;
mod get_local_time;

mod get_logical_drives;
mod get_long_path_name_w;
mod get_module_file_name_a;
mod get_module_file_name_w;
mod get_module_handle_a;
mod get_module_handle_w;
mod get_native_system_info;
mod get_oemcp;
mod get_proc_address;
mod get_process_affinity_mask;
mod get_process_heap;
mod get_startup_info_a;
mod get_startup_info_w;
mod get_std_handle;
mod get_string_type_w;
mod get_system_directory_a;
mod get_system_directory_w;
mod get_system_info;
mod get_system_time;
mod get_system_time_as_file_time;
mod get_system_windows_directory_a;
mod get_system_windows_directory_w;
mod get_temp_path_w;
mod get_thread_context;
mod get_thread_preferred_ui_languages;
mod get_thread_ui_language;
mod get_tick_count;
mod get_time_zone_information;
mod get_user_default_lang_id;
mod get_user_default_ui_language;
mod get_version;
mod get_version_ex_w;
mod get_windows_directory_a;
mod get_windows_directory_w;
mod global_add_atom_a;
mod heap_alloc;
mod heap_create;
mod heap_destroy;
mod heap_free;
mod heap_set_information;
mod initialize_critical_section;
mod initialize_critical_section_and_spin_count;
mod initialize_critical_section_ex;
mod interlocked_increment;
mod is_debugger_present;
mod is_processor_feature_present;
mod is_valid_code_page;
mod is_valid_locale;
mod lc_map_string_w;
mod leave_critical_section;
mod load_library_a;
mod load_library_ex_a;
mod load_library_ex_w;
mod load_library_w;
mod load_resource;
mod local_alloc;
mod lock_resource;
mod lstrcat;
mod lstrcmp_a;
mod lstrcmp_w;
mod lstrcmpi_a;
mod lstrcpy;
mod lstrlen;
mod map_view_of_file;
mod move_file_a;
mod move_file_w;
mod multi_byte_to_wide_char;
mod open_process;
mod open_process_token;
mod open_thread;
mod query_performance_counter;
mod raise_exception;
mod read_file;
mod read_process_memory;
mod reg_close_key;
mod reg_create_key_ex_a;
mod reg_create_key_ex_w;
mod reg_open_key_a;
mod reg_open_key_ex_w;
mod reg_open_key_w;
mod reg_set_value_ex_a;
mod reg_set_value_ex_w;
mod resume_thread;
mod set_error_mode;
mod set_file_pointer;
mod set_handle_count;
mod set_last_error;
mod set_thread_context;
mod set_thread_locale;
mod set_unhandled_exception_filter;
mod sizeof_resource;
mod sleep;
mod system_time_to_tz_specific_local_time;
mod terminate_process;
mod thread32_first;
mod thread32_next;
mod tls_alloc;
mod tls_free;
mod tls_get_value;
mod tls_set_value;
mod unhandled_exception_filter;
mod verify_version_info_w;
mod virtual_alloc;
mod virtual_alloc_ex;
mod virtual_alloc_ex_numa;
mod virtual_free;
mod virtual_lock;

mod virtual_protect;
mod virtual_protect_ex;
mod virtual_query;
mod virtual_query_ex;
mod wait_for_single_object;
mod wide_char_to_multi_byte;
mod win_exec;
mod write_file;
mod write_process_memory;

mod __tlg_create_wsz;
mod _guard_check_icall_nop;
mod _lclose;
mod _lcreat;
mod _llseek;
mod _lopen;
mod _lread;
mod _lwrite;
mod _nlg__dispatch2;
mod _nlg__notify;
mod _tlg_enable_callback;
mod _tlg_keyword_on;
mod _tlg_write_transfer__event_write_transfer;

mod activate_act_ctx_worker;
mod add_atom_a;
mod add_atom_w;
mod add_integrity_label_to_boundary_descriptor;
mod add_local_alternate_computer_name_w;

mod adjust_calendar_date;

mod application_recovery_finished;
mod application_recovery_in_progress;
mod backup_read;
mod backup_seek;
mod backup_write;

mod base_check_appcompat_cache_ex_worker;

mod base_check_appcompat_cache_worker;
mod base_check_elevation;

mod base_cleanup_appcompat_cache_support_worker;
mod base_destroy_vdm_environment;
mod base_dll_initialize;
mod base_dll_read_write_ini_file;
mod base_exit_thread_pool_thread;
mod base_flush_appcompat_cache_worker;
mod base_free_app_compat_data_for_process_worker;
mod base_generate_app_compat_data;

mod base_is_dos_application;
mod base_read_app_compat_data_for_process_worker;
mod base_set_last_nt_error;

mod base_update_appcompat_cache_worker;
mod base_update_vdm_entry;
mod base_verify_unicode_string;
mod base_write_error_elevation_required_event;
mod basep8_bit_string_to_dynamic_unicode_string;
mod basep_accumulate_io_rate_control_information_buffer_size;
mod basep_allocate_activation_context_activation_block;
mod basep_ansi_string_to_dynamic_unicode_string;
mod basep_app_container_environment_extension;
mod basep_app_x_extension;
mod basep_check_app_compat;
mod basep_check_web_blade_hashes;
mod basep_check_win_safer_restrictions;
mod basep_configure_app_cert_dlls;
mod basep_construct_sxs_create_process_message;
mod basep_copy_encryption;
mod basep_free_activation_context_activation_block;
mod basep_free_app_compat_data;
mod basep_get_computer_name_from_nt_path;
mod basep_get_exe_arch_type;
mod basep_get_mini_version_for_create;
mod basep_init_app_compat_data;
mod basep_initialize_apphelp_globals;
mod basep_initialize_termsrv_fpns;
mod basep_is_process_allowed;
mod basep_is_test_signing_enabled;
mod basep_notify_load_string_resource;
mod basep_post_success_app_x_extension;
mod basep_process_invalid_image;
mod basep_query_app_compat;
mod basep_regenerate_act_ctx_with_language;
mod basep_release_sxs_create_process_utility_struct;
mod basep_report_fault;
mod basep_set_file_encryption_compression;
mod basep_sxs_create_streams;
mod basep_tp_io_callback;
mod basep_tp_io_cleanup_callback;
mod basep_tp_io_finalization_callback;
mod beep_implementation;
mod begin_update_resource_a;
mod begin_update_resource_w;
mod bind_io_completion_callback;
mod build_comm_dcb_and_timeouts_a;
mod build_comm_dcb_and_timeouts_w;
mod build_comm_dcba;
mod build_comm_dcbw;
mod byte_match;
mod calibrate_timer;
mod call_named_pipe_a;

mod cancel_timer_queue_timer;

mod check_elevation;
mod check_for_read_only_resource;
mod check_name_legal_dos8_dot3_a;
mod check_name_legal_dos8_dot3_w;

mod close_console_handle;

mod comm_config_dialog_a;
mod compare_calendar_dates;

mod compat_cache_lookup_exe;
mod console_menu_control;

mod convert_cal_date_time_to_system_time;

mod convert_nls_day_of_week_to_win32_day_of_week;
mod convert_system_time_to_cal_date_time;

mod copy_file_ex_a;

mod copy_file_transacted_a;
mod copy_file_transacted_w;
mod copy_lz_file;
mod cptp_quirks_init_once;
mod create_act_ctx_a;

mod create_act_ctx_w_worker;
mod create_boundary_descriptor_a;

mod create_directory_ex_a;

mod create_directory_transacted_a;
mod create_directory_transacted_w;
mod create_file_mapping_numa_a;

mod create_hard_link_transacted_a;

mod create_job_object_a;
mod create_job_object_w;
mod create_mailslot_a;
mod create_mailslot_w;

mod create_semaphore_a;
mod create_semaphore_ex_a;
mod create_socket_handle;
mod create_symbolic_link_a;
mod create_symbolic_link_transacted_a;
mod create_symbolic_link_transacted_w;

mod create_tape_partition;

mod create_waitable_timer_a;
mod create_waitable_timer_ex_a;
mod create_waitable_timer_w;

mod deactivate_act_ctx_worker;

mod debug_set_process_kill_on_exit;
mod define_dos_device_a;

mod delete_atom;
mod delete_file_transacted_w;

mod delete_timer_queue;

mod delete_volume_mount_point_a;
mod device_io_control_implementation;
mod device_name_compare;
mod disable_thread_profiling;
mod dns_hostname_to_computer_name_a;

mod dns_hostname_to_computer_name_w;
mod dos_date_time_to_file_time;
mod dos_path_to_session_path_a;
mod dos_path_to_session_path_w;
mod duplicate_encryption_info_file_ext;
mod enable_thread_profiling;
mod end_update_resource_a;
mod end_update_resource_w;

mod enum_calendar_info_a;
mod enum_calendar_info_ex_a;

mod enum_date_formats_a;
mod enum_date_formats_ex_a;

mod enum_language_group_locales_a;

mod enum_system_code_pages_a;

mod enum_system_geo_id;
mod enum_system_geo_names;
mod enum_system_language_groups_a;

mod enum_time_formats_a;

mod enum_ui_languages_a;

mod enumerate_local_computer_names_a;
mod enumerate_local_computer_names_w;
mod erase_tape;
mod fatal_exit;

mod find_act_ctx_section_guid_worker;
mod find_act_ctx_section_string_a;

mod find_act_ctx_section_string_w_worker;
mod find_atom_a;
mod find_atom_w;
mod find_first_volume_a;
mod find_first_volume_mount_point_a;
mod find_first_volume_mount_point_w;
mod find_next_volume_a;
mod find_next_volume_mount_point_a;
mod find_next_volume_mount_point_w;

mod find_volume_mount_point_close;

mod fold_string_a;

mod free_memory_job_object;

mod get_active_processor_count;
mod get_active_processor_group_count;

mod get_application_restart_settings_worker;
mod get_atom_name_a;
mod get_atom_name_w;
mod get_binary_type_w;

mod get_calendar_date_format;
mod get_calendar_date_format_ex;
mod get_calendar_days_in_month;
mod get_calendar_difference_in_days;
mod get_calendar_info_a;
mod get_calendar_months_in_year;
mod get_calendar_supported_date_range;
mod get_calendar_week_number;
mod get_com_plus_package_install_status;
mod get_compat_flags;

mod get_compressed_file_size_transacted_w;

mod get_computer_name_w;
mod get_config_dialog_name;
mod get_console_char_type;
mod get_console_font_info;
mod get_console_hardware_state;
mod get_console_input_wait_handle;
mod get_console_keyboard_layout_name_a;
mod get_console_keyboard_layout_name_w;
mod get_corrupt_detection_state;
mod get_currency_format_a;

mod get_currency_format_w;

mod get_current_act_ctx_worker;
mod get_current_ja_era_index;
mod get_current_thread;

mod get_date_format_a_worker;

mod get_date_format_w_worker;
mod get_default_comm_config_a;
mod get_default_comm_config_w;
mod get_device_power_state;
mod get_dll_directory_a;
mod get_dll_directory_w;
mod get_duration_format;

mod get_encrypted_file_version_ext;

mod get_exit_code_process_implementation;

mod get_expanded_name_a;
mod get_expanded_name_w;
mod get_file_attributes_transacted_a;
mod get_file_bandwidth_reservation;

mod get_firmware_environment_variable_a;
mod get_firmware_environment_variable_w;
mod get_firmware_type;
mod get_friendly_match_comm;
mod get_full_path_name_transacted_a;
mod get_full_path_name_transacted_w;
mod get_geo_info_a;
mod get_geo_info_ex;
mod get_geo_info_w;
mod get_handle_context;
mod get_japanese_era_items;
mod get_known_japan_era_count;

mod get_long_path_name_a;
mod get_mailslot_info;
mod get_max_japan_eras;
mod get_max_japan_years;
mod get_maximum_processor_count;
mod get_maximum_processor_group_count;

mod get_named_pipe_client_computer_name_a;

mod get_named_pipe_client_process_id;
mod get_named_pipe_client_session_id;
mod get_named_pipe_handle_state_a;

mod get_named_pipe_server_session_id;

mod get_numa_available_memory_node;
mod get_numa_available_memory_node_ex;

mod get_numa_node_number_from_handle;
mod get_numa_node_processor_mask;

mod get_numa_processor_node;
mod get_numa_processor_node_ex;
mod get_numa_proximity_node;

mod get_number_format_a;

mod get_number_format_w;
mod get_number_of_console_fonts;

mod get_private_profile_int_a;
mod get_private_profile_int_w;
mod get_private_profile_section_a;
mod get_private_profile_section_names_a;
mod get_private_profile_section_names_w;
mod get_private_profile_section_w;
mod get_private_profile_string_a;
mod get_private_profile_string_w;
mod get_private_profile_struct_a;
mod get_private_profile_struct_w;
mod get_process_dep_policy;

mod get_process_io_counters;

mod get_process_working_set_size;

mod get_profile_int_a;
mod get_profile_int_w;
mod get_profile_section_a;
mod get_profile_section_w;
mod get_profile_string_a;
mod get_profile_string_w;

mod get_short_path_name_a;
mod get_short_path_name_w;

mod get_system_dep_policy;

mod get_system_power_status;

mod get_system_registry_quota;
mod get_tape_parameters;
mod get_tape_position;
mod get_tape_status;

mod get_thread_selector_entry;

mod get_time_format_w_worker;

mod get_user_default_geo_name;

mod get_user_geo_id;

mod get_volume_name_for_volume_mount_point_a;
mod get_volume_path_names_for_volume_name_a;

mod global_add_atom_ex_a;
mod global_add_atom_ex_w;
mod global_add_atom_w;

mod global_delete_atom;
mod global_find_atom_a;
mod global_find_atom_w;
mod global_fix;
mod global_flags;

mod global_get_atom_name_a;
mod global_get_atom_name_w;
mod global_handle;
mod global_lock;
mod global_memory_status;

mod global_re_alloc;
mod global_size;
mod global_un_wire;
mod global_unfix;
mod global_unlock;
mod global_wire;
mod heap32_first;
mod heap32_list_first;
mod heap32_list_next;
mod heap32_next;

mod init_atom_table;
mod init_once_get_string_table_offset;
mod initialize16_bit_critical_section;

mod interlocked_compare_exchange;

mod interlocked_decrement;

mod interlocked_exchange;
mod interlocked_exchange_add;

mod internal_co_std_marshal_object;
mod invalidate_console_di_bits;
mod is_bad_code_ptr;
mod is_bad_huge_read_ptr;
mod is_bad_huge_write_ptr;
mod is_bad_read_ptr;
mod is_bad_string_ptr_a;
mod is_bad_string_ptr_w;
mod is_bad_write_ptr;
mod is_calendar_leap_day;
mod is_calendar_leap_month;
mod is_calendar_leap_year;
mod is_caller_admin_or_system;

mod is_fusion_fully_supported;

mod is_oobe_supported;

mod is_system_luid;
mod is_system_resume_automatic;
mod is_terminal_server_compatible;
mod is_thread_a_fiber;
mod is_valid_cal_date_time;

mod load_app_init_dlls_implementation;

mod load_string_base_w;
mod local_compact;
mod local_flags;

mod local_handle;

mod local_shrink;
mod local_size;
mod lz_close;
mod lz_copy;
mod lz_init;
mod lz_open_file_a;
mod lz_open_file_w;
mod lz_read;
mod lz_seek;
mod lz_start;
mod machine_prefers_numeric_first_year;

mod move_file_ex_a;

mod move_file_transacted_a;
mod move_file_transacted_w;
mod mul_div;

mod non_installer_default;

mod notify_ui_language_change;
mod nt_vdm64_create_process_internal_w;
mod nt_wow64_console_launch_server_process;
mod nt_wow64_csr_base_check_run_app;
mod nt_wow64_csr_basep_create_act_ctx;
mod nt_wow64_csr_basep_create_process;
mod nt_wow64_csr_basep_define_dos_device;
mod nt_wow64_csr_basep_get_process_shutdown_param;
mod nt_wow64_csr_basep_nls_get_user_info;
mod nt_wow64_csr_basep_nls_update_cache_count;
mod nt_wow64_csr_basep_refresh_ini_file_mapping;
mod nt_wow64_csr_basep_set_client_time_zone_information;
mod nt_wow64_csr_basep_set_process_shutdown_param;
mod nt_wow64_csr_basep_sound_sentry_notification;
mod oobe_complete;
mod oobe_complete_wnf_callback;
mod oobe_complete_wnf_query_callback;
mod oobe_complete_wnf_wait_callback;
mod open_file;
mod open_file_mapping_a;

mod open_job_object_a;
mod open_job_object_w;
mod open_mutex_a;
mod open_private_namespace_a;

mod open_sort_id_key;

mod power_clear_request;
mod power_create_request;
mod power_set_request;
mod prepare_tape;

mod priv_move_file_identity_w;
mod process32_first_w;
mod process32_next;
mod process32_next_w;

mod query_act_ctx_settings_w_worker;

mod query_act_ctx_w_worker;
mod query_dos_device_a;

mod query_information_job_object;
mod query_io_rate_control_information_job_object;

mod query_thread_profiling;

mod quirk_get_data_worker;
mod quirk_is_enabled2_worker;
mod quirk_is_enabled_for_package2_worker;
mod quirk_is_enabled_for_package3_worker;
mod quirk_is_enabled_for_package4_worker;
mod quirk_is_enabled_for_package_worker;
mod quirk_is_enabled_for_process_worker;
mod quirk_is_enabled_worker;
mod raise_invalid16_bit_exe_error;

mod read_thread_profiling_data;

mod register_application_recovery_callback;
mod register_application_restart;

mod register_console_ime;
mod register_console_os2;
mod register_console_vdm;
mod register_wait_for_input_idle;
mod register_wait_until_oobe_completed;
mod remove_directory_transacted_w;
mod remove_local_alternate_computer_name_w;
mod remove_secure_memory_cache_callback;
mod replace_file_a;

mod replace_partition_unit;
mod request_wakeup_latency;

mod resource_data_match;
mod return_mem16_data;

mod sbp_is_trace_enabled;
mod sdbp_check_from_version;
mod sdbp_check_matching_device;
mod sdbp_check_matching_dir;
mod sdbp_check_matching_files;
mod sdbp_check_matching_registry;
mod sdbp_check_matching_text;
mod sdbp_check_matching_wildcard_files;
mod sdbp_check_matching_wildcard_registry;
mod sdbp_check_os_kind;
mod sdbp_check_package_attributes;
mod sdbp_check_runtime_platform;
mod sdbp_check_upto_version;
mod sdbp_check_version;
mod sdbp_get_path_app_patch;
mod sdbp_get_path_app_patch_pre_rs3;
mod sdbp_get_path_appraiser;
mod sdbp_get_path_custom_sdb;
mod sdbp_get_path_custom_sdb_pre_rs3;
mod sdbp_get_path_system;

mod set_calendar_info_a;

mod set_com_plus_package_install_status;

mod set_console_cursor;
mod set_console_font;
mod set_console_hardware_state;
mod set_console_icon;
mod set_console_key_shortcuts;
mod set_console_local_eudc;
mod set_console_maximum_window_size;
mod set_console_menu_close;
mod set_console_os2_oem_format;
mod set_console_palette;

mod set_default_comm_config_a;
mod set_dll_directory_a;
mod set_dll_directory_w;

mod set_environment_strings_a;

mod set_file_attributes_transacted_w;
mod set_file_bandwidth_reservation;

mod set_file_short_name_a;
mod set_file_short_name_w;
mod set_firmware_environment_variable_a;
mod set_firmware_environment_variable_w;
mod set_information_job_object;
mod set_io_rate_control_information_job_object;
mod set_local_primary_computer_name_w;

mod set_locale_info_a;

mod set_mailslot_info;
mod set_message_waiting_indicator;
mod set_named_pipe_attribute;

mod set_process_affinity_mask;
mod set_process_dep_policy;

mod set_process_working_set_size;

mod set_system_power_state;
mod set_system_time_adjustment;

mod set_tape_parameters;
mod set_tape_position;
mod set_thread_affinity_mask;
mod set_thread_execution_state;

mod set_timer_queue_timer;
mod set_user_geo_id;
mod set_user_geo_name;
mod set_volume_label_a;
mod set_volume_label_w;
mod set_volume_mount_point_a;
mod set_volume_mount_point_w;

mod show_console_cursor;

mod sort_change_case;
mod sort_compare_string;
mod sort_find_string;
mod sort_get_hash_code;
mod sort_is_defined_string;
mod string_cb_cat_ex_w;
mod string_cb_copy_ex_w;
mod string_cb_copy_w;
mod string_cch_cat_w;
mod string_cch_copy_ex_w;
mod string_cch_copy_nw;
mod string_cch_copy_w;
mod string_cch_length_w;
mod string_copy_worker_a;
mod string_copy_worker_w;
mod string_copy_worker_w_2_0;
mod string_ex_validate_dest_w;
mod string_length_worker_a;
mod string_length_worker_w;
mod string_table;
mod string_v_printf_worker_w;
mod string_v_printf_worker_w_0;
mod string_validate_dest_and_length_w;

mod sxs_manifest;

mod termsrv_convert_sys_root_to_user_dir;
mod termsrv_create_reg_entry;
mod termsrv_delete_key;
mod termsrv_delete_value;
mod termsrv_get_pre_set_value;
mod termsrv_open_reg_entry;
mod termsrv_open_user_classes;
mod termsrv_restore_key;
mod termsrv_set_value_key;
mod termsrv_sync_user_ini_file;
mod termsrv_sync_user_ini_file_ext;
mod time_begin_period;
mod time_end_period;
mod time_get_dev_caps;
mod time_get_time;
mod time_init;
mod trace_logging_register_ex__event_register__event_set_information;

mod u_long_add;
mod u_long_long_to_u_long;
mod u_long_mult;
mod unregister_application_recovery_callback;
mod unregister_application_restart;

mod unregister_wait;

mod unregister_wait_until_oobe_completed;
mod update_calendar_day_of_week;
mod update_resource_a;
mod update_resource_w;
mod vdm_console_operation;
mod vdm_operation_started;

mod verify_console_io_handle;

mod verify_version_info_a;

mod wait_named_pipe_a;

mod wer_get_flags_worker;

mod wer_register_file_worker;

mod wer_set_flags;

mod wer_unregister_file_worker;

mod wer_unregister_memory_block_worker;
mod werp_acquire_peb_lock;
mod werp_check_ok_to_register;
mod werp_get_debugger;
mod werp_init_peb_store;
mod werp_initiate_remote_recovery;
mod werp_launch_ae_debug;
mod werp_map_protection_level;
mod werp_notify_use_string_resource_worker;
mod werp_recovery_invoked_remotely;

mod wow64_enable_wow64_fs_redirection;
mod wow64_get_thread_selector_entry;

mod wow64_suspend_thread;
mod wow64_system_service_call;
mod wow64_transition_resolve;
mod write_console_input_vdma;
mod write_console_input_vdmw;
mod write_private_profile_section_a;
mod write_private_profile_section_w;
mod write_private_profile_string_a;
mod write_private_profile_string_w;
mod write_private_profile_struct_a;
mod write_private_profile_struct_w;
mod write_profile_section_a;
mod write_profile_section_w;
mod write_profile_string_a;
mod write_profile_string_w;
mod write_tapemark;

mod zombify_act_ctx_worker;
pub use add_vectored_exception_handler::*;
pub use are_file_apis_ansi::*;
pub use close_handle::*;
pub use connect_named_pipe::*;
pub use copy_file_a::*;
pub use copy_file_w::*;
pub use create_event_a::*;
pub use create_file_mapping_a::*;
pub use create_file_mapping_w::*;
pub use create_file_w::*;
pub use create_mutex_a::*;
pub use create_mutex_w::*;
pub use create_named_pipe_a::*;
pub use create_process_a::*;
pub use create_remote_thread::*;
pub use create_thread::*;
pub use create_toolhelp32_snapshot::*;
pub use crypt_create_hash::*;
pub use decode_pointer::*;
pub use delete_file_a::*;
pub use disconnect_named_pipe::*;
pub use encode_pointer::*;
pub use enter_critical_section::*;
pub use exit_process::*;
pub use expand_environment_strings_a::*;
pub use expand_environment_strings_w::*;
pub use file_time_to_dos_date_time::*;
pub use file_time_to_local_file_time::*;
pub use file_time_to_system_time::*;
pub use find_close::*;
pub use find_first_file_a::*;
pub use find_first_file_w::*;
pub use find_next_file_a::*;
pub use find_next_file_w::*;
pub use find_resource_a::*;
pub use find_resource_w::*;
pub use fls_alloc::*;
pub use fls_get_value::*;
pub use fls_set_value::*;
pub use free_library::*;
pub use free_resource::*;
pub use get_acp::*;
pub use get_command_line_a::*;
pub use get_command_line_w::*;
pub use get_computer_name_a::*;
pub use get_cp_info::*;
pub use get_current_directory_a::*;
pub use get_current_directory_w::*;
pub use get_current_process::*;
pub use get_current_process_id::*;
pub use get_current_thread_id::*;
pub use get_disk_free_space_a::*;
pub use get_environment_strings::*;
pub use get_environment_strings_w::*;
pub use get_file_attributes_a::*;
pub use get_file_attributes_w::*;
pub use get_file_type::*;
pub use get_full_path_name_a::*;
pub use get_full_path_name_w::*;
pub use get_last_error::*;
pub use get_local_time::*;
pub use get_logical_drives::*;

pub use get_long_path_name_w::*;
pub use get_module_file_name_a::*;
pub use get_module_file_name_w::*;
pub use get_module_handle_a::*;
pub use get_module_handle_w::*;
pub use get_native_system_info::*;
pub use get_oemcp::*;
pub use get_proc_address::*;
pub use get_process_affinity_mask::*;
pub use get_process_heap::*;
pub use get_startup_info_a::*;
pub use get_startup_info_w::*;
pub use get_std_handle::*;
pub use get_string_type_w::*;
pub use get_system_directory_a::*;
pub use get_system_directory_w::*;
pub use get_system_info::*;
pub use get_system_time::*;
pub use get_system_time_as_file_time::*;
pub use get_system_windows_directory_a::*;
pub use get_system_windows_directory_w::*;
pub use get_temp_path_w::*;
pub use get_thread_context::*;
pub use get_thread_preferred_ui_languages::*;
pub use get_thread_ui_language::*;
pub use get_tick_count::*;
pub use get_time_zone_information::*;
pub use get_user_default_lang_id::*;
pub use get_user_default_ui_language::*;
pub use get_version::*;
pub use get_version_ex_w::*;
pub use get_windows_directory_a::*;
pub use get_windows_directory_w::*;
pub use global_add_atom_a::*;
pub use heap_alloc::*;
pub use heap_create::*;
pub use heap_destroy::*;
pub use heap_free::*;
pub use heap_set_information::*;
pub use initialize_critical_section::*;
pub use initialize_critical_section_and_spin_count::*;
pub use initialize_critical_section_ex::*;
pub use interlocked_increment::*;
pub use is_debugger_present::*;
pub use is_processor_feature_present::*;
pub use is_valid_code_page::*;
pub use is_valid_locale::*;
pub use lc_map_string_w::*;
pub use leave_critical_section::*;
pub use load_library_a::*;
pub use load_library_ex_a::*;
pub use load_library_ex_w::*;
pub use load_library_w::*;
pub use load_resource::*;
pub use local_alloc::*;
pub use lock_resource::*;
pub use lstrcat::*;
pub use lstrcmp_w::*;
pub use lstrcmpi_a::*;
pub use lstrcpy::*;
pub use lstrlen::*;
pub use map_view_of_file::*;
pub use move_file_a::*;
pub use move_file_w::*;
pub use multi_byte_to_wide_char::*;
pub use open_process::*;
pub use open_process_token::*;
pub use open_thread::*;
pub use query_performance_counter::*;
pub use raise_exception::*;
pub use read_file::*;
pub use read_process_memory::*;
pub use reg_close_key::*;
pub use reg_create_key_ex_a::*;
pub use reg_create_key_ex_w::*;
pub use reg_open_key_a::*;
pub use reg_open_key_ex_w::*;
pub use reg_open_key_w::*;
pub use reg_set_value_ex_a::*;
pub use reg_set_value_ex_w::*;
pub use resume_thread::*;
pub use set_error_mode::*;
pub use set_file_pointer::*;
pub use set_handle_count::*;
pub use set_last_error::*;
pub use set_thread_context::*;
pub use set_thread_locale::*;
pub use set_unhandled_exception_filter::*;
pub use sizeof_resource::*;
pub use sleep::*;
pub use system_time_to_tz_specific_local_time::*;
pub use terminate_process::*;
pub use thread32_first::*;
pub use thread32_next::*;
pub use tls_alloc::*;
pub use tls_free::*;
pub use tls_get_value::*;
pub use tls_set_value::*;
pub use unhandled_exception_filter::*;
pub use verify_version_info_w::*;
pub use virtual_alloc::*;
pub use virtual_alloc_ex::*;
pub use virtual_alloc_ex_numa::*;
pub use virtual_free::*;
pub use virtual_lock::*;

use crate::emu::Emu;
pub use __tlg_create_wsz::*;
pub use _guard_check_icall_nop::*;
pub use _lclose::*;
pub use _lcreat::*;
pub use _llseek::*;
pub use _lopen::*;
pub use _lread::*;
pub use _lwrite::*;
pub use _nlg__dispatch2::*;
pub use _nlg__notify::*;
pub use _tlg_enable_callback::*;
pub use _tlg_keyword_on::*;
pub use _tlg_write_transfer__event_write_transfer::*;

pub use activate_act_ctx_worker::*;
pub use add_atom_a::*;
pub use add_atom_w::*;
pub use add_integrity_label_to_boundary_descriptor::*;
pub use add_local_alternate_computer_name_w::*;

pub use adjust_calendar_date::*;

pub use application_recovery_finished::*;
pub use application_recovery_in_progress::*;
pub use backup_read::*;
pub use backup_seek::*;
pub use backup_write::*;

pub use base_check_appcompat_cache_ex_worker::*;

pub use base_check_appcompat_cache_worker::*;
pub use base_check_elevation::*;

pub use base_cleanup_appcompat_cache_support_worker::*;
pub use base_destroy_vdm_environment::*;
pub use base_dll_initialize::*;
pub use base_dll_read_write_ini_file::*;
pub use base_exit_thread_pool_thread::*;
pub use base_flush_appcompat_cache_worker::*;
pub use base_free_app_compat_data_for_process_worker::*;
pub use base_generate_app_compat_data::*;

pub use base_is_dos_application::*;
pub use base_read_app_compat_data_for_process_worker::*;
pub use base_set_last_nt_error::*;

pub use base_update_appcompat_cache_worker::*;
pub use base_update_vdm_entry::*;
pub use base_verify_unicode_string::*;
pub use base_write_error_elevation_required_event::*;
pub use basep8_bit_string_to_dynamic_unicode_string::*;
pub use basep_accumulate_io_rate_control_information_buffer_size::*;
pub use basep_allocate_activation_context_activation_block::*;
pub use basep_ansi_string_to_dynamic_unicode_string::*;
pub use basep_app_container_environment_extension::*;
pub use basep_app_x_extension::*;
pub use basep_check_app_compat::*;
pub use basep_check_web_blade_hashes::*;
pub use basep_check_win_safer_restrictions::*;
pub use basep_configure_app_cert_dlls::*;
pub use basep_construct_sxs_create_process_message::*;
pub use basep_copy_encryption::*;
pub use basep_free_activation_context_activation_block::*;
pub use basep_free_app_compat_data::*;
pub use basep_get_computer_name_from_nt_path::*;
pub use basep_get_exe_arch_type::*;
pub use basep_get_mini_version_for_create::*;
pub use basep_init_app_compat_data::*;
pub use basep_initialize_apphelp_globals::*;
pub use basep_initialize_termsrv_fpns::*;
pub use basep_is_process_allowed::*;
pub use basep_is_test_signing_enabled::*;
pub use basep_notify_load_string_resource::*;
pub use basep_post_success_app_x_extension::*;
pub use basep_process_invalid_image::*;
pub use basep_query_app_compat::*;
pub use basep_regenerate_act_ctx_with_language::*;
pub use basep_release_sxs_create_process_utility_struct::*;
pub use basep_report_fault::*;
pub use basep_set_file_encryption_compression::*;
pub use basep_sxs_create_streams::*;
pub use basep_tp_io_callback::*;
pub use basep_tp_io_cleanup_callback::*;
pub use basep_tp_io_finalization_callback::*;
pub use beep_implementation::*;
pub use begin_update_resource_a::*;
pub use begin_update_resource_w::*;
pub use bind_io_completion_callback::*;
pub use build_comm_dcb_and_timeouts_a::*;
pub use build_comm_dcb_and_timeouts_w::*;
pub use build_comm_dcba::*;
pub use build_comm_dcbw::*;
pub use byte_match::*;
pub use calibrate_timer::*;
pub use call_named_pipe_a::*;

pub use cancel_timer_queue_timer::*;

pub use check_elevation::*;
pub use check_for_read_only_resource::*;
pub use check_name_legal_dos8_dot3_a::*;
pub use check_name_legal_dos8_dot3_w::*;

pub use close_console_handle::*;

pub use comm_config_dialog_a::*;
pub use compare_calendar_dates::*;

pub use compat_cache_lookup_exe::*;
pub use console_menu_control::*;

pub use convert_cal_date_time_to_system_time::*;

pub use convert_nls_day_of_week_to_win32_day_of_week::*;
pub use convert_system_time_to_cal_date_time::*;

pub use copy_file_ex_a::*;

pub use copy_file_transacted_a::*;
pub use copy_file_transacted_w::*;
pub use copy_lz_file::*;
pub use cptp_quirks_init_once::*;
pub use create_act_ctx_a::*;

pub use create_act_ctx_w_worker::*;
pub use create_boundary_descriptor_a::*;

pub use create_directory_ex_a::*;

pub use create_directory_transacted_a::*;
pub use create_directory_transacted_w::*;
pub use create_file_mapping_numa_a::*;

pub use create_hard_link_transacted_a::*;

pub use create_job_object_a::*;
pub use create_job_object_w::*;
pub use create_mailslot_a::*;
pub use create_mailslot_w::*;

pub use create_semaphore_a::*;
pub use create_semaphore_ex_a::*;
pub use create_socket_handle::*;
pub use create_symbolic_link_a::*;
pub use create_symbolic_link_transacted_a::*;
pub use create_symbolic_link_transacted_w::*;

pub use create_tape_partition::*;

pub use create_waitable_timer_a::*;
pub use create_waitable_timer_ex_a::*;
pub use create_waitable_timer_w::*;

pub use deactivate_act_ctx_worker::*;

pub use debug_set_process_kill_on_exit::*;
pub use define_dos_device_a::*;

pub use delete_atom::*;
pub use delete_file_transacted_w::*;

pub use delete_timer_queue::*;

pub use delete_volume_mount_point_a::*;
pub use device_io_control_implementation::*;
pub use device_name_compare::*;
pub use disable_thread_profiling::*;
pub use dns_hostname_to_computer_name_a::*;

pub use dns_hostname_to_computer_name_w::*;
pub use dos_date_time_to_file_time::*;
pub use dos_path_to_session_path_a::*;
pub use dos_path_to_session_path_w::*;
pub use duplicate_encryption_info_file_ext::*;
pub use enable_thread_profiling::*;
pub use end_update_resource_a::*;
pub use end_update_resource_w::*;

pub use enum_calendar_info_a::*;
pub use enum_calendar_info_ex_a::*;

pub use enum_date_formats_a::*;
pub use enum_date_formats_ex_a::*;

pub use enum_language_group_locales_a::*;

pub use enum_system_code_pages_a::*;

pub use enum_system_geo_id::*;
pub use enum_system_geo_names::*;
pub use enum_system_language_groups_a::*;

pub use enum_time_formats_a::*;

pub use enum_ui_languages_a::*;

pub use enumerate_local_computer_names_a::*;
pub use enumerate_local_computer_names_w::*;
pub use erase_tape::*;
pub use fatal_exit::*;

pub use find_act_ctx_section_guid_worker::*;
pub use find_act_ctx_section_string_a::*;

pub use find_act_ctx_section_string_w_worker::*;
pub use find_atom_a::*;
pub use find_atom_w::*;
pub use find_first_volume_a::*;
pub use find_first_volume_mount_point_a::*;
pub use find_first_volume_mount_point_w::*;
pub use find_next_volume_a::*;
pub use find_next_volume_mount_point_a::*;
pub use find_next_volume_mount_point_w::*;

pub use find_volume_mount_point_close::*;

pub use fold_string_a::*;

pub use free_memory_job_object::*;

pub use get_active_processor_count::*;
pub use get_active_processor_group_count::*;

pub use get_application_restart_settings_worker::*;
pub use get_atom_name_a::*;
pub use get_atom_name_w::*;
pub use get_binary_type_w::*;

pub use get_calendar_date_format::*;
pub use get_calendar_date_format_ex::*;
pub use get_calendar_days_in_month::*;
pub use get_calendar_difference_in_days::*;
pub use get_calendar_info_a::*;
pub use get_calendar_months_in_year::*;
pub use get_calendar_supported_date_range::*;
pub use get_calendar_week_number::*;
pub use get_com_plus_package_install_status::*;
pub use get_compat_flags::*;

pub use get_compressed_file_size_transacted_w::*;

pub use get_computer_name_w::*;
pub use get_config_dialog_name::*;
pub use get_console_char_type::*;
pub use get_console_font_info::*;
pub use get_console_hardware_state::*;
pub use get_console_input_wait_handle::*;
pub use get_console_keyboard_layout_name_a::*;
pub use get_console_keyboard_layout_name_w::*;
pub use get_corrupt_detection_state::*;
pub use get_currency_format_a::*;

pub use get_currency_format_w::*;

pub use get_current_act_ctx_worker::*;
pub use get_current_ja_era_index::*;
pub use get_current_thread::*;

pub use get_date_format_a_worker::*;

pub use get_date_format_w_worker::*;
pub use get_default_comm_config_a::*;
pub use get_default_comm_config_w::*;
pub use get_device_power_state::*;
pub use get_dll_directory_a::*;
pub use get_dll_directory_w::*;
pub use get_duration_format::*;

pub use get_encrypted_file_version_ext::*;

pub use get_exit_code_process_implementation::*;

pub use get_expanded_name_a::*;
pub use get_expanded_name_w::*;
pub use get_file_attributes_transacted_a::*;
pub use get_file_bandwidth_reservation::*;

pub use get_firmware_environment_variable_a::*;
pub use get_firmware_environment_variable_w::*;
pub use get_firmware_type::*;
pub use get_friendly_match_comm::*;
pub use get_full_path_name_transacted_a::*;
pub use get_full_path_name_transacted_w::*;
pub use get_geo_info_a::*;
pub use get_geo_info_ex::*;
pub use get_geo_info_w::*;
pub use get_handle_context::*;
pub use get_japanese_era_items::*;
pub use get_known_japan_era_count::*;

pub use get_long_path_name_a::*;
pub use get_mailslot_info::*;
pub use get_max_japan_eras::*;
pub use get_max_japan_years::*;
pub use get_maximum_processor_count::*;
pub use get_maximum_processor_group_count::*;

pub use get_named_pipe_client_computer_name_a::*;

pub use get_named_pipe_client_process_id::*;
pub use get_named_pipe_client_session_id::*;
pub use get_named_pipe_handle_state_a::*;

pub use get_named_pipe_server_session_id::*;

pub use get_numa_available_memory_node::*;
pub use get_numa_available_memory_node_ex::*;

pub use get_numa_node_number_from_handle::*;
pub use get_numa_node_processor_mask::*;

pub use get_numa_processor_node::*;
pub use get_numa_processor_node_ex::*;
pub use get_numa_proximity_node::*;

pub use get_number_format_a::*;

pub use get_number_format_w::*;
pub use get_number_of_console_fonts::*;

pub use get_private_profile_int_a::*;
pub use get_private_profile_int_w::*;
pub use get_private_profile_section_a::*;
pub use get_private_profile_section_names_a::*;
pub use get_private_profile_section_names_w::*;
pub use get_private_profile_section_w::*;
pub use get_private_profile_string_a::*;
pub use get_private_profile_string_w::*;
pub use get_private_profile_struct_a::*;
pub use get_private_profile_struct_w::*;
pub use get_process_dep_policy::*;

pub use get_process_io_counters::*;

pub use get_process_working_set_size::*;

pub use get_profile_int_a::*;
pub use get_profile_int_w::*;
pub use get_profile_section_a::*;
pub use get_profile_section_w::*;
pub use get_profile_string_a::*;
pub use get_profile_string_w::*;

pub use get_short_path_name_a::*;
pub use get_short_path_name_w::*;

pub use get_system_dep_policy::*;

pub use get_system_power_status::*;

pub use get_system_registry_quota::*;
pub use get_tape_parameters::*;
pub use get_tape_position::*;
pub use get_tape_status::*;

pub use get_thread_selector_entry::*;

pub use get_time_format_w_worker::*;

pub use get_user_default_geo_name::*;

pub use get_user_geo_id::*;

pub use get_volume_name_for_volume_mount_point_a::*;
pub use get_volume_path_names_for_volume_name_a::*;

pub use global_add_atom_ex_a::*;
pub use global_add_atom_ex_w::*;
pub use global_add_atom_w::*;

pub use global_delete_atom::*;
pub use global_find_atom_a::*;
pub use global_find_atom_w::*;
pub use global_fix::*;
pub use global_flags::*;

pub use global_get_atom_name_a::*;
pub use global_get_atom_name_w::*;
pub use global_handle::*;
pub use global_lock::*;
pub use global_memory_status::*;

pub use global_re_alloc::*;
pub use global_size::*;
pub use global_un_wire::*;
pub use global_unfix::*;
pub use global_unlock::*;
pub use global_wire::*;
pub use heap32_first::*;
pub use heap32_list_first::*;
pub use heap32_list_next::*;
pub use heap32_next::*;

pub use init_atom_table::*;
pub use init_once_get_string_table_offset::*;
pub use initialize16_bit_critical_section::*;

pub use interlocked_compare_exchange::*;

pub use interlocked_decrement::*;

pub use interlocked_exchange::*;
pub use interlocked_exchange_add::*;

pub use internal_co_std_marshal_object::*;
pub use invalidate_console_di_bits::*;
pub use is_bad_code_ptr::*;
pub use is_bad_huge_read_ptr::*;
pub use is_bad_huge_write_ptr::*;
pub use is_bad_read_ptr::*;
pub use is_bad_string_ptr_a::*;
pub use is_bad_string_ptr_w::*;
pub use is_bad_write_ptr::*;
pub use is_calendar_leap_day::*;
pub use is_calendar_leap_month::*;
pub use is_calendar_leap_year::*;
pub use is_caller_admin_or_system::*;
pub use is_fusion_fully_supported::*;
pub use is_oobe_supported::*;
pub use is_system_luid::*;
pub use is_system_resume_automatic::*;
pub use is_terminal_server_compatible::*;
pub use is_thread_a_fiber::*;
pub use is_valid_cal_date_time::*;
pub use load_app_init_dlls_implementation::*;
pub use load_string_base_w::*;
pub use local_compact::*;
pub use local_flags::*;
pub use local_handle::*;
pub use local_shrink::*;
pub use local_size::*;
pub use lstrcmp_a::*;
pub use lz_close::*;
pub use lz_copy::*;
pub use lz_init::*;
pub use lz_open_file_a::*;
pub use lz_open_file_w::*;
pub use lz_read::*;
pub use lz_seek::*;
pub use lz_start::*;
pub use machine_prefers_numeric_first_year::*;
pub use move_file_ex_a::*;
pub use move_file_transacted_a::*;
pub use move_file_transacted_w::*;
pub use mul_div::*;
pub use non_installer_default::*;
pub use notify_ui_language_change::*;
pub use nt_vdm64_create_process_internal_w::*;
pub use nt_wow64_console_launch_server_process::*;
pub use nt_wow64_csr_base_check_run_app::*;
pub use nt_wow64_csr_basep_create_act_ctx::*;
pub use nt_wow64_csr_basep_create_process::*;
pub use nt_wow64_csr_basep_define_dos_device::*;
pub use nt_wow64_csr_basep_get_process_shutdown_param::*;
pub use nt_wow64_csr_basep_nls_get_user_info::*;
pub use nt_wow64_csr_basep_nls_update_cache_count::*;
pub use nt_wow64_csr_basep_refresh_ini_file_mapping::*;
pub use nt_wow64_csr_basep_set_client_time_zone_information::*;
pub use nt_wow64_csr_basep_set_process_shutdown_param::*;
pub use nt_wow64_csr_basep_sound_sentry_notification::*;
pub use oobe_complete::*;
pub use oobe_complete_wnf_callback::*;
pub use oobe_complete_wnf_query_callback::*;
pub use oobe_complete_wnf_wait_callback::*;
pub use open_file::*;
pub use open_file_mapping_a::*;
pub use open_job_object_a::*;
pub use open_job_object_w::*;
pub use open_mutex_a::*;
pub use open_private_namespace_a::*;
pub use open_sort_id_key::*;
pub use power_clear_request::*;
pub use power_create_request::*;
pub use power_set_request::*;
pub use prepare_tape::*;
pub use priv_move_file_identity_w::*;
pub use process32_first_w::*;
pub use process32_next::*;
pub use process32_next_w::*;
pub use query_act_ctx_settings_w_worker::*;
pub use query_act_ctx_w_worker::*;
pub use query_dos_device_a::*;
pub use query_information_job_object::*;
pub use query_io_rate_control_information_job_object::*;
pub use query_thread_profiling::*;
pub use quirk_get_data_worker::*;
pub use quirk_is_enabled2_worker::*;
pub use quirk_is_enabled_for_package2_worker::*;
pub use quirk_is_enabled_for_package3_worker::*;
pub use quirk_is_enabled_for_package4_worker::*;
pub use quirk_is_enabled_for_package_worker::*;
pub use quirk_is_enabled_for_process_worker::*;
pub use quirk_is_enabled_worker::*;
pub use raise_invalid16_bit_exe_error::*;
pub use read_thread_profiling_data::*;
pub use register_application_recovery_callback::*;
pub use register_application_restart::*;
pub use register_console_ime::*;
pub use register_console_os2::*;
pub use register_console_vdm::*;
pub use register_wait_for_input_idle::*;
pub use register_wait_until_oobe_completed::*;
pub use remove_directory_transacted_w::*;
pub use remove_local_alternate_computer_name_w::*;
pub use remove_secure_memory_cache_callback::*;
pub use replace_file_a::*;
pub use replace_partition_unit::*;
pub use request_wakeup_latency::*;
pub use resource_data_match::*;
pub use return_mem16_data::*;
pub use sbp_is_trace_enabled::*;
pub use sdbp_check_from_version::*;
pub use sdbp_check_matching_device::*;
pub use sdbp_check_matching_dir::*;
pub use sdbp_check_matching_files::*;
pub use sdbp_check_matching_registry::*;
pub use sdbp_check_matching_text::*;
pub use sdbp_check_matching_wildcard_files::*;
pub use sdbp_check_matching_wildcard_registry::*;
pub use sdbp_check_os_kind::*;
pub use sdbp_check_package_attributes::*;
pub use sdbp_check_runtime_platform::*;
pub use sdbp_check_upto_version::*;
pub use sdbp_check_version::*;
pub use sdbp_get_path_app_patch::*;
pub use sdbp_get_path_app_patch_pre_rs3::*;
pub use sdbp_get_path_appraiser::*;
pub use sdbp_get_path_custom_sdb::*;
pub use sdbp_get_path_custom_sdb_pre_rs3::*;
pub use sdbp_get_path_system::*;

pub use set_calendar_info_a::*;

pub use set_com_plus_package_install_status::*;

pub use set_console_cursor::*;
pub use set_console_font::*;
pub use set_console_hardware_state::*;
pub use set_console_icon::*;
pub use set_console_key_shortcuts::*;
pub use set_console_local_eudc::*;
pub use set_console_maximum_window_size::*;
pub use set_console_menu_close::*;
pub use set_console_os2_oem_format::*;
pub use set_console_palette::*;

pub use set_default_comm_config_a::*;
pub use set_dll_directory_a::*;
pub use set_dll_directory_w::*;

pub use set_environment_strings_a::*;

pub use set_file_attributes_transacted_w::*;
pub use set_file_bandwidth_reservation::*;

pub use set_file_short_name_a::*;
pub use set_file_short_name_w::*;
pub use set_firmware_environment_variable_a::*;
pub use set_firmware_environment_variable_w::*;
pub use set_information_job_object::*;
pub use set_io_rate_control_information_job_object::*;
pub use set_local_primary_computer_name_w::*;

pub use set_locale_info_a::*;

pub use set_mailslot_info::*;
pub use set_message_waiting_indicator::*;
pub use set_named_pipe_attribute::*;

pub use set_process_affinity_mask::*;
pub use set_process_dep_policy::*;

pub use set_process_working_set_size::*;

pub use set_system_power_state::*;
pub use set_system_time_adjustment::*;

pub use set_tape_parameters::*;
pub use set_tape_position::*;
pub use set_thread_affinity_mask::*;
pub use set_thread_execution_state::*;

pub use set_timer_queue_timer::*;
pub use set_user_geo_id::*;
pub use set_user_geo_name::*;
pub use set_volume_label_a::*;
pub use set_volume_label_w::*;
pub use set_volume_mount_point_a::*;
pub use set_volume_mount_point_w::*;

pub use show_console_cursor::*;

pub use sort_change_case::*;
pub use sort_compare_string::*;
pub use sort_find_string::*;
pub use sort_get_hash_code::*;
pub use sort_is_defined_string::*;
pub use string_cb_cat_ex_w::*;
pub use string_cb_copy_ex_w::*;
pub use string_cb_copy_w::*;
pub use string_cch_cat_w::*;
pub use string_cch_copy_ex_w::*;
pub use string_cch_copy_nw::*;
pub use string_cch_copy_w::*;
pub use string_cch_length_w::*;
pub use string_copy_worker_a::*;
pub use string_copy_worker_w::*;
pub use string_copy_worker_w_2_0::*;
pub use string_ex_validate_dest_w::*;
pub use string_length_worker_a::*;
pub use string_length_worker_w::*;
pub use string_table::*;
pub use string_v_printf_worker_w::*;
pub use string_v_printf_worker_w_0::*;
pub use string_validate_dest_and_length_w::*;

pub use sxs_manifest::*;

pub use termsrv_convert_sys_root_to_user_dir::*;
pub use termsrv_create_reg_entry::*;
pub use termsrv_delete_key::*;
pub use termsrv_delete_value::*;
pub use termsrv_get_pre_set_value::*;
pub use termsrv_open_reg_entry::*;
pub use termsrv_open_user_classes::*;
pub use termsrv_restore_key::*;
pub use termsrv_set_value_key::*;
pub use termsrv_sync_user_ini_file::*;
pub use termsrv_sync_user_ini_file_ext::*;
pub use time_begin_period::*;
pub use time_end_period::*;
pub use time_get_dev_caps::*;
pub use time_get_time::*;
pub use time_init::*;
pub use trace_logging_register_ex__event_register__event_set_information::*;

pub use u_long_add::*;
pub use u_long_long_to_u_long::*;
pub use u_long_mult::*;
pub use unregister_application_recovery_callback::*;
pub use unregister_application_restart::*;

pub use unregister_wait::*;

pub use unregister_wait_until_oobe_completed::*;
pub use update_calendar_day_of_week::*;
pub use update_resource_a::*;
pub use update_resource_w::*;
pub use vdm_console_operation::*;
pub use vdm_operation_started::*;

pub use verify_console_io_handle::*;

pub use verify_version_info_a::*;
pub use virtual_protect::*;
pub use virtual_protect_ex::*;
pub use virtual_query::*;
pub use virtual_query_ex::*;

pub use wait_for_single_object::*;
pub use wait_named_pipe_a::*;

pub use wer_get_flags_worker::*;

pub use wer_register_file_worker::*;

pub use wer_set_flags::*;

pub use wer_unregister_file_worker::*;

pub use wer_unregister_memory_block_worker::*;
pub use werp_acquire_peb_lock::*;
pub use werp_check_ok_to_register::*;
pub use werp_get_debugger::*;
pub use werp_init_peb_store::*;
pub use werp_initiate_remote_recovery::*;
pub use werp_launch_ae_debug::*;
pub use werp_map_protection_level::*;
pub use werp_notify_use_string_resource_worker::*;
pub use werp_recovery_invoked_remotely::*;
pub use wide_char_to_multi_byte::*;
pub use win_exec::*;

pub use wow64_enable_wow64_fs_redirection::*;
pub use wow64_get_thread_selector_entry::*;

pub use wow64_suspend_thread::*;
pub use wow64_system_service_call::*;
pub use wow64_transition_resolve::*;
pub use write_console_input_vdma::*;
pub use write_console_input_vdmw::*;
pub use write_file::*;
pub use write_private_profile_section_a::*;
pub use write_private_profile_section_w::*;
pub use write_private_profile_string_a::*;
pub use write_private_profile_string_w::*;
pub use write_private_profile_struct_a::*;
pub use write_private_profile_struct_w::*;
pub use write_process_memory::*;
pub use write_profile_section_a::*;
pub use write_profile_section_w::*;
pub use write_profile_string_a::*;
pub use write_profile_string_w::*;
pub use write_tapemark::*;

pub use zombify_act_ctx_worker::*;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = guess_api_name(emu, addr);
    match api.as_str() {
        "AddVectoredExceptionHandler" => AddVectoredExceptionHandler(emu),
        "AreFileApisANSI" => AreFileApisANSI(emu),
        "CloseHandle" => CloseHandle(emu),
        "ConnectNamedPipe" => ConnectNamedPipe(emu),
        "DeleteFileA" => DeleteFileA(emu),
        "CopyFileA" => CopyFileA(emu),
        "CopyFileW" => CopyFileW(emu),
        "CreateEventA" => CreateEventA(emu),
        "CreateFileMappingA" => CreateFileMappingA(emu),
        "CreateFileMappingW" => CreateFileMappingW(emu),
        "CreateFileW" => CreateFileW(emu),
        "CreateMutexA" => CreateMutexA(emu),
        "CreateMutexW" => CreateMutexW(emu),
        "CreateNamedPipeA" => CreateNamedPipeA(emu),
        "CreateProcessA" => CreateProcessA(emu),
        "CreateRemoteThread" => CreateRemoteThread(emu),
        "CreateThread" => CreateThread(emu),
        "CreateToolhelp32Snapshot" => CreateToolhelp32Snapshot(emu),
        "CryptCreateHash" => CryptCreateHash(emu),
        "DecodePointer" => DecodePointer(emu),
        "DisconnectNamedPipe" => DisconnectNamedPipe(emu),
        "EncodePointer" => EncodePointer(emu),
        "EnterCriticalSection" => EnterCriticalSection(emu),
        "ExitProcess" => ExitProcess(emu),
        "ExpandEnvironmentStringsA" => ExpandEnvironmentStringsA(emu),
        "ExpandEnvironmentStringsW" => ExpandEnvironmentStringsW(emu),
        "FileTimeToDosDateTime" => FileTimeToDosDateTime(emu),
        "FileTimeToLocalFileTime" => FileTimeToLocalFileTime(emu),
        "FileTimeToSystemTime" => FileTimeToSystemTime(emu),
        "FindClose" => FindClose(emu),
        "FindFirstFileA" => FindFirstFileA(emu),
        "FindFirstFileW" => FindFirstFileW(emu),
        "FindNextFileA" => FindNextFileA(emu),
        "FindNextFileW" => FindNextFileW(emu),
        "FindResourceA" => FindResourceA(emu),
        "FindResourceW" => FindResourceW(emu),
        "FlsAlloc" => FlsAlloc(emu),
        "FlsGetValue" => FlsGetValue(emu),
        "FlsSetValue" => FlsSetValue(emu),
        "FreeLibrary" => FreeLibrary(emu),
        "FreeResource" => FreeResource(emu),
        "GetACP" => GetACP(emu),
        "GetThreadId" => GetThreadId(emu),
        "GetCommandLineA" => GetCommandLineA(emu),
        "GetCommandLineW" => GetCommandLineW(emu),
        "GetComputerNameA" => GetComputerNameA(emu),
        "GetCPInfo" => GetCPInfo(emu),
        "GetCurrentDirectoryA" => GetCurrentDirectoryA(emu),
        "GetCurrentDirectoryW" => GetCurrentDirectoryW(emu),
        "GetCurrentProcess" => GetCurrentProcess(emu),
        "GetCurrentProcessId" => GetCurrentProcessId(emu),
        "GetDiskFreeSpaceA" => GetDiskFreeSpaceA(emu),
        "GetCurrentThreadId" => GetCurrentThreadId(emu),
        "GetEnvironmentStrings" => GetEnvironmentStrings(emu),
        "GetEnvironmentStringsW" => GetEnvironmentStringsW(emu),
        "GetFileAttributesA" => GetFileAttributesA(emu),
        "GetFileAttributesW" => GetFileAttributesW(emu),
        "GetFileType" => GetFileType(emu),
        "GetFullPathNameA" => GetFullPathNameA(emu),
        "GetFullPathNameW" => GetFullPathNameW(emu),
        "GetLastError" => GetLastError(emu),
        "GetLogicalDrives" => GetLogicalDrives(emu),
        "GetLocalTime" => GetLocalTime(emu),
        "GetLongPathNameW" => GetLongPathNameW(emu),
        "GetModuleFileNameA" => GetModuleFileNameA(emu),
        "GetModuleFileNameW" => GetModuleFileNameW(emu),
        "GetModuleHandleA" => GetModuleHandleA(emu),
        "GetModuleHandleW" => GetModuleHandleW(emu),
        "GetNativeSystemInfo" => GetNativeSystemInfo(emu),
        "GetOEMCP" => GetOEMCP(emu),
        "GetProcAddress" => GetProcAddress(emu),
        "GetProcessAffinityMask" => GetProcessAffinityMask(emu),
        "GetProcessHeap" => GetProcessHeap(emu),
        "GetStartupInfoA" => GetStartupInfoA(emu),
        "GetStartupInfoW" => GetStartupInfoW(emu),
        "GetStdHandle" => GetStdHandle(emu),
        "GetStringTypeW" => GetStringTypeW(emu),
        "GetSystemDirectoryA" => GetSystemDirectoryA(emu),
        "GetSystemDirectoryW" => GetSystemDirectoryW(emu),
        "GetSystemInfo" => GetSystemInfo(emu),
        "GetSystemTime" => GetSystemTime(emu),
        "GetSystemTimeAsFileTime" => GetSystemTimeAsFileTime(emu),
        "GetSystemWindowsDirectoryA" => GetSystemWindowsDirectoryA(emu),
        "GetSystemWindowsDirectoryW" => GetSystemWindowsDirectoryW(emu),
        "GetTempPathW" => GetTempPathW(emu),
        "GetThreadContext" => GetThreadContext(emu),
        "GetThreadPreferredUILanguages" => GetThreadPreferredUILanguages(emu),
        "GetThreadUILanguage" => GetThreadUILanguage(emu),
        "GetTickCount" => GetTickCount(emu),
        "GetTimeZoneInformation" => GetTimeZoneInformation(emu),
        "GetUserDefaultLangID" => GetUserDefaultLangID(emu),
        "GetUserDefaultUILanguage" => GetUserDefaultUILanguage(emu),
        "GetVersion" => GetVersion(emu),
        "GetVersionExW" => GetVersionExW(emu),
        "GetWindowsDirectoryA" => GetWindowsDirectoryA(emu),
        "GetWindowsDirectoryW" => GetWindowsDirectoryW(emu),
        "GlobalAddAtomA" => GlobalAddAtomA(emu),
        "HeapAlloc" => HeapAlloc(emu),
        "HeapCreate" => HeapCreate(emu),
        "HeapDestroy" => HeapDestroy(emu),
        "HeapFree" => HeapFree(emu),
        "HeapSetInformation" => HeapSetInformation(emu),
        "InitializeCriticalSection" => InitializeCriticalSection(emu),
        "InitializeCriticalSectionAndSpinCount" => InitializeCriticalSectionAndSpinCount(emu),
        "InitializeCriticalSectionEx" => InitializeCriticalSectionEx(emu),
        "InterlockedIncrement" => InterlockedIncrement(emu),
        "IsDebuggerPresent" => IsDebuggerPresent(emu),
        "IsProcessorFeaturePresent" => IsProcessorFeaturePresent(emu),
        "IsValidCodePage" => IsValidCodePage(emu),
        "IsValidLocale" => IsValidLocale(emu),
        "LCMapStringW" => LCMapStringW(emu),
        "LeaveCriticalSection" => LeaveCriticalSection(emu),
        "LoadLibraryA" => LoadLibraryA(emu),
        "LoadLibraryExA" => LoadLibraryExA(emu),
        "LoadLibraryExW" => LoadLibraryExW(emu),
        "LoadLibraryW" => LoadLibraryW(emu),
        "LoadResource" => LoadResource(emu),
        "LocalAlloc" => LocalAlloc(emu),
        "LockResource" => LockResource(emu),
        "lstrcat" => lstrcat(emu),
        "lstrcpy" => lstrcpy(emu),
        "lstrlen" => lstrlen(emu),
        "MapViewOfFile" => MapViewOfFile(emu),
        "MoveFileA" => MoveFileA(emu),
        "MoveFileW" => MoveFileW(emu),
        "MultiByteToWideChar" => MultiByteToWideChar(emu),
        "OpenProcess" => OpenProcess(emu),
        "OpenProcessToken" => OpenProcessToken(emu),
        "OpenThread" => OpenThread(emu),
        "QueryPerformanceCounter" => QueryPerformanceCounter(emu),
        "RaiseException" => RaiseException(emu),
        "ReadFile" => ReadFile(emu),
        "ReadProcessMemory" => ReadProcessMemory(emu),
        "RegCloseKey" => RegCloseKey(emu),
        "RegCreateKeyExA" => RegCreateKeyExA(emu),
        "RegCreateKeyExW" => RegCreateKeyExW(emu),
        "RegOpenKeyA" => RegOpenKeyA(emu),
        "RegOpenKeyExW" => RegOpenKeyExW(emu),
        "RegOpenKeyW" => RegOpenKeyW(emu),
        "RegSetValueExA" => RegSetValueExA(emu),
        "RegSetValueExW" => RegSetValueExW(emu),
        "ResumeThread" => ResumeThread(emu),
        "SetErrorMode" => SetErrorMode(emu),
        "SetFilePointer" => SetFilePointer(emu),
        "SetHandleCount" => SetHandleCount(emu),
        "SetLastError" => SetLastError(emu),
        "SetThreadContext" => SetThreadContext(emu),
        "SetThreadLocale" => SetThreadLocale(emu),
        "SetUnhandledExceptionFilter" => SetUnhandledExceptionFilter(emu),
        "SizeofResource" => SizeofResource(emu),
        "Sleep" => Sleep(emu),
        "SystemTimeToTzSpecificLocalTime" => SystemTimeToTzSpecificLocalTime(emu),
        "TerminateProcess" => TerminateProcess(emu),
        "Thread32First" => Thread32First(emu),
        "Thread32Next" => Thread32Next(emu),
        "TlsAlloc" => TlsAlloc(emu),
        "TlsFree" => TlsFree(emu),
        "TlsGetValue" => TlsGetValue(emu),
        "TlsSetValue" => TlsSetValue(emu),
        "UnhandledExceptionFilter" => UnhandledExceptionFilter(emu),
        "VerifyVersionInfoW" => VerifyVersionInfoW(emu),
        "VirtualAlloc" => VirtualAlloc(emu),
        "VirtualAllocEx" => VirtualAllocEx(emu),
        "VirtualAllocExNuma" => VirtualAllocExNuma(emu),
        "VirtualLock" => VirtualLock(emu),
        "VirtualFree" => VirtualFree(emu),

        "VirtualProtect" => VirtualProtect(emu),
        "VirtualProtectEx" => VirtualProtectEx(emu),
        "VirtualQuery" => VirtualQuery(emu),
        "VirtualQueryEx" => VirtualQueryEx(emu),
        "WaitForSingleObject" => WaitForSingleObject(emu),
        "WideCharToMultiByte" => WideCharToMultiByte(emu),
        "WinExec" => WinExec(emu),
        "WriteFile" => WriteFile(emu),
        "WriteProcessMemory" => WriteProcessMemory(emu),
        "_TlgCreateWsz" => _TlgCreateWsz(emu),
        "_lclose" => _lclose(emu),
        "_lcreat" => _lcreat(emu),
        "_llseek" => _llseek(emu),
        "_lopen" => _lopen(emu),
        "_lread" => _lread(emu),
        "_lwrite" => _lwrite(emu),
        "ActivateActCtxWorker" => ActivateActCtxWorker(emu),
        "AddAtomA" => AddAtomA(emu),
        "AddAtomW" => AddAtomW(emu),
        "AddIntegrityLabelToBoundaryDescriptor" => AddIntegrityLabelToBoundaryDescriptor(emu),
        "AddLocalAlternateComputerNameW" => AddLocalAlternateComputerNameW(emu),
        "AdjustCalendarDate" => AdjustCalendarDate(emu),
        "ApplicationRecoveryFinished" => ApplicationRecoveryFinished(emu),
        "ApplicationRecoveryInProgress" => ApplicationRecoveryInProgress(emu),
        "BackupRead" => BackupRead(emu),
        "BackupSeek" => BackupSeek(emu),
        "BackupWrite" => BackupWrite(emu),
        "BaseCheckAppcompatCacheExWorker" => BaseCheckAppcompatCacheExWorker(emu),
        "BaseCheckAppcompatCacheWorker" => BaseCheckAppcompatCacheWorker(emu),
        "BaseCheckElevation" => BaseCheckElevation(emu),
        "BaseCleanupAppcompatCacheSupportWorker" => BaseCleanupAppcompatCacheSupportWorker(emu),
        "BaseDestroyVDMEnvironment" => BaseDestroyVDMEnvironment(emu),
        "BaseDllInitialize" => BaseDllInitialize(emu),
        "BaseDllReadWriteIniFile" => BaseDllReadWriteIniFile(emu),
        "BaseExitThreadPoolThread" => BaseExitThreadPoolThread(emu),
        "BaseFlushAppcompatCacheWorker" => BaseFlushAppcompatCacheWorker(emu),
        "BaseFreeAppCompatDataForProcessWorker" => BaseFreeAppCompatDataForProcessWorker(emu),
        "BaseGenerateAppCompatData" => BaseGenerateAppCompatData(emu),
        "BaseIsDosApplication" => BaseIsDosApplication(emu),
        "BaseReadAppCompatDataForProcessWorker" => BaseReadAppCompatDataForProcessWorker(emu),
        "BaseSetLastNTError" => BaseSetLastNTError(emu),
        "BaseUpdateAppcompatCacheWorker" => BaseUpdateAppcompatCacheWorker(emu),
        "BaseUpdateVDMEntry" => BaseUpdateVDMEntry(emu),
        "BaseVerifyUnicodeString" => BaseVerifyUnicodeString(emu),
        "BaseWriteErrorElevationRequiredEvent" => BaseWriteErrorElevationRequiredEvent(emu),
        "Basep8BitStringToDynamicUnicodeString" => Basep8BitStringToDynamicUnicodeString(emu),
        "BasepAccumulateIoRateControlInformationBufferSize" => {
            BasepAccumulateIoRateControlInformationBufferSize(emu)
        }
        "BasepAllocateActivationContextActivationBlock" => {
            BasepAllocateActivationContextActivationBlock(emu)
        }
        "BasepAnsiStringToDynamicUnicodeString" => BasepAnsiStringToDynamicUnicodeString(emu),
        "BasepAppContainerEnvironmentExtension" => BasepAppContainerEnvironmentExtension(emu),
        "BasepAppXExtension" => BasepAppXExtension(emu),
        "BasepCheckAppCompat" => BasepCheckAppCompat(emu),
        "BasepCheckWebBladeHashes" => BasepCheckWebBladeHashes(emu),
        "BasepCheckWinSaferRestrictions" => BasepCheckWinSaferRestrictions(emu),
        "BasepConfigureAppCertDlls" => BasepConfigureAppCertDlls(emu),
        "BasepConstructSxsCreateProcessMessage" => BasepConstructSxsCreateProcessMessage(emu),
        "BasepCopyEncryption" => BasepCopyEncryption(emu),
        "BasepFreeActivationContextActivationBlock" => {
            BasepFreeActivationContextActivationBlock(emu)
        }
        "BasepFreeAppCompatData" => BasepFreeAppCompatData(emu),
        "BasepGetComputerNameFromNtPath" => BasepGetComputerNameFromNtPath(emu),
        "BasepGetExeArchType" => BasepGetExeArchType(emu),
        "BasepGetMiniVersionForCreate" => BasepGetMiniVersionForCreate(emu),
        "BasepInitAppCompatData" => BasepInitAppCompatData(emu),
        "BasepInitializeApphelpGlobals" => BasepInitializeApphelpGlobals(emu),
        "BasepInitializeTermsrvFpns" => BasepInitializeTermsrvFpns(emu),
        "BasepIsProcessAllowed" => BasepIsProcessAllowed(emu),
        "BasepIsTestSigningEnabled" => BasepIsTestSigningEnabled(emu),
        "BasepNotifyLoadStringResource" => BasepNotifyLoadStringResource(emu),
        "BasepPostSuccessAppXExtension" => BasepPostSuccessAppXExtension(emu),
        "BasepProcessInvalidImage" => BasepProcessInvalidImage(emu),
        "BasepQueryAppCompat" => BasepQueryAppCompat(emu),
        "BasepRegenerateActCtxWithLanguage" => BasepRegenerateActCtxWithLanguage(emu),
        "BasepReleaseSxsCreateProcessUtilityStruct" => {
            BasepReleaseSxsCreateProcessUtilityStruct(emu)
        }
        "BasepReportFault" => BasepReportFault(emu),
        "BasepSetFileEncryptionCompression" => BasepSetFileEncryptionCompression(emu),
        "BasepSxsCreateStreams" => BasepSxsCreateStreams(emu),
        "BasepTpIoCallback" => BasepTpIoCallback(emu),
        "BasepTpIoCleanupCallback" => BasepTpIoCleanupCallback(emu),
        "BasepTpIoFinalizationCallback" => BasepTpIoFinalizationCallback(emu),
        "BeepImplementation" => BeepImplementation(emu),
        "BeginUpdateResourceA" => BeginUpdateResourceA(emu),
        "BeginUpdateResourceW" => BeginUpdateResourceW(emu),
        "BindIoCompletionCallback" => BindIoCompletionCallback(emu),
        "BuildCommDCBAndTimeoutsA" => BuildCommDCBAndTimeoutsA(emu),
        "BuildCommDCBAndTimeoutsW" => BuildCommDCBAndTimeoutsW(emu),
        "BuildCommDCBA" => BuildCommDCBA(emu),
        "BuildCommDCBW" => BuildCommDCBW(emu),
        "ByteMatch" => ByteMatch(emu),
        "CalibrateTimer" => CalibrateTimer(emu),
        "CallNamedPipeA" => CallNamedPipeA(emu),
        "CancelTimerQueueTimer" => CancelTimerQueueTimer(emu),
        "CheckElevation" => CheckElevation(emu),
        "CheckForReadOnlyResource" => CheckForReadOnlyResource(emu),
        "CheckNameLegalDOS8Dot3A" => CheckNameLegalDOS8Dot3A(emu),
        "CheckNameLegalDOS8Dot3W" => CheckNameLegalDOS8Dot3W(emu),
        "CloseConsoleHandle" => CloseConsoleHandle(emu),
        "CommConfigDialogA" => CommConfigDialogA(emu),
        "CompareCalendarDates" => CompareCalendarDates(emu),
        "CompatCacheLookupExe" => CompatCacheLookupExe(emu),
        "ConsoleMenuControl" => ConsoleMenuControl(emu),
        "ConvertCalDateTimeToSystemTime" => ConvertCalDateTimeToSystemTime(emu),
        "ConvertNLSDayOfWeekToWin32DayOfWeek" => ConvertNLSDayOfWeekToWin32DayOfWeek(emu),
        "ConvertSystemTimeToCalDateTime" => ConvertSystemTimeToCalDateTime(emu),
        "CopyFileExA" => CopyFileExA(emu),
        "CopyFileTransactedA" => CopyFileTransactedA(emu),
        "CopyFileTransactedW" => CopyFileTransactedW(emu),
        "CopyLZFile" => CopyLZFile(emu),
        "CptpQuirksInitOnce" => CptpQuirksInitOnce(emu),
        "CreateActCtxA" => CreateActCtxA(emu),
        "CreateActCtxWWorker" => CreateActCtxWWorker(emu),
        "CreateBoundaryDescriptorA" => CreateBoundaryDescriptorA(emu),
        "CreateDirectoryExA" => CreateDirectoryExA(emu),
        "CreateDirectoryTransactedA" => CreateDirectoryTransactedA(emu),
        "CreateDirectoryTransactedW" => CreateDirectoryTransactedW(emu),
        "CreateFileMappingNumaA" => CreateFileMappingNumaA(emu),
        "CreateHardLinkTransactedA" => CreateHardLinkTransactedA(emu),
        "CreateSemaphoreA" => CreateSemaphoreA(emu),
        "CreateSemaphoreExA" => CreateSemaphoreExA(emu),
        "CreateSocketHandle" => CreateSocketHandle(emu),
        "CreateSymbolicLinkA" => CreateSymbolicLinkA(emu),
        "CreateSymbolicLinkTransactedA" => CreateSymbolicLinkTransactedA(emu),
        "CreateSymbolicLinkTransactedW" => CreateSymbolicLinkTransactedW(emu),
        "CreateTapePartition" => CreateTapePartition(emu),
        "CreateWaitableTimerA" => CreateWaitableTimerA(emu),
        "CreateWaitableTimerExA" => CreateWaitableTimerExA(emu),
        "CreateWaitableTimerW" => CreateWaitableTimerW(emu),
        "DeactivateActCtxWorker" => DeactivateActCtxWorker(emu),
        "DebugSetProcessKillOnExit" => DebugSetProcessKillOnExit(emu),
        "DefineDosDeviceA" => DefineDosDeviceA(emu),
        "DeleteAtom" => DeleteAtom(emu),
        "DeleteFileTransactedW" => DeleteFileTransactedW(emu),
        "DeleteTimerQueue" => DeleteTimerQueue(emu),
        "DeleteVolumeMountPointA" => DeleteVolumeMountPointA(emu),
        "DeviceIoControlImplementation" => DeviceIoControlImplementation(emu),
        "DeviceNameCompare" => DeviceNameCompare(emu),
        "DisableThreadProfiling" => DisableThreadProfiling(emu),
        "DnsHostnameToComputerNameA" => DnsHostnameToComputerNameA(emu),
        "DnsHostnameToComputerNameW" => DnsHostnameToComputerNameW(emu),
        "DosDateTimeToFileTime" => DosDateTimeToFileTime(emu),
        "DosPathToSessionPathA" => DosPathToSessionPathA(emu),
        "DosPathToSessionPathW" => DosPathToSessionPathW(emu),
        "DuplicateEncryptionInfoFileExt" => DuplicateEncryptionInfoFileExt(emu),
        "EnableThreadProfiling" => EnableThreadProfiling(emu),
        "EndUpdateResourceA" => EndUpdateResourceA(emu),
        "EndUpdateResourceW" => EndUpdateResourceW(emu),
        "EnumCalendarInfoA" => EnumCalendarInfoA(emu),
        "EnumCalendarInfoExA" => EnumCalendarInfoExA(emu),
        "EnumDateFormatsA" => EnumDateFormatsA(emu),
        "EnumDateFormatsExA" => EnumDateFormatsExA(emu),
        "EnumLanguageGroupLocalesA" => EnumLanguageGroupLocalesA(emu),
        "EnumSystemCodePagesA" => EnumSystemCodePagesA(emu),
        "EnumSystemGeoID" => EnumSystemGeoID(emu),
        "EnumSystemGeoNames" => EnumSystemGeoNames(emu),
        "EnumSystemLanguageGroupsA" => EnumSystemLanguageGroupsA(emu),
        "EnumTimeFormatsA" => EnumTimeFormatsA(emu),
        "EnumUILanguagesA" => EnumUILanguagesA(emu),
        "EnumerateLocalComputerNamesA" => EnumerateLocalComputerNamesA(emu),
        "EnumerateLocalComputerNamesW" => EnumerateLocalComputerNamesW(emu),
        "EraseTape" => EraseTape(emu),
        "FatalExit" => FatalExit(emu),
        "FindActCtxSectionGuidWorker" => FindActCtxSectionGuidWorker(emu),
        "FindActCtxSectionStringA" => FindActCtxSectionStringA(emu),
        "FindActCtxSectionStringWWorker" => FindActCtxSectionStringWWorker(emu),
        "FindAtomA" => FindAtomA(emu),
        "FindAtomW" => FindAtomW(emu),
        "FindFirstVolumeA" => FindFirstVolumeA(emu),
        "FindFirstVolumeMountPointA" => FindFirstVolumeMountPointA(emu),
        "FindFirstVolumeMountPointW" => FindFirstVolumeMountPointW(emu),
        "FindNextVolumeA" => FindNextVolumeA(emu),
        "FindNextVolumeMountPointA" => FindNextVolumeMountPointA(emu),
        "FindNextVolumeMountPointW" => FindNextVolumeMountPointW(emu),
        "FindVolumeMountPointClose" => FindVolumeMountPointClose(emu),
        "FoldStringA" => FoldStringA(emu),
        "FreeMemoryJobObject" => FreeMemoryJobObject(emu),
        "GetActiveProcessorCount" => GetActiveProcessorCount(emu),
        "GetActiveProcessorGroupCount" => GetActiveProcessorGroupCount(emu),
        "GetApplicationRestartSettingsWorker" => GetApplicationRestartSettingsWorker(emu),
        "GetAtomNameA" => GetAtomNameA(emu),
        "GetAtomNameW" => GetAtomNameW(emu),
        "GetBinaryTypeW" => GetBinaryTypeW(emu),
        "GetCalendarDateFormat" => GetCalendarDateFormat(emu),
        "GetCalendarDateFormatEx" => GetCalendarDateFormatEx(emu),
        "GetCalendarDaysInMonth" => GetCalendarDaysInMonth(emu),
        "GetCalendarDifferenceInDays" => GetCalendarDifferenceInDays(emu),
        "GetCalendarInfoA" => GetCalendarInfoA(emu),
        "GetCalendarMonthsInYear" => GetCalendarMonthsInYear(emu),
        "GetCalendarSupportedDateRange" => GetCalendarSupportedDateRange(emu),
        "GetCalendarWeekNumber" => GetCalendarWeekNumber(emu),
        "GetComPlusPackageInstallStatus" => GetComPlusPackageInstallStatus(emu),
        "GetCompatFlags" => GetCompatFlags(emu),
        "GetCompressedFileSizeTransactedW" => GetCompressedFileSizeTransactedW(emu),
        "GetComputerNameW" => GetComputerNameW(emu),
        "GetConfigDialogName" => GetConfigDialogName(emu),
        "GetConsoleCharType" => GetConsoleCharType(emu),
        "GetConsoleFontInfo" => GetConsoleFontInfo(emu),
        "GetConsoleHardwareState" => GetConsoleHardwareState(emu),
        "GetConsoleInputWaitHandle" => GetConsoleInputWaitHandle(emu),
        "GetConsoleKeyboardLayoutNameA" => GetConsoleKeyboardLayoutNameA(emu),
        "GetConsoleKeyboardLayoutNameW" => GetConsoleKeyboardLayoutNameW(emu),
        "GetCorruptDetectionState" => GetCorruptDetectionState(emu),
        "GetCurrencyFormatA" => GetCurrencyFormatA(emu),
        "GetCurrencyFormatW" => GetCurrencyFormatW(emu),
        "GetCurrentActCtxWorker" => GetCurrentActCtxWorker(emu),
        "GetCurrentJaEraIndex" => GetCurrentJaEraIndex(emu),
        "GetCurrentThread" => GetCurrentThread(emu),
        "GetDateFormatAWorker" => GetDateFormatAWorker(emu),
        "GetDateFormatWWorker" => GetDateFormatWWorker(emu),
        "GetDefaultCommConfigA" => GetDefaultCommConfigA(emu),
        "GetDefaultCommConfigW" => GetDefaultCommConfigW(emu),
        "GetDevicePowerState" => GetDevicePowerState(emu),
        "GetDllDirectoryA" => GetDllDirectoryA(emu),
        "GetDllDirectoryW" => GetDllDirectoryW(emu),
        "GetDurationFormat" => GetDurationFormat(emu),
        "GetEncryptedFileVersionExt" => GetEncryptedFileVersionExt(emu),
        "GetExitCodeProcessImplementation" => GetExitCodeProcessImplementation(emu),
        "GetExpandedNameA" => GetExpandedNameA(emu),
        "GetExpandedNameW" => GetExpandedNameW(emu),
        "GetFileAttributesTransactedA" => GetFileAttributesTransactedA(emu),
        "GetFileBandwidthReservation" => GetFileBandwidthReservation(emu),
        "GetFirmwareEnvironmentVariableA" => GetFirmwareEnvironmentVariableA(emu),
        "GetFirmwareEnvironmentVariableW" => GetFirmwareEnvironmentVariableW(emu),
        "GetFirmwareType" => GetFirmwareType(emu),
        "GetFriendlyMatchComm" => GetFriendlyMatchComm(emu),
        "GetFullPathNameTransactedA" => GetFullPathNameTransactedA(emu),
        "GetFullPathNameTransactedW" => GetFullPathNameTransactedW(emu),
        "GetGeoInfoA" => GetGeoInfoA(emu),
        "GetGeoInfoEx" => GetGeoInfoEx(emu),
        "GetGeoInfoW" => GetGeoInfoW(emu),
        "GetHandleContext" => GetHandleContext(emu),
        "GetJapaneseEraItems" => GetJapaneseEraItems(emu),
        "GetKnownJapanEraCount" => GetKnownJapanEraCount(emu),
        "GetLongPathNameA" => GetLongPathNameA(emu),
        "GetMailslotInfo" => GetMailslotInfo(emu),
        "GetMaxJapanEras" => GetMaxJapanEras(emu),
        "GetMaxJapanYears" => GetMaxJapanYears(emu),
        "GetMaximumProcessorCount" => GetMaximumProcessorCount(emu),
        "GetMaximumProcessorGroupCount" => GetMaximumProcessorGroupCount(emu),
        "GetNamedPipeClientComputerNameA" => GetNamedPipeClientComputerNameA(emu),
        "GetNamedPipeClientProcessId" => GetNamedPipeClientProcessId(emu),
        "GetNamedPipeClientSessionId" => GetNamedPipeClientSessionId(emu),
        "GetNamedPipeHandleStateA" => GetNamedPipeHandleStateA(emu),
        "GetNamedPipeServerSessionId" => GetNamedPipeServerSessionId(emu),
        "GetNumaAvailableMemoryNode" => GetNumaAvailableMemoryNode(emu),
        "GetNumaAvailableMemoryNodeEx" => GetNumaAvailableMemoryNodeEx(emu),
        "GetNumaNodeNumberFromHandle" => GetNumaNodeNumberFromHandle(emu),
        "GetNumaNodeProcessorMask" => GetNumaNodeProcessorMask(emu),
        "GetNumaProcessorNode" => GetNumaProcessorNode(emu),
        "GetNumaProcessorNodeEx" => GetNumaProcessorNodeEx(emu),
        "GetNumaProximityNode" => GetNumaProximityNode(emu),
        "GetNumberFormatA" => GetNumberFormatA(emu),
        "GetNumberFormatW" => GetNumberFormatW(emu),
        "GetNumberOfConsoleFonts" => GetNumberOfConsoleFonts(emu),
        "GetPrivateProfileIntA" => GetPrivateProfileIntA(emu),
        "GetPrivateProfileIntW" => GetPrivateProfileIntW(emu),
        "GetPrivateProfileSectionA" => GetPrivateProfileSectionA(emu),
        "GetPrivateProfileSectionNamesA" => GetPrivateProfileSectionNamesA(emu),
        "GetPrivateProfileSectionNamesW" => GetPrivateProfileSectionNamesW(emu),
        "GetPrivateProfileSectionW" => GetPrivateProfileSectionW(emu),
        "GetPrivateProfileStringA" => GetPrivateProfileStringA(emu),
        "GetPrivateProfileStringW" => GetPrivateProfileStringW(emu),
        "GetPrivateProfileStructA" => GetPrivateProfileStructA(emu),
        "GetPrivateProfileStructW" => GetPrivateProfileStructW(emu),
        "GetProcessDEPPolicy" => GetProcessDEPPolicy(emu),
        "GetProcessIoCounters" => GetProcessIoCounters(emu),
        "GetProcessWorkingSetSize" => GetProcessWorkingSetSize(emu),
        "GetProfileIntA" => GetProfileIntA(emu),
        "GetProfileIntW" => GetProfileIntW(emu),
        "GetProfileSectionA" => GetProfileSectionA(emu),
        "GetProfileSectionW" => GetProfileSectionW(emu),
        "GetProfileStringA" => GetProfileStringA(emu),
        "GetProfileStringW" => GetProfileStringW(emu),
        "GetShortPathNameA" => GetShortPathNameA(emu),
        "GetShortPathNameW" => GetShortPathNameW(emu),
        "GetSystemDEPPolicy" => GetSystemDEPPolicy(emu),
        "GetSystemPowerStatus" => GetSystemPowerStatus(emu),
        "GetSystemRegistryQuota" => GetSystemRegistryQuota(emu),
        "GetTapeParameters" => GetTapeParameters(emu),
        "GetTapePosition" => GetTapePosition(emu),
        "GetTapeStatus" => GetTapeStatus(emu),
        "GetThreadSelectorEntry" => GetThreadSelectorEntry(emu),
        "GetTimeFormatWWorker" => GetTimeFormatWWorker(emu),
        "GetUserDefaultGeoName" => GetUserDefaultGeoName(emu),
        "GetUserGeoID" => GetUserGeoID(emu),
        "GetVolumeNameForVolumeMountPointA" => GetVolumeNameForVolumeMountPointA(emu),
        "GetVolumePathNamesForVolumeNameA" => GetVolumePathNamesForVolumeNameA(emu),
        "GlobalAddAtomExA" => GlobalAddAtomExA(emu),
        "GlobalAddAtomExW" => GlobalAddAtomExW(emu),
        "GlobalAddAtomW" => GlobalAddAtomW(emu),
        "GlobalDeleteAtom" => GlobalDeleteAtom(emu),
        "GlobalFindAtomA" => GlobalFindAtomA(emu),
        "GlobalFindAtomW" => GlobalFindAtomW(emu),
        "GlobalFix" => GlobalFix(emu),
        "GlobalFlags" => GlobalFlags(emu),
        "GlobalGetAtomNameA" => GlobalGetAtomNameA(emu),
        "GlobalGetAtomNameW" => GlobalGetAtomNameW(emu),
        "GlobalHandle" => GlobalHandle(emu),
        "GlobalLock" => GlobalLock(emu),
        "GlobalMemoryStatus" => GlobalMemoryStatus(emu),
        "GlobalReAlloc" => GlobalReAlloc(emu),
        "GlobalSize" => GlobalSize(emu),
        "GlobalUnWire" => GlobalUnWire(emu),
        "GlobalUnfix" => GlobalUnfix(emu),
        "GlobalUnlock" => GlobalUnlock(emu),
        "GlobalWire" => GlobalWire(emu),
        "Heap32First" => Heap32First(emu),
        "Heap32ListFirst" => Heap32ListFirst(emu),
        "Heap32ListNext" => Heap32ListNext(emu),
        "Heap32Next" => Heap32Next(emu),
        "InitAtomTable" => InitAtomTable(emu),
        "InitOnceGetStringTableOffset" => InitOnceGetStringTableOffset(emu),
        "Initialize16BitCriticalSection" => Initialize16BitCriticalSection(emu),
        "InterlockedCompareExchange" => InterlockedCompareExchange(emu),
        "InterlockedDecrement" => InterlockedDecrement(emu),
        "InterlockedExchange" => InterlockedExchange(emu),
        "InterlockedExchangeAdd" => InterlockedExchangeAdd(emu),
        "InternalCoStdMarshalObject" => InternalCoStdMarshalObject(emu),
        "InvalidateConsoleDIBits" => InvalidateConsoleDIBits(emu),
        "IsBadCodePtr" => IsBadCodePtr(emu),
        "IsBadHugeReadPtr" => IsBadHugeReadPtr(emu),
        "IsBadHugeWritePtr" => IsBadHugeWritePtr(emu),
        "IsBadReadPtr" => IsBadReadPtr(emu),
        "IsBadStringPtrA" => IsBadStringPtrA(emu),
        "IsBadStringPtrW" => IsBadStringPtrW(emu),
        "IsBadWritePtr" => IsBadWritePtr(emu),
        "IsCalendarLeapDay" => IsCalendarLeapDay(emu),
        "IsCalendarLeapMonth" => IsCalendarLeapMonth(emu),
        "IsCalendarLeapYear" => IsCalendarLeapYear(emu),
        "IsCallerAdminOrSystem" => IsCallerAdminOrSystem(emu),
        "IsFusionFullySupported" => IsFusionFullySupported(emu),
        "IsOOBESupported" => IsOOBESupported(emu),
        "IsSystemLUID" => IsSystemLUID(emu),
        "IsSystemResumeAutomatic" => IsSystemResumeAutomatic(emu),
        "IsTerminalServerCompatible" => IsTerminalServerCompatible(emu),
        "IsThreadAFiber" => IsThreadAFiber(emu),
        "IsValidCalDateTime" => IsValidCalDateTime(emu),
        "LoadAppInitDllsImplementation" => LoadAppInitDllsImplementation(emu),
        "LoadStringBaseW" => LoadStringBaseW(emu),
        "LocalCompact" => LocalCompact(emu),
        "LocalFlags" => LocalFlags(emu),
        "LocalHandle" => LocalHandle(emu),
        "LocalShrink" => LocalShrink(emu),
        "LocalSize" => LocalSize(emu),
        "lstrcmpA" => lstrcmpA(emu),
        "lstrcmp" => lstrcmpA(emu),
        "lstrcmpW" => lstrcmpW(emu),
        "lstrcmpiA" => lstrcmpiA(emu),
        "LZClose" => LZClose(emu),
        "LZCopy" => LZCopy(emu),
        "LZInit" => LZInit(emu),
        "LZOpenFileA" => LZOpenFileA(emu),
        "LZOpenFileW" => LZOpenFileW(emu),
        "LZRead" => LZRead(emu),
        "LZSeek" => LZSeek(emu),
        "LZStart" => LZStart(emu),
        "MachinePrefersNumericFirstYear" => MachinePrefersNumericFirstYear(emu),
        "MoveFileExA" => MoveFileExA(emu),
        "MoveFileTransactedA" => MoveFileTransactedA(emu),
        "MoveFileTransactedW" => MoveFileTransactedW(emu),
        "MulDiv" => MulDiv(emu),
        "NonInstallerDefault" => NonInstallerDefault(emu),
        "NotifyUILanguageChange" => NotifyUILanguageChange(emu),
        "NtVdm64CreateProcessInternalW" => NtVdm64CreateProcessInternalW(emu),
        "NtWow64ConsoleLaunchServerProcess" => NtWow64ConsoleLaunchServerProcess(emu),
        "NtWow64CsrBaseCheckRunApp" => NtWow64CsrBaseCheckRunApp(emu),
        "NtWow64CsrBasepCreateActCtx" => NtWow64CsrBasepCreateActCtx(emu),
        "NtWow64CsrBasepCreateProcess" => NtWow64CsrBasepCreateProcess(emu),
        "NtWow64CsrBasepDefineDosDevice" => NtWow64CsrBasepDefineDosDevice(emu),
        "NtWow64CsrBasepGetProcessShutdownParam" => NtWow64CsrBasepGetProcessShutdownParam(emu),
        "NtWow64CsrBasepNlsGetUserInfo" => NtWow64CsrBasepNlsGetUserInfo(emu),
        "NtWow64CsrBasepNlsUpdateCacheCount" => NtWow64CsrBasepNlsUpdateCacheCount(emu),
        "NtWow64CsrBasepRefreshIniFileMapping" => NtWow64CsrBasepRefreshIniFileMapping(emu),
        "NtWow64CsrBasepSetClientTimeZoneInformation" => {
            NtWow64CsrBasepSetClientTimeZoneInformation(emu)
        }
        "NtWow64CsrBasepSetProcessShutdownParam" => NtWow64CsrBasepSetProcessShutdownParam(emu),
        "NtWow64CsrBasepSoundSentryNotification" => NtWow64CsrBasepSoundSentryNotification(emu),
        "OOBEComplete" => OOBEComplete(emu),
        "OOBECompleteWnfCallback" => OOBECompleteWnfCallback(emu),
        "OOBECompleteWnfQueryCallback" => OOBECompleteWnfQueryCallback(emu),
        "OOBECompleteWnfWaitCallback" => OOBECompleteWnfWaitCallback(emu),
        "OpenFile" => OpenFile(emu),
        "OpenFileMappingA" => OpenFileMappingA(emu),
        "OpenJobObjectA" => OpenJobObjectA(emu),
        "OpenJobObjectW" => OpenJobObjectW(emu),
        "OpenMutexA" => OpenMutexA(emu),
        "OpenPrivateNamespaceA" => OpenPrivateNamespaceA(emu),
        "OpenSortIdKey" => OpenSortIdKey(emu),
        "PowerClearRequest" => PowerClearRequest(emu),
        "PowerCreateRequest" => PowerCreateRequest(emu),
        "PowerSetRequest" => PowerSetRequest(emu),
        "PrepareTape" => PrepareTape(emu),
        "PrivMoveFileIdentityW" => PrivMoveFileIdentityW(emu),
        "Process32FirstW" => Process32FirstW(emu),
        "Process32Next" => Process32Next(emu),
        "Process32NextW" => Process32NextW(emu),
        "QueryActCtxSettingsWWorker" => QueryActCtxSettingsWWorker(emu),
        "QueryActCtxWWorker" => QueryActCtxWWorker(emu),
        "QueryDosDeviceA" => QueryDosDeviceA(emu),
        "QueryInformationJobObject" => QueryInformationJobObject(emu),
        "QueryIoRateControlInformationJobObject" => QueryIoRateControlInformationJobObject(emu),
        "QueryThreadProfiling" => QueryThreadProfiling(emu),
        "QuirkGetDataWorker" => QuirkGetDataWorker(emu),
        "QuirkIsEnabled2Worker" => QuirkIsEnabled2Worker(emu),
        "QuirkIsEnabledForPackage2Worker" => QuirkIsEnabledForPackage2Worker(emu),
        "QuirkIsEnabledForPackage3Worker" => QuirkIsEnabledForPackage3Worker(emu),
        "QuirkIsEnabledForPackage4Worker" => QuirkIsEnabledForPackage4Worker(emu),
        "QuirkIsEnabledForPackageWorker" => QuirkIsEnabledForPackageWorker(emu),
        "QuirkIsEnabledForProcessWorker" => QuirkIsEnabledForProcessWorker(emu),
        "QuirkIsEnabledWorker" => QuirkIsEnabledWorker(emu),
        "RaiseInvalid16BitExeError" => RaiseInvalid16BitExeError(emu),
        "ReadThreadProfilingData" => ReadThreadProfilingData(emu),
        "RegisterApplicationRecoveryCallback" => RegisterApplicationRecoveryCallback(emu),
        "RegisterApplicationRestart" => RegisterApplicationRestart(emu),
        "RegisterConsoleIME" => RegisterConsoleIME(emu),
        "RegisterConsoleOS2" => RegisterConsoleOS2(emu),
        "RegisterConsoleVDM" => RegisterConsoleVDM(emu),
        "RegisterWaitForInputIdle" => RegisterWaitForInputIdle(emu),
        "RegisterWaitUntilOOBECompleted" => RegisterWaitUntilOOBECompleted(emu),
        "RemoveDirectoryTransactedW" => RemoveDirectoryTransactedW(emu),
        "RemoveLocalAlternateComputerNameW" => RemoveLocalAlternateComputerNameW(emu),
        "RemoveSecureMemoryCacheCallback" => RemoveSecureMemoryCacheCallback(emu),
        "ReplaceFileA" => ReplaceFileA(emu),
        "ReplacePartitionUnit" => ReplacePartitionUnit(emu),
        "RequestWakeupLatency" => RequestWakeupLatency(emu),
        "ResourceDataMatch" => ResourceDataMatch(emu),
        "ReturnMem16Data" => ReturnMem16Data(emu),
        "SbpIsTraceEnabled" => SbpIsTraceEnabled(emu),
        "SdbpCheckFromVersion" => SdbpCheckFromVersion(emu),
        "SdbpCheckMatchingDevice" => SdbpCheckMatchingDevice(emu),
        "SdbpCheckMatchingDir" => SdbpCheckMatchingDir(emu),
        "SdbpCheckMatchingFiles" => SdbpCheckMatchingFiles(emu),
        "SdbpCheckMatchingRegistry" => SdbpCheckMatchingRegistry(emu),
        "SdbpCheckMatchingText" => SdbpCheckMatchingText(emu),
        "SdbpCheckMatchingWildcardFiles" => SdbpCheckMatchingWildcardFiles(emu),
        "SdbpCheckMatchingWildcardRegistry" => SdbpCheckMatchingWildcardRegistry(emu),
        "SdbpCheckOSKind" => SdbpCheckOSKind(emu),
        "SdbpCheckPackageAttributes" => SdbpCheckPackageAttributes(emu),
        "SdbpCheckRuntimePlatform" => SdbpCheckRuntimePlatform(emu),
        "SdbpCheckUptoVersion" => SdbpCheckUptoVersion(emu),
        "SdbpCheckVersion" => SdbpCheckVersion(emu),
        "SdbpGetPathAppPatch" => SdbpGetPathAppPatch(emu),
        "SdbpGetPathAppPatchPreRS3" => SdbpGetPathAppPatchPreRS3(emu),
        "SdbpGetPathAppraiser" => SdbpGetPathAppraiser(emu),
        "SdbpGetPathCustomSdb" => SdbpGetPathCustomSdb(emu),
        "SdbpGetPathCustomSdbPreRS3" => SdbpGetPathCustomSdbPreRS3(emu),
        "SdbpGetPathSystem" => SdbpGetPathSystem(emu),
        "SetCalendarInfoA" => SetCalendarInfoA(emu),
        "SetComPlusPackageInstallStatus" => SetComPlusPackageInstallStatus(emu),
        "SetConsoleCursor" => SetConsoleCursor(emu),
        "SetConsoleFont" => SetConsoleFont(emu),
        "SetConsoleHardwareState" => SetConsoleHardwareState(emu),
        "SetConsoleIcon" => SetConsoleIcon(emu),
        "SetConsoleKeyShortcuts" => SetConsoleKeyShortcuts(emu),
        "SetConsoleLocalEUDC" => SetConsoleLocalEUDC(emu),
        "SetConsoleMaximumWindowSize" => SetConsoleMaximumWindowSize(emu),
        "SetConsoleMenuClose" => SetConsoleMenuClose(emu),
        "SetConsoleOS2OemFormat" => SetConsoleOS2OemFormat(emu),
        "SetConsolePalette" => SetConsolePalette(emu),
        "SetDefaultCommConfigA" => SetDefaultCommConfigA(emu),
        "SetDllDirectoryA" => SetDllDirectoryA(emu),
        "SetDllDirectoryW" => SetDllDirectoryW(emu),
        "SetEnvironmentStringsA" => SetEnvironmentStringsA(emu),
        "SetFileAttributesTransactedW" => SetFileAttributesTransactedW(emu),
        "SetFileBandwidthReservation" => SetFileBandwidthReservation(emu),
        "SetFileShortNameA" => SetFileShortNameA(emu),
        "SetFileShortNameW" => SetFileShortNameW(emu),
        "SetFirmwareEnvironmentVariableA" => SetFirmwareEnvironmentVariableA(emu),
        "SetFirmwareEnvironmentVariableW" => SetFirmwareEnvironmentVariableW(emu),
        "SetInformationJobObject" => SetInformationJobObject(emu),
        "SetIoRateControlInformationJobObject" => SetIoRateControlInformationJobObject(emu),
        "SetLocalPrimaryComputerNameW" => SetLocalPrimaryComputerNameW(emu),
        "SetLocaleInfoA" => SetLocaleInfoA(emu),
        "SetMailslotInfo" => SetMailslotInfo(emu),
        "SetMessageWaitingIndicator" => SetMessageWaitingIndicator(emu),
        "SetNamedPipeAttribute" => SetNamedPipeAttribute(emu),
        "SetProcessAffinityMask" => SetProcessAffinityMask(emu),
        "SetProcessDEPPolicy" => SetProcessDEPPolicy(emu),
        "SetProcessWorkingSetSize" => SetProcessWorkingSetSize(emu),
        "SetSystemPowerState" => SetSystemPowerState(emu),
        "SetSystemTimeAdjustment" => SetSystemTimeAdjustment(emu),
        "SetTapeParameters" => SetTapeParameters(emu),
        "SetTapePosition" => SetTapePosition(emu),
        "SetThreadAffinityMask" => SetThreadAffinityMask(emu),
        "SetThreadExecutionState" => SetThreadExecutionState(emu),
        "SetTimerQueueTimer" => SetTimerQueueTimer(emu),
        "SetUserGeoID" => SetUserGeoID(emu),
        "SetUserGeoName" => SetUserGeoName(emu),
        "SetVolumeLabelA" => SetVolumeLabelA(emu),
        "SetVolumeLabelW" => SetVolumeLabelW(emu),
        "SetVolumeMountPointA" => SetVolumeMountPointA(emu),
        "SetVolumeMountPointW" => SetVolumeMountPointW(emu),
        "ShowConsoleCursor" => ShowConsoleCursor(emu),
        "SortChangeCase" => SortChangeCase(emu),
        "SortCompareString" => SortCompareString(emu),
        "SortFindString" => SortFindString(emu),
        "SortGetHashCode" => SortGetHashCode(emu),
        "SortIsDefinedString" => SortIsDefinedString(emu),
        "StringCbCatExW" => StringCbCatExW(emu),
        "StringCbCopyExW" => StringCbCopyExW(emu),
        "StringCbCopyW" => StringCbCopyW(emu),
        "StringCchCatW" => StringCchCatW(emu),
        "StringCchCopyExW" => StringCchCopyExW(emu),
        "StringCchCopyNW" => StringCchCopyNW(emu),
        "StringCchCopyW" => StringCchCopyW(emu),
        "StringCchLengthW" => StringCchLengthW(emu),
        "StringCopyWorkerA" => StringCopyWorkerA(emu),
        "StringCopyWorkerW" => StringCopyWorkerW(emu),
        "StringCopyWorkerW_2_0" => StringCopyWorkerW_2_0(emu),
        "StringExValidateDestW" => StringExValidateDestW(emu),
        "StringLengthWorkerA" => StringLengthWorkerA(emu),
        "StringLengthWorkerW" => StringLengthWorkerW(emu),
        "StringTable" => StringTable(emu),
        "StringVPrintfWorkerW" => StringVPrintfWorkerW(emu),
        "StringValidateDestAndLengthW" => StringValidateDestAndLengthW(emu),
        "SXSManifest" => SXSManifest(emu),
        "TermsrvConvertSysRootToUserDir" => TermsrvConvertSysRootToUserDir(emu),
        "TermsrvCreateRegEntry" => TermsrvCreateRegEntry(emu),
        "TermsrvDeleteKey" => TermsrvDeleteKey(emu),
        "TermsrvDeleteValue" => TermsrvDeleteValue(emu),
        "TermsrvGetPreSetValue" => TermsrvGetPreSetValue(emu),
        "TermsrvOpenRegEntry" => TermsrvOpenRegEntry(emu),
        "TermsrvOpenUserClasses" => TermsrvOpenUserClasses(emu),
        "TermsrvRestoreKey" => TermsrvRestoreKey(emu),
        "TermsrvSetValueKey" => TermsrvSetValueKey(emu),
        "TermsrvSyncUserIniFile" => TermsrvSyncUserIniFile(emu),
        "TermsrvSyncUserIniFileExt" => TermsrvSyncUserIniFileExt(emu),
        "timeBeginPeriod" => timeEndPeriod(emu),
        "timeEndPeriod" => timeEndPeriod(emu),
        "timeGetDevCaps" => timeGetDevCaps(emu),
        "timeGetTime" => timeGetTime(emu),
        "TimeInit" => TimeInit(emu),
        "TraceLoggingRegisterEx_EventRegister_EventSetInformation" => {
            TraceLoggingRegisterEx_EventRegister_EventSetInformation(emu)
        }
        "ULongAdd" => ULongAdd(emu),
        "ULongLongToULong" => ULongLongToULong(emu),
        "ULongMult" => ULongMult(emu),
        "UnregisterApplicationRecoveryCallback" => UnregisterApplicationRecoveryCallback(emu),
        "UnregisterApplicationRestart" => UnregisterApplicationRestart(emu),
        "UnregisterWait" => UnregisterWait(emu),
        "UnregisterWaitUntilOOBECompleted" => UnregisterWaitUntilOOBECompleted(emu),
        "UpdateCalendarDayOfWeek" => UpdateCalendarDayOfWeek(emu),
        "UpdateResourceA" => UpdateResourceA(emu),
        "UpdateResourceW" => UpdateResourceW(emu),
        "VDMConsoleOperation" => VDMConsoleOperation(emu),
        "VDMOperationStarted" => VDMOperationStarted(emu),
        "VerifyConsoleIoHandle" => VerifyConsoleIoHandle(emu),
        "VerifyVersionInfoA" => VerifyVersionInfoA(emu),
        "WaitNamedPipeA" => WaitNamedPipeA(emu),
        "WerGetFlagsWorker" => WerGetFlagsWorker(emu),
        "WerRegisterFileWorker" => WerRegisterFileWorker(emu),
        "WerSetFlags" => WerSetFlags(emu),
        "WerUnregisterFileWorker" => WerUnregisterFileWorker(emu),
        "WerUnregisterMemoryBlockWorker" => WerUnregisterMemoryBlockWorker(emu),
        "WerpAcquirePebLock" => WerpAcquirePebLock(emu),
        "WerpCheckOkToRegister" => WerpCheckOkToRegister(emu),
        "WerpGetDebugger" => WerpGetDebugger(emu),
        "WerpInitPEBStore" => WerpInitPEBStore(emu),
        "WerpInitiateRemoteRecovery" => WerpInitiateRemoteRecovery(emu),
        "WerpLaunchAeDebug" => WerpLaunchAeDebug(emu),
        "WerpMapProtectionLevel" => WerpMapProtectionLevel(emu),
        "WerpNotifyUseStringResourceWorker" => WerpNotifyUseStringResourceWorker(emu),
        "WerpRecoveryInvokedRemotely" => WerpRecoveryInvokedRemotely(emu),
        "Wow64EnableWow64FsRedirection" => Wow64EnableWow64FsRedirection(emu),
        "Wow64GetThreadSelectorEntry" => Wow64GetThreadSelectorEntry(emu),
        "Wow64SuspendThread" => Wow64SuspendThread(emu),
        "Wow64SystemServiceCall" => Wow64SystemServiceCall(emu),
        "Wow64TransitionResolve" => Wow64TransitionResolve(emu),
        "WriteConsoleInputVDMA" => WriteConsoleInputVDMA(emu),
        "WriteConsoleInputVDMW" => WriteConsoleInputVDMW(emu),
        "WritePrivateProfileSectionA" => WritePrivateProfileSectionA(emu),
        "WritePrivateProfileSectionW" => WritePrivateProfileSectionW(emu),
        "WritePrivateProfileStringA" => WritePrivateProfileStringA(emu),
        "WritePrivateProfileStringW" => WritePrivateProfileStringW(emu),
        "WritePrivateProfileStructA" => WritePrivateProfileStructA(emu),
        "WritePrivateProfileStructW" => WritePrivateProfileStructW(emu),
        "WriteProfileSectionA" => WriteProfileSectionA(emu),
        "WriteProfileSectionW" => WriteProfileSectionW(emu),
        "WriteProfileStringA" => WriteProfileStringA(emu),
        "WriteProfileStringW" => WriteProfileStringW(emu),
        "WriteTapemark" => WriteTapemark(emu),
        "ZombifyActCtxWorker" => ZombifyActCtxWorker(emu),

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

fn GetThreadId(emu: &mut Emu) {
    let hndl = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!GetThreadId bad handle parameter") as u64;

    emu.stack_pop32(false);

    for i in 0..emu.threads.len() {
        if emu.threads[i].handle == hndl {
            emu.regs_mut().rax = emu.threads[i].id;
            log_red!(
                emu,
                "kernel32!GetThreadId hndl:{} (requested handle exists and its tid {})",
                hndl,
                emu.threads[i].id
            );
            return;
        }
    }
    log_red!(emu, "kernel32!GetThreadId hndl:{} (requested handle doesn't exist, returning a fake handle for now but should return zero.)", hndl);
    emu.regs_mut().rax = 0x2c2878; // if handle not found should return zero.
}

lazy_static! {
    static ref COUNT_READ: Mutex<u32> = Mutex::new(0);
    static ref COUNT_WRITE: Mutex<u32> = Mutex::new(0);
    static ref LAST_ERROR: Mutex<u32> = Mutex::new(0);
}

/// kernel32 API ////

pub fn dump_module_iat(emu: &mut emu::Emu, module: &str) {
    let mut flink = peb32::Flink::new(emu);
    flink.load(emu);
    let first_ptr = flink.get_ptr();

    loop {
        if flink.mod_name.to_lowercase().contains(module) && flink.export_table_rva > 0 {
            for i in 0..flink.num_of_funcs {
                if flink.pe_hdr == 0 {
                    continue;
                }

                let ordinal = flink.get_function_ordinal(emu, i);
                log::info!(
                    "0x{:x} {}!{}",
                    ordinal.func_va,
                    &flink.mod_name,
                    &ordinal.func_name
                );
            }
        }
        flink.next(emu);

        if flink.get_ptr() == first_ptr {
            break;
        }
    }
}

pub fn resolve_api_name_in_module(emu: &mut emu::Emu, module: &str, name: &str) -> u64 {
    let mut flink = peb32::Flink::new(emu);
    flink.load(emu);
    let first_ptr = flink.get_ptr();

    loop {
        if flink
            .mod_name
            .to_lowercase()
            .contains(&module.to_lowercase())
        {
            if flink.export_table_rva > 0 {
                for i in 0..flink.num_of_funcs {
                    if flink.pe_hdr == 0 {
                        continue;
                    }

                    let ordinal = flink.get_function_ordinal(emu, i);
                    if ordinal.func_name == name {
                        //if ordinal.func_name.contains(name) {
                        return ordinal.func_va;
                    }
                }
            }
        }
        flink.next(emu);

        //log::info!("flink: 0x{:x} first_ptr: 0x{:x} num_of_funcs: {}", flink.get_ptr(), first_ptr, flink.num_of_funcs);

        if flink.get_ptr() == first_ptr {
            break;
        }
    }

    0 //TODO: use Option<>
}

pub fn resolve_api_addr_to_name(emu: &mut emu::Emu, addr: u64) -> String {
    let mut flink = peb32::Flink::new(emu);
    flink.load(emu);
    let first_ptr = flink.get_ptr();

    loop {
        if flink.export_table_rva > 0 {
            for i in 0..flink.num_of_funcs {
                if flink.pe_hdr == 0 {
                    continue;
                }

                let ordinal = flink.get_function_ordinal(emu, i);
                if ordinal.func_va == addr {
                    let apiname = ordinal.func_name.to_string();
                    return apiname;
                }
            }
        }
        flink.next(emu);

        if flink.get_ptr() == first_ptr {
            break;
        }
    }

    "".to_string()
}

pub fn resolve_api_name(emu: &mut emu::Emu, name: &str) -> u64 {
    let mut flink = peb32::Flink::new(emu);
    flink.load(emu);
    let first_ptr = flink.get_ptr();

    loop {
        if flink.export_table_rva > 0 {
            for i in 0..flink.num_of_funcs {
                if flink.pe_hdr == 0 {
                    continue;
                }

                let ordinal = flink.get_function_ordinal(emu, i);
                if ordinal.func_name == name {
                    //if ordinal.func_name.contains(name) {
                    return ordinal.func_va;
                }
            }
        }
        flink.next(emu);

        //log::info!("flink: 0x{:x} first_ptr: 0x{:x} num_of_funcs: {}", flink.get_ptr(), first_ptr, flink.num_of_funcs);

        if flink.get_ptr() == first_ptr {
            break;
        }
    }

    0 //TODO: use Option<>
}

pub fn search_api_name(emu: &mut emu::Emu, name: &str) -> (u64, String, String) {
    let mut flink = peb32::Flink::new(emu);
    flink.load(emu);
    let first_ptr = flink.get_ptr();

    loop {
        if flink.export_table_rva > 0 {
            for i in 0..flink.num_of_funcs {
                if flink.pe_hdr == 0 {
                    continue;
                }

                let ordinal = flink.get_function_ordinal(emu, i);
                if ordinal.func_name.contains(name) {
                    return (
                        ordinal.func_va,
                        flink.mod_name.clone(),
                        ordinal.func_name.clone(),
                    );
                }
            }
        }
        flink.next(emu);

        if flink.get_ptr() == first_ptr {
            break;
        }
    }

    (0, String::new(), String::new()) //TODO: use Option<>
}

pub fn guess_api_name(emu: &mut emu::Emu, addr: u32) -> String {
    let mut flink = peb32::Flink::new(emu);
    flink.load(emu);
    let first_ptr = flink.get_ptr();

    loop {
        //let mod_name = flink.mod_name.clone();

        if flink.export_table_rva > 0 {
            for i in 0..flink.num_of_funcs {
                if flink.pe_hdr == 0 {
                    continue;
                }

                let ordinal = flink.get_function_ordinal(emu, i);

                if ordinal.func_va == addr as u64 {
                    return ordinal.func_name.clone();
                }
            }
        }

        flink.next(emu);

        if flink.get_ptr() == first_ptr {
            break;
        }
    }

    "function not found".to_string()
}

pub fn load_library(emu: &mut emu::Emu, libname: &str) -> u64 {
    let mut dll = libname.to_string().to_lowercase();

    if dll.is_empty() {
        emu.regs_mut().rax = 0;
        return 0;
    }

    if !dll.ends_with(".dll") && !dll.ends_with(".exe") {
        dll.push_str(".dll");
    }

    let mut dll_path = emu.cfg.maps_folder.clone();
    dll_path.push('/');
    dll_path.push_str(&dll);

    match peb32::get_module_base(&dll, emu) {
        Some(base) => {
            // already linked
            /*
            if emu.cfg.verbose > 0 {
                log::info!("dll {} already linked.", dll);
            }*/
            base
        }
        None => {
            // do link
            if std::path::Path::new(dll_path.as_str()).exists() {
                let (base, pe_off) = emu.load_pe32(&dll_path, false, 0);
                peb32::dynamic_link_module(base as u64, pe_off, &dll, emu);
                base as u64
            } else {
                panic!("dll {} not found, have you loaded maps?", dll_path);
            }
        }
    }
}
