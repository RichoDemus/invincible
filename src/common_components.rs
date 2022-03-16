use bevy::prelude::*;
use std::fmt::Formatter;

#[derive(Default, Debug, Component)]
pub struct Name(pub String);

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
