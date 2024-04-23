use bevy::{prelude::*, render::{render_resource::ShaderType, extract_component::ExtractComponent}};

#[derive(Component, ShaderType, ExtractComponent, PartialEq, Clone, Default)]
pub struct OutlinePostProcessSettings {
    pub weight: f32,
    pub threshold: f32,
    #[cfg(feature = "webgl2")]
    _padding: Vec2,
}
