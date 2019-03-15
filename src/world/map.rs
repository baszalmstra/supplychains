use amethyst::{
    assets::{AssetStorage, Handle},
    renderer::{Mesh}
};

#[derive(Copy, Clone)]
pub enum Voxel {
    Air,
    Grass,
}

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