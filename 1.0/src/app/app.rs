use super::super::window::*;
use winit::EventsLoop;
pub struct App {
    pub window: winit::Window,
}

impl App {
    pub fn new(events_loop: &EventsLoop) -> App {
        let window = window_create("whatever".to_string(), 800, 600, events_loop);
        return App { window: window };
    }
}
