use crate::world::Voxel;
use noise::{Value, NoiseFn, Seedable};
use nalgebra::{Vector2};

pub const CHUNK_WIDTH: usize = 16;
pub const CHUNK_HEIGHT: usize = 16;
pub const CHUNK_LAYERS: usize = 64;

pub struct Chunk {
    pub voxels: Vec<Voxel>,
    pub position: Vector2<i64>,
}

impl Chunk {
    /// Generates a new chunk with the left-front position at `position`.
    pub fn generate<P>(position: P, seed: u32) -> Chunk
    where
        P:Into<Vector2<i64>>
    {
        let mut noise = noise::Value::new().set_seed(seed);
        let mut perlin = noise::Perlin::new().set_seed(seed);

        let mut chunk = Chunk {
            voxels: Vec::with_capacity(CHUNK_WIDTH*CHUNK_HEIGHT*CHUNK_LAYERS),
            position: position.into()
        };

        for i in 0..CHUNK_WIDTH*CHUNK_HEIGHT*CHUNK_LAYERS {
            chunk.voxels.push(Voxel::Air)
        }

        for x in 0..CHUNK_WIDTH {
            for z in 0..CHUNK_HEIGHT {
                let height = 30 + (10 as f64
                    * perlin.get([
                    0.09 * (0.0021 + (x as i64 + chunk.position[0]) as f64 / 3.0),
                    0.5,
                    0.09 * (0.0021 + (z as i64 + chunk.position[1]) as f64 / 3.0),
                ])) as i64;

                for y in 0..CHUNK_LAYERS {
                    let index = Chunk::index(x,y,z);
                    if (y as i64) < height {
                        // Dirt
                        chunk.voxels[index] = Voxel::Grass {
                            shade: (noise.get([x as f64, y as f64 ,z as f64]) * 255.0) as u8
                        };
//                        };
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
            Some(unsafe { self.get_unchecked(x as usize, y as usize, z as usize)})
        }
    }

    pub unsafe fn get_unchecked(&self, x: usize, y: usize, z: usize) -> &Voxel {
        unsafe { self.voxels.get_unchecked(Chunk::index(x,y,z)) }
    }

    pub const fn index(x: usize, y: usize, z:usize) -> usize {
        y*(CHUNK_WIDTH*CHUNK_HEIGHT)+x*CHUNK_HEIGHT+z
    }
}
