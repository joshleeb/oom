use sdl2::image::LoadSurface;
use tile::Tile;
use sprite::tile::{TileLayout, TileSprite, TileSpritesheet};
use sprite::SpritesheetLayout;
use sdl2::surface::Surface;

pub struct Map<'a> {
    tiles: Vec<Tile<'a>>,
}

impl<'a> Map<'a> {
    pub fn from_pixelmap(spritesheet: &'a TileSpritesheet<'a>, path: &str) -> Self {
        let mut tiles: Vec<Tile<'a>> = vec![];
        let mut world_pos = (0, 0);

        let surface = Surface::from_file(path).unwrap();
        let pixelbuf = surface.without_lock().unwrap();
        let rgb_values: Vec<u8> = pixelbuf.iter().cloned().collect();

        let sprite_size = <TileLayout as SpritesheetLayout>::get_dimensions();

        for color in rgb_values.chunks(3) {
            if let Some(sprite) = color_to_sprite(color[0], color[1], color[2]) {
                tiles.push(Tile::new(&spritesheet, sprite, world_pos.0, world_pos.1));
            }

            world_pos.0 += sprite_size.0 as i32;

            if world_pos.0 >= (surface.height() * sprite_size.0) as i32 {
                world_pos.1 += sprite_size.1 as i32;
                world_pos.0 = 0;
            }
        }

        Map { tiles }
    }

    pub fn tiles(self) -> Vec<Tile<'a>> {
        self.tiles
    }
}

fn color_to_sprite(r: u8, g: u8, b: u8) -> Option<TileSprite> {
    match (r, g, b) {
        (127, 178, 56) => Some(TileSprite::Grass),
        (151, 109, 77) => Some(TileSprite::Wood),
        (164, 168, 184) => Some(TileSprite::Cobblestone),
        (247, 233, 163) => Some(TileSprite::Sand),
        (255, 0, 0) => Some(TileSprite::Lava),
        (64, 64, 255) => Some(TileSprite::Water),
        _ => None,
    }
}
