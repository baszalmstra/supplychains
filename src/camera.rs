use nalgebra::{Isometry3, Perspective3, Matrix4};

pub struct Camera {
    view: Isometry3<f32>,
    field_of_view: f32,
    far_plane: f32,
    near_plane: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            view: Isometry3::identity(),
            field_of_view: 1.2,
            far_plane: 1024.0,
            near_plane: 0.1,
        }
    }
}

impl Camera {
    pub fn get_perspective(&self, aspect: f32) -> Matrix4<f32> {
        Perspective3::new(aspect, self.field_of_view, self.near_plane, self.far_plane).into()
    }

    pub fn view(&self) -> &Isometry3<f32> {
        &self.view
    }

    pub fn set_view(&mut self, transform: Isometry3<f32>) {
        self.view = transform
    }

    pub fn view_matrix(&self) -> Matrix4<f32> {
        self.view.into()
    }
}
