use crate::world::Voxel;
use noise::{Value, NoiseFn, Seedable};
use nalgebra::{Vector2};

pub const CHUNK_WIDTH: usize = 16;
pub const CHUNK_HEIGHT: usize = 16;
pub const CHUNK_LAYERS: usize = 64;

pub struct Chunk {
    pub voxels: [[[Voxel; CHUNK_HEIGHT]; CHUNK_WIDTH]; CHUNK_LAYERS],
    pub position: Vector2<i64>,
}

impl Chunk {
    /// Generates a new chunk with the left-front position at `position`.
    pub fn generate<P>(position: P, seed: u32) -> Chunk
    where
        P:Into<Vector2<i64>>
    {
        let mut noise = noise::Value::new();
        noise.set_seed(seed);

        let mut perlin = noise::Perlin::new();
        perlin.set_seed(seed);

        let mut chunk = Chunk {
            voxels: [[[Default::default(); CHUNK_HEIGHT]; CHUNK_WIDTH]; CHUNK_LAYERS],
            position: position.into()
        };

        for x in 0..CHUNK_WIDTH {
            for z in 0..CHUNK_HEIGHT {
                let height = (CHUNK_LAYERS as f64
                    * perlin.get([
                    0.005 * (0.0021 + (x as i64 + chunk.position[0]) as f64 / 3.0),
                    0.5,
                    0.005 * (0.0021 + (z as i64 + chunk.position[1]) as f64 / 3.0),
                ])) as i64;

                for y in 0..CHUNK_LAYERS {
                    if (y as i64) < height {
                        // Dirt
                        chunk.voxels[y][x][z] = Voxel::Grass {
                            shade: (noise.get([x as f64,0. ,z as f64]) * 255.0) as u8
                        };
//                        if (cy * CHUNK_SIZE as i64 + k as i64) < height - 5 {
//                            // Stone
//                            if coal_noise > 10 && coal_noise < 15 {
//                                chunk[i][k][j] = BlockId::from(6);
//                            } else {
//                                chunk[i][k][j] = BlockId::from(5);
//                            }
//                        }
//                    } else if (cy * CHUNK_SIZE as i64 + k as i64) == height {
//                        // Grass
//                        chunk[i][k][j] = BlockId::from(2);
                    }
                }
            }
        }

        chunk
    }


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
