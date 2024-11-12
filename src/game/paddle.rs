use crate::{rect::Rect, renderer::Renderer};

pub struct Paddle {
    rect: Rect,
}

impl Paddle {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            rect: Rect::new(x, y, 10, 100),
        }
    }

    pub fn update(&mut self) {}

    pub fn draw(&self, renderer: &mut Renderer) {
        renderer.draw_rect(&self.rect);
    }
}
