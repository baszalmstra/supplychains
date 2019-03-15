use nalgebra::{Isometry3, Perspective3};

pub struct Camera {
    transform: Isometry3<f32>,
    field_of_view: f32,
    far_plane: f32,
    near_plane: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            transform: Isometry3::identity(),
            field_of_view: 1.8,
            far_plane: 1024.0,
            near_plane: 0.1,
        }
    }
}

impl Camera {
    pub fn get_perspective(&self, aspect: f32) -> Perspective3<f32> {
        Perspective3::new(aspect, self.field_of_view, self.near_plane, self.far_plane)
    }

    pub fn transform(&self) -> &Isometry3<f32> {
        &self.transform
    }
}
