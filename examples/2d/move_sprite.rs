//! Renders a 2D scene containing a single, moving sprite.
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(sprite_movement)
        .run();
}

#[derive(Component)]
enum Direction {
    Clockwise,
    CounterClockwise,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("branding/icon.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        },
        Direction::Clockwise,
    ));
}

/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    for (mut logo, mut transform) in &mut sprite_position {
        let x = transform.translation.x;
        let y = transform.translation.y;
        let mut angle = 2. * (y / (x + (x.powf(2.) + y.powf(2.)).sqrt())).atan();
        let step = 0.1 * time.delta_seconds();
        match *logo {
            Direction::Clockwise => angle -= step,
            Direction::CounterClockwise => angle += step,
        }
        transform.translation.x = angle.cos() * 200.0;
        transform.translation.y = angle.sin() * 200.0;
        //println!("{} {} {}", &angle, transform.translation.x, transform.translation.y);

        if angle.abs() < step {
            match *logo {
                Direction::Clockwise => *logo = Direction::CounterClockwise,
                Direction::CounterClockwise => *logo = Direction::Clockwise,
            }
        }
    }
}
