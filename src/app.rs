use crate::states::{State, Trans};
use crate::timing::Time;
use glium::glutin::{self, dpi::LogicalSize};
use glium::Surface;
use std::error::Error;
use std::time::{Duration, Instant};

pub struct Application<'a> {
    events_loop: glutin::EventsLoop,
    display: glium::Display,
    state: Box<dyn State + 'a>,
}

impl<'a> Application<'a> {
    pub fn new<S: State + 'a>(initial_state: S) -> ApplicationBuilder<S> {
        ApplicationBuilder::new(initial_state)
    }

    pub fn run(&mut self) {
        // Initialise the current state
        self.state.on_start(&self.display);

        // Run the game loop
        let mut close = false;
        let mut time: Time = Default::default();
        let mut start = Instant::now();
        while !close {
            // Update the game
            self.state.update(&time);

            // Redraw the game
            self.state.draw(&self.display);

            // Handle window events
            let state = &mut self.state;
            self.events_loop
                .poll_events(|event| match state.handle_event(event) {
                    Trans::Exit => close = true,
                    _ => (),
                });

            // Update timing
            let now = Instant::now();
            time.set_delta_time(now - start);
            time.increment_frame_number();
            start = now;
        }

        // Release resources
        self.state.on_stop();
    }
}

pub struct ApplicationBuilder<S> {
    initial_state: S,
    window: glutin::WindowBuilder,
}

impl<S> ApplicationBuilder<S> {
    pub fn new(initial_state: S) -> ApplicationBuilder<S> {
        info!("Initializing Supply Chains...");
        info!("Version: {}", env!("CARGO_PKG_VERSION"));

        ApplicationBuilder {
            initial_state,
            window: glutin::WindowBuilder::new(),
        }
    }

    pub fn with_title<T: Into<String>>(mut self, title: T) -> Self {
        self.window = self.window.with_title(title);
        self
    }

    pub fn with_dimensions(mut self, dimensions: LogicalSize) -> Self {
        self.window = self.window.with_dimensions(dimensions);
        self
    }

    pub fn build<'a>(self) -> Result<Application<'a>, failure::Error>
    where
        S: State + 'a,
    {
        let events_loop = glutin::EventsLoop::new();
        let context = glutin::ContextBuilder::new()
            .with_depth_buffer(24);
        let display = glium::Display::new(self.window, context, &events_loop)
            .map_err(|e| failure::err_msg(format!("Glium error: {0}", e.description())))?;

        Ok(Application {
            events_loop,
            display,
            state: Box::new(self.initial_state),
        })
    }
}