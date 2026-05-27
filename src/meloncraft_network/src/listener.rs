use std::net::TcpListener;

use bevy::ecs::resource::Resource;

#[derive(Resource, Debug)]
pub struct NewClientListener(pub TcpListener);
