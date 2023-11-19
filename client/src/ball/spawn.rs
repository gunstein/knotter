use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use uuid::Uuid;

use super::BALL_RADIUS;
use super::components::*;
use super::resources::*;

pub fn spawn_static_ball(
    commands: &mut Commands, 
    ball_mesh_resource: &Res<HandleForBallMesh>,
    ball_material_resource: &Res<HandleForBallMaterial>,
    point_on_sphere: (f32, f32, f32),
    upserted: bool,
    uuid: Option<Uuid>,
) {
    bevy::log::info!("spawn_static_ball");
    
    // Decide on the UUID to use: either the one provided, or generate a new one
    let ball_uuid = uuid.unwrap_or_else(Uuid::new_v4);

    // The rest of your function remains unchanged
    let mut spawned_entity = commands.spawn((
        PbrBundle {
            mesh: ball_mesh_resource.handle.clone(),
            material: ball_material_resource.handle.clone(),
            ..default()
        },
    ));

    spawned_entity.insert((
        TransformBundle::from(Transform::from_xyz(point_on_sphere.0, point_on_sphere.1, point_on_sphere.2)),
        Collider::ball(BALL_RADIUS),
        Friction::coefficient(0.0),
        Restitution::coefficient(1.0),
        RigidBody::Fixed,
        CollisionGroups {
            memberships: Group::GROUP_2,
            filters: (Group::GROUP_1 | Group::GROUP_2),
        },
        StaticBall,
        BallUuid(ball_uuid)  // Use the decided UUID
    ));

    if upserted {
        spawned_entity.insert(Upserted);
    }   
}

pub fn spawn_moving_ball(commands: &mut Commands, 
    ball_mesh_resource: &Res<HandleForBallMesh>,
    ball_material_resource: &Res<HandleForBallMaterial>,
    point_on_sphere: (f32, f32, f32),
    impulse: Vec3,
    uuid: Option<Uuid>,
 ) {
    bevy::log::info!("spawn_moving_ball");
    // Decide on the UUID to use: either the one provided, or generate a new one
    let ball_uuid = uuid.unwrap_or_else(Uuid::new_v4);

    //ball
    let mut spawned_entity = commands.spawn(
        PbrBundle {
            mesh: ball_mesh_resource.handle.clone(),
            material: ball_material_resource.handle.clone(),
            ..default()
        },
    );

    spawned_entity.insert((
        TransformBundle::from(Transform::from_xyz(point_on_sphere.0, point_on_sphere.1, point_on_sphere.2)),
        Sleeping::disabled(),
        Ccd::enabled(),
        Collider::ball(BALL_RADIUS),
        Friction::coefficient(0.0),
        RigidBody::Dynamic,
        CollisionGroups {
            memberships: Group::GROUP_2,
            filters: (Group::GROUP_1 | Group::GROUP_2),
        },
        Restitution {
            coefficient: 1.0,
            combine_rule: CoefficientCombineRule::Max,
        },
        ExternalForce {
            force: Vec3::new(0.0, 0.0, 0.0),
            torque: Vec3::new(0.0, 0.0, 0.0),
        },
        ExternalImpulse {
            impulse: impulse,
            torque_impulse: Vec3::new(0.0, 0.0, 0.0),
        },
        Velocity {
            linvel: Vec3::new(0.0, 0.0, 0.0),
            angvel: Vec3::new(0.0, 0.0, 0.0),
        },
        ActiveEvents::COLLISION_EVENTS,
        ReadMassProperties::default(),
        MovingBall,
        BallUuid(ball_uuid),
    ));

    spawned_entity.insert(Speed(0.0));          
}

pub fn spawn_speed_marker(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    depth: f32,
    radius: f32,
    translation: Vec3,
    rotation: Quat,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Capsule {
            radius,
            depth,
            ..Default::default()
        })),
        material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
        transform: Transform {
            translation,
            rotation,
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(CapsuleDepth(depth))
    .insert(CapsuleRotation(rotation))
    .insert(SpeedMarker);
}