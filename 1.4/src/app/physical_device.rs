use super::super::utils::tools::*;
use ash::version::InstanceV1_0;
use ash::vk;
use ash::{vk_version_major, vk_version_minor, vk_version_patch};
pub struct QueueFamilyIndices {
    graphics_family: i32,
}

impl QueueFamilyIndices {
    pub fn is_complete(&self) -> bool {
        self.graphics_family >= 0
    }
}
pub fn pick_physical_device(instance: &ash::Instance) -> vk::PhysicalDevice {
    let physical_devices = unsafe {
        instance
            .enumerate_physical_devices()
            .expect("枚举GPU失败!")
    };

    println!(
        "找到{}个Vulkan支持的设备:",
        physical_devices.len()
    );

    let mut result = None;
    for &physical_device in physical_devices.iter() {
        if is_physical_device_suitable(instance, physical_device) {
            if result.is_none() {
                result = Some(physical_device)
            }
        }
    }

    match result {
        None => panic!("找不到合适的GPU! 请检查GPU驱动安装"),
        Some(physical_device) => physical_device,
    }
}

pub fn is_physical_device_suitable(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
) -> bool {
    let device_properties = unsafe { instance.get_physical_device_properties(physical_device) };
    let device_features = unsafe { instance.get_physical_device_features(physical_device) };
    let device_queue_families =
        unsafe { instance.get_physical_device_queue_family_properties(physical_device) };

    let device_type = match device_properties.device_type {
        vk::PhysicalDeviceType::CPU => "Cpu",
        vk::PhysicalDeviceType::INTEGRATED_GPU => "集成 GPU",
        vk::PhysicalDeviceType::DISCRETE_GPU => "独立 GPU",
        vk::PhysicalDeviceType::VIRTUAL_GPU => "虚拟 GPU",
        vk::PhysicalDeviceType::OTHER => "未知设备",
        _ => panic!(),
    };

    let device_name = vk_to_string(&device_properties.device_name);
    println!(
        "\t设备名称: {}, id: {}, type: {}",
        device_name, device_properties.device_id, device_type
    );

    let major_version = vk_version_major!(device_properties.api_version);
    let minor_version = vk_version_minor!(device_properties.api_version);
    let patch_version = vk_version_patch!(device_properties.api_version);

    println!(
        "\tAPI版本: {}.{}.{}",
        major_version, minor_version, patch_version
    );

    println!("\t支持的 Queue Family: {}", device_queue_families.len());
    println!("\t\tQueue Count | Graphics, Compute, Transfer, Sparse Binding");
    for queue_family in device_queue_families.iter() {
        let is_graphics_support = if queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
            "支持"
        } else {
            "不支持"
        };
        let is_compute_support = if queue_family.queue_flags.contains(vk::QueueFlags::COMPUTE) {
            "支持"
        } else {
            "不支持"
        };;
        let is_transfer_support = if queue_family.queue_flags.contains(vk::QueueFlags::TRANSFER) {
            "支持"
        } else {
            "不支持"
        };;
        let is_sparse_support = if queue_family
            .queue_flags
            .contains(vk::QueueFlags::SPARSE_BINDING)
        {
            "支持"
        } else {
            "不支持"
        };;

        println!(
            "\t\t{}\t    | {},  {},  {},  {}",
            queue_family.queue_count,
            is_graphics_support,
            is_compute_support,
            is_transfer_support,
            is_sparse_support
        );
    }

    // there are plenty of features
    println!(
        "\t几何着色器: {}",
        if device_features.geometry_shader == 1 {
            "支持"
        } else {
            "不支持"
        }
    );

    let indices = find_queue_family(instance, physical_device);

    return indices.is_complete();
}

pub fn find_queue_family(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
) -> QueueFamilyIndices {
    let queue_families =
        unsafe { instance.get_physical_device_queue_family_properties(physical_device) };

    let mut queue_family_indices = QueueFamilyIndices {
        graphics_family: -1,
    };

    let mut index = 0;
    for queue_family in queue_families.iter() {
        if queue_family.queue_count > 0
            && queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS)
        {
            queue_family_indices.graphics_family = index;
        }

        if queue_family_indices.is_complete() {
            break;
        }

        index += 1;
    }

    queue_family_indices
}
