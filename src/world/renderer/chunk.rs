use crate::world::chunk::{Chunk, CHUNK_LAYERS, CHUNK_HEIGHT, CHUNK_WIDTH};
use glium::backend::Facade;
use crate::camera::Camera;
use crate::world::voxel::{VoxelAdjacency, Voxel, VoxelFace, VOXEL_SIZE};
use glium::{DrawParameters, Depth};

#[derive(Debug, Copy, Clone)]
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
        let mut vertices:Vec<Vertex> = Vec::new();
        let mut indices:Vec<u16> = Vec::new();

        // Iterate over all blocks and create their geometry
        for y in 0 .. CHUNK_LAYERS as isize {
            for x in 0 .. CHUNK_WIDTH as isize {
                for z in 0 .. CHUNK_HEIGHT as isize {
                    // Build adjacency
                    let adjacency = VoxelAdjacency::new(
                        data.get(x-1, y, z).unwrap_or(&Voxel::Air),
                        data.get(x+1, y, z).unwrap_or(&Voxel::Air),
                        data.get(x, y, z-1).unwrap_or(&Voxel::Air),
                        data.get(x, y, z+1).unwrap_or(&Voxel::Air),
                        data.get(x, y+1, z).unwrap_or(&Voxel::Air),
                        data.get(x, y-1, z).unwrap_or(&Voxel::Air)
                    );

                    // Construct block vertices and indices
                    build_voxel(data.get_unsafe(x,y,z), (x as usize,y as usize,z as usize), adjacency, &mut vertices, &mut indices);
                }
            }
        };

        // Generate buffer resources
        let vertex_buffer = glium::VertexBuffer::new(facade, &vertices)?;
        let index_buffer = glium::IndexBuffer::new(
            facade,
            glium::index::PrimitiveType::TrianglesList,
            &indices)?;

        // Create the mesh from the buffers
        Ok(Mesh {
            vertex_buffer,
            index_buffer,
        })
    }
}

pub fn build_voxel(voxel:&Voxel, (x,y,z):(usize, usize, usize), adjacency: VoxelAdjacency, vertices:&mut Vec<Vertex>, indices:&mut Vec<u16>) {
    // Skip air voxels
    if voxel.is_empty() {
        return
    }

    // Get the vertices base
    let mut base_vertex = vertices.len() as u16;

    // Iterate over all faces
    for face in VoxelFace::values() {

        // Skip other solid sides
        if !adjacency.face(face).is_empty() {
            continue;
        }

        // Add all the vertices of the face
        let face_indices = &FACES[face as usize];
        for vertex_index in face_indices.iter() {
            let vertex = &VERTICES[*vertex_index];
            vertices.push(Vertex {
                position: [
                    (vertex[0] + x as f32) * VOXEL_SIZE.0,
                    (vertex[1] + y as f32) * VOXEL_SIZE.1,
                    (vertex[2] + z as f32) * VOXEL_SIZE.2],
                color: COLORS[*vertex_index]
            })
        }

        // Add the triangle
        indices.push(base_vertex + 3);
        indices.push(base_vertex + 2);
        indices.push(base_vertex + 1);
        indices.push(base_vertex + 1);
        indices.push(base_vertex + 0);
        indices.push(base_vertex + 3);

        base_vertex += face_indices.len() as u16;
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

        let draw_parameters = DrawParameters {
            depth: Depth {
                write: true,
                test: glium::DepthTest::IfLess,
                ..Default::default()
            },
            ..Default::default()
        };

        for chunk in iter {
            surface.draw(
                &chunk.vertex_buffer,
                &chunk.index_buffer,
                &self.program,
                &uniforms,
                &draw_parameters
            ).unwrap();
        }
    }
}

const FACES: [[usize; 4]; 6] = [
    [4, 7, 3, 0],
    [6, 5, 1, 2],
    [5, 4, 0, 1],
    [7, 6, 2, 3],
    [6, 7, 4, 5],
    [1, 0, 3, 2],
];

const VERTICES: [[f32; 3]; 8] = [
    [0., 0., 0.],
    [1., 0., 0.],
    [1., 0., 1.],
    [0., 0., 1.],
    [0., 1., 0.],
    [1., 1., 0.],
    [1., 1., 1.],
    [0., 1., 1.],
];

const COLORS: [[f32; 4]; 8] = [
    [0., 0., 0., 1.],
    [1., 0., 0., 1.],
    [1., 0., 1., 1.],
    [0., 0., 1., 1.],
    [0., 1., 0., 1.],
    [1., 1., 0., 1.],
    [1., 1., 1., 1.],
    [0., 1., 1., 1.],
];