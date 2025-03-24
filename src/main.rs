use macroquad::prelude::*;
use gravity_sim::object::*;

#[macroquad::main("Sim")]
async fn main() {
    let vert_shader = String::from_utf8(load_file("res/default.vert").await.unwrap()).unwrap();
    let frag_shader = String::from_utf8(load_file("res/default.frag").await.unwrap()).unwrap();

    let shader = ShaderSource::Glsl {
        vertex: vert_shader.as_str(),
        fragment: frag_shader.as_str(),
    };

    let params = MaterialParams {
        uniforms: vec![
            UniformDesc::new("light_pos", UniformType::Float3),
            UniformDesc::new("color", UniformType::Float4),
            UniformDesc::new("ambient_light", UniformType::Float1),
        ],
        ..Default::default()
    };

    let material = load_material(shader, params).unwrap();
    material.set_uniform("light_pos", vec3(0., 10., 10.));
    material.set_uniform("ambient_light", 0.15f32);

    let mut pool = ObjectPool::new();
    pool.push(Object::default());

    loop {
        clear_background(BLACK);

        set_camera(&Camera3D {
            position: vec3(15., 10., 0.),
            up: vec3(0., 1., 0.),
            target: vec3(0., 0., 0.),
            ..Default::default()
        });

        gl_use_material(&material);

        material.set_uniform("color", vec4(1., 1., 1., 1.));

        pool.draw_all();

        gl_use_default_material();
        set_default_camera();
        next_frame().await
    }
}
