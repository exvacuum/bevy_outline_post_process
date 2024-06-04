use bevy::{
    prelude::*,
    render::{extract_component::ExtractComponent, render_resource::ShaderType},
};

/// Component which, when inserted into an entity with a camera and normal prepass, enables an outline effect for that
/// camera.
#[derive(Component, ShaderType, ExtractComponent, PartialEq, Clone, Default)]
pub struct OutlinePostProcessSettings {
    /// Weight of outlines in pixels.
    pub weight: f32,
    /// A threshold for normal differences, values below this threshold will not become outlines.
    /// Higher values will result in more outlines which may look better on smooth surfaces.
    pub threshold: f32,
}
