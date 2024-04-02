#import bevy_pbr::{
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
}

#ifdef PREPASS_PIPELINE
#import bevy_pbr::{
    prepass_io::{VertexOutput, FragmentOutput},
    pbr_deferred_functions::deferred_output,
}
#else
#import bevy_pbr::{
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
    utils::{random1D, PI},
}
#endif


struct StarsMaterial {
    relative_pos_x: f32,
    relative_pos_y: f32,
    _padding1: f32,
    _padding2: f32,
}

@group(2) @binding(100)
var<uniform> stars_material: StarsMaterial;

fn random(input: f32) -> f32 {
    //return fract(sin(input * 1.1) * 43758.5453);
    return fract(sin(quantize(input * 1.1, PI * 1.1)) * 3758.5453);
}
fn quantize(input: f32, quantize_size: f32) -> f32 {
    return floor(input / quantize_size) * quantize_size;
}
fn fract_range(input: f32, max: f32) -> f32 {
    return fract(input / max) * max;
}

fn hash(a: vec2<f32>, seed: f32) -> f32 {
    var h: vec3<f32> = vec3<f32>(a, seed);
    h = fract(h * 0.1031);
    h += dot(h, h.yzx + 33.33);
    return fract((h.x + h.y) * h.z);
}

// Use this function to generate a random number based on two f32 values
fn random2d(x: f32, y: f32, seed: f32) -> f32 {
    return hash(vec2<f32>(x, y), seed);
}

fn draw_star(in: VertexOutput, speed: f32, propability: f32, seed: f32) -> bool {
    let x = in.position.x + in.world_position.x * speed + stars_material.relative_pos_x * speed;
    let y = in.position.y - in.world_position.y * speed + stars_material.relative_pos_y * speed;
    let random_value = random2d(quantize(x, 1.5), quantize(y, 1.5), seed);
    if random_value < propability {
        return true;
    } else {
        return false;
    }
}

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
#ifdef PREPASS_PIPELINE
    // in deferred mode we can't modify anything after that, as lighting is run in a separate fullscreen shader.
    let out = deferred_output(in, pbr_input);
#else
    var out: FragmentOutput;
    // apply lighting
    //out.color = apply_pbr_lighting(pbr_input);

    let position = in.world_position;
    //if draw_star(in, 0.0, 0.0001) {
    //    out.color = vec4<f32>(1.0, 1.0, 1.0, 1.0);
    //}
    if draw_star(in, 0.1, 0.00001, 1.0) {
        out.color = vec4<f32>(1.0, 1.0, 1.0, 1.0);
    }
    if draw_star(in, 0.11, 0.00001, 2.0) {
        out.color = vec4<f32>(1.0, 1.0, 1.0, 1.0);
    }
    if draw_star(in, 0.12, 0.00001, 3.0) {
        out.color = vec4<f32>(1.0, 1.0, 1.0, 1.0);
    }

    // we can optionally modify the lit color before post-processing is applied
    //out.color = vec4<f32>(vec4<u32>(out.color * f32(my_extended_material.quantize_steps))) / f32(my_extended_material.quantize_steps);

    // apply in-shader post processing (fog, alpha-premultiply, and also tonemapping, debanding if the camera is non-hdr)
    // note this does not include fullscreen postprocessing effects like bloom.
    //out.color = main_pass_post_lighting_processing(pbr_input, out.color);

    // we can optionally modify the final result here
    //out.color = out.color * 2.0;
#endif

    return out;
}