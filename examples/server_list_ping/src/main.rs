use bevy::app::App;
use bevy::MinimalPlugins;
use meloncraft::network::MeloncraftNetworkPlugin;

pub fn main() {

    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(MeloncraftNetworkPlugin);
    app.run();
}
