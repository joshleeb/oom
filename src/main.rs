#![feature(vec_remove_item)]
#![feature(nll)]

extern crate sdl2;

use player::Player;
use sdl2::event::Event;
use tile::Tile;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use world::World;
use sprite::orb::OrbSpritesheet;
use sprite::tile::{TileSprite, TileSpritesheet};
use std::thread;
use std::time::Duration;

mod animation;
mod tile;
mod player;
mod sprite;
mod world;

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

    let mut world = World::new();

    let player_spritesheet = OrbSpritesheet::from_spritesheet(&texture_creator, "assets/orb.png");
    let mut player = Player::new(&player_spritesheet);
    world.add_item(Box::new(&mut player));

    let tile_spritesheet = TileSpritesheet::from_spritesheet(&texture_creator, "assets/tiles.png");
    let mut grass = Tile::new(&tile_spritesheet, TileSprite::Grass);
    world.add_item(Box::new(&mut grass));

    clear_canvas(&mut canvas);
    canvas.present();

    'main: loop {
        world.update();

        for event in events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,
                _ => {}
            };

            world.update_with_event(&event);
        }

        clear_canvas(&mut canvas);
        world.render(&mut canvas);
        canvas.present();

        thread::sleep(Duration::from_millis(1000 / 60));
    }
}
