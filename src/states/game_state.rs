use crate::{
    states::{State, Trans},
    world::{self, renderer::chunk}
};
use glium::glutin::Event;
use glium::Surface;
use crate::camera::Camera;
use nalgebra::{
    Vector3,
    geometry::{Isometry3, Translation3}
};
use nalgebra::geometry::UnitQuaternion;
use crate::timing::Time;

pub struct GameState {
    renderer: Option<chunk::Renderer>,
    meshes: Vec<chunk::Mesh>,
    camera: Camera,
}

impl State for GameState {
    fn on_start(&mut self, display: &glium::Display) {
        self.renderer = Some(chunk::Renderer::new(display).unwrap());

        self.meshes.push(chunk::Mesh::from_data(display, world::Chunk::default()).unwrap());
    }

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

    fn update(&mut self, time:&Time) -> Trans {
        let mut view = (*self.camera.view()).clone();
        view.rotation *= &UnitQuaternion::from_axis_angle(&Vector3::y_axis(), time.delta_seconds()*0.3);
        self.camera.set_view(view);
        Trans::Continue
    }

    fn draw(&mut self, display: &glium::Display) -> Trans {
        // Begin drawing to the backbuffer
        let mut target = display.draw();

        // Clear the screen
        target.clear_color(0.01, 0.01, 0.01, 0.01);
        target.clear_depth(1.0 );

        match &self.renderer {
            Some(renderer) => renderer.draw(
                &mut target,
                &self.camera,
                self.meshes.iter()),
            _ => ()
        }

        // Finish drawing
        target.finish().unwrap();

        Trans::Continue
    }
}

impl Default for GameState {
    fn default() -> Self {
        let mut camera = Camera::default();
        camera.set_view(Isometry3::look_at_rh(
            &Vector3::new(0.0, 30.0, -30.0).into(),
            &Vector3::new(0.0, 0.0, 0.0).into(),
            &Vector3::new(0.0, 1.0, 0.0),
        ));
        GameState {
            meshes: Vec::new(),
            renderer: None,
            camera,
        }
    }
}
