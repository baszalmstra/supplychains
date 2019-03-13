pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    use glium::{glutin, Surface};

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title(format!("Supply Chains ({})", VERSION))
        .with_dimensions((1280.0, 800.0).into());
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let mut closed = false;
    while !closed {
        let mut target = display.draw();
        target.clear_color(0.02, 0.02, 0.02, 1.0);
        target.finish().unwrap();

        events_loop.poll_events(|ev| {
            match ev {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => closed = true,
                    _ => (),
                },
                _ => (),
            }
        });
    }
}
