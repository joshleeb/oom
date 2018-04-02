use animation::Animation;
use sdl2::render::Canvas;
use std::cmp;
use sdl2::keyboard::{KeyboardState, Keycode, Scancode};
use sdl2::rect::Rect;
use sdl2::video::Window;
use sprite::SpritesheetLayout;
use sprite::orb::{OrbLayout, OrbSprite, OrbSpritesheet};
use std::time::Duration;

const SCALE: u32 = 2;
const MOVEMENT_SPEED: i32 = 4;
const SHOULD_ANIMATE: bool = true;

pub struct Player<'s> {
    spritesheet: &'s OrbSpritesheet<'s>,
    current_sprite: OrbSprite,
    world_dimensions: (u32, u32),
    pub world_posx: i32,
    pub world_posy: i32,
    pub movement_speed: i32,
    animation: Animation<OrbSprite>,
    keydowns: Vec<Keycode>,
}

impl<'s> Player<'s> {
    pub fn new(
        spritesheet: &'s OrbSpritesheet<'s>,
        world_dimensions: (u32, u32),
        world_posx: i32,
        world_posy: i32,
    ) -> Self {
        Player {
            spritesheet,
            current_sprite: OrbSprite::Large,
            world_dimensions,
            world_posx,
            world_posy,
            movement_speed: MOVEMENT_SPEED,
            animation: Animation::new(
                vec![
                    OrbSprite::Large,
                    OrbSprite::Medium,
                    OrbSprite::Small,
                    OrbSprite::Medium,
                ],
                Duration::from_millis(150),
            ),
            keydowns: vec![],
        }
    }

    pub fn size() -> (u32, u32) {
        let spritesheet_size = <OrbLayout as SpritesheetLayout>::get_dimensions();
        (SCALE * spritesheet_size.0, SCALE * spritesheet_size.1)
    }

    pub fn world_position(&self) -> (i32, i32) {
        (self.world_posx, self.world_posy)
    }

    pub fn update(&mut self, kbstate: &KeyboardState) {
        self.update_animation();
        self.update_keydowns(kbstate);
        self.update_world_position();
    }

    pub fn render(&self, canvas: &mut Canvas<Window>, viewport: Rect, show_perimeter: bool) {
        self.spritesheet.draw(
            canvas,
            &self.current_sprite,
            SCALE,
            self.world_posx - viewport.x,
            self.world_posy - viewport.y,
            show_perimeter,
        );
    }

    fn update_world_position(&mut self) {
        match self.keydowns.last() {
            Some(Keycode::W) => {
                self.world_posy = cmp::max(0, self.world_posy - self.movement_speed)
            }
            Some(Keycode::A) => {
                self.world_posx = cmp::max(0, self.world_posx - self.movement_speed)
            }
            Some(Keycode::S) => {
                self.world_posy = cmp::min(
                    (self.world_dimensions.1 - Player::size().1) as i32,
                    self.world_posy + self.movement_speed,
                )
            }
            Some(Keycode::D) => {
                self.world_posx = cmp::min(
                    (self.world_dimensions.0 - Player::size().0) as i32,
                    self.world_posx + self.movement_speed,
                )
            }
            _ => {}
        }
    }

    fn update_animation(&mut self) {
        if SHOULD_ANIMATE {
            if let Some(sprite) = self.animation.next_frame() {
                self.current_sprite = sprite.clone();
            }
        }
    }

    fn update_keydowns(&mut self, kbstate: &KeyboardState) {
        for scancode in &[Scancode::W, Scancode::A, Scancode::S, Scancode::D] {
            if let Some(keycode) = &Keycode::from_scancode(*scancode) {
                if kbstate.is_scancode_pressed(*scancode) {
                    if !self.keydowns.contains(keycode) {
                        self.keydowns.push(*keycode);
                    }
                } else if self.keydowns.contains(keycode) {
                    self.keydowns.remove_item(keycode);
                }
            }
        }
    }
}
