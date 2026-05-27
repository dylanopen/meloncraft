use std::net::TcpListener;

use bevy::app::{App, Plugin, PostUpdate};

use crate::listener::{self, NewClientListener};
use crate::{stream_reader, stream_writer};

pub struct MeloncraftNetworkPlugin;

impl Plugin for MeloncraftNetworkPlugin {
    fn build(&self, app: &mut App) {
        let tcp_listener = TcpListener::bind("127.0.0.1:25565").unwrap();
        tcp_listener.set_nonblocking(true).unwrap();
        app.insert_resource(NewClientListener(tcp_listener));
        app.add_systems(PostUpdate, listener::handle_new_clients);
        app.add_systems(PostUpdate, stream_reader::read_streams);
        app.add_systems(PostUpdate, stream_writer::write_streams);
    }
}
