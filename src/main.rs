use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use rand::Rng;
use smooth_bevy_cameras::{
    controllers::orbit::{OrbitCameraBundle, OrbitCameraController, OrbitCameraPlugin},
    LookTransformPlugin,
};
use smooth_bevy_cameras::{LookTransform, LookTransformBundle, Smoother};

use bevy::input::mouse::{MouseButtonInput, MouseMotion};

use bevy_mod_picking::prelude::*;

use std::f32::consts::PI;

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
struct Speed(f32);

#[derive(Component)]
pub struct FixedBall;

#[derive(Component)]
pub struct Globe;

#[derive(Resource)]
pub struct GlobePos(pub Vec3);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(
            0xF9 as f32 / 255.0,
            0xF9 as f32 / 255.0,
            0xFF as f32 / 255.0,
        )))
        //.insert_resource(WinitSettings::desktop_app())
        .insert_resource(GlobePos(Vec3::new(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(LookTransformPlugin)
        .add_plugins(OrbitCameraPlugin::default())
        // Mod Picking
        .add_plugins(DefaultPickingPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        //.add_plugin(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup_graphics)
        .add_systems(Startup, setup_physics)
        .add_systems(Update, push_ball_against_globe)
        .add_systems(Update, handle_ball_events)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.1,
    });
  
        // directional 'sun' light
        const HALF_SIZE: f32 = 10.0;
        commands.spawn(DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: false,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 2.0, 0.0),
                rotation: Quat::from_rotation_x(-PI / 4.),
                ..default()
            },
            ..default()
        });
    /* 
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1000.0,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 6.0, 0.0),
        ..default()
    });*/

    /*commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(3.0, 1.0, 0.0),
        ..default()
    });*/
    
    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::ORANGE_RED,
        brightness: 0.02,
    });
    /*commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 6.0, 0.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Z),
        ..Default::default()
    });*/

    commands
    .spawn(Camera3dBundle::default())
    .insert(OrbitCameraBundle::new(
        OrbitCameraController::default(),
        Vec3::new(0.0, 6.0, 0.0),
        Vec3::new(0., 0., 0.),
        Vec3::Y,
    ))
    .insert(RaycastPickCamera::default());
}

pub fn setup_physics(mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut rapier_config: ResMut<RapierConfiguration>,
    globe_pos: Res<GlobePos>,) {
    rapier_config.gravity = Vec3::new(0.0, 0.0, 0.0);
    /*
     * The globe
     */
    let globe_radius = 1.0;

    commands.spawn(
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: globe_radius,
                ..default()
            })),
            material: materials.add(Color::BLACK.into()),
            ..default()
        }
        )
        //TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)),
        .insert((TransformBundle::from(Transform::from_translation(globe_pos.0)),
        RigidBody::Fixed,
        //Restitution::coefficient(0.0),
        Restitution {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        },
        Collider::ball(globe_radius),
        Friction::coefficient(0.0),
        Globe
        )
    );


    //Generate moving balls
    //let mut rng = rand::thread_rng();
    //let chosen_index = rng.gen_range(0..5);
    //let random_number: f32 = rng.uniform(1.0..2.0);

    let ball_radius: f32 = 0.05;

    let ball_mesh_handle: Handle<Mesh> = meshes.add(Mesh::from(shape::UVSphere {
        radius: ball_radius,
        ..default()
    }));

    let ball_material_handle = materials.add(Color::ORANGE_RED.into());

    let mut rng = rand::thread_rng();
    for _i in 0..50 {
        let point_on_sphere = random_point_on_sphere(ball_radius + globe_radius);

        //ball
        commands.spawn(
            PbrBundle {
                mesh: ball_mesh_handle.clone(),
                material: ball_material_handle.clone(),
                ..default()
            }
            )
        .insert((
            //TransformBundle::from(Transform::from_xyz([-1.0, 1.0][rng.gen_range(0..2)] *rng.gen_range(1.0..2.0), [-1.0, 1.0][rng.gen_range(0..2)] * rng.gen_range(1.0..2.0), [-1.0, 1.0][rng.gen_range(0..2)] * rng.gen_range(1.0..2.0))),
            TransformBundle::from(Transform::from_xyz(point_on_sphere.0, point_on_sphere.1, point_on_sphere.2)),
            Sleeping::disabled(),
            Ccd::enabled(),
            Collider::ball(ball_radius),
            Friction::coefficient(0.0),
            RigidBody::Dynamic,
            //Restitution::coefficient(1.0),
            Restitution {
                coefficient: 1.0,
                combine_rule: CoefficientCombineRule::Max,
            },
            ExternalForce {
                force: Vec3::new(0.0, 0.0, 0.0),
                torque: Vec3::new(0.0, 0.0, 0.0),
            },
            ExternalImpulse {
                impulse: Vec3::new(rng.gen_range(-0.0001..0.0001), rng.gen_range(-0.0001..0.0001), rng.gen_range(-0.0001..0.0001)),
                //impulse: Vec3::new(0.0, 0.0, 0.0),
                torque_impulse: Vec3::new(0.0, 0.0, 0.0),
            },
            Velocity {
                linvel: Vec3::new(0.0, 0.0, 0.0),
                angvel: Vec3::new(0.0, 0.0, 0.0),
            },
            ActiveEvents::COLLISION_EVENTS,
            ReadMassProperties::default(),
            Ball,
            Speed(0.0)
        ));        
    }
   
    //Generate fixed balls
    let fixed_ball_radius: f32 = 0.05;

    let fixed_ball_mesh_handle: Handle<Mesh> = meshes.add(Mesh::from(shape::UVSphere {
        radius: fixed_ball_radius,
        ..default()
    }));

    let fixed_ball_material_handle = materials.add(Color::BLUE.into());
    

    for _i in 0..30 {
        let point_on_sphere = random_point_on_sphere(fixed_ball_radius + globe_radius);

        //ball
        commands.spawn(
            PbrBundle {
                mesh: fixed_ball_mesh_handle.clone(),
                material: fixed_ball_material_handle.clone(),
                ..default()
            }
            )
        .insert((
            //TransformBundle::from(Transform::from_xyz([-1.0, 1.0][rng.gen_range(0..2)] *rng.gen_range(1.0..2.0), [-1.0, 1.0][rng.gen_range(0..2)] * rng.gen_range(1.0..2.0), [-1.0, 1.0][rng.gen_range(0..2)] * rng.gen_range(1.0..2.0))),
            TransformBundle::from(Transform::from_xyz(point_on_sphere.0, point_on_sphere.1, point_on_sphere.2)),
            Collider::ball(fixed_ball_radius),
            Friction::coefficient(0.0),
            Restitution::coefficient(1.0),
            RigidBody::Fixed,
            FixedBall
        ));        
    }
}


fn push_ball_against_globe(
    mut query_balls: Query<(&mut ExternalForce, &Transform, &ReadMassProperties), With<Ball>>,
    globe_pos: Res<GlobePos>,
) {
    //println!("gvtest1");
    let gravity = 9.8;

    for (mut ball_force, ball_transform, ball_mass_props) in query_balls.iter_mut() {
        let force = gravity * ball_mass_props.0.mass;
        let force_unit_vec = (globe_pos.0 - ball_transform.translation).normalize();

        //println!("force_unit_vec {:?} ", force_unit_vec);

        ball_force.force = force_unit_vec * force;

    }
}


fn random_point_on_sphere(r: f32) -> (f32, f32, f32) {
    let mut rng = rand::thread_rng();

    // Keep generating random points until we find one that lies on the surface of the sphere.
    loop {
        let x = rng.gen_range(-r..r);
        let y = rng.gen_range(-r..r);
        let z = rng.gen_range(-r..r);

        // Check if the point lies on the surface of the sphere.
        if (x*x + y*y + z*z - r*r).abs() < f32::EPSILON {
            //println!("xyz {:?} ", (x,y,z));
            //println!("x*x + y*y + z*z - r*r : {:?} ", x*x + y*y + z*z - r*r);
            return (x, y, z);
        }
    }
}


fn handle_ball_events(
    mut query_balls: Query<(Entity, &mut Velocity, &mut Speed), With<Ball>>,
    mut contact_events: EventReader<CollisionEvent>,
) {
    for contact_event in contact_events.iter() {
        if let CollisionEvent::Started(h1, h2, _event_flag) = contact_event {
            //println!("gvtest0");
            for (entity_ball, mut velocity, mut speed) in query_balls.iter_mut() {
                //println!("gvtest1");
                if h1 == &entity_ball || h2 == &entity_ball {
                    //Keep incoming speed
                    if speed.0 == 0.0{
                        speed.0 = velocity.linvel.length();
                    }
                }
            }
        }
        if let CollisionEvent::Stopped(h1, h2, _event_flag) = contact_event {
            for (entity_ball, mut velocity, speed) in query_balls.iter_mut() {
                if h1 == &entity_ball || h2 == &entity_ball {
                    //Set outgoing speed size/length to be equal incoming speed size/lenght
                    let normalized_velocity = velocity.linvel.normalize();
                    velocity.linvel = normalized_velocity * speed.0;
                    //println!("gvtest2");
                    //println!("velocity.linvel.length = {:?},  speed = {:?}", velocity.linvel.length(), speed.0);
                }
            }   
        }
    }
}



/* 
//-Raycast mot sentrum av globen.
//-Hvis treffer globe (og ikke annen ball) så spawn ny ball i raycast-treff-punkt + radius ut i retning fra globe-senter.
// Må få mouse-koordinater
fn spawn_ball_on_globe(
    mut query_globe: Query<(Entity), With<Globe>>,
    mut query_balls: Query<(&mut ExternalForce, &mut Velocity, &Transform, &Collider), With<Ball>>,
    cameras: Query<(&OrbitCameraController, &LookTransform, &Transform, &Camera, &GlobalTransform)>,
    rapier_context: Res<RapierContext>,
    mouse: Res<Input<MouseButton>>,
    windows: Res<Windows>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return
    }
    
    let window = windows.get_primary().unwrap();
    let mouse_pos;
    if let Some(_position) = window.cursor_position() {
        // cursor is inside the window, position given
        mouse_pos = _position;
    } else {
        // cursor is not inside the window
        return
    }

    let (mut transform, scene_transform, camera, global_transform) =
        if let Some((_, transform, scene_transform, camera, global_transform)) = cameras.iter_mut().find(|c| c.0.enabled) {
            (transform, scene_transform, camera, global_transform)
        } else {
            return;
        };
    
    //let ray_pos = Vec3::new(1.0, 2.0, 3.0);
    let ray_pos = transform.eye;
    //let ray_dir = Vec3::new(0.0, 1.0, 0.0);
    let ray_dir = camera.viewport_to_world(global_transform, mouse_pos).unwrap();
    let max_toi = 4.0;
    let solid = true;
    //let filter = None;

    if let Some((entity, toi)) = rapier_context.cast_ray(
        ray_pos, ray_dir.direction, max_toi, solid, QueryFilter::default()
    ) {
        // The first collider hit has the entity `entity` and it hit after
        // the ray travelled a distance equal to `ray_dir * toi`.
        let hit_point = ray_pos + ray_dir * toi;
        println!("Entity {:?} hit at point {}", entity, hit_point);
    }

    for (mut ball_force, _ball_velocity, ball_transform, ball_collider) in query_balls.iter_mut() {
        let ray_pos = Vec3::new(1.0, 2.0, 3.0);//camera
        let ray_dir = Vec3::new(0.0, 1.0, 0.0);//Unitvector from camera to mouse/touch
        let max_toi = 100.0;
        let cast_velocity = Vec3::new(0.0, 0.0, -1.0);
        let filter = QueryFilter {
            groups: Some(
                CollisionGroups {
                    memberships: Group::GROUP_3,
                    filters: Group::GROUP_1,
                }
                .into(),
            ),
            ..default()
        };

        if let Some((_entity, hit)) = rapier_context.cast_shape(
            ball_transform.translation,
            ball_transform.rotation,
            cast_velocity,
            ball_collider,
            max_toi,
            filter,
        ) {
            if hit.toi > 0.0 {
                ball_force.force = Vec3::new(0.0, 0.0, -0.0001);
            } else {
                ball_force.force = Vec3::new(0.0, 0.0, 0.0);
            }
        }
    }
}
*/