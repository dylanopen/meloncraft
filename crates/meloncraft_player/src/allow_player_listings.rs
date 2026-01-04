use bevy::prelude::Component;

#[derive(Component, Clone, Debug, Copy, PartialEq)]
pub struct AllowPlayerListings(pub bool);
