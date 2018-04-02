use sdl2::image::LoadSurface;
use tile::Tile;
use sprite::tile::{TileSprite, TileSpritesheet};
use sdl2::surface::Surface;

pub struct Map<'a> {
    pub tiles: Vec<Tile<'a>>,
    pub width: u32,
    pub height: u32,
}

impl<'a> Map<'a> {
    pub fn from_pixelmap(spritesheet: &'a TileSpritesheet<'a>, path: &str) -> Self {
        let mut tiles: Vec<Tile<'a>> = vec![];
        let mut world_pos = (0, 0);

        let surface = Surface::from_file(path).unwrap();
        let pixelbuf = surface.without_lock().unwrap();

        let sprite_size = Tile::size();
        let world_width = surface.width() * sprite_size.0;
        let world_height = surface.height() * sprite_size.1;

        for color in pixelbuf.to_vec().chunks(3) {
            if let Some(sprite) = color_to_sprite(color[0], color[1], color[2]) {
                tiles.push(Tile::new(spritesheet, sprite, world_pos.0, world_pos.1));
            }

            world_pos.0 += sprite_size.0 as i32;

            if world_pos.0 >= (surface.height() * sprite_size.0) as i32 {
                world_pos.1 += sprite_size.1 as i32;
                world_pos.0 = 0;
            }
        }

        Map {
            tiles,
            width: world_width,
            height: world_height,
        }
    }
}

fn color_to_sprite(r: u8, g: u8, b: u8) -> Option<TileSprite> {
    match (r, g, b) {
        (127, 178, 56) => Some(TileSprite::Grass),
        (151, 109, 77) => Some(TileSprite::Wood),
        (164, 168, 184) => Some(TileSprite::Cobblestone),
        (247, 233, 163) => Some(TileSprite::Sand),
        (255, 0, 0) => Some(TileSprite::Lava),
        (32, 237, 41) => Some(TileSprite::DiamondOre),
        (64, 64, 255) => Some(TileSprite::Water),
        _ => None,
    }
}
