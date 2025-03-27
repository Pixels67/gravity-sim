use gravity_sim::object::Object;
use gravity_sim::world::World;
use macroquad::audio::{load_sound, play_sound, PlaySoundParams};
use macroquad::prelude::*;

const WINDOW_TITLE: &str = "Gravity Sim";
const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 720;
const AA_SAMPLE_COUNT: i32 = 4;

const BG_COLOR: Color = Color {
    r: 0.06,
    g: 0.08,
    b: 0.1,
    a: 1.,
};

#[macroquad::main(config)]
async fn main() {
    let mut world = World::default();
    world.objects.push(Object::default());
    let st = load_sound("res/music/music.ogg")
        .await
        .expect("Failed to load audio file");

    play_sound(
        &st,
        PlaySoundParams {
            looped: true,
            volume: 0.35,
        },
    );

    loop {
        clear_background(BG_COLOR);
        draw_text(&get_fps().to_string(), 5., 20., 32., WHITE);

        world.update(get_frame_time());

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
