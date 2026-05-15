use bevy::prelude::*;

use crate::gameplay::Economy;
use crate::player::Player;
use crate::world::Accelerator;

const INTERACTION_DISTANCE: f32 = 2.5;

pub struct UiPlugin;

#[derive(Component)]
struct HudText;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
            .add_systems(Update, update_hud);
    }
}

fn setup_ui(mut commands: Commands) {
    commands.spawn((
        HudText,
        TextBundle::from_sections([
            TextSection::new(
                "Energy: 0\n",
                TextStyle {
                    font_size: 26.0,
                    color: Color::srgb(0.7, 0.9, 1.0),
                    ..default()
                },
            ),
            TextSection::new(
                "Matter: 0\n",
                TextStyle {
                    font_size: 24.0,
                    color: Color::srgb(0.6, 1.0, 0.8),
                    ..default()
                },
            ),
            TextSection::new(
                "Hint: Move with WASD, look with mouse, run accelerator with E",
                TextStyle {
                    font_size: 20.0,
                    color: Color::srgb(0.95, 0.95, 0.95),
                    ..default()
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Val::Px(16.0),
            top: Val::Px(12.0),
            ..default()
        }),
    ));
}

fn update_hud(
    economy: Res<Economy>,
    mut text_q: Query<&mut Text, With<HudText>>,
    player_q: Query<&Transform, With<Player>>,
    accelerator_q: Query<&Transform, With<Accelerator>>,
) {
    if !economy.is_changed()
        && player_q
            .get_single()
            .ok()
            .zip(accelerator_q.get_single().ok())
            .is_none()
    {
        return;
    }

    let Ok(mut text) = text_q.get_single_mut() else {
        return;
    };

    text.sections[0].value = format!("Energy: {}\n", economy.energy);
    text.sections[1].value = format!("Matter: {}\n", economy.matter);

    let hint = if let (Ok(player), Ok(accelerator)) =
        (player_q.get_single(), accelerator_q.get_single())
    {
        let distance = player.translation.distance(accelerator.translation);
        if distance <= INTERACTION_DISTANCE {
            "Hint: Press E to run accelerator"
        } else {
            "Hint: Move closer to the accelerator (center cube)"
        }
    } else {
        "Hint: Loading scene..."
    };
    text.sections[2].value = hint.to_string();
}
