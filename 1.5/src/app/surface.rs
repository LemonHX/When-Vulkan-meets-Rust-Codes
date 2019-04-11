use ash::vk;
use super::super::platforms::create_surface;
pub struct SurfaceStuff {
    pub surface_loader: ash::extensions::khr::Surface,
    pub surface: vk::SurfaceKHR,
}

pub fn create_surface(
    entry: &ash::Entry,
    instance: &ash::Instance,
    window: &winit::Window,
) -> SurfaceStuff {
    let surface = unsafe {
        create_surface::create_surface(entry, instance, window)
            .expect("Failed to create surface.")
    };
    let surface_loader = ash::extensions::khr::Surface::new(entry, instance);

    SurfaceStuff {
        surface_loader,
        surface,
    }
}
