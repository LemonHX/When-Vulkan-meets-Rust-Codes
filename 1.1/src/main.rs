use vk_00::app::app::*;
use vk_00::app::vki::*;
use vk_00::events_handler::*;
fn main() {
    let mut evl = winit::EventsLoop::new();
    let vulkan_app = App::new(&evl);
    evl.run_forever(event_handler);
}
