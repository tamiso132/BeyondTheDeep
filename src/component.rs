use bevy::prelude::{Commands, Component, Entity, IntoSystemSetConfigs, Query};
use rand::Rng;

#[derive(Component)]
struct Weapon {
    damage: u16,
    weapon_range: u8,
}

#[derive(Component)]
struct Health {
    current: u16,
    max_health: u16,
    armor_class: u8,
}

#[derive(Component)]
struct Damage {
    total: u16,
}

#[derive(Component)]
struct Death {}

#[derive(Component)]
struct Turn {
    initiative: u8,
    movement_speed: u8,
    action: u8,
}
fn turn_system(mut commands: Commands, mut query: Query<(Entity, &mut Health, &Damage)>) {
    for (entity, mut health, damage) in &mut query {
        health.current -= damage.total;

        if health.current <= 0 {
            commands.entity(entity).insert(Death {});
        }
    }
}

fn dice_roll(max: u8) -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..max)
}
