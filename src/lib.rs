#![warn(missing_docs)]

//! A plugin for the Bevy game engine which provides an outline post-process effect. The effect
//! makes use of a normal prepass to generate outlines where differences in the normal buffer
//! occur.

use bevy::{
    asset::embedded_asset,
    core_pipeline::core_3d::graph::{Core3d, Node3d},
    prelude::*,
    render::{
        extract_component::{ExtractComponentPlugin, UniformComponentPlugin},
        render_graph::{RenderGraphApp, ViewNodeRunner},
        RenderApp,
    },
};

pub use nodes::OutlineRenderLabel;

/// Components used by this plugin.
pub mod components;
mod nodes;
mod resources;

/// Plugin which provides an outline post-processing effect.
pub struct OutlinePostProcessPlugin;

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

        render_app
            .add_render_graph_node::<ViewNodeRunner<nodes::OutlineRenderNode>>(
                Core3d,
                nodes::OutlineRenderLabel,
            )
            .add_render_graph_edges(
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
