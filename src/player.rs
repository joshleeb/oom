use animation::Animation;
use sdl2::render::Canvas;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2::video::Window;
use sprite::SpritesheetLayout;
use sprite::orb::{OrbLayout, OrbSprite, OrbSpritesheet};
use std::time::Duration;

const SCALE: u32 = 2;

pub struct Player<'s> {
    spritesheet: &'s OrbSpritesheet<'s>,
    current_sprite: OrbSprite,
    pub movement_speed: i32,
    pub world_posx: i32,
    pub world_posy: i32,
    animation: Animation<OrbSprite>,
}

impl<'s> Player<'s> {
    pub fn new(spritesheet: &'s OrbSpritesheet<'s>) -> Self {
        Player {
            spritesheet,
            current_sprite: OrbSprite::Large,

            movement_speed: 12,

            world_posx: 100,
            world_posy: 100,

            animation: Animation::new(
                vec![
                    OrbSprite::Large,
                    OrbSprite::Medium,
                    OrbSprite::Small,
                    OrbSprite::Medium,
                ],
                Duration::from_millis(150),
            ),
        }
    }

    pub fn update(&mut self) {
        if let Some(sprite) = self.animation.next_frame() {
            self.current_sprite = sprite.clone();
        }
    }

    pub fn update_with_event(&mut self, event: &Event) {
        if let Event::KeyDown {
            keycode: Some(keycode),
            ..
        } = event
        {
            self.handle_keydown(keycode)
        }
    }

    pub fn world_position(&self) -> (i32, i32) {
        (self.world_posx, self.world_posy)
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        let screen_size = canvas.output_size().unwrap();
        let sprite_size = <OrbLayout as SpritesheetLayout>::get_dimensions();

        self.spritesheet.draw_sprite_with_scale(
            canvas,
            &self.current_sprite,
            SCALE,
            ((screen_size.0 - sprite_size.0) / 2) as i32, // Screen posX
            ((screen_size.1 - sprite_size.1) / 2) as i32, // Screen posY
        );
    }

    fn handle_keydown(&mut self, keycode: &Keycode) {
        match keycode {
            Keycode::W => self.world_posy -= self.movement_speed,
            Keycode::A => self.world_posx -= self.movement_speed,
            Keycode::S => self.world_posy += self.movement_speed,
            Keycode::D => self.world_posx += self.movement_speed,
            _ => {}
        }
    }
}
