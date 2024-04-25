use bevy::{prelude::*, render::{RenderApp, extract_component::{UniformComponentPlugin, ExtractComponentPlugin}, render_graph::{RenderGraphApp, ViewNodeRunner}}, asset::embedded_asset, core_pipeline::core_3d::graph::{Core3d, Node3d}};

pub use nodes::OutlineRenderLabel;

pub struct OutlinePostProcessPlugin;

pub mod components;
mod resources;
mod nodes;

impl Plugin for OutlinePostProcessPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "../assets/shaders/outline_post_process.wgsl");

        app.add_plugins((
            UniformComponentPlugin::<components::OutlinePostProcessSettings>::default(),
            ExtractComponentPlugin::<components::OutlinePostProcessSettings>::default(),
        ));
    
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.add_render_graph_node::<ViewNodeRunner<nodes::OutlineRenderNode>>(
            Core3d,
            nodes::OutlineRenderLabel,
        ).add_render_graph_edges(
            Core3d, 
            (
                Node3d::Tonemapping,
                nodes::OutlineRenderLabel,
                Node3d::EndMainPassPostProcessing,
            ),
        );
    }
    
    fn finish(&self, app: &mut App) {
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.init_resource::<resources::OutlinePostProcessPipeline>();
    }
}
