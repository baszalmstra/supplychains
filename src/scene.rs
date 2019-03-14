//! Provides utilities for building and describing scenes in your game.

use std::fmt::Debug;

use serde::{de::DeserializeOwned, Serialize};

use amethyst::assets::{Format, PrefabData, PrefabError, ProgressCounter};
use amethyst::controls::ControlTagPrefab;
use amethyst::core::{specs::prelude::Entity, Transform};
use amethyst::renderer::{
    CameraPrefab, GraphicsPrefab, InternalShape, LightPrefab, Mesh, MeshData, ObjFormat,
    TextureFormat,
};

use crate::auto_fov::AutoFov;
use amethyst::utils::removal::Removal;

/// Basic `Prefab` scene node, meant to be used for fast prototyping, and most likely replaced
/// for more complex scenarios.
///
/// ### Type parameters:
///
/// - `V`: Vertex format to use for generated `Mesh`es, must to be one of:
///     * `Vec<PosTex>`
///     * `Vec<PosNormTex>`
///     * `Vec<PosNormTangTex>`
///     * `ComboMeshCreator`
/// - `R`: The type of id used by the Removal component.
/// - `M`: `Format` to use for loading `Mesh`es from file
#[derive(Deserialize, Serialize, PrefabData)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct ScenePrefab<V, R = (), M = ObjFormat>
where
    M: Format<Mesh> + Clone,
    M::Options: DeserializeOwned + Serialize + Clone,
    R: PartialEq + Debug + Clone + Send + Sync + 'static,
    V: From<InternalShape> + Into<MeshData>,
{
    graphics: Option<GraphicsPrefab<V, M, TextureFormat>>,
    transform: Option<Transform>,
    light: Option<LightPrefab>,
    camera: Option<CameraPrefab>,
    auto_fov: Option<AutoFov>,
    control_tag: Option<ControlTagPrefab>,
    removal: Option<Removal<R>>,
}

impl<V, R, M> Default for ScenePrefab<V, R, M>
where
    M: Format<Mesh> + Clone,
    M::Options: DeserializeOwned + Serialize + Clone,
    R: PartialEq + Debug + Clone + Send + Sync + 'static,
    V: From<InternalShape> + Into<MeshData>,
{
    fn default() -> Self {
        ScenePrefab {
            graphics: None,
            transform: None,
            light: None,
            camera: None,
            auto_fov: None,
            control_tag: None,
            removal: None,
        }
    }
}
