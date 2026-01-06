pub mod raw;

use bevy::app::Plugin;

use self::raw::RawRegistries;

pub struct MeloncraftDefaultRegistriesPlugin;

impl Plugin for MeloncraftDefaultRegistriesPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        let raw_registries = RawRegistries::default();
        dbg!(&raw_registries.0.0);
        app.insert_resource(raw_registries);
    }
}
