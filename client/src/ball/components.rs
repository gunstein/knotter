use bevy::prelude::*;
use uuid::Uuid;

/*#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Ball; */

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct StaticBall;

#[derive(Component)]
pub struct MovingBall;

#[derive(Component)]
pub struct Upserted;

#[derive(Component)]
pub struct BallUuid(pub Uuid);

#[derive(Component)]
pub struct CapsuleDepth(pub f32);

#[derive(Component)]
pub struct CapsuleRotation(pub Quat);

#[derive(Component)]
pub struct SpeedMarker;