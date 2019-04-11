use winit::{ControlFlow, Event, VirtualKeyCode, WindowEvent};
pub fn event_handler(event: Event) -> winit::ControlFlow {
    match event {
        // handling keyboard event
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::KeyboardInput { input, .. } => {
                if let Some(VirtualKeyCode::Escape) = input.virtual_keycode {
                    ControlFlow::Break
                } else {
                    ControlFlow::Continue
                }
            }
            WindowEvent::CloseRequested => ControlFlow::Break,

            _ => ControlFlow::Continue,
        },
        _ => ControlFlow::Continue,
    }
}
