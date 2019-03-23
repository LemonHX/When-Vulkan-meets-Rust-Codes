use winit::EventsLoop;

pub fn window_create(
        title: String,
        width: u32,
        height: u32,
        events_loop: &EventsLoop,
) -> winit::Window {
        winit::WindowBuilder::new()
                .with_title(title)
                .with_dimensions((width, height).into())
                .build(events_loop)
                .expect("Err 窗口创建失败 原因:")
}
