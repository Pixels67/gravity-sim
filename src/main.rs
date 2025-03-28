use gravity_sim::object::Object;
use gravity_sim::world::World;
use macroquad::audio::{load_sound, play_sound, PlaySoundParams};
use macroquad::prelude::*;
use std::env;

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

    play_music().await;

    loop {
        clear_background(BG_COLOR);

        world.update(get_frame_time());
        draw_text(&get_fps().to_string(), 5., 20., 32., WHITE);

        next_frame().await;
    }
}

#[cfg(target_arch = "wasm32")]
async fn play_music() {
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
}

#[cfg(not(target_arch = "wasm32"))]
async fn play_music() {
    let st = load_sound(
        env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .join("res/music/music.ogg")
            .to_str()
            .unwrap(),
    )
    .await
    .expect("Failed to load audio file");

    play_sound(
        &st,
        PlaySoundParams {
            looped: true,
            volume: 0.35,
        },
    );
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
