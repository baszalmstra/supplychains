use crate::world::chunk::Chunk;
use glium::backend::Facade;
use crate::camera::Camera;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 4]
}

implement_vertex!(Vertex, position, color);

pub struct Mesh {
    vertex_buffer: glium::VertexBuffer<Vertex>,
    index_buffer: glium::IndexBuffer<u16>
}

impl Mesh {
    pub fn from_data<F:?Sized>(facade: &F, data:Chunk) -> Result<Mesh, failure::Error>
    where
        F: Facade
    {
        let vertices = vec![
            Vertex { position: [1.0, 0.0, 0.0], color: [0.0, 1.0, 0.0, 1.0] },
            Vertex { position: [0.0, 0.0, 0.0], color: [1.0, 0.0, 0.0, 1.0] },
            Vertex { position: [0.0, 1.0, 0.0], color: [0.0, 0.0, 1.0, 1.0] },
        ];

        let indices:Vec<u16> = vec![
            0,1,2
        ];

        let vertex_buffer = glium::VertexBuffer::new(facade, &vertices)?;
        let index_buffer = glium::IndexBuffer::new(
            facade,
            glium::index::PrimitiveType::TrianglesList,
            &indices)?;

        Ok(Mesh {
            vertex_buffer,
            index_buffer,
        })
    }
}

pub struct Renderer {
    program: glium::Program,
}

impl Renderer {
    pub fn new<F:?Sized>(facade: &F) -> Result<Renderer, failure::Error>
    where
        F: Facade
    {
        let vertex_shader_src = r#"
            #version 140

            in vec3 position;
            in vec4 color;

            out vec4 v_color;

            uniform mat4 projection;
            uniform mat4 view;
            uniform mat4 model;

            void main() {
                v_color = color;
                gl_Position = projection * view * vec4(position, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 140
            in vec4 v_color;

            out vec4 color;

            void main() {
                color = v_color;
            }
        "#;

        let program = glium::Program::from_source(
            facade,
            vertex_shader_src,
            fragment_shader_src,
            None)?;

        Ok(Renderer {
            program
        })
    }

    pub fn draw<'a, S, I>(&self, surface: &mut S, camera:&Camera, iter: I)
    where
        S: glium::Surface,
        I: Iterator<Item=&'a Mesh>
    {
        use nalgebra::{Matrix4};

        let proj_mat:[[f32;4];4] = camera.get_perspective(1280.0/800.0).into();
        let view_mat:[[f32;4];4] = camera.view_matrix().into();

        let uniforms = uniform! {
            projection: proj_mat,
            view: view_mat,
        };

        for chunk in iter {
            surface.draw(
                &chunk.vertex_buffer,
                &chunk.index_buffer,
                &self.program,
                &uniforms,
                &Default::default()
            ).unwrap();
        }
    }
}