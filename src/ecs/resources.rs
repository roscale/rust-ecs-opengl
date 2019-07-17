use crate::Camera;
use specs::Entity;
use std::collections::{VecDeque, HashMap};
use nalgebra_glm::{Vec2, vec2};
use glfw::{Key, Action};

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

    pub key_states: HashMap<Key, Action>
}

impl Default for InputCache {
    fn default() -> Self {
        InputCache {
            last_cursor_pos: vec2(0.0, 0.0),
            cursor_rel_pos: vec2(0.0, 0.0),
            key_states: HashMap::default()
        }
    }
}

impl InputCache {
    pub fn is_key_pressed(&self, key: Key) -> bool {
        match self.key_states.get(&key) {
            None => false,
            Some(action) => *action == Action::Press || *action == Action::Repeat
        }
    }
}