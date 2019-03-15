use crate::timing::Time;
use glium::glutin::Event;

mod game_state;

pub enum Trans {
    Continue,
    Exit,
}

pub trait State {
    fn on_start(&mut self, display: &glium::Display) {}
    fn on_stop(&mut self) {}

    fn handle_event(&mut self, event: Event) -> Trans {
        Trans::Continue
    }

    fn update(&mut self, time: &Time) -> Trans {
        Trans::Continue
    }

    fn draw(&mut self, display: &glium::Display) -> Trans {
        Trans::Continue
    }
}

pub use game_state::GameState;
