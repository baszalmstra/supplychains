use amethyst::{
    core::{Time, Transform},
    ecs::{Component, DenseVecStorage, System, Read, ReadStorage, WriteStorage, Join},
};

#[derive(Copy, Clone, PartialEq)]
pub struct Rotator {
    pub speed: f32
}

impl Component for Rotator {
    type Storage = DenseVecStorage<Self>;
}

pub struct RotatorSystem;

impl<'a> System<'a> for RotatorSystem {
    type SystemData = (
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Rotator>,
        Read<'a, Time>
    );

    fn run(&mut self, (mut transform, rotator, time): Self::SystemData) {
        for (mut transform, rotator) in (&mut transform, &rotator).join() {
            transform.yaw_local(rotator.speed*time.delta_seconds());
        }
    }
}

