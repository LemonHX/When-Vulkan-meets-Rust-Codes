use super::validation::*;

use super::surface::*;
use ash::version::DeviceV1_0;
use ash::version::InstanceV1_0;
use ash::vk;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;
pub struct QueueFamilyIndices {
    pub graphics_family: i32,
    pub present_family: i32,
}

impl QueueFamilyIndices {
    pub fn new() -> QueueFamilyIndices {
        QueueFamilyIndices {
            graphics_family: -1,
            present_family: -1,
        }
    }
    pub fn is_complete(&self) -> bool {
        self.graphics_family >= 0 && self.present_family >= 0
    }
}

pub fn create_logical_device(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    validation: &ValidationInfo,
    surface_stuff: &SurfaceStuff,
) -> (ash::Device, QueueFamilyIndices) {
    let indices = find_queue_family(instance, physical_device, surface_stuff);

    use std::collections::HashSet;
    let mut unique_queue_families = HashSet::new();
    unique_queue_families.insert(indices.graphics_family as u32);
    unique_queue_families.insert(indices.present_family as u32);

    let queue_priorities = [1.0_f32];
    let mut queue_create_infos = vec![];
    for &queue_family in unique_queue_families.iter() {
        let queue_create_info = vk::DeviceQueueCreateInfo {
            s_type: vk::StructureType::DEVICE_QUEUE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::DeviceQueueCreateFlags::empty(),
            queue_family_index: queue_family,
            p_queue_priorities: queue_priorities.as_ptr(),
            queue_count: queue_priorities.len() as u32,
        };
        queue_create_infos.push(queue_create_info);
    }

    let physical_device_features = vk::PhysicalDeviceFeatures {
        ..Default::default() // default just enable no feature.
    };

    let requred_validation_layer_raw_names: Vec<CString> = validation
        .required_validation_layers
        .iter()
        .map(|layer_name| CString::new(*layer_name).unwrap())
        .collect();
    let enable_layer_names: Vec<*const c_char> = requred_validation_layer_raw_names
        .iter()
        .map(|layer_name| layer_name.as_ptr())
        .collect();

    let enable_extension_names = [
        ash::extensions::khr::Swapchain::name().as_ptr(), // currently just enable the Swapchain extension.
    ];

    let device_create_info = vk::DeviceCreateInfo {
        s_type: vk::StructureType::DEVICE_CREATE_INFO,
        p_next: ptr::null(),
        flags: vk::DeviceCreateFlags::empty(),
        queue_create_info_count: queue_create_infos.len() as u32,
        p_queue_create_infos: queue_create_infos.as_ptr(),
        enabled_layer_count: if validation.is_enable {
            enable_layer_names.len()
        } else {
            0
        } as u32,
        pp_enabled_layer_names: if validation.is_enable {
            enable_layer_names.as_ptr()
        } else {
            ptr::null()
        },
        enabled_extension_count: enable_extension_names.len() as u32,
        pp_enabled_extension_names: enable_extension_names.as_ptr(),
        p_enabled_features: &physical_device_features,
    };

    let device: ash::Device = unsafe {
        instance
            .create_device(physical_device, &device_create_info, None)
            .expect("Failed to create logical device!")
    };

    (device, indices)
}
pub fn find_queue_family(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    surface_stuff: &SurfaceStuff,
) -> QueueFamilyIndices {
    let queue_families =
        unsafe { instance.get_physical_device_queue_family_properties(physical_device) };

    let mut queue_family_indices = QueueFamilyIndices::new();

    let mut index = 0;
    for queue_family in queue_families.iter() {
        if queue_family.queue_count > 0
            && queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS)
        {
            queue_family_indices.graphics_family = index;
        }

        let is_present_support = unsafe {
            surface_stuff
                .surface_loader
                .get_physical_device_surface_support(
                    physical_device,
                    index as u32,
                    surface_stuff.surface,
                )
        };
        if queue_family.queue_count > 0 && is_present_support {
            queue_family_indices.present_family = index;
        }

        if queue_family_indices.is_complete() {
            break;
        }

        index += 1;
    }

    queue_family_indices
}
