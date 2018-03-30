use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sprite::pokemon::{PokemonSprite, PokemonSpritesheet};

pub struct Player<'s> {
    spritesheet: PokemonSpritesheet<'s>,
    current_sprite: PokemonSprite,
}

impl<'s> Player<'s> {
    pub fn new(spritesheet: PokemonSpritesheet<'s>) -> Self {
        Player {
            spritesheet,
            current_sprite: PokemonSprite::Front,
        }
    }

    pub fn update(&mut self, event: Event) {
        match event {
            Event::KeyDown {
                keycode: Some(keycode),
                ..
            } => self.handle_keydown(keycode),
            Event::KeyUp {
                keycode: Some(keycode),
                ..
            } => self.handle_keyup(keycode),
            _ => {}
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        self.spritesheet
            .draw_sprite_with_scale(canvas, &self.current_sprite, 4, 100, 100)
    }

    fn handle_keyup(&mut self, keycode: Keycode) {
        match keycode {
            Keycode::W => self.current_sprite = PokemonSprite::Back,
            Keycode::A => self.current_sprite = PokemonSprite::Left,
            Keycode::S => self.current_sprite = PokemonSprite::Front,
            Keycode::D => self.current_sprite = PokemonSprite::Right,
            _ => {},
        };
    }

    fn handle_keydown(&mut self, keycode: Keycode) {
        let (still, walk) = match keycode {
            Keycode::W => (PokemonSprite::Back, PokemonSprite::BackWalk),
            Keycode::A => (PokemonSprite::Left, PokemonSprite::LeftWalk),
            Keycode::S => (PokemonSprite::Front, PokemonSprite::FrontWalk),
            Keycode::D => (PokemonSprite::Right, PokemonSprite::RightWalk),
            _ => return,
        };

        if self.current_sprite == still {
            self.current_sprite = walk;
        } else {
            self.current_sprite = still;
        }
    }
}
