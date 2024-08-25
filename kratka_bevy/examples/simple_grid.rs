//! Demonstrates how to initialize a simple grid and visualize it using the dev tools' overlay feature

use bevy::prelude::*;
use kratka_bevy::grid_layout::GridLayout;
use kratka_bevy::{dev_tools, GridPosition};
use kratka_bevy::dev_tools::GridDevTools;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(dev_tools::plugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let grid_layout = GridLayout::default();
    info!("Spawned a {}x{} grid", &grid_layout.width, &grid_layout.height);
    commands.spawn(
        (grid_layout,
         GridDevTools {
             draw_grid: true,
             ..default()
         }),
    );
}
