use macroquad::prelude::*;

#[macroquad::main("Sim")]
async fn main() {
    let vert_shader = String::from_utf8(load_file("src/default.vert").await.unwrap()).unwrap();
    let frag_shader = String::from_utf8(load_file("src/default.frag").await.unwrap()).unwrap();

    let shader = ShaderSource::Glsl {
        vertex: vert_shader.as_str(),
        fragment: frag_shader.as_str(),
    };

    let material = load_material(shader, MaterialParams::default()).unwrap();

    loop {
        clear_background(BLACK);

        set_camera(&Camera3D {
            position: vec3(-15., 10., 0.),
            up: vec3(0., 1., 0.),
            target: vec3(0., 0., 0.),
            ..Default::default()
        });

        gl_use_material(&material);

        draw_sphere(Vec3::new(0., 0., 0.), 1., None, WHITE);

        gl_use_default_material();
        set_default_camera();
        next_frame().await
    }
}
