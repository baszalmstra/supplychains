use crate::world::Voxel;

pub const CHUNK_WIDTH: usize = 32;
pub const CHUNK_HEIGHT: usize = 32;
pub const CHUNK_LAYERS: usize = 32;

pub struct Chunk {
    pub voxels: [[[Voxel; CHUNK_HEIGHT]; CHUNK_WIDTH]; CHUNK_LAYERS],
}

impl Default for Chunk {
    fn default() -> Self {
        Chunk {
            voxels: [[[Default::default(); CHUNK_HEIGHT]; CHUNK_WIDTH]; CHUNK_LAYERS],
        }
    }
}
