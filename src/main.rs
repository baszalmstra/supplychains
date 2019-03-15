#[macro_use]
extern crate amethyst;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;

use amethyst::{
    assets::{HotReloadBundle, HotReloadStrategy, Loader, AssetStorage},
    assets::{PrefabLoader, PrefabLoaderSystem, RonFormat},
    controls::{ArcBallControlBundle, ArcBallControlTag, HideCursor},
    core::transform::{TransformBundle, Transform},
    input::is_key_down,
    input::InputBundle,
    prelude::*,
    renderer::{
        DisplayConfig, DrawPbm, DrawShaded, DrawSkybox, MeshBuilder, Pipeline, PosColor, PosTex,
        PosNormTex, RenderBundle, Rgba, SkyboxColor, Stage, Mesh, Material, MeshData, DrawFlat,
        MaterialDefaults, Texture
    },
    utils::application_root_dir,
    winit::VirtualKeyCode,
    LoggerConfig, StdoutLog,
};

mod auto_fov;
mod scene;
mod world;

type ScenePrefab = scene::ScenePrefab<Vec<PosTex>>;

struct Example;

impl SimpleState for Example {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let prefab_handle = data.world.exec(|loader: PrefabLoader<'_, ScenePrefab>| {
            loader.load("prefabs/arc_ball_camera.ron", RonFormat, (), ())
        });
        data.world.create_entity().with(prefab_handle).build();

        *data.world.write_resource::<SkyboxColor>() = SkyboxColor {
            nadir: (0.003, 0.003, 0.003, 1.0).into(),
            zenith: (0.03, 0.03, 0.03, 1.0).into(),
        };

        data.world.write_resource::<HideCursor>().hide = true;

        let albedo = {
            let loader = data.world.read_resource::<Loader>();
            loader.load_from_data([1.0, 1.0, 1.0, 1.0].into(), (), &data.world.read_resource::<AssetStorage<Texture>>())
        };

        let mat_defaults = data.world.read_resource::<MaterialDefaults>().0.clone();

        let mesh = {
            use world::voxel::Vertex;
            let loader = data.world.read_resource::<Loader>();
            let triangle = vec![
                Vertex {
                    position: [0.0, 40.0, 0.0].into(),
                    color: [1.0, 0.0, 0.0, 1.0].into(),
                    tex_coord: [0.5, 0.0].into(),

                },
                Vertex {
                    position: [10.0, 0.0, 10.0].into(),
                    color: [0.0, 1.0, 0.0, 1.0].into(),
                    tex_coord: [0.0, 1.0].into(),
                },
                Vertex {
                    position: [-10.0, 0.0, 0.0].into(),
                    color: [0.0, 0.0, 1.0, 1.0].into(),
                    tex_coord: [1.0, 1.0].into(),
                },
            ];

            loader.load_from_data(
                MeshData::Creator(triangle),
                (),
                &data.world.read_resource::<AssetStorage<Mesh>>())
        };

        let material = Material {
            albedo,
            ..mat_defaults.clone()
        };

        data.world
            .create_entity()
            .with(mesh)
            .with( Transform::default() )
            .with( material )
            .build();
    }

    fn handle_event(
        &mut self,
        _: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                Trans::Quit
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(LoggerConfig {
        stdout: StdoutLog::Colored,
        ..LoggerConfig::default()
    });

    let app_root = application_root_dir();
    let resources_path = format!("{}/resources", app_root);

    let display_config_path = format!("{}/display_config.ron", resources_path);
    let config = DisplayConfig::load(&display_config_path);

    let key_bindings_path = format!("{}/input.ron", resources_path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.02, 0.02, 0.02, 1.0], 1.0)
            .with_pass(DrawFlat::<world::voxel::Vertex>::new())
            .with_pass(DrawSkybox::new()),
    );

    let game_data = GameDataBuilder::default()
        .with(PrefabLoaderSystem::<ScenePrefab>::default(), "prefab", &[])
        .with(auto_fov::AutoFovSystem, "auto_fov", &["prefab"])
        .with_bundle(HotReloadBundle::new(HotReloadStrategy::default()))?
        .with_bundle(TransformBundle::new().with_dep(&[]))?
        .with_bundle(
            InputBundle::<String, String>::new().with_bindings_from_file(&key_bindings_path)?,
        )?
        .with_bundle(ArcBallControlBundle::<String, String>::new())?
        .with_bundle(RenderBundle::new(pipe, Some(config)))?;

    let mut game = Application::new(resources_path, Example, game_data)?;

    game.run();

    Ok(())
}
