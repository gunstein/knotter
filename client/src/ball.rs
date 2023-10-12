use bevy::{prelude::*, window::CursorGrabMode};
use bevy_rapier3d::prelude::*;
//use smooth_bevy_cameras::controllers::orbit::OrbitCameraController;
//use smooth_bevy_cameras::LookTransform;
use rand::Rng;
use super::globe;
use crate::AppState;
use super::ball;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app
            //.add_systems(Startup, spawn_static_balls)
            .add_systems(Startup, init_ball_resources)
            //.add_systems(Startup, spawn_moving_balls)
            .add_systems(Update, push_ball_against_globe)
            .add_systems(Update, handle_ball_collision)
            .add_systems(Update, upsert_ball_on_globe.run_if(in_state(AppState::Upsert)))
            .add_systems(Update, upsert_set_speed.run_if(in_state(AppState::UpsertSetSpeed)))
            .insert_resource(BallRadius(0.05))
            .register_type::<Ball>();
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Ball; 


#[derive(Component)]
struct Speed(f32);

#[derive(Component)]
pub struct StaticBall;

#[derive(Component)]
pub struct MovingBall;

#[derive(Component)]
pub struct Upserted;

#[derive(Component)]
pub struct CapsuleDepth(f32);

#[derive(Component)]
pub struct CapsuleRotation(Quat);

#[derive(Component)]
pub struct SpeedMarker;

#[derive(Resource)]
struct HandleForBallMesh {
    handle: Handle<Mesh>,
}

#[derive(Resource)]
struct HandleForBallMaterial {
    handle: Handle<StandardMaterial>,
}

#[derive(Resource)]
pub struct BallRadius(pub f32);


//add mesh and material for ball and add to resource
fn init_ball_resources(mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ball_radius: Res<BallRadius>) {

    let ball_mesh_handle: Handle<Mesh> = meshes.add(Mesh::from(shape::UVSphere {
        radius: ball_radius.0,
        ..default()
    }));

    commands.insert_resource(HandleForBallMesh { handle: ball_mesh_handle });     

    let ball_material_handle = materials.add(Color::BLUE.into());
    commands.insert_resource(HandleForBallMaterial { handle: ball_material_handle });
    
}


fn spawn_static_ball(commands: &mut Commands, 
    ball_mesh_resource: &Res<HandleForBallMesh>,
    ball_material_resource: &Res<HandleForBallMaterial>,
    ball_radius: &Res<BallRadius>,
    point_on_sphere: (f32, f32, f32),
    upserted: bool) {

    //ball
    let mut spawned_entity = commands.spawn((
        PbrBundle {
            mesh: ball_mesh_resource.handle.clone(),
            material: ball_material_resource.handle.clone(),
            ..default()
        },
    ));

    spawned_entity.insert((
        //TransformBundle::from(Transform::from_xyz([-1.0, 1.0][rng.gen_range(0..2)] *rng.gen_range(1.0..2.0), [-1.0, 1.0][rng.gen_range(0..2)] * rng.gen_range(1.0..2.0), [-1.0, 1.0][rng.gen_range(0..2)] * rng.gen_range(1.0..2.0))),
        TransformBundle::from(Transform::from_xyz(point_on_sphere.0, point_on_sphere.1, point_on_sphere.2)),
        Collider::ball(ball_radius.0),
        Friction::coefficient(0.0),
        Restitution::coefficient(1.0),
        RigidBody::Fixed,
        CollisionGroups {
            memberships: Group::GROUP_2,
            filters: (Group::GROUP_1 | Group::GROUP_2),
        },
        StaticBall
    ));

    if upserted {
        spawned_entity.insert(Upserted);
    }              
}

fn spawn_moving_ball(commands: &mut Commands, 
    ball_mesh_resource: &Res<HandleForBallMesh>,
    ball_material_resource: &Res<HandleForBallMaterial>,
    ball_radius: &Res<BallRadius>,
    point_on_sphere: (f32, f32, f32),
    impulse: Vec3 ) {

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
        Collider::ball(ball_radius.0),
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
    ));

    spawned_entity.insert(Speed(0.0));          
}



fn push_ball_against_globe(
    mut query_balls: Query<(&mut ExternalForce, &Transform, &ReadMassProperties), With<MovingBall>>,
    globe_pos: Res<globe::GlobePos>,
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

fn handle_ball_collision(
    mut query_balls: Query<(Entity, &mut Velocity, &mut Speed), With<MovingBall>>,
    mut contact_events: EventReader<CollisionEvent>,
) {

    for contact_event in contact_events.iter() {
        //Keep incoming speed in Speed component
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


//-Raycast mot sentrum av globen.
//-Hvis treffer globe (og ikke annen ball) så spawn ny ball i raycast-treff-punkt + radius ut i retning fra globe-senter.
// Må få mouse-koordinater
fn upsert_ball_on_globe(
    mut commands: Commands,
    ball_mesh_resource: Res<HandleForBallMesh>,
    ball_material_resource: Res<HandleForBallMaterial>,
    ball_radius: Res<BallRadius>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    rapier_context: Res<RapierContext>,
    mouse: Res<Input<MouseButton>>,
    windows: Query<&mut Window>,
    query_globe: Query<Entity, With<globe::Globe>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return
    }
    
    //println!("Left button pushed");

    let window = windows.single();

    let Some(cursor_position) = window.cursor_position() else { return; };

    for (camera, camera_transform) in &cameras {
        // First, compute a ray from the mouse position.
        let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else { return; };

        let ball_shape = Collider::ball(ball_radius.0);
        let shape_rot = Quat::from_rotation_z(0.0);
        // Then cast the ray.
        if let Some((entity, hit)) = rapier_context.cast_shape(
            ray.origin,
            shape_rot,
            ray.direction,
            &ball_shape,
            f32::MAX,
            QueryFilter::default(),
        ){
            for entity_globe in query_globe.iter() {
                if entity_globe == entity {
                    let hit_point = ray.origin + ray.direction * hit.toi;
                    spawn_static_ball(&mut commands, 
                        &ball_mesh_resource,
                        &ball_material_resource,
                        &ball_radius,
                        (hit_point.x, hit_point.y, hit_point.z),
                        true
                    );
                }
            }
            // The first collider hit has the entity `entity`. The `hit` is a
            // structure containing details about the hit configuration.
            //println!("Hit the entity {:?} with the configuration: {:?}", entity, hit);
        }            
    }

    //println!("Finished");

    next_state.set(AppState::UpsertSetSpeed);

}

//Use mouse to set speed and direction of ball
fn upsert_set_speed(
    mut commands: Commands,
    ball_radius: Res<BallRadius>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    rapier_context: Res<RapierContext>,
    mouse: Res<Input<MouseButton>>,
    windows: Query<&mut Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query_upsert_ball: Query<(Entity, &Transform, &Handle<StandardMaterial>), With<ball::Upserted>>,
    query_speed_marker: Query<(Entity, &CapsuleDepth, &CapsuleRotation), With<SpeedMarker>>,
    mut next_state: ResMut<NextState<AppState>>,
    ball_mesh_resource: Res<HandleForBallMesh>,
    ball_material_resource: Res<HandleForBallMaterial>,
){  
    let window = windows.single();
    let Some(cursor_position) = window.cursor_position() else { return; };
    
    
    //Do raycast to check pointer is on globe
    for (camera, camera_transform) in &cameras {
        //Find ball marked for upsert
        //println!("gvtest1");
        let upsert_ball = if let Ok(result) = query_upsert_ball.get_single_mut() {
            result
        } else {
            next_state.set(AppState::Upsert);//??
            return;
        };
        

        //println!("gvtest2");
        // First, compute a ray from the mouse position.
        let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else { return; };
        //Only hit globe, globe is only member of CollisionGroup GROUP_1
        let filter = QueryFilter {
            groups: Some(
                CollisionGroups {
                    memberships: Group::GROUP_2,
                    filters: Group::GROUP_1,
                }
            ),
            ..default()
        };
        //println!("gvtest3");
        // Then cast the ray. Maybe cast_ray_and_get_normal, if I need the hit point normal.
        if let Some((entity, toi)) = rapier_context.cast_ray(
            ray.origin,
            ray.direction,
            f32::MAX,
            true,
            filter,
        ){
            //println!("gvtest mouse on globe");
            //if left mousebutton is pressed, then make entity dynamic  and set speed, set next state to upsert
            //if speed length is close to zero, spawn static ball
            if mouse.just_pressed(MouseButton::Left) {
                //println!("gvtest left mouse button");
                //Keep material and position
                let ball_position = upsert_ball.1.translation;
                //let ball_material = upsert_ball.2;

                //despawn speed marker
                let (speed_marker, capsule_depth, capsule_rotation) = query_speed_marker.single();
                commands.entity(speed_marker).despawn();
                
                if capsule_depth.0 > 0.05{
                    //spawn dynamic
                    //despawn upsert ball
                    commands.entity(upsert_ball.0).despawn();

                     //compute impulse
                     let forward_direction = capsule_rotation.0.mul_vec3(Vec3::Y).normalize();
                     let impulse_magnitude = capsule_depth.0 * 0.0006; //Should scale?
                     //let impulse = forward_direction * impulse_magnitude;
                     let impulse = forward_direction * impulse_magnitude;

                     spawn_moving_ball(&mut commands, 
                        &ball_mesh_resource,
                        &ball_material_resource,
                        &ball_radius,
                        (ball_position.x, ball_position.y, ball_position.z),
                        impulse );
                }
                else{
                    //Remove Upsert component on ball. The ball is then permanent static.
                    commands.entity(upsert_ball.0).remove::<Upserted>();
                }

                //nextstate = upsert
                next_state.set(AppState::Upsert);
            }
            else{
                //else draw pipe
                //println!("gvtest no button pressed");
                //despawn previous pipe speed marker if it exists
                if let Ok((entity, _, _))  = query_speed_marker.get_single(){
                  commands.entity(entity).despawn();
                  //println!("Despawn query_speed_marker.");
                }

                //println!("gvtest x");

                // Starting and ending points of your line
                let start = upsert_ball.1.translation;
                let mut end = ray.origin + ray.direction * toi;

                let normal_start = start.normalize();
                let normal_end = end.normalize();
                let average_normal = (normal_start + normal_end).normalize();
                                
                // Compute the length and orientation of the line segment
                let mut length = start.distance(end);
                let orientation = (end - start).normalize();                

                let max_length= 0.5;

                // Check if the current length exceeds the maximum
                if length > max_length {
                    end = start + orientation * max_length; // Adjust the end point
                    length = max_length; // Update the length to be the maximum length
                }

                // Compute the middle point of the line segment
                let middle = (start + end) / 2.0;
                let shifted_middle = middle + average_normal * 0.1; //Move the middle a litle bit outside globe

                // Convert direction to a rotation Quat
                let forward = Vec3::Y; 
                let rotation = Quat::from_rotation_arc(forward, orientation);

                let capsule_depth = length - 0.05 * 2.0;
                commands.spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Capsule {
                        radius: 0.05, // Adjust the thickness of your line
                        depth: capsule_depth, // Subtract the capsule's endcaps
                        ..Default::default()
                    })),
                    material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
                    transform: Transform {
                        translation: shifted_middle,
                        rotation: rotation,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(CapsuleDepth(capsule_depth))
                .insert(CapsuleRotation(rotation))
                .insert(SpeedMarker);
            }
        } 
        else {
            //println!("gvtest mouse not on globe");

            //despawn speed marker
            if let Ok((entity, _, _))  = query_speed_marker.get_single(){
                commands.entity(entity).despawn();
                //println!("Despawn query_speed_marker.");
            }
            //if left mousebutton clicked then despawn pipe and set next state to Upsert (user clicked outside globe)
            //Remove upserted component so ball will be a premanent static ball
            if mouse.just_pressed(MouseButton::Left) {
                commands.entity(upsert_ball.0).remove::<Upserted>();

                //nextstate = upsert
                next_state.set(AppState::Upsert);
            }
        }       
    }
}