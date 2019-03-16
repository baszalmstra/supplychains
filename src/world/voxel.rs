pub const VOXEL_SIZE:(f32,f32,f32) = (2., 2., 2.);

#[derive(Copy, Clone)]
pub enum Voxel {
    Air,
    Grass { shade: u8 },
}

impl Default for Voxel {
    fn default() -> Self {
        Voxel::Air
    }
}

impl Voxel {
    /// Returns true if this voxel represents an 'empty' cell indicating that no geometry is
    /// present.
    pub fn is_empty(&self) -> bool {
        match *self {
            Voxel::Air => true,
            _ => false
        }
    }

    /// Returns a value ranging from 0 to 1 which indicates how much light can penetrate through
    /// this voxel. Where 0 means no light can pass through at all and 1 means all light is passed
    /// through.
    pub fn light_transparency(&self) -> f32 {
        match *self {
            Voxel::Air => 1.,
            _ => 0.
        }
    }

    pub fn shade(&self) -> f32 {
        match *self {
            Voxel::Grass { shade} => shade as f32 / 255.0,
            _ => 1.0
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
    adjacency: [&'a Voxel; 27],
}

impl<'a> VoxelAdjacency<'a> {
    pub fn new(
        adjacency: [&'a Voxel; 27]
    ) -> VoxelAdjacency<'a> {
        VoxelAdjacency {
            adjacency
        }
    }

    pub fn center(&self) -> &Voxel {
        self.get(0,0,0)
    }

    pub fn face(&self, face: VoxelFace) -> &Voxel {
        match face {
            VoxelFace::Left => self.get(-1,0,0),
            VoxelFace::Right => self.get(1,0,0),
            VoxelFace::Front => self.get(0,0,-1),
            VoxelFace::Back => self.get(0,0,1),
            VoxelFace::Top => self.get(0,1,0),
            VoxelFace::Bottom => self.get(0,-0,0)
        }
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

    pub fn get(&self, x:isize, y: isize, z:isize) -> &Voxel {
        self.get_index_unsafe(VoxelAdjacency::adjacency_index(x,y,z))
    }

    pub fn get_index_unsafe(&self, index:usize) -> &Voxel {
        &self.adjacency[index]
    }

    pub const fn adjacency_index(x:isize, y: isize, z:isize) -> usize {
//        debug_assert!(x >= -1 && x <= 1 &&
//            y >= -1 && z <= 1 &&
//            z >= -1 && y <= 1);

        ((y+1)*9+(x+1)*3+(z+1)) as usize
    }
}
