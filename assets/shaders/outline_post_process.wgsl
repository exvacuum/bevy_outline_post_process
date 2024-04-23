#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;

struct OutlinePostProcessSettings {
	weight: f32,
	threshold: f32,
#ifdef SIXTEEN_BYTE_ALIGNMENT
	_padding: vec2<f32>,
#endif
}

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var screen_sampler: sampler;
@group(0) @binding(2) var normal_texture: texture_2d<f32>;
@group(0) @binding(3) var normal_sampler: sampler;
@group(0) @binding(4) var<uniform> settings: OutlinePostProcessSettings;

@fragment
fn fragment(
	in: FullscreenVertexOutput
) -> @location(0) vec4<f32> {
    let screen_color = textureSample(screen_texture, screen_sampler, in.uv);
    let outline_width = settings.weight / vec2f(textureDimensions(screen_texture)); 

    let uv_top = vec2f(in.uv.x, in.uv.y - outline_width.y);
    let uv_right = vec2f(in.uv.x + outline_width.x, in.uv.y);
    let uv_top_right = vec2f(in.uv.x + outline_width.x, in.uv.y - outline_width.y);

    let normal = textureSample(normal_texture, normal_sampler, in.uv);
    let normal_top = textureSample(normal_texture, normal_sampler, uv_top);
    let normal_right = textureSample(normal_texture, normal_sampler, uv_right);
    let normal_top_right = textureSample(normal_texture, normal_sampler, uv_top_right);

    let delta_top = abs(normal - normal_top);
    let delta_right = abs(normal - normal_right);
    let delta_top_right = abs(normal - normal_top_right);

    let delta_max = max(delta_top, max(delta_right, delta_top_right));
    let delta_raw = max(delta_max.x, max(delta_max.y, delta_max.z));

    let delta_clipped = clamp((delta_raw * 2.0) - settings.threshold, 0.0, 1.0);

    let outline = vec4f(delta_clipped, delta_clipped, delta_clipped, 0.0);

    return screen_color - outline;
}
