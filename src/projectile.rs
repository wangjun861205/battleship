use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct XVelocity(pub f32);

#[derive(Debug, Component)]
pub struct YVelocity(pub f32);

#[derive(Debug, Component)]
pub struct ZVelocity(pub f32);

#[derive(Debug)]
pub enum BrustType {
    HitIntoWater,
    HitIntoShip,
}

#[derive(Debug)]
pub struct BrustEvent {
    typ: BrustType,
    x: f32,
    y: f32,
}

#[derive(Resource)]
pub struct HitIntoWaterPicture(Handle<Image>);

pub fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle = asset_server.load("pictures/hit_into_water.png");
    commands.insert_resource(HitIntoWaterPicture(handle));
}

pub fn brust_system(mut commands: Commands, hit_into_water: Res<HitIntoWaterPicture>, mut event_reader: EventReader<BrustEvent>) {
    for event in event_reader.iter() {
        commands.spawn(SpriteBundle {
            transform: Transform::from_xyz(event.x, event.y, 0.0).with_scale(Vec3::new(0.1, 0.1, 1.0)),
            texture: hit_into_water.0.clone(),
            ..Default::default()
        });
    }
}

pub fn projectile_system(mut commands: Commands, time: Res<Time>, mut event_writer: EventWriter<BrustEvent>, mut query: Query<(Entity, &mut Transform, &XVelocity, &YVelocity, &mut ZVelocity)>) {
    for (entity, mut transform, x_velocity, y_velocity, mut z_velocity) in query.iter_mut() {
        let t = time.delta_seconds();
        let translation = &mut transform.translation;
        translation.x += x_velocity.0 * t;
        translation.y += y_velocity.0 * t;
        translation.z += z_velocity.0 * t;
        if translation.z < 0.0 {
            commands.entity(entity).despawn();
            event_writer.send(BrustEvent {
                typ: BrustType::HitIntoWater,
                x: translation.x,
                y: translation.y,
            });
            return;
        }
        z_velocity.0 -= 9.8 * t;
    }
}
