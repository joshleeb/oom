use sdl2::render::Canvas;
use sdl2::video::Window;
use tile::Tile;
use map::Map;
use sdl2::event::Event;
use sdl2::rect::Rect;

pub trait InWorld {
    fn world_rect(&self) -> Rect;

    fn update(&mut self) {}
    fn event_update(&mut self, &Event) {}

    fn render(&self, &mut Canvas<Window>, i32, i32) {}
}

pub struct World<'a> {
    items: Vec<Tile<'a>>,
    width: u32,
    height: u32,
}

impl<'a> World<'a> {
    pub fn new(map: Map<'a>) -> Self {
        World {
            items: map.tiles,
            width: map.width,
            height: map.height,
        }
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn update(&mut self) {
        for item in &mut self.items {
            item.update();
        }
    }

    pub fn event_update(&mut self, event: &Event) {
        for item in &mut self.items {
            item.event_update(event);
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>, viewport: Rect) {
        for item in &self.items {
            let world_rect = item.world_rect();

            // TODO: optimise not to render items outside viewport.
            item.render(canvas, world_rect.x - viewport.x, world_rect.y - viewport.y);
        }
    }
}
