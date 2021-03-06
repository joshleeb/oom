#![feature(vec_remove_item)]
#![feature(iterator_step_by)]
#![feature(nll)]

extern crate sdl2;

use player::Player;
use sdl2::event::Event;
use map::Map;
use sdl2::keyboard::{KeyboardState, Keycode};
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use world::World;
use sprite::orb::OrbSpritesheet;
use sprite::tile::TileSpritesheet;
use std::thread;
use std::time::Duration;

mod animation;
mod camera;
mod map;
mod player;
mod sprite;
mod tile;
mod world;

const SCREEN_HEIGHT: u32 = 800;
const SCREEN_WIDTH: u32 = 1200;

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

    let tile_spritesheet = TileSpritesheet::new(&texture_creator, "assets/tiles.png");
    let mut world = World::new(Map::from_pixelmap(&tile_spritesheet, "assets/map.png"));

    let player_spritesheet = OrbSpritesheet::new(&texture_creator, "assets/orb.png");
    let mut player = Player::new(&player_spritesheet, world.dimensions(), 0, 0);

    clear_canvas(&mut canvas);
    canvas.present();

    let mut show_perimeter = false;

    'main: loop {
        for event in events.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::P),
                    ..
                } => show_perimeter = !show_perimeter,
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,
                _ => {}
            };

            world.event_update(&event);
        }
        world.update();
        player.update(&KeyboardState::new(&events));

        clear_canvas(&mut canvas);
        camera::render(&mut canvas, &player, &world, show_perimeter);
        canvas.present();

        thread::sleep(Duration::from_millis(1000 / 60));
    }
}
