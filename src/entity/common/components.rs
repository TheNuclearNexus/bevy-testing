use bevy::prelude::*;

#[derive(Component, Reflect, Clone, PartialEq, Eq)]
#[reflect(Component)]
pub enum Direction {
    Left,
    Right,
}

impl Direction {
    pub fn flip(&self) -> bool {
        match self {
            Direction::Left => true,
            Direction::Right => false,
        }
    }
}

#[derive(Component, Reflect, Clone)]
#[reflect(Component)]
pub struct Groundedness(bool);

impl Groundedness {
    pub fn set(&mut self, val: bool) {
        self.0 = val
    }
}

impl AsRef<bool> for Groundedness {
    fn as_ref(&self) -> &bool {
        &self.0
    }
}

impl Default for Groundedness {
    fn default() -> Self {
        Self(Default::default())
    }
}
