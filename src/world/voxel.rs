pub const VOXEL_SIZE:(f32,f32,f32) = (2., 0.5, 2.);

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

impl Voxel {
    pub fn is_empty(&self) -> bool {
        match *self {
            Voxel::Air => true,
            _ => false
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum VoxelFace {
    Left = 0,
    Right = 1,
    Front = 2,
    Back = 3,
    Top = 4,
    Bottom = 5,
}

impl VoxelFace {
    pub fn values() -> impl Iterator<Item=VoxelFace> {
        use VoxelFace::*;
        static VALUES:[VoxelFace;6] = [VoxelFace::Left,VoxelFace::Right,VoxelFace::Front,VoxelFace::Back,VoxelFace::Top,VoxelFace::Bottom];
        VALUES.iter().map(|v| *v)
    }
}

pub struct VoxelAdjacency<'a> {
    adjacency: [&'a Voxel; 6],
}

impl<'a> VoxelAdjacency<'a> {
    pub fn new(
        left: &'a Voxel,
        right: &'a Voxel,
        front: &'a Voxel,
        back: &'a Voxel,
        top: &'a Voxel,
        bottom: &'a Voxel,
    ) -> VoxelAdjacency<'a> {
        VoxelAdjacency {
            adjacency: [left, right, front, back, top, bottom],
        }
    }

    pub fn face(&self, face: VoxelFace) -> &Voxel {
        &self.adjacency[face as usize]
    }

    pub fn left(&self) -> &Voxel {
        self.face(VoxelFace::Left)
    }
    pub fn right(&self) -> &Voxel {
        self.face(VoxelFace::Right)
    }
    pub fn top(&self) -> &Voxel {
        self.face(VoxelFace::Top)
    }
    pub fn bottom(&self) -> &Voxel {
        self.face(VoxelFace::Bottom)
    }
    pub fn front(&self) -> &Voxel {
        self.face(VoxelFace::Front)
    }
    pub fn back(&self) -> &Voxel {
        self.face(VoxelFace::Back)
    }
}
