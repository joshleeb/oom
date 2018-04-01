use sdl2::render::Canvas;
use sdl2::video::Window;
use world::InWorld;
use sprite::tile::{TileSprite, TileSpritesheet};

pub struct Tile<'s> {
    spritesheet: TileSpritesheet<'s>,
    sprite: TileSprite,

    scale: u32,

    pub world_posx: i32,
    pub world_posy: i32,
}

impl<'s> Tile<'s> {
    pub fn new(spritesheet: TileSpritesheet<'s>, sprite: TileSprite) -> Self {
        Tile {
            spritesheet,
            sprite,

            scale: 2,

            world_posx: 0,
            world_posy: 0,
        }
    }
}

impl<'a> InWorld for Tile<'a> {
    fn world_position(&self) -> (i32, i32) {
        (self.world_posx, self.world_posy)
    }

    fn render(&self, canvas: &mut Canvas<Window>) {
        self.spritesheet.draw_sprite_with_scale(
            canvas,
            &self.sprite,
            self.scale,
            self.world_posx,
            self.world_posy,
        );
    }
}
