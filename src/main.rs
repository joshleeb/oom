extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::surface::Surface;
use sdl2::video::Window;
use std::path::Path;
use std::thread;
use std::time::Duration;

static CHARACTER_SPRITE_PATH: &'static str = "assets/character.bmp";

const FRAMES_PER_ANIMATION: i32 = 4;
const SCREEN_HEIGHT: u32 = 600;
const SCREEN_WIDTH: u32 = 800;
const SPRITE_TILE_SIZE: u32 = 32;
const SCALE: u32 = 4;

fn select_sprite(sprite_rect: &mut Rect, ticks: i32) {
    sprite_rect.set_x(SPRITE_TILE_SIZE as i32 * ((ticks / 100) % FRAMES_PER_ANIMATION));
}

fn clear_canvas(canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
}

fn main() {
    let sdl_ctx = sdl2::init().unwrap();
    let video_subsystem = sdl_ctx.video().unwrap();

    let window = video_subsystem
        .window("game window", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().accelerated().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let character_surface = Surface::load_bmp(Path::new(CHARACTER_SPRITE_PATH)).unwrap();
    let texture = texture_creator
        .create_texture_from_surface(&character_surface)
        .unwrap();

    let mut timer = sdl_ctx.timer().unwrap();
    let mut events = sdl_ctx.event_pump().unwrap();

    // Character walk animation sprite rect.
    let mut sprite = Rect::new(0, 0, SPRITE_TILE_SIZE, SPRITE_TILE_SIZE);

    // Rect on screen the sprite will be drawn into
    let mut screen_rect = Rect::new(0, 0, SPRITE_TILE_SIZE * SCALE, SPRITE_TILE_SIZE * SCALE);
    screen_rect.center_on(Point::new(
        (SCREEN_WIDTH - SPRITE_TILE_SIZE) as i32 / 2,
        (SCREEN_HEIGHT - SPRITE_TILE_SIZE) as i32 / 2,
    ));

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

        select_sprite(&mut sprite, timer.ticks() as i32);
        clear_canvas(&mut canvas);
        canvas
            .copy(&texture, Some(sprite), Some(screen_rect))
            .unwrap();
        canvas.present();

        thread::sleep(Duration::from_millis(100));
    }
}
