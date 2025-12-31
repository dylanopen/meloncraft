use std::collections::HashMap;
use std::net::SocketAddr;
use bevy::prelude::{Entity, Resource};

#[derive(Resource)]
pub struct ConnectedClients(pub HashMap<SocketAddr, Entity>);