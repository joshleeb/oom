extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

fn empty_point() -> Point {
    Point::new(0, 0)
}

fn draw_points(canvas: &mut Canvas<Window>, points: &mut Vec<Point>) {
    // Clear canvas or else we get some weird behaviour with a red background.
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    // Draw all the points. These draw commands get put into a back buffer and
    // don't actually draw to the canvas until we call `present`.
    for point in points {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.draw_line(empty_point(), point.clone()).unwrap();
    }

    // Actually draw to the canvas, everything in the back buffer.
    canvas.present();
}

fn main() {
    let sdl_ctx = sdl2::init().unwrap();
    let video_subsystem = sdl_ctx.video().unwrap();

    let window = video_subsystem
        .window("game window", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut events = sdl_ctx.event_pump().unwrap();
    let mut points: Vec<Point> = vec![];

    'main: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,
                Event::MouseButtonDown { x, y, .. } => {
                    println!("point {:?}", (x, y));

                    points.push(Point::new(x, y));
                    draw_points(&mut canvas, &mut points);
                }
                _ => {}
            }
        }
    }
}
