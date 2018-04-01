use player::Player;
use sdl2::render::Canvas;
use sdl2::rect::Rect;
use sdl2::video::Window;
use world::World;

pub fn render(canvas: &mut Canvas<Window>, player: &Player, world: &World) {
    let screen_size = canvas.output_size().unwrap();
    let world_pos = world_position(player, screen_size);
    let viewport = Rect::new(world_pos.0, world_pos.1, screen_size.0, screen_size.1);

    world.render(canvas, viewport);
    player.render(canvas);
}

fn world_position(player: &Player, screen_size: (u32, u32)) -> (i32, i32) {
    let player_pos = player.world_position();
    (
        player_pos.0 - (screen_size.0 as i32) / 2,
        player_pos.1 - (screen_size.1 as i32) / 2,
    )
}
