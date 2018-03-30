use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use std::marker::PhantomData;
use std::path::Path;

pub mod pokemon;

pub trait SpritesheetLayout {
    type Sprite;

    fn get_dimensions() -> (usize, usize);
    fn get_sprite(&Self::Sprite) -> Rect;
}

pub struct Spritesheet<'t, SL> {
    texture: Texture<'t>,
    layout: PhantomData<SL>,
}

impl<'t, SL: SpritesheetLayout> Spritesheet<'t, SL> {
    pub fn from_spritesheet(
        texture_creator: &'t TextureCreator<WindowContext>,
        spritesheet: &str,
    ) -> Self {
        let texture = texture_creator
            .load_texture(Path::new(spritesheet))
            .unwrap();

        Spritesheet {
            texture,
            layout: PhantomData,
        }
    }

    pub fn draw_sprite_with_scale(
        &self,
        canvas: &mut Canvas<Window>,
        sprite: &SL::Sprite,
        scale: usize,
        x: usize,
        y: usize,
    ) {
        let dimensions = <SL as SpritesheetLayout>::get_dimensions();

        let screen_rect = Rect::new(
            x as i32,
            y as i32,
            (dimensions.0 * scale) as u32,
            (dimensions.1 * scale) as u32,
        );

        canvas
            .copy(
                &self.texture,
                Some(<SL as SpritesheetLayout>::get_sprite(sprite)),
                Some(screen_rect),
            )
            .unwrap();
    }
}
