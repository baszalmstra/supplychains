use crate::world::Voxel;

pub const CHUNK_WIDTH: usize = 32;
pub const CHUNK_HEIGHT: usize = 32;
pub const CHUNK_LAYERS: usize = 32;

pub struct Chunk {
    pub voxels: [[[Voxel; CHUNK_HEIGHT]; CHUNK_WIDTH]; CHUNK_LAYERS],
}

impl Default for Chunk {
    fn default() -> Self {
        let mut result = Chunk {
            voxels: [[[Default::default(); CHUNK_HEIGHT]; CHUNK_WIDTH]; CHUNK_LAYERS],
        };

        for x in 0..CHUNK_WIDTH {
            for z in 0..CHUNK_HEIGHT {
                result.voxels[0][x][z] = Voxel::Grass;
            }
        }

        for x in 16..CHUNK_WIDTH-8 {
            for z in 16..CHUNK_HEIGHT-8 {
                result.voxels[1][x][z] = Voxel::Grass;
            }
        }

        for x in 20..CHUNK_WIDTH-8 {
            for z in 20..CHUNK_HEIGHT-8 {
                result.voxels[2][x][z] = Voxel::Grass;
            }
        }

        result
    }
}

impl Chunk {
    pub fn get(&self, x: isize, y: isize, z: isize) -> Option<&Voxel> {
        if x < 0
            || y < 0
            || z < 0
            || x >= CHUNK_WIDTH as isize
            || y >= CHUNK_LAYERS as isize
            || z >= CHUNK_HEIGHT as isize
        {
            None
        } else {
            Some(&self.voxels[y as usize][x as usize][z as usize])
        }
    }

    pub fn get_unsafe(&self, x: isize, y: isize, z: isize) -> &Voxel {
        &self.voxels[y as usize][x as usize][z as usize]
    }
}
