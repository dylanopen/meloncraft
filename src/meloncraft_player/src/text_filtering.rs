//! Module for struct component [`EnableTextFiltering`].

use bevy::prelude::Component;

/// Whether the client has enabled text filtering or not.
/// 
/// If they do, the server should filter text before sending it to the client, and if they don't,
/// the server can send unfiltered text to the client. The exact filtering method is up to you, but
/// it should filter out innapropriate text.
///
/// *This is a component for player entities*.
///
/// The server should use this component when sending text packets to determine whether they should
/// filter the text before sending it to the client.
///
/// ## Protocol representation
/// See [`EnableTextFiltering::0`] for the protocol ID for each boolean value.
#[derive(Component, Debug, Clone, Copy)]
pub struct EnableTextFiltering(

    pub bool,
);
