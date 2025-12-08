use valence::{DefaultPlugins, app::App};


pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(meloncraft::init::startup::StartupPlugin)
        .add_plugins(meloncraft::init::client::ClientInitializationPlugin)
        .add_plugins(meloncraft::world::world_load::WorldLoadPlugin)
        .add_plugins(meloncraft::command::gamemode::GamemodeCommandPlugin)
        .add_plugins(meloncraft::block_destroy::BlockDestroyPlugin)
        .run();
}

