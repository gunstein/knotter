use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct GlobePlugin;

impl Plugin for GlobePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_globe)
            .insert_resource(GlobeName(crate::get_query_param("globe").unwrap()))
            .insert_resource(GlobePos(Vec3::new(0.0, 0.0, 0.0)))
            .insert_resource(GlobeRadius(1.0))
            .register_type::<Globe>();
    }
}

#[derive(Resource)]
pub struct GlobeName(pub String);

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Globe; 

#[derive(Resource)]
pub struct GlobePos(pub Vec3);

#[derive(Resource)]
pub struct GlobeRadius(pub f32);

fn spawn_globe(mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    globe_pos: Res<GlobePos>,
    globe_radius: Res<GlobeRadius>,) {

    commands.spawn(
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: globe_radius.0,
                ..default()
            })),
            material: materials.add(Color::BLACK.into()),
            ..default()
        }
        )
        .insert((TransformBundle::from(Transform::from_translation(globe_pos.0)),
        RigidBody::Fixed,
        Restitution {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        },
        Collider::ball(globe_radius.0),
        Friction::coefficient(0.0),
        CollisionGroups {
            memberships: Group::GROUP_1,
            filters: (Group::GROUP_2),
        },
        Globe
        )
    );
}
