use sdl2::rect::Rect;
use sprite::{Spritesheet, SpritesheetLayout};

const SPRITE_SIZE: u32 = 16;

pub type TileSpritesheet<'t> = Spritesheet<'t, TileLayout>;

#[derive(Clone, PartialEq)]
pub enum TileSprite {
    Cobblestone,
    Grass,
    Lava,
    Sand,
    Water,
    Wood,
}

pub struct TileLayout;

impl SpritesheetLayout for TileLayout {
    type Sprite = TileSprite;

    fn get_dimensions() -> (u32, u32) {
        (SPRITE_SIZE, SPRITE_SIZE)
    }

    fn get_sprite(spr: &Self::Sprite) -> Rect {
        let grid = match *spr {
            TileSprite::Cobblestone => (1, 0),
            TileSprite::Grass => (0, 0),
            TileSprite::Lava => (11, 18),
            TileSprite::Sand => (2, 1),
            TileSprite::Water => (10, 18),
            TileSprite::Wood => (4, 0),
        };

        Rect::new(
            grid.0 * SPRITE_SIZE as i32,
            grid.1 * SPRITE_SIZE as i32,
            SPRITE_SIZE as u32,
            SPRITE_SIZE as u32,
        )
    }
}
