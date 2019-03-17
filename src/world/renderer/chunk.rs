use crate::world::chunk::{Chunk, CHUNK_LAYERS, CHUNK_HEIGHT, CHUNK_WIDTH};
use glium::backend::Facade;
use crate::camera::Camera;
use crate::world::voxel::{VoxelAdjacency, Voxel, VoxelFace, VOXEL_SIZE};
use glium::{DrawParameters, Depth};
use std::{mem, ptr};
use nalgebra::{Vector4, Vector3, Matrix4};
use nalgebra::geometry::{Isometry3, Translation3};

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 4],
    normal: [f32; 3],
}

implement_vertex!(Vertex, position, color, normal);

pub struct Mesh {
    vertex_buffer: glium::VertexBuffer<Vertex>,
    index_buffer: glium::IndexBuffer<u16>,
    model: Matrix4<f32>
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
                    let adjacency = unsafe {
                        let mut adjacency:[&Voxel; 27] = mem::uninitialized();
                        let mut next_index = 0;
                        for dy in -1..=1 {
                            for dx in -1..=1 {
                                for dz in -1..=1 {
                                    ptr::write(&mut adjacency[next_index], data.get(x + dx, y + dy, z + dz).unwrap_or(&Voxel::Air));
                                    next_index += 1;
                                }
                            }
                        }
                        VoxelAdjacency::new(adjacency)
                    };

                    // Construct block vertices and indices
                    build_voxel(&adjacency, (x,y,z), &mut vertices, &mut indices);
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
            model: Translation3::new(data.position[0] as f32, -30., data.position[1] as f32).into()
        })
    }
}

pub fn build_voxel(adjacency:&VoxelAdjacency, (x,y,z):(isize, isize, isize), vertices:&mut Vec<Vertex>, indices:&mut Vec<u16>) {
    // Skip air voxels
    if adjacency.center().is_empty() {
        return
    }

    // Iterate over all faces
    let mut base_vertex = vertices.len() as u16;
    for face in VoxelFace::values() {

        // Skip other solid sides
        if !adjacency.face(face).is_empty() {
            continue;
        }

        let occlusion = compute_face_average_occlusion(adjacency, face);

        // Add all the vertices of the face
        let face_indices = &FACES[face as usize];
        for (i, vertex_index) in face_indices.iter().enumerate() {
            let vertex = &VERTICES[*vertex_index];
            let occlusion = occlusion[i];
            let shade = (adjacency.center().shade() * 0.3) + 0.7;
            let color = [0.0,0.8*shade,0.02,1.];
            vertices.push(Vertex {
                position: [
                    (vertex[0] + x as f32) * VOXEL_SIZE.0,
                    (vertex[1] + y as f32) * VOXEL_SIZE.1,
                    (vertex[2] + z as f32) * VOXEL_SIZE.2],
                color: [
                    color[0] * occlusion,
                    color[1] * occlusion,
                    color[2] * occlusion,
                    color[3]
                ],
                normal: FACE_NORMALS[face as usize]
            })
        }

        // Add the triangle
        if occlusion[0] + occlusion[2] < occlusion[1] + occlusion[3] {
            indices.push(base_vertex + 3);
            indices.push(base_vertex + 2);
            indices.push(base_vertex + 1);
            indices.push(base_vertex + 1);
            indices.push(base_vertex + 0);
            indices.push(base_vertex + 3);
        } else {
            indices.push(base_vertex + 0);
            indices.push(base_vertex + 2);
            indices.push(base_vertex + 1);
            indices.push(base_vertex + 0);
            indices.push(base_vertex + 3);
            indices.push(base_vertex + 2);
        }

        base_vertex += face_indices.len() as u16;
    }

}

fn compute_face_average_occlusion(adjacency:&VoxelAdjacency, face:VoxelFace) -> [f32; 4] {
    let vertex_indices = OCCLUSION_INDICES[face as usize];
    [
        compute_vertex_average_occlusion(adjacency, vertex_indices[0]),
        compute_vertex_average_occlusion(adjacency, vertex_indices[1]),
        compute_vertex_average_occlusion(adjacency, vertex_indices[2]),
        compute_vertex_average_occlusion(adjacency, vertex_indices[3])
    ]
}

fn compute_vertex_average_occlusion(adjacency:&VoxelAdjacency, indices:[usize;4]) -> f32 {
    indices.iter()
        .map(|index| adjacency.get_index_unsafe(*index).light_transparency())
        .sum::<f32>() / 4.0
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
            in vec3 normal;

            out vec4 v_color;

            uniform mat4 projection;
            uniform mat4 view;
            uniform mat4 model;

            uniform vec3 light_color;

            void main() {
                v_color = vec4(color.rgb * max(0, dot(normal, vec3(0,1,0))) * light_color * 0.53 + color.rgb * light_color * 0.47, color.a);
                gl_Position = projection * view * model * vec4(position, 1.0);
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

        let model:Matrix4<f32> = Isometry3::translation(-16., 0., -16.).into();
        let light_color:Vector3<f32> = Vector3::new(0.8,0.77,0.81);

        let proj_mat:[[f32;4];4] = camera.get_perspective(1280.0/800.0).into();
        let view_mat:[[f32;4];4] = camera.view_matrix().into();
        let light_color_vec:[f32;3] = light_color.into();

        let draw_parameters = DrawParameters {
            depth: Depth {
                write: true,
                test: glium::DepthTest::IfLess,
                ..Default::default()
            },
            ..Default::default()
        };

        for chunk in iter {

            let model_mat:[[f32;4];4] = chunk.model.into();

            let uniforms = uniform! {
                projection: proj_mat,
                view: view_mat,
                model: model_mat,
                light_color: light_color_vec
            };

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

const OCCLUSION_INDICES:[[[usize; 4]; 4]; 6] = [
    // Left
    [
        // 4
        [
            VoxelAdjacency::adjacency_index(-1,  0, -1),
            VoxelAdjacency::adjacency_index(-1,  0,  0),
            VoxelAdjacency::adjacency_index(-1,  1,  0),
            VoxelAdjacency::adjacency_index(-1,  1, -1),
        ],

        // 7
        [
            VoxelAdjacency::adjacency_index(-1,  0,  0),
            VoxelAdjacency::adjacency_index(-1,  0,  1),
            VoxelAdjacency::adjacency_index(-1,  1,  1),
            VoxelAdjacency::adjacency_index(-1,  1,  0),
        ],

        // 3
        [
            VoxelAdjacency::adjacency_index(-1, -1,  0),
            VoxelAdjacency::adjacency_index(-1, -1,  1),
            VoxelAdjacency::adjacency_index(-1,  0,  1),
            VoxelAdjacency::adjacency_index(-1,  0,  0),
        ],

        // 0
        [
            VoxelAdjacency::adjacency_index(-1, -1, -1),
            VoxelAdjacency::adjacency_index(-1, -1,  0),
            VoxelAdjacency::adjacency_index(-1,  0,  0),
            VoxelAdjacency::adjacency_index(-1,  0, -1),
        ],
    ],

    // Right
    [
        // 6
        [
            VoxelAdjacency::adjacency_index(1,  0,  0),
            VoxelAdjacency::adjacency_index(1,  0,  1),
            VoxelAdjacency::adjacency_index(1,  1,  1),
            VoxelAdjacency::adjacency_index(1,  1,  0),
        ],

        // 5
        [
            VoxelAdjacency::adjacency_index(1,  0, -1),
            VoxelAdjacency::adjacency_index(1,  0,  0),
            VoxelAdjacency::adjacency_index(1,  1,  0),
            VoxelAdjacency::adjacency_index(1,  1, -1),
        ],

        // 1
        [
            VoxelAdjacency::adjacency_index(1, -1, -1),
            VoxelAdjacency::adjacency_index(1, -1,  0),
            VoxelAdjacency::adjacency_index(1,  0,  0),
            VoxelAdjacency::adjacency_index(1,  0, -1),
        ],

        // 2
        [
            VoxelAdjacency::adjacency_index(1, -1,  0),
            VoxelAdjacency::adjacency_index(1, -1,  1),
            VoxelAdjacency::adjacency_index(1,  0,  1),
            VoxelAdjacency::adjacency_index(1,  0,  0),
        ],
    ],

    // Front
    [
        // 5
        [
            VoxelAdjacency::adjacency_index( 0,  0, -1),
            VoxelAdjacency::adjacency_index( 0,  1, -1),
            VoxelAdjacency::adjacency_index( 1,  0, -1),
            VoxelAdjacency::adjacency_index( 1,  1, -1),
        ],

        // 4
        [
            VoxelAdjacency::adjacency_index(-1,  0, -1),
            VoxelAdjacency::adjacency_index(-1,  1, -1),
            VoxelAdjacency::adjacency_index( 0,  0, -1),
            VoxelAdjacency::adjacency_index( 0,  1, -1),
        ],

        // 0
        [
            VoxelAdjacency::adjacency_index(-1, -1, -1),
            VoxelAdjacency::adjacency_index(-1,  0, -1),
            VoxelAdjacency::adjacency_index( 0, -1, -1),
            VoxelAdjacency::adjacency_index( 0,  0, -1),
        ],

        // 1
        [
            VoxelAdjacency::adjacency_index(0, -1, -1),
            VoxelAdjacency::adjacency_index(0,  0, -1),
            VoxelAdjacency::adjacency_index(1, -1, -1),
            VoxelAdjacency::adjacency_index(1,  0, -1),
        ],
    ],

    // Back
    [
        // 7
        [
            VoxelAdjacency::adjacency_index(-1,  0,  1),
            VoxelAdjacency::adjacency_index(-1,  1,  1),
            VoxelAdjacency::adjacency_index( 0,  0,  1),
            VoxelAdjacency::adjacency_index( 0,  1,  1),
        ],

        // 6
        [
            VoxelAdjacency::adjacency_index( 0,  0,  1),
            VoxelAdjacency::adjacency_index( 0,  1,  1),
            VoxelAdjacency::adjacency_index( 1,  0,  1),
            VoxelAdjacency::adjacency_index( 1,  1,  1),
        ],

        // 2
        [
            VoxelAdjacency::adjacency_index(0, -1,  1),
            VoxelAdjacency::adjacency_index(0,  0,  1),
            VoxelAdjacency::adjacency_index(1, -1,  1),
            VoxelAdjacency::adjacency_index(1,  0,  1),
        ],

        // 3
        [
            VoxelAdjacency::adjacency_index(-1, -1,  1),
            VoxelAdjacency::adjacency_index(-1,  0,  1),
            VoxelAdjacency::adjacency_index( 0, -1,  1),
            VoxelAdjacency::adjacency_index( 0,  0,  1),
        ],
    ],

    // Top
    [
        // 6
        [
            VoxelAdjacency::adjacency_index( 0,  1,  0),
            VoxelAdjacency::adjacency_index( 0,  1,  1),
            VoxelAdjacency::adjacency_index( 1,  1,  0),
            VoxelAdjacency::adjacency_index( 1,  1,  1),
        ],

        // 7
        [
            VoxelAdjacency::adjacency_index(-1,  1,  0),
            VoxelAdjacency::adjacency_index(-1,  1,  1),
            VoxelAdjacency::adjacency_index( 0,  1,  0),
            VoxelAdjacency::adjacency_index( 0,  1,  1),
        ],

        // 4
        [
            VoxelAdjacency::adjacency_index(-1,  1, -1),
            VoxelAdjacency::adjacency_index(-1,  1,  0),
            VoxelAdjacency::adjacency_index( 0,  1, -1),
            VoxelAdjacency::adjacency_index( 0,  1,  0),
        ],

        // 5
        [
            VoxelAdjacency::adjacency_index( 0,  1, -1),
            VoxelAdjacency::adjacency_index( 0,  1,  0),
            VoxelAdjacency::adjacency_index( 1,  1, -1),
            VoxelAdjacency::adjacency_index( 1,  1,  0),
        ],
    ],

    // Bottom
    [
        // 1
        [
            VoxelAdjacency::adjacency_index( 0, -1, -1),
            VoxelAdjacency::adjacency_index( 0, -1,  0),
            VoxelAdjacency::adjacency_index( 1, -1, -1),
            VoxelAdjacency::adjacency_index( 1, -1,  0),
        ],

        // 0
        [
            VoxelAdjacency::adjacency_index(-1, -1, -1),
            VoxelAdjacency::adjacency_index(-1, -1,  0),
            VoxelAdjacency::adjacency_index( 0, -1, -1),
            VoxelAdjacency::adjacency_index( 0, -1,  0),
        ],

        // 3
        [
            VoxelAdjacency::adjacency_index(-1, -1,  0),
            VoxelAdjacency::adjacency_index(-1, -1,  1),
            VoxelAdjacency::adjacency_index( 0, -1,  0),
            VoxelAdjacency::adjacency_index( 0, -1,  1),
        ],

        // 2
        [
            VoxelAdjacency::adjacency_index( 0, -1,  0),
            VoxelAdjacency::adjacency_index( 0, -1,  1),
            VoxelAdjacency::adjacency_index( 1, -1,  0),
            VoxelAdjacency::adjacency_index( 1, -1,  1),
        ],
    ]
];

const FACES: [[usize; 4]; 6] = [
    [4, 7, 3, 0],
    [6, 5, 1, 2],
    [5, 4, 0, 1],
    [7, 6, 2, 3],
    [6, 7, 4, 5],
    [1, 0, 3, 2],
];

const FACE_NORMALS: [[f32; 3]; 6] = [
    [-1.,  0.,  0.],
    [ 1.,  0.,  0.],
    [ 0.,  0., -1.],
    [ 0.,  0.,  1.],
    [ 0.,  1.,  0.],
    [ 0., -1.,  0.]
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn occlusion_adjacencies() {
        let mut locations = Vec::new();
        for dy in -1..=1 {
            for dx in -1..=1 {
                for dz in -1..=1 {
                    locations.push((dx,dy,dz));
                }
            }
        }

        assert_eq!(locations.len(), 27);

        for face in VoxelFace::values() {
            let normal = FACE_NORMALS[face as usize];
            for (i, vertex_index) in FACES[face as usize].iter().enumerate() {
                let vertex = VERTICES[*vertex_index as usize];
                let count = locations.iter()
                    .filter(|(x,y,z)| {
                        *x as f32 *normal[0] + *y as f32 * normal[1]  + *z as f32 * normal[2] > 0.7
                    })
                    .filter(|(x,y,z)| {
                        *x as f32 <= vertex[0] && *x as f32 + 2. > vertex[0] &&
                        *y as f32 <= vertex[1] && *y as f32 + 2. > vertex[1] &&
                        *z as f32 <= vertex[2] && *z as f32 + 2. > vertex[2]
                    })
                    .filter(|(x,y,z)| {
                        OCCLUSION_INDICES[face as usize][i].contains(&VoxelAdjacency::adjacency_index(*x,*y,*z))
                    })
                    .count();

                assert_eq!(4, count);
            }
        }
    }
}