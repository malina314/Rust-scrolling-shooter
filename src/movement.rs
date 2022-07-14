use crate::{AppState, Movable, Velocity};
use bevy::prelude::*;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::InGame).with_system(movement_system));
    }
}

fn movement_system(
    mut commands: Commands,
    windows: Res<Windows>,
    mut query: Query<(Entity, &Velocity, &mut Transform, &Movable)>,
) {
    const TIME_STEP: f32 = 1. / 60.;
    const BASE_SPEED: f32 = 500.;

    for (entity, velocity, mut transform, movable) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;

        // despawn when out of screen
        if movable.auto_despawn {
            const MARGIN: f32 = 200.;

            let window = windows.primary();

            if translation.y.abs() > window.height() / 2. + MARGIN
                || translation.x.abs() > window.width() / 2. + MARGIN
            {
                commands.entity(entity).despawn();
            }
        }
    }
}
