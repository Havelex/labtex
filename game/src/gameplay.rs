use bevy::prelude::*;

use crate::player::Player;
use crate::world::Accelerator;

const INTERACTION_DISTANCE: f32 = 2.5;

pub struct GameplayPlugin;

#[derive(Resource, Default)]
pub struct Economy {
    pub energy: i32,
    pub matter: i32,
}

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Economy>()
            .add_systems(Update, run_accelerator);
    }
}

fn run_accelerator(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut economy: ResMut<Economy>,
    player_q: Query<&Transform, With<Player>>,
    accelerator_q: Query<&Transform, With<Accelerator>>,
) {
    if !keys.just_pressed(KeyCode::KeyE) {
        return;
    }

    let Ok(player_transform) = player_q.get_single() else {
        return;
    };
    let Ok(accelerator_transform) = accelerator_q.get_single() else {
        return;
    };

    let distance = player_transform
        .translation
        .distance(accelerator_transform.translation);
    if distance > INTERACTION_DISTANCE {
        return;
    }

    let roll = ((time.elapsed_seconds_wrapped() * 1000.0) as i32 % 10) + 1;
    economy.energy += roll * 2;
    economy.matter += roll / 3;

    info!(
        "Accelerator run => roll: {}, energy: {}, matter: {}",
        roll, economy.energy, economy.matter
    );
}
