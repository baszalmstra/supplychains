use amethyst::{
    core::{
        nalgebra::{Vector2, Vector3}
    },
    renderer::{Position, Color, TexCoord, With, VertexFormat, Attribute, Attributes, AttributeFormat}
};
use gfx::{pso::buffer::Element};
use gfx_core::memory::Pod;

/// Vertex format with position, RGBA8 color and normal attributes.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vertex {
    /// Position of the vertex in 3D space.
    pub position: Vector3<f32>,
    /// RGBA color value of the vertex.
    pub color: [f32; 4],
    /// UV texture coordinates used by the vertex.
    pub tex_coord: Vector2<f32>,
}

unsafe impl Pod for Vertex {}

impl VertexFormat for Vertex {
    const ATTRIBUTES: Attributes<'static> = &[
        (Position::NAME, <Self as With<Position>>::FORMAT),
        (Color::NAME, <Self as With<Color>>::FORMAT),
        (TexCoord::NAME, <Self as With<TexCoord>>::FORMAT),
    ];
}

impl With<Position> for Vertex {
    const FORMAT: AttributeFormat = Element {
        offset: 0,
        format: Position::FORMAT,
    };
}

impl With<Color> for Vertex {
    const FORMAT: AttributeFormat = Element {
        offset: Position::SIZE,
        format: Color::FORMAT,
    };
}

impl With<TexCoord> for Vertex {
    const FORMAT: AttributeFormat = Element {
        offset: Position::SIZE + Color::SIZE,
        format: TexCoord::FORMAT,
    };
}