use super::super::window::*;
use super::vki::*;
use winit::EventsLoop;
pub const WINDOW_TITLE: &'static str = "When Vulkan meets Rust";

pub struct App {
    pub window: winit::Window,
    pub vki: VKI,
}

impl App {
    pub fn new(events_loop: &EventsLoop) -> App {
        let window = window_create(WINDOW_TITLE.to_string(), 800, 600, events_loop);
        return App {
            window: window,
            vki: VKI::new(),
        };
    }
}
