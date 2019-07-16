use crate::Camera;
use specs::Entity;
use std::collections::VecDeque;
use nalgebra_glm::{Vec2, vec2};

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
    pub last_cursor_pos: Vec2,
    pub cursor_rel_pos: Vec2,
}

impl Default for InputCache {
    fn default() -> Self {
        InputCache {
            last_cursor_pos: vec2(0.0, 0.0),
            cursor_rel_pos: vec2(0.0, 0.0),
        }
    }
}