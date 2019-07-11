use crate::Camera;
use specs::Entity;

pub struct ActiveCamera {
    pub entity: Option<Entity>
}

impl Default for ActiveCamera {
    fn default() -> Self {
        ActiveCamera { entity: None }
    }
}

pub struct InputManager {
    
}