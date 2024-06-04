use bevy::{
    prelude::*,
    render::{extract_component::ExtractComponent, render_resource::ShaderType},
};

#[derive(Component, ShaderType, ExtractComponent, PartialEq, Clone, Default)]
pub struct OutlinePostProcessSettings {
    pub weight: f32,
    pub threshold: f32,
}
