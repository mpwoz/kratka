//! Demonstrates how to initialize a simple grid and visualize it using the dev tools' overlay feature

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use kratka_bevy::grid_layout::GridLayout;
use kratka_bevy::{dev_tools, grid_layout, GridPosition};
use kratka_bevy::dev_tools::GridDevTools;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((grid_layout::plugin, dev_tools::plugin))
        .add_systems(Startup, setup)
        .add_plugins(WorldInspectorPlugin::new()) // egui for easy value tweaking
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let grid_layout = GridLayout {
        square_size: 57.,
        width: 8,
        height: 5,
        origin: Vec2::new(-200., -100.),
        ..default()
    };

    commands.spawn(
        (
            Name::new("Grid Layout"),
            grid_layout,
            GridDevTools {
                draw_grid: true,
                ..default()
            }),
    );
}
