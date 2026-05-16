use bevy::prelude::Component;

#[derive(Component, Clone, Debug, Copy, Eq, PartialEq)]
pub struct AllowPlayerListings(pub bool);
