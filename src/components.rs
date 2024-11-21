use bevy::{
    prelude::*,
    render::{extract_component::ExtractComponent, render_resource::ShaderType},
};

/// Component which, when inserted into an entity with a camera and normal prepass, enables an outline effect for that
/// camera.
#[derive(Component, ShaderType, ExtractComponent, PartialEq, Clone)]
pub struct OutlinePostProcessSettings {
    /// Weight of outlines in pixels.
    weight: f32,
    /// A threshold for normal differences, values below this threshold will not become outlines.
    /// Higher values will result in more outlines which may look better on smooth surfaces.
    normal_threshold: f32,
    /// Whether to use adaptive outlines. White outlines will be drawn around darker objects, while black ones will be drawn around lighter ones.
    adaptive: u32,
    /// Threshold of illumination for outlines to apply. Requires deferred prepass.
    light_threshold: f32,
}

impl OutlinePostProcessSettings {
    /// Create a new instance with the given settings
    pub fn new(weight: f32, normal_threshold: f32, adaptive: bool, light_threshold: f32) -> Self {
        Self {
            weight,
            normal_threshold,
            adaptive: adaptive as u32,
            light_threshold,
        }
    }
}

impl Default for OutlinePostProcessSettings {
    fn default() -> Self {
        Self {
            weight: 1.0,
            normal_threshold: 0.0,
            adaptive: 0,
            light_threshold: 0.0,
        }
    }
}
