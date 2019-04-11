use super::super::platforms::required_extension::*;
use super::super::utils::tools::*;
use super::consts::*;
use super::logical_device::*;
use super::physical_device::*;
use super::surface::*;
use super::swapchain::*;
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
    pub swapchain_stuff: SwapChainStuff,
}

impl VKI {
    pub fn new(window: &winit::Window) -> VKI {
        let entry = ash::Entry::new().unwrap();
        let instance = create_instance(
            &entry,
            WINDOW_TITLE,
            VALIDATION.is_enable,
            &VALIDATION.required_validation_layers.to_vec(),
        );
        let surface_stuff = create_surface(&entry, &instance, window);
        let (debug_report, debug_callback) = setup_debug_callback(&entry, &instance);
        let physical_device = pick_physical_device(&instance, &surface_stuff);
        let (logical_device, family_indices) =
            create_logical_device(&instance, physical_device, &VALIDATION, &surface_stuff);
        let graphics_queue =
            unsafe { logical_device.get_device_queue(family_indices.graphics_family as u32, 0) };
        let present_queue =
            unsafe { logical_device.get_device_queue(family_indices.present_family as u32, 0) };
        let swapchain_stuff = create_swapchain(
            &instance,
            &logical_device,
            physical_device,
            &surface_stuff,
            &family_indices,
        );
        return VKI {
            entry: entry,
            instance: instance,
            debug_report: debug_report,
            debug_callback: debug_callback,
            physical_device: physical_device,
            logical_device: logical_device,
            graphics_queue: graphics_queue,
            surface_stuff: surface_stuff,
            swapchain_stuff: swapchain_stuff,
        };
    }
}
impl Drop for VKI {
    fn drop(&mut self) {
        unsafe {
            self.swapchain_stuff
                .swapchain_loader
                .destroy_swapchain(self.swapchain_stuff.swapchain, None);
            self.logical_device.destroy_device(None);
            self.surface_stuff
                .surface_loader
                .destroy_surface(self.surface_stuff.surface, None);
            if VALIDATION.is_enable {
                self.debug_report
                    .destroy_debug_report_callback(self.debug_callback, None);
            }
            self.instance.destroy_instance(None);
        }
    }
}
