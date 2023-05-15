extern crate bevy;
extern crate custom_error;

mod components;
mod util;

use bevy::prelude::*;
use components::{bus::Bus, dh6502_cpu::M6502};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
    
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    // Rectangle
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(1.0, 1.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(-50., 0., 0.)),
        ..default()
    });
}

#[test]
fn test_clock() {
    let mut cpu: M6502 = M6502::new();
    let mut bus: Bus = Bus::new();
    M6502::reset(&mut cpu, &bus);
    for _ in 0..8 {
        M6502::clock(&mut cpu, &mut bus);
    }
    assert!(cpu.cycles == 0);
}
