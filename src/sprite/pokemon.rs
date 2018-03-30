use sdl2::rect::Rect;
use sprite::SpritesheetLayout;

pub enum PokemonSprite {
    Front,
    Back,
}

pub struct PokemonSpritesheet;

const SPRITE_SIZE: usize = 32;

impl SpritesheetLayout for PokemonSpritesheet {
    type Sprite = PokemonSprite;

    fn get_dimensions() -> (usize, usize) {
        (SPRITE_SIZE, SPRITE_SIZE)
    }

    fn get_sprite(spr: Self::Sprite) -> Rect {
        let grid = match spr {
            PokemonSprite::Back => (0, 0),
            PokemonSprite::Front => (2, 1),
        };

        Rect::new(
            grid.0 * SPRITE_SIZE as i32,
            grid.1 * SPRITE_SIZE as i32,
            SPRITE_SIZE as u32,
            SPRITE_SIZE as u32,
        )
    }
}
