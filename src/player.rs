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

    scale: u32,
    movement_speed: i32,

    position_x: i32,
    position_y: i32,

    // Handle a list of keycodes that are pressed down. Helps when a user presses down multiple
    // keys, specifically when changing direction. This isn't ideal to manage it within the player
    // struct and this current implementation doesn't support keys outside of W, A, S, or D, so we
    // would probably want to abstract this out into our own keyboard build on top of SDL.
    keydowns: Vec<Keycode>,

    // These could be made generic across sprites.
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
            movement_speed: 12,

            position_x: 100,
            position_y: 100,

            keydowns: vec![],

            // These animation blocks should be refactored using a clever macro.
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
        self.remove_keydown(keycode);

        if self.keydowns.is_empty() {
            self.reset_animation(keycode)
        }
    }

    fn handle_keydown(&mut self, keycode: Keycode) {
        self.add_keydown(keycode);

        let animation = match keycode {
            Keycode::W => &mut self.walk_back,
            Keycode::A => &mut self.walk_left,
            Keycode::S => &mut self.walk_front,
            Keycode::D => &mut self.walk_right,
            _ => return,
        };

        if let Some(sprite) = animation.next_frame() {
            self.current_sprite = sprite.clone();

            // Here we tie movement to animation to sync the player moving on the screen and the
            // sprite changing for the animation. When doing this properly we will want this to be
            // the other way round, syncing the animation with movement.
            match keycode {
                Keycode::W => self.position_y -= self.movement_speed,
                Keycode::A => self.position_x -= self.movement_speed,
                Keycode::S => self.position_y += self.movement_speed,
                Keycode::D => self.position_x += self.movement_speed,
                _ => return,
            };
        }
    }

    // This is called when the user lifts a key, causing the player to show a `*Still` sprite.
    fn reset_animation(&mut self, keycode: Keycode) {
        self.current_sprite = match keycode {
            Keycode::W => PokemonSprite::BackStill,
            Keycode::A => PokemonSprite::LeftStill,
            Keycode::S => PokemonSprite::FrontStill,
            Keycode::D => PokemonSprite::RightStill,
            _ => return,
        }
    }

    fn add_keydown(&mut self, keycode: Keycode) {
        if !self.keydowns.contains(&keycode) {
            self.keydowns.push(keycode);
        }
    }

    fn remove_keydown(&mut self, keycode: Keycode) {
        if self.keydowns.contains(&keycode) {
            self.keydowns.remove_item(&keycode);
        }
    }
}
