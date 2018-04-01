use sdl2::render::Canvas;
use sdl2::video::Window;
use world::InWorld;
use sdl2::rect::Rect;
use sprite::tile::{TileLayout, TileSprite, TileSpritesheet};
use sprite::SpritesheetLayout;

pub struct Tile<'s> {
    spritesheet: &'s TileSpritesheet<'s>,
    sprite: TileSprite,

    scale: u32,

    pub world_posx: i32,
    pub world_posy: i32,
}

impl<'s> Tile<'s> {
    pub fn new(
        spritesheet: &'s TileSpritesheet<'s>,
        sprite: TileSprite,
        world_posx: i32,
        world_posy: i32,
    ) -> Self {
        Tile {
            spritesheet,
            sprite,
            scale: 2,
            world_posx,
            world_posy,
        }
    }
}

impl<'a> InWorld for Tile<'a> {
    fn world_rect(&self) -> Rect {
        let dimensions = <TileLayout as SpritesheetLayout>::get_dimensions();
        Rect::new(self.world_posx, self.world_posy, dimensions.0, dimensions.1)
    }

    fn render(&self, canvas: &mut Canvas<Window>, screen_x: i32, screen_y: i32) {
        self.spritesheet.draw_sprite_with_scale(
            canvas,
            &self.sprite,
            self.scale,
            screen_x,
            screen_y,
        );
    }
}
