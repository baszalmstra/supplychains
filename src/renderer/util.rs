use amethyst::{
    ecs::{
        Read, ReadStorage, Join
    },
    core::{
        Transform, GlobalTransform
    },
    renderer::{
        ActiveCamera, Camera
    }
};

/// Returns the main camera and its `GlobalTransform`
pub fn get_camera<'a>(
    active: Read<'a, ActiveCamera>,
    camera: &'a ReadStorage<'a, Camera>,
    global: &'a ReadStorage<'a, GlobalTransform>,
) -> Option<(&'a Camera, &'a GlobalTransform)> {
    active
        .entity
        .and_then(|entity| {
            let cam = camera.get(entity);
            let transform = global.get(entity);
            cam.into_iter().zip(transform.into_iter()).next()
        })
        .or_else(|| (camera, global).join().next())
}
