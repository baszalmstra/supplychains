use amethyst::{
    core::{
        nalgebra::{Vector2, Vector3}
    },
    renderer::{
        Separate, Renderer, Position, Color, TexCoord, With, VertexFormat, Attribute,
        Attributes, AttributeFormat, MeshCreator, MeshBuilder
    }
};
use gfx::{pso::buffer::Element};
use gfx_core::memory::Pod;
use amethyst::{
    assets::{AssetStorage, Handle},
    renderer::{Mesh}
};
use crate::world::voxel::Voxel;

pub enum VoxelFace {
    Top     = 0,
    Bottom  = 2,
    Left    = 3,
    Right   = 4,
    Front   = 5,
    Back    = 6
}

pub const CHUNK_WIDTH:usize  = 16;
pub const CHUNK_HEIGHT:usize = 16;
pub const CHUNK_LAYERS:usize = 16;

#[derive(Debug, Clone)]
pub struct ChunkData {
    pub voxels: [[[Voxel; CHUNK_LAYERS]; CHUNK_HEIGHT]; CHUNK_WIDTH]
}

impl Default for ChunkData {
    fn default() -> Self {
        ChunkData {
            voxels: [[[Voxel::Air; CHUNK_LAYERS]; CHUNK_HEIGHT]; CHUNK_WIDTH]
        }
    }
}

impl MeshCreator for ChunkData {
    fn build(self: Box<ChunkData>, renderer: &mut Renderer) -> amethyst::renderer::error::Result<Mesh> {
        let triangle = vec![
            Vertex {
                position: [0.0, 40.0, 0.0].into(),
                color: [1.0, 0.0, 0.0, 1.0].into(),
                tex_coord: [0.5, 0.0].into(),

            },
            Vertex {
                position: [10.0, 0.0, 10.0].into(),
                color: [0.0, 1.0, 0.0, 1.0].into(),
                tex_coord: [0.0, 1.0].into(),
            },
            Vertex {
                position: [-10.0, 0.0, 0.0].into(),
                color: [0.0, 0.0, 1.0, 1.0].into(),
                tex_coord: [1.0, 1.0].into(),
            },
        ];

        MeshBuilder::new(triangle).build(&mut renderer.factory)
    }

    fn vertices(&self) -> &Vec<Separate<Position>> {
        unimplemented!()
    }

    fn box_clone(&self) -> Box<MeshCreator> {
        Box::new(self.clone())
    }
}

//pub fn generate_mesh(
//    data:&ChunkData,
//    storage:AssetStorage<Mesh>) -> Handle<Mesh> {
//
//
//
//    for (x, slice) in data.voxels.iter().enumerate() {
//        for (y, row) in slice.iter().enumerate() {
//            for (z, voxel) in row.iter().enumerate() {
//
//            }
//        }
//    }
//}

/// Vertex format with position, RGBA8 color and normal attributes.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vertex {
    /// Position of the vertex in 3D space.
    pub position: Vector3<f32>,
    /// RGBA color value of the vertex.
    pub color: [f32; 4],
    /// UV texture coordinates used by the vertex.
    pub tex_coord: Vector2<f32>,
}

unsafe impl Pod for Vertex {}

impl VertexFormat for Vertex {
    const ATTRIBUTES: Attributes<'static> = &[
        (Position::NAME, <Self as With<Position>>::FORMAT),
        (Color::NAME, <Self as With<Color>>::FORMAT),
        (TexCoord::NAME, <Self as With<TexCoord>>::FORMAT),
    ];
}

impl With<Position> for Vertex {
    const FORMAT: AttributeFormat = Element {
        offset: 0,
        format: Position::FORMAT,
    };
}

impl With<Color> for Vertex {
    const FORMAT: AttributeFormat = Element {
        offset: Position::SIZE,
        format: Color::FORMAT,
    };
}

impl With<TexCoord> for Vertex {
    const FORMAT: AttributeFormat = Element {
        offset: Position::SIZE + Color::SIZE,
        format: TexCoord::FORMAT,
    };
}