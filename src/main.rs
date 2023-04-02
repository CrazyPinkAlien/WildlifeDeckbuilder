use bevy::app::App;
use bevy::DefaultPlugins;

pub mod card;
pub mod materials;

// Main entry point
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)

        .run();
}
