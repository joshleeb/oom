use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::event::Event;

pub trait InWorld {
    fn world_position(&self) -> (i32, i32);

    fn update(&mut self) {}
    fn update_with_event(&mut self, &Event) {}

    fn render(&self, &mut Canvas<Window>) {}
}

pub struct World<'i> {
    items: Vec<Box<&'i mut InWorld>>,
}

impl<'i> World<'i> {
    pub fn new() -> Self {
        World { items: vec![] }
    }

    pub fn add_item(&mut self, item: Box<&'i mut InWorld>) {
        self.items.push(item)
    }

    pub fn update(&mut self) {
        for item in &mut self.items {
            item.update();
        }
    }

    pub fn update_with_event(&mut self, event: &Event) {
        for item in &mut self.items {
            item.update_with_event(event);
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        for item in &self.items {
            item.render(canvas);
        }
    }
}
