use crate::Camera;
use specs::Entity;
use std::collections::VecDeque;
use cgmath::{Vector2, vec2};

pub struct ActiveCamera {
    pub entity: Option<Entity>
}

impl Default for ActiveCamera {
    fn default() -> Self {
        ActiveCamera { entity: None }
    }
}

#[derive(Default)]
pub struct InputEventQueue {
    pub queue: VecDeque<glfw::WindowEvent>
}

pub struct InputCache {
    pub last_cursor_pos: Vector2<f32>,
    pub cursor_rel_pos: Vector2<f32>,
}

impl Default for InputCache {
    fn default() -> Self {
        InputCache {
            last_cursor_pos: vec2(0.0, 0.0),
            cursor_rel_pos: vec2(0.0, 0.0),
        }
    }
}