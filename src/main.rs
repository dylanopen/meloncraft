use valence::{DefaultPlugins, app::App};


pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(meloncraft::init::startup::StartupPlugin)
        .add_plugins(meloncraft::init::client::ClientInitializationPlugin)
        .run();
}

