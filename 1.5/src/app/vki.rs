use super::super::platforms::required_extension::*;
use super::super::utils::tools::*;
use super::surface::*;
use super::consts::*;
use super::logical_device::*;
use super::physical_device::*;
use super::validation::*;
use ash::version::DeviceV1_0;
use ash::version::EntryV1_0;
use ash::version::InstanceV1_0;
use ash::vk;
use std::ffi::CString;
use std::ptr;
pub struct VKI {
    pub entry: ash::Entry,
    pub instance: ash::Instance,
    pub debug_report: ash::extensions::ext::DebugReport,
    pub debug_callback: vk::DebugReportCallbackEXT,
    pub physical_device: vk::PhysicalDevice,
    pub logical_device: ash::Device,
    pub graphics_queue: vk::Queue,
    pub surface_stuff: SurfaceStuff,
}

impl VKI {
    pub fn new(window:&winit::Window) -> VKI {
        let entry = ash::Entry::new().unwrap();
        let instance = create_instance(
            &entry,
            WINDOW_TITLE,
            VALIDATION.is_enable,
            &VALIDATION.required_validation_layers.to_vec(),
        );
        let surface_stuff = create_surface(&entry,&instance,window);
        let (debug_report, debug_callback) = setup_debug_callback(&entry, &instance);
        let physical_device = pick_physical_device(&instance,&surface_stuff);
        let (logical_device, graphics_queue) =
            create_logical_device(&instance, physical_device, &VALIDATION);
        return VKI {
            entry: entry,
            instance: instance,
            debug_report: debug_report,
            debug_callback: debug_callback,
            physical_device: physical_device,
            logical_device:logical_device,
            graphics_queue:graphics_queue,
            surface_stuff:surface_stuff,
        };
    }
}
impl Drop for VKI {
    fn drop(&mut self) {
        unsafe {
            self.logical_device.destroy_device(None);
            self.surface_stuff.surface_loader.destroy_surface(self.surface_stuff.surface, None);
            if VALIDATION.is_enable {
                self.debug_report
                    .destroy_debug_report_callback(self.debug_callback, None);
            }
            self.instance.destroy_instance(None);
        }
    }
}
