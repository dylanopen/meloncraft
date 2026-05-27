use std::net::TcpListener;

use bevy::app::{App, Plugin, PreUpdate};

use crate::listener::{self, NewClientListener};

pub struct MeloncraftNetworkPlugin;

impl Plugin for MeloncraftNetworkPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(NewClientListener(
            TcpListener::bind("127.0.0.1:25565").unwrap(),
        ));
        app.add_systems(PreUpdate, listener::handle_new_clients);
    }
}
