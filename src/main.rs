extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sprite::Spritesheet;
use sprite::pokemon::{PokemonSprite, PokemonSpritesheet};
use std::thread;
use std::time::Duration;

mod sprite;

const SCREEN_HEIGHT: u32 = 600;
const SCREEN_WIDTH: u32 = 800;

fn clear_canvas(canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
}

fn main() {
    let sdl_ctx = sdl2::init().unwrap();
    let video_subsystem = sdl_ctx.video().unwrap();

    let mut events = sdl_ctx.event_pump().unwrap();

    let window = video_subsystem
        .window("game window", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().accelerated().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let professor = Spritesheet::<PokemonSpritesheet>::from_spritesheet(
        &texture_creator,
        "assets/professor.bmp",
    );

    clear_canvas(&mut canvas);
    canvas.present();

    'main: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,
                _ => {}
            }
        }

        clear_canvas(&mut canvas);

        professor.draw_sprite(&mut canvas, PokemonSprite::Back, 0, 0);
        professor.draw_sprite_with_scale(&mut canvas, PokemonSprite::Front, 4, 100, 100);

        canvas.present();

        thread::sleep(Duration::from_millis(100));
    }
}
