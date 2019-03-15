use crate::states::{State, Trans};
use glium::glutin::Event;
use glium::Surface;

pub struct GameState;

impl State for GameState {
    fn handle_event(&mut self, event: Event) -> Trans {
        use glium::glutin;

        match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::CloseRequested => return Trans::Exit,
                _ => (),
            },
            _ => (),
        }

        Trans::Continue
    }

    fn draw(&mut self, display: &glium::Display) -> Trans {
        // Begin drawing to the backbuffer
        let mut surface = display.draw();

        // Clear the screen
        surface.clear_color(0.01, 0.01, 0.01, 0.01);

        // Finish drawing
        surface.finish().unwrap();

        Trans::Continue
    }
}
