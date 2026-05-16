use bevy::prelude::Component;

#[derive(Debug, Clone, PartialEq, Eq, Component)]
pub enum JoinState {
    LoginStart,
}