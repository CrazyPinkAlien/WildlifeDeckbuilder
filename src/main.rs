use bevy::app::App;
use bevy::DefaultPlugins;
use bevy_egui::EguiPlugin;

// Main entry point
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)

        .run();
}
