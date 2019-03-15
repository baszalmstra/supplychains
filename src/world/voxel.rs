#[derive(Copy, Clone)]
pub enum Voxel {
    Air,
    Grass,
}

impl Default for Voxel {
    fn default() -> Self {
        Voxel::Air
    }
}
