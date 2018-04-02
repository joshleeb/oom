use player::Player;
use sdl2::render::Canvas;
use sdl2::rect::Rect;
use std::cmp;
use sdl2::video::Window;
use world::World;

pub fn render(canvas: &mut Canvas<Window>, player: &Player, world: &World, show_perimeter: bool) {
    let screen_size = canvas.output_size().unwrap();
    let world_pos = world_position(world, player, screen_size);
    let viewport = Rect::new(world_pos.0, world_pos.1, screen_size.0, screen_size.1);

    world.render(canvas, viewport);
    player.render(canvas, viewport, show_perimeter);
}

fn world_position(world: &World, player: &Player, screen_size: (u32, u32)) -> (i32, i32) {
    (
        // Camera world position X.
        cmp::max(
            0,
            cmp::min(
                (world.dimensions().0 - screen_size.0) as i32,
                player.world_position().0 - (screen_size.0 as i32) / 2,
            ),
        ),
        // Camera world position Y.
        cmp::max(
            0,
            cmp::min(
                (world.dimensions().1 - screen_size.1) as i32,
                player.world_position().1 - (screen_size.1 as i32) / 2,
            ),
        ),
    )
}
