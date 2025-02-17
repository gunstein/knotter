use bevy::prelude::*;
use bevy::math::*;
use bevy_rapier3d::prelude::*;
use uuid::Uuid;

use super::BALL_RADIUS;
use super::components::*;
use super::resources::*;
use super::color_material_map::*;

pub fn spawn_static_ball(
    commands: &mut Commands, 
    ball_mesh_resource: &Res<HandleForBallMesh>,
    ball_materials_resource: &mut ResMut<ColorMaterialMap>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    color: Color,
    point_on_sphere: (f32, f32, f32),
    upserted: bool,
    uuid: Option<Uuid>,
) {
    //bevy::log::info!("spawn_static_ball, ball_materials_resource length = {:?}", ball_materials_resource.map.len());
    
    // Decide on the UUID to use: either the one provided, or generate a new one
    let ball_uuid = uuid.unwrap_or_else(Uuid::new_v4);

    let material_handle = ball_materials_resource
        .map
        .entry(ColorKey(color))
        .or_insert_with(|| materials.add(color))
        .clone();

    let mut spawned_entity = commands.spawn(PbrBundle {
        mesh: ball_mesh_resource.handle.clone(),
        material: material_handle,
        ..default()
    });

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
    ball_materials_resource: &mut ResMut<ColorMaterialMap>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    color: Color,
    point_on_sphere: (f32, f32, f32),
    impulse: Vec3,
    uuid: Option<Uuid>,
 ) {
    //bevy::log::info!("spawn_moving_ball");
    // Decide on the UUID to use: either the one provided, or generate a new one
    let ball_uuid = uuid.unwrap_or_else(Uuid::new_v4);

    //ball
    //get material for color and spawn
    let material_handle = ball_materials_resource
        .map
        .entry(ColorKey(color))
        .or_insert_with(|| materials.add(color))
        .clone();

    let mut spawned_entity = commands.spawn(PbrBundle {
        mesh: ball_mesh_resource.handle.clone(),
        material: material_handle,
        ..default()
    });
    
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
        mesh: meshes.add(Mesh::from(Capsule3d {
            radius,
            half_length: depth,
            ..Default::default()
        })),
        material: materials.add(Color::rgb(0.7, 0.7, 0.7)),
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