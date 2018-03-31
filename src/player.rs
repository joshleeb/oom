use animation::Animation;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sprite::pokemon::{PokemonSprite, PokemonSpritesheet};
use std::time::Duration;

type PokemonAnimation = Animation<PokemonSprite>;

pub struct Player<'s> {
    spritesheet: PokemonSpritesheet<'s>,
    current_sprite: PokemonSprite,

    scale: usize,
    position_x: usize,
    position_y: usize,

    walk_back: PokemonAnimation,
    walk_left: PokemonAnimation,
    walk_front: PokemonAnimation,
    walk_right: PokemonAnimation,
}

impl<'s> Player<'s> {
    pub fn new(spritesheet: PokemonSpritesheet<'s>) -> Self {
        Player {
            spritesheet,
            current_sprite: PokemonSprite::FrontStill,

            scale: 4,
            position_x: 100,
            position_y: 100,

            walk_back: PokemonAnimation::new(
                vec![
                    PokemonSprite::BackStill,
                    PokemonSprite::BackWalkLeft,
                    PokemonSprite::BackStill,
                    PokemonSprite::BackWalkRight,
                ],
                Duration::from_millis(150),
            ),
            walk_left: PokemonAnimation::new(
                vec![
                    PokemonSprite::LeftStill,
                    PokemonSprite::LeftWalkLeft,
                    PokemonSprite::LeftStill,
                    PokemonSprite::LeftWalkRight,
                ],
                Duration::from_millis(150),
            ),
            walk_front: PokemonAnimation::new(
                vec![
                    PokemonSprite::FrontStill,
                    PokemonSprite::FrontWalkLeft,
                    PokemonSprite::FrontStill,
                    PokemonSprite::FrontWalkRight,
                ],
                Duration::from_millis(150),
            ),
            walk_right: PokemonAnimation::new(
                vec![
                    PokemonSprite::RightStill,
                    PokemonSprite::RightWalkLeft,
                    PokemonSprite::RightStill,
                    PokemonSprite::RightWalkRight,
                ],
                Duration::from_millis(150),
            ),
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
        self.spritesheet.draw_sprite_with_scale(
            canvas,
            &self.current_sprite,
            self.scale,
            self.position_x,
            self.position_y,
        );
    }

    fn handle_keyup(&mut self, keycode: Keycode) {
        match keycode {
            Keycode::W | Keycode::A | Keycode::S | Keycode::D => self.reset_animation(keycode),
            _ => {}
        };
    }

    fn handle_keydown(&mut self, keycode: Keycode) {
        let animation = match keycode {
            Keycode::W => &mut self.walk_back,
            Keycode::A => &mut self.walk_left,
            Keycode::S => &mut self.walk_front,
            Keycode::D => &mut self.walk_right,
            _ => return,
        };

        if let Some(sprite) = animation.next_frame() {
            self.current_sprite = sprite.clone();
        }
    }

    fn reset_animation(&mut self, keycode: Keycode) {
        self.current_sprite = match keycode {
            Keycode::W => PokemonSprite::BackStill,
            Keycode::A => PokemonSprite::LeftStill,
            Keycode::S => PokemonSprite::FrontStill,
            Keycode::D => PokemonSprite::RightStill,
            _ => return,
        }
    }
}
