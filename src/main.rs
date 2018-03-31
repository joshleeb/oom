#![feature(vec_remove_item)]

extern crate sdl2;

use player::Player;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sprite::pokemon::PokemonSpritesheet;
use std::thread;
use std::time::Duration;

mod animation;
mod player;
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

    let spritesheet =
        PokemonSpritesheet::from_spritesheet(&texture_creator, "assets/professor.png");
    let mut player = Player::new(spritesheet);

    clear_canvas(&mut canvas);
    canvas.present();

    'main: loop {
        // let pressed_keycodes: Vec<Keycode> = events
        //     .keyboard_state()
        //     .pressed_scancodes()
        //     .map(|s| Keycode::from_scancode(s))
        //     .filter(|v| !v.is_none())
        //     .map(|v| v.unwrap())
        //     .collect();

        for event in events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,
                _ => {}
            };

            player.update(event);
        }

        clear_canvas(&mut canvas);

        player.render(&mut canvas);
        canvas.present();

        thread::sleep(Duration::from_millis(1000 / 60));
    }
}
