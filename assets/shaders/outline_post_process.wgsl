#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;
#import bevy_pbr::{ rgb9e5 };

struct OutlinePostProcessSettings {
	weight: f32,
	normal_threshold: f32,
    	adaptive: u32,
	light_threshold: f32,
}

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var screen_sampler: sampler;
@group(0) @binding(2) var normal_texture: texture_2d<f32>;
@group(0) @binding(3) var normal_sampler: sampler;
// @group(0) @binding(4) var deferred_texture: texture_2d<u32>;
@group(0) @binding(5) var<uniform> settings: OutlinePostProcessSettings;

@fragment
fn fragment(
	in: FullscreenVertexOutput
) -> @location(0) vec4<f32> {
    let screen_color = textureSample(screen_texture, screen_sampler, in.uv);
    // let deferred_dimensions = textureDimensions(deferred_texture);
    // let deferred_texel = textureLoad(deferred_texture, vec2u(vec2f(deferred_dimensions) * in.uv), 0);
    // let deferred_color = vec4f(vec3f(unpack4x8unorm(deferred_texel.r).rgb), 1.0);
    // let emissive = vec4f(rgb9e5::rgb9e5_to_vec3_(deferred_texel.g), 1.0);
    // let light = screen_color / ((deferred_color) + vec4f(0.001, 0.001, 0.001, 1.0));

    let luma = (0.2126 * screen_color.r + 0.7152 * screen_color.g + 0.0722 * screen_color.b);
    // let light_luma = (0.2126 * light.r + 0.7152 * light.g + 0.0722 * light.b);
    // let emissive_luma = (0.2126 * emissive.r + 0.7152 * emissive.g + 0.0722 * emissive.b);
    let final_luma = luma ;
	// + light_luma + emissive_luma;

	// if final_luma > settings.light_threshold {
	    let outline_width = settings.weight / vec2f(textureDimensions(screen_texture)); 

	    let uv_top = vec2f(in.uv.x, in.uv.y - outline_width.y);
	    let uv_right = vec2f(in.uv.x + outline_width.x, in.uv.y);
	    let uv_top_right = vec2f(in.uv.x + outline_width.x, in.uv.y - outline_width.y);

	    let normal = textureSample(normal_texture, normal_sampler, in.uv);
	    let normal_top = textureSample(normal_texture, normal_sampler, uv_top);
	    let normal_right = textureSample(normal_texture, normal_sampler, uv_right);
	    let normal_top_right = textureSample(normal_texture, normal_sampler, uv_top_right);

	    let normal_delta_top = abs(normal - normal_top);
	    let normal_delta_right = abs(normal - normal_right);
	    let normal_delta_top_right = abs(normal - normal_top_right);

	    let normal_delta_max = max(normal_delta_top, max(normal_delta_right, normal_delta_top_right));
	    let normal_delta_raw = max(normal_delta_max.x, max(normal_delta_max.y, normal_delta_max.z));

	    let show_outline = f32(normal_delta_raw > settings.normal_threshold);

	    var outline = vec4f(show_outline, show_outline, show_outline, 0.0);
	    if settings.adaptive != 0 && luma < 0.5 {
		outline = outline * -1;
	    }
	    return screen_color - outline;
	// + emissive;
	// }
	// return screen_color + emissive;
}
