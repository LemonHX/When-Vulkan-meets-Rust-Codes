use ash::vk_make_version;
use super::validation::ValidationInfo;
pub const WINDOW_TITLE: &'static str = "When Vulkan meets Rust";
pub const APPLICATION_VERSION: u32 = vk_make_version!(1, 0, 0);
pub const ENGINE_VERSION: u32 = vk_make_version!(1, 0, 0);
pub const API_VERSION: u32 = vk_make_version!(1, 0, 92);

pub const VALIDATION: ValidationInfo = ValidationInfo {
    is_enable: true,
    required_validation_layers: ["VK_LAYER_LUNARG_standard_validation"],
};