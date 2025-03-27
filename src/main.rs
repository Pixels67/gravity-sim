use gravity_sim::object::Object;
use gravity_sim::world::World;
use macroquad::prelude::*;

const WINDOW_TITLE: &str = "Gravity Sim";
const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 720;
const AA_SAMPLE_COUNT: i32 = 8;

const GRAV_CONST: f32 = 0.5;
const UPDATE_INTERVAL: f32 = 0.01;
const BG_COLOR: Color = Color {
    r: 0.06,
    g: 0.08,
    b: 0.1,
    a: 1.,
};

#[macroquad::main(config)]
async fn main() {
    let mut world = World::new(GRAV_CONST, UPDATE_INTERVAL);

    world.add_object(Object::new(
        vec3(0., 0., 0.),
        vec3(0., 0., 0.),
        1.,
        0.5,
        GREEN,
    ));

    loop {
        clear_background(BG_COLOR);
        draw_text(&get_fps().to_string(), 5., 20., 32., WHITE);

        world.update(get_frame_time());
        world.draw_all();

        next_frame().await;
    }
}

fn config() -> Conf {
    Conf {
        window_title: WINDOW_TITLE.to_string(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        sample_count: AA_SAMPLE_COUNT,
        ..Default::default()
    }
}
