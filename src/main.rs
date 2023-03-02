mod projectile;

use bevy::prelude::*;
use projectile::{brust_system, projectile_system, BrustEvent, XVelocity, YVelocity, ZVelocity};

fn setup_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((Sprite::default(), Transform::from_xyz(0.0, 0.0, 30.0), XVelocity(10.0), YVelocity(10.0), ZVelocity(1.0)));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<BrustEvent>()
        .add_startup_system(projectile::setup_system)
        .add_startup_system(setup_system)
        .add_system(projectile_system)
        .add_system(brust_system)
        .run()
}
