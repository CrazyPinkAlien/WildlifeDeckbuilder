use bevy::app::App;
use bevy::DefaultPlugins;
use bevy::prelude::SystemSet;
use bevy::time::FixedTimestep;

use crate::smooth_move::smooth_move;

pub mod smooth_move;

// Main entry point
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)

        .add_system_set(
            SystemSet::new()
                // These systems run once per second
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(smooth_move)
        )

        .run();
}
