use super::super::events_handler::*;
use super::vki::*;
use winit::EventsLoop;
use super::super::window::*;
pub const WINDOW_TITLE: &'static str = "When Vulkan meets Rust";

pub struct App {
    pub window: winit::Window,
    pub vki: VKI,
    pub events_loop: EventsLoop,
}

impl App {
    pub fn new() -> App {
        let ev = EventsLoop::new();
        let window = window_create(WINDOW_TITLE, 800, 600, &ev);
        return App {
            window: window,
            vki: VKI::new(),
            events_loop: ev,
        };
    }
    pub fn main_loop(&mut self) {
        self.events_loop.run_forever(event_handler);
    }
}
