use animation::Animation;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sprite::orb::{OrbLayout, OrbSprite, OrbSpritesheet};
use sprite::SpritesheetLayout;
use std::time::Duration;

pub struct Player<'s> {
    spritesheet: OrbSpritesheet<'s>,
    current_sprite: OrbSprite,

    scale: u32,
    pub movement_speed: i32,

    pub world_posx: i32,
    pub world_posy: i32,

    animation: Animation<OrbSprite>,
}

impl<'s> Player<'s> {
    pub fn new(spritesheet: OrbSpritesheet<'s>) -> Self {
        Player {
            spritesheet,
            current_sprite: OrbSprite::Large,

            scale: 2,
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

    pub fn update_animation(&mut self) {
        if let Some(sprite) = self.animation.next_frame() {
            self.current_sprite = sprite.clone();
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        let screen_size = canvas.output_size().unwrap();
        let sprite_size = <OrbLayout as SpritesheetLayout>::get_dimensions();

        self.spritesheet.draw_sprite_with_scale(
            canvas,
            &self.current_sprite,
            self.scale,
            ((screen_size.0 - sprite_size.0) / 2) as i32,   // Screen posX
            ((screen_size.1 - sprite_size.1) / 2) as i32,   // Screen posY
        );
    }
}
