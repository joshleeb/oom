use sdl2::rect::Rect;
use sprite::{Spritesheet, SpritesheetLayout};

pub type PokemonSpritesheet<'t> = Spritesheet<'t, PokemonLayout>;

#[derive(Clone, PartialEq)]
pub enum PokemonSprite {
    FrontStill,
    FrontWalkLeft,
    FrontWalkRight,

    BackStill,
    BackWalkLeft,
    BackWalkRight,

    LeftStill,
    LeftWalkLeft,
    LeftWalkRight,

    RightStill,
    RightWalkLeft,
    RightWalkRight,
}

pub struct PokemonLayout;

const SPRITE_SIZE: u32 = 32;

impl SpritesheetLayout for PokemonLayout {
    type Sprite = PokemonSprite;

    fn get_dimensions() -> (u32, u32) {
        (SPRITE_SIZE, SPRITE_SIZE)
    }

    fn get_sprite(spr: &Self::Sprite) -> Rect {
        let grid = match *spr {
            PokemonSprite::FrontStill => (2, 1),
            PokemonSprite::FrontWalkLeft => (2, 2),
            PokemonSprite::FrontWalkRight => (2, 3),

            PokemonSprite::BackStill => (0, 0),
            PokemonSprite::BackWalkLeft => (1, 3),
            PokemonSprite::BackWalkRight => (2, 0),

            PokemonSprite::LeftStill => (0, 2),
            PokemonSprite::LeftWalkLeft => (0, 3),
            PokemonSprite::LeftWalkRight => (0, 1),

            PokemonSprite::RightStill => (1, 0),
            PokemonSprite::RightWalkLeft => (1, 1),
            PokemonSprite::RightWalkRight => (1, 2),
        };

        Rect::new(
            grid.0 * SPRITE_SIZE as i32,
            grid.1 * SPRITE_SIZE as i32,
            SPRITE_SIZE as u32,
            SPRITE_SIZE as u32,
        )
    }
}
