use winit::{dpi::PhysicalSize, keyboard::KeyCode};

use crate::{input::Input, rect::Rect, renderer::Renderer};

#[derive(PartialEq)]
enum MoveDirection {
    IDLE,
    UP,
    DOWN,
}

const MOVE_SPEED: f32 = 10.0;

pub struct Player {
    pub rect: Rect,
    move_direction: MoveDirection,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            rect: Rect::new(x, y, 10, 100),
            move_direction: MoveDirection::IDLE,
        }
    }

    pub fn collide(&mut self, window_size: PhysicalSize<u32>) {
        if self.rect.y < 0.0 {
            self.rect.y = 0.0;
        } else if self.rect.y + self.rect.height as f32 > window_size.height as f32 {
            self.rect.y = window_size.height as f32 - self.rect.height as f32;
        }
    }

    pub fn input(&mut self, input: &Input) {
        if input.is_key_down(KeyCode::KeyW) {
            self.move_direction = MoveDirection::UP;
        } else if input.is_key_down(KeyCode::KeyS) {
            self.move_direction = MoveDirection::DOWN;
        } else {
            self.move_direction = MoveDirection::IDLE;
        };
    }

    pub fn update(&mut self) {
        if self.move_direction == MoveDirection::UP {
            self.rect.y -= MOVE_SPEED;
        } else if self.move_direction == MoveDirection::DOWN {
            self.rect.y += MOVE_SPEED;
        }
    }

    pub fn draw(&self, renderer: &mut Renderer) {
        renderer.draw_rect(&self.rect);
    }
}