use super::super::platforms::required_extension::*;
use super::validation::*;
use ash::version::EntryV1_0;
use ash::version::InstanceV1_0;
use ash::vk;
use ash::vk_make_version;
use std::ffi::CString;
use std::ptr;
pub const WINDOW_TITLE: &'static str = "When Vulkan meets Rust";
pub const APPLICATION_VERSION: u32 = vk_make_version!(1, 0, 0);
pub const ENGINE_VERSION: u32 = vk_make_version!(1, 0, 0);
pub const API_VERSION: u32 = vk_make_version!(1, 0, 92);

pub struct VKI {
    pub entry: ash::Entry,
    pub instance: ash::Instance,
    pub debug_report: ash::extensions::ext::DebugReport,
    pub debug_callback: vk::DebugReportCallbackEXT,
}

impl VKI {
    pub fn new() -> VKI {
        let entry = ash::Entry::new().unwrap();
        let instance = VKI::create_instance(&entry);
        let (debug_report, debug_callback) = setup_debug_callback(&entry, &instance);
        return VKI {
            entry: entry,
            instance: instance,
            debug_report: debug_report,
            debug_callback: debug_callback,
        };
    }
    fn create_instance(entry: &ash::Entry) -> ash::Instance {
        if VALIDATION.is_enable && check_validation_layer_support(entry) == false {
            panic!("Validation layers requested, but not available!");
        }

        let app_name = CString::new(WINDOW_TITLE).unwrap();
        let engine_name = CString::new("Vulkan Engine").unwrap();
        let app_info = vk::ApplicationInfo {
            p_application_name: app_name.as_ptr(),
            s_type: vk::StructureType::APPLICATION_INFO,
            p_next: ptr::null(),
            application_version: APPLICATION_VERSION,
            p_engine_name: engine_name.as_ptr(),
            engine_version: ENGINE_VERSION,
            api_version: API_VERSION,
        };

        // VK_EXT debug report has been requested here.
        let extension_names = required_extension_names();

        let requred_validation_layer_raw_names: Vec<CString> = VALIDATION
            .required_validation_layers
            .iter()
            .map(|layer_name| CString::new(*layer_name).unwrap())
            .collect();
        let enable_layer_names: Vec<*const i8> = requred_validation_layer_raw_names
            .iter()
            .map(|layer_name| layer_name.as_ptr())
            .collect();

        let create_info = vk::InstanceCreateInfo {
            s_type: vk::StructureType::INSTANCE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::InstanceCreateFlags::empty(),
            p_application_info: &app_info,
            pp_enabled_layer_names: if VALIDATION.is_enable {
                enable_layer_names.as_ptr()
            } else {
                ptr::null()
            },
            enabled_layer_count: if VALIDATION.is_enable {
                enable_layer_names.len()
            } else {
                0
            } as u32,
            pp_enabled_extension_names: extension_names.as_ptr(),
            enabled_extension_count: extension_names.len() as u32,
        };

        let instance: ash::Instance = unsafe {
            entry
                .create_instance(&create_info, None)
                .expect("Failed to create Instance!")
        };

        instance
    }
}
impl Drop for VKI {
    fn drop(&mut self) {
        unsafe {
            if VALIDATION.is_enable {
                self.debug_report
                    .destroy_debug_report_callback(self.debug_callback, None);
            }
            self.instance.destroy_instance(None);
        }
    }
}
