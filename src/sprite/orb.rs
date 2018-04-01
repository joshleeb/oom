use sdl2::rect::Rect;
use sprite::{Spritesheet, SpritesheetLayout};

const SPRITE_SIZE: u32 = 32;

pub type OrbSpritesheet<'t> = Spritesheet<'t, OrbLayout>;

#[derive(Clone, PartialEq)]
pub enum OrbSprite {
    Small,
    Medium,
    Large,
}

pub struct OrbLayout;

impl SpritesheetLayout for OrbLayout {
    type Sprite = OrbSprite;

    fn get_dimensions() -> (u32, u32) {
        (SPRITE_SIZE, SPRITE_SIZE)
    }

    fn get_sprite(spr: &Self::Sprite) -> Rect {
        let grid = match *spr {
            OrbSprite::Small => (3, 0),
            OrbSprite::Medium => (4, 0),
            OrbSprite::Large => (5, 0),
        };

        Rect::new(
            grid.0 * SPRITE_SIZE as i32,
            grid.1 * SPRITE_SIZE as i32,
            SPRITE_SIZE as u32,
            SPRITE_SIZE as u32,
        )
    }
}
