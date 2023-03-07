mod battleship;
mod projectile;

use battleship::Battleship;
use bevy::{math::f64, prelude::*};
use projectile::{brust_system, projectile_system, BrustEvent, XVelocity, YVelocity, ZVelocity};
use std::f32::consts::PI;

fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((Sprite::default(), Transform::from_xyz(0.0, 0.0, 30.0), XVelocity(10.0), YVelocity(10.0), ZVelocity(1.0)));
    let handle = asset_server.load("pictures/battleship.jpeg");
    commands
        .spawn(SpriteBundle {
            texture: handle,
            transform: Transform::from_xyz(0.0, 0.0, 50.0).with_scale(Vec3::new(0.5, 0.5, 1.0)),
            ..Default::default()
        })
        .insert(Battleship { direction: 0.0, velocity: 1.0 });
}

#[derive(Default)]
pub struct BulletMovement {
    x: f32,
    y: f32,
    z: f32,
    direction: f32,
    horizontal_velocity: f32,
    vertical_velocity: f32,
}

#[derive(Default)]
pub struct BattleshipMovement {
    x: f32,
    y: f32,
    direction: f32,
    length: f32,
    width: f32,
    height: f32,
}

// dx / dy = tan(a)
// dy = dx / tan(a)
// y = y0 + dy
// x = x0 + dx
// y = y0 + dx / tan(a)
// y = y0 + (x - x0) / tan(a)

// y = y1 + dy
// x = x1 + dx
// y = y1 + (x - x1) / tan(b)

// y0 + (x - x0) / tan(a) = y1 + (x - x1) / tan(b)
// y0 * tan(a) * tan(b) + (x - x0) * tan(b) = y1 * tan(a) * tan(b) + (x - x1) * tan(a)
// y0 * tan(a) * tan(b) + tan(b) * x - tan(b) * x0 = y1 * tan(a) * tan(b) + tan(a) * x - x1 * tan(a)
// (tan(b) - tan(a)) * x = tan(a) * tan(b) * (y1 - y0) + tan(b) * x0 - tan(a) * x1
// x = (tan(a) * tan(b) * (y1 - y0) + tan(b) * x0 - tan(a) * x1) / (tan(b) - tan(a))

// dy / dx = -tan(a)

// y = y0 + cos(a) * length * 0.5 + dy
// x = x0 + sin(a) * length * 0.5 + dx

// y = y0 + cos(a) * length * 0.5 - tan(a) * dx
// dx = x - x0 - sin(a) * length * 0.5

// y = y0 + cos(a) * length * 0.5 - tan(a) * (x - x0 - sin(a) * length * 0.5)

// y0 + cos(a) * length * 0.5 - tan(a) * (x - x0 - sin(a) * length * 0.5) = y1 + (x - x1) / tan(b)

// y0 + cos(a) * length * 0.5 - tan(a) * x + tan(a) * x0 + tan(a) * sin(a) * length * 0.5 = y1 + (x - x1) / tan(b)

// tan(b) * y0 + tan(b) * cos(a) * length * 0.5 - tan(b) * tan(a) * x + tan(b) * tan(a) * x0 + tan(b) * tan(a) * sin(a) * length * 0.5 = tan(b) * y + x - x1

// y = a + bx

// y = y0 + tan(b)(x - x0)

fn detect_frontend_collision(bullet: BulletMovement, battleship: BattleshipMovement) -> Option<(f32, f32, f32)> {
    let ship_radian = battleship.direction * PI / 180.0;
    let ship_x = battleship.x + ship_radian.sin() * battleship.length * 0.5;
    let ship_y = battleship.y + ship_radian.cos() * battleship.length * 0.5;
    let tan_bullet = (bullet.direction * PI / 180.0).tan();
    let tan_ship = (battleship.direction * PI / 180.0).tan();
    let collision_x = (tan_bullet * tan_ship * (ship_y - bullet.y) + tan_ship * bullet.x - tan_bullet * ship_x) / (tan_ship - tan_bullet);
    let collision_y = bullet.y + (collision_x - bullet.x) / tan_bullet;
    let distance = ((bullet.x - collision_x).powf(2.0) + (bullet.y - collision_y).powf(2.0)).sqrt();
    None
}

// yb = y0 - cos(a) * length * 0.5
// xb = x0 - sin(a) * length * 0.5

fn detect_collision(bullet: BulletMovement, battleship: BattleshipMovement) -> bool {
    let tan_bullet = (bullet.direction * PI / 180.0).tan();
    let tan_ship = (battleship.direction * PI / 180.0).tan();
    let collision_x = (tan_bullet * tan_ship * (battleship.y - bullet.y) + tan_ship * bullet.x - tan_bullet * battleship.x) / (tan_ship - tan_bullet);
    let collision_y = bullet.y + (collision_x - bullet.x) / tan_bullet;
    let ship_x1 = battleship.x - (battleship.direction * PI / 180.0).sin() * battleship.length * 0.5;
    let ship_x2 = battleship.x + (battleship.direction * PI / 180.0).sin() * battleship.length * 0.5;
    let ship_y1 = battleship.y - (battleship.direction * PI / 180.0).cos() * battleship.length * 0.5;
    let ship_y2 = battleship.y + (battleship.direction * PI / 180.0).cos() * battleship.length * 0.5;
    let ship_min_x = ship_x1.min(ship_x2);
    let ship_max_x = ship_x1.max(ship_x2);
    let ship_min_y = ship_y1.min(ship_y2);
    let ship_max_y = ship_y1.max(ship_y2);
    collision_x >= ship_min_x && collision_x <= ship_max_x && collision_y >= ship_min_y && collision_y <= ship_max_y
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_detect_collision() {
        let bullet = BulletMovement {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            direction: 45.0,
            ..Default::default()
        };
        let battleship = BattleshipMovement {
            x: 100.0,
            y: 100.0,
            direction: 315.0,
            length: 10.0,
            ..Default::default()
        };
        println!("tan 45 degree {}", (45.0 * PI / 180.0).tan());
        println!("tan 315 degree {}", (315.0 * PI / 180.0).tan());
        assert!(detect_collision(bullet, battleship) == true)
    }
}
