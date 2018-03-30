use sdl2::rect::Rect;
use sprite::{Spritesheet, SpritesheetLayout};

pub type PokemonSpritesheet<'t> = Spritesheet<'t, PokemonLayout>;

#[derive(PartialEq)]
pub enum PokemonSprite {
    Front,
    FrontWalk,

    Back,
    BackWalk,

    Left,
    LeftWalk,

    Right,
    RightWalk,
}

pub struct PokemonLayout;

const SPRITE_SIZE: usize = 32;

impl SpritesheetLayout for PokemonLayout {
    type Sprite = PokemonSprite;

    fn get_dimensions() -> (usize, usize) {
        (SPRITE_SIZE, SPRITE_SIZE)
    }

    fn get_sprite(spr: &Self::Sprite) -> Rect {
        let grid = match *spr {
            PokemonSprite::Front => (2, 1),
            PokemonSprite::FrontWalk => (2, 2),

            PokemonSprite::Back => (0, 0),
            PokemonSprite::BackWalk => (1, 3),

            PokemonSprite::Left => (0, 2),
            PokemonSprite::LeftWalk => (0, 3),

            PokemonSprite::Right => (1, 0),
            PokemonSprite::RightWalk => (1, 1),
        };

        Rect::new(
            grid.0 * SPRITE_SIZE as i32,
            grid.1 * SPRITE_SIZE as i32,
            SPRITE_SIZE as u32,
            SPRITE_SIZE as u32,
        )
    }
}
