use macroquad::miniquad::window::*;
use macroquad::prelude::*;
use gravity_sim::object::*;
use gravity_sim::world::*;

const BG_COLOR: Color = Color{r: 0.05, g: 0.05, b: 0.05, a: 1.};
const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;
const GRAV_CONST: f32 = 0.5;
const UPDATE_INTERVAL: f32 = 0.001;

#[macroquad::main("Sim")]
async fn main() {
    let mut world = World::new(GRAV_CONST, UPDATE_INTERVAL);
    set_window_size(WINDOW_WIDTH, WINDOW_HEIGHT);

    world.add_object(Object::new(vec3( 5., 0., 0.), vec3(0., 0.,  0.01), 1., 0.5, RED));
    world.add_object(Object::new(vec3( 0., 0., 0.), vec3(0., 0.,  0.),   1., 0.5, GREEN));
    world.add_object(Object::new(vec3(-5., 0., 0.), vec3(0., 0., -0.01), 1., 0.5, BLUE));

    loop {
        clear_background(BG_COLOR);
        draw_text(&get_fps().to_string(), 5., 20., 32., WHITE);

        world.update(get_frame_time());
        world.draw_all();

        next_frame().await
    }
}
