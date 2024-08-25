use bevy::prelude::*;
use crate::grid_layout::GridLayout;
use crate::GridPosition;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, draw_overlay);

    // reflection
    app.register_type::<GridDevTools>();
}

#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct GridDevTools {
    pub draw_grid: bool,
}

pub fn draw_overlay(mut gizmos: Gizmos, query: Query<(&GridLayout, &GridDevTools)>) {
    for (grid, settings) in query.iter() {
        if !settings.draw_grid {
            continue;
        }

        for x in 0..grid.width {
            for y in 0..grid.height {
                let pos = grid.grid_to_world(&GridPosition::new(x as f32, y as f32));
                gizmos.rect_2d(pos, 0., Vec2::new(grid.square_size, grid.square_size), Color::srgb(0.9, 0.9, 0.9))
            }
        }
    }
}
