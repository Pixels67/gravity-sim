use macroquad::prelude::*;
use gravity_sim::object::*;
use gravity_sim::world::*;

const BG_COLOR: Color = Color{r: 0.05, g: 0.05, b: 0.05, a: 1.};
const GRAV_CONST: f32 = 1.;

#[macroquad::main("Sim")]
async fn main() {
    let mut world = World::new(GRAV_CONST);

    world.add_object(Object::new(vec3( 5., 0., 0.), vec3(0., 0.,  0.025), 1., 0.5, RED));
    world.add_object(Object::new(vec3(-5., 0., 0.), vec3(0., 0., -0.025), 1., 0.5, BLUE));

    loop {
        clear_background(BG_COLOR);

        world.update(get_frame_time());
        world.draw_all();

        next_frame().await
    }
}
