use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use shared::domain::dtos::ball_transaction_dto::BallTransactionDto;
use uuid::Uuid;

use super::BALL_RADIUS;
use super::components::*;
use super::resources::*;
use super::spawn::*;
use crate::AppState;
use crate::globe;
use shared::domain::dtos::ball_dto::BallDto;
use shared::domain::dtos::position_dto::PositionDto;
use shared::domain::dtos::impulse_dto::ImpulseDto;
use bevy_mod_reqwest::{*, reqwest::Url};
use std::collections::HashSet;

const SPEED_MARKER_MAX_LENGTH: f32 = 0.5;

//add mesh and material for ball and add to resource
pub fn init_ball_resources(mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    let ball_mesh_handle: Handle<Mesh> = meshes.add(Mesh::from(shape::UVSphere {
        radius: BALL_RADIUS,
        ..default()
    }));

    commands.insert_resource(HandleForBallMesh { handle: ball_mesh_handle });     

    let ball_material_handle = materials.add(Color::BLUE.into());
    commands.insert_resource(HandleForBallMaterial { handle: ball_material_handle });
    
}

pub fn push_ball_against_globe(
    mut query_balls: Query<(&mut ExternalForce, &Transform, &ReadMassProperties), With<MovingBall>>,
    globe_pos: Res<globe::GlobePos>,
) {
    let gravity = 9.8;

    for (mut ball_force, ball_transform, ball_mass_props) in query_balls.iter_mut() {
        let force = gravity * ball_mass_props.0.mass;
        let force_unit_vec = (globe_pos.0 - ball_transform.translation).normalize();

        //println!("force_unit_vec {:?} ", force_unit_vec);

        ball_force.force = force_unit_vec * force;

    }
}

pub fn handle_ball_collision(
    mut query_balls: Query<(Entity, &mut Velocity, &mut Speed), With<MovingBall>>,
    mut contact_events: EventReader<CollisionEvent>,
) {

    for contact_event in contact_events.iter() {
        //Keep incoming speed in Speed component
        if let CollisionEvent::Started(h1, h2, _event_flag) = contact_event {
            //println!("gvtest0");
            for (entity_ball, velocity, mut speed) in query_balls.iter_mut() {
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


//-Raycast mot sentrum av globen.
//-Hvis treffer globe (og ikke annen ball) så spawn ny ball i raycast-treff-punkt + radius ut i retning fra globe-senter.
// Må få mouse-koordinater
//Add a ball to the globe and set state to EditUpsertSetSpeed
pub fn edit_upsert_ball_on_globe(
    mut commands: Commands,
    ball_mesh_resource: Res<HandleForBallMesh>,
    ball_material_resource: Res<HandleForBallMaterial>,
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

        let ball_shape = Collider::ball(BALL_RADIUS);
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
                        (hit_point.x, hit_point.y, hit_point.z),
                        true,
                        None
                    );
                }
            }
            // The first collider hit has the entity `entity`. The `hit` is a
            // structure containing details about the hit configuration.
            //println!("Hit the entity {:?} with the configuration: {:?}", entity, hit);
        }            
    }

    //println!("Finished");

    next_state.set(AppState::EditUpsertSetSpeed);

}

//Use mouse to set speed and direction of ball
//Draw speed marker as long as left mouse button is pressed down.
pub fn edit_upsert_set_speed(
    mut commands: Commands,
    cameras: Query<(&Camera, &GlobalTransform)>,
    rapier_context: Res<RapierContext>,
    mouse: Res<Input<MouseButton>>,
    windows: Query<&mut Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query_upsert_ball: Query<(Entity, &Transform, &Handle<StandardMaterial>), With<Upserted>>,
    query_speed_marker: Query<(Entity, &CapsuleDepth, &CapsuleRotation), With<SpeedMarker>>,
    mut next_state: ResMut<NextState<AppState>>,
){
    //Only works when left mouse button is pressed
    if !mouse.pressed(MouseButton::Left) {
        return
    }

    let window = windows.single();
    let Some(cursor_position) = window.cursor_position() else { return; };

    //Do raycast to check pointer is on globe
    for (camera, camera_transform) in &cameras {
        //Find ball marked for upsert, if not found something is wrong so go to EditUpsertState
        //println!("gvtest1");
        let upsert_ball = if let Ok(result) = query_upsert_ball.get_single_mut() {
            result
        } else {
            next_state.set(AppState::EditUpsert);
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
        if let Some((_, toi)) = rapier_context.cast_ray(
            ray.origin,
            ray.direction,
            f32::MAX,
            true,
            filter,
        ){
            //Draw speedmarker
            //println!("gvtest no button pressed");
            //despawn previous pipe speed marker if it exists
            if let Ok((entity, _, _))  = query_speed_marker.get_single(){
                commands.entity(entity).despawn();
                //println!("Despawn query_speed_marker.");
            }

            // Starting and ending points of your line
            let start = upsert_ball.1.translation;
            let mut end = ray.origin + ray.direction * toi;

            let normal_start = start.normalize();
            let normal_end = end.normalize();
            let average_normal = (normal_start + normal_end).normalize();
                            
            // Compute the length and orientation of the line segment
            let mut length = start.distance(end);
            let orientation = (end - start).normalize();                

            // Check if the current length exceeds the maximum
            if length > SPEED_MARKER_MAX_LENGTH {
                end = start + orientation * SPEED_MARKER_MAX_LENGTH; // Adjust the end point
                length = SPEED_MARKER_MAX_LENGTH; // Update the length to be the maximum length
            }

            // Compute the middle point of the line segment
            let middle = (start + end) / 2.0;
            let shifted_middle = middle + average_normal * 0.1; //Move the middle a litle bit outside globe

            // Convert direction to a rotation Quat
            let forward = Vec3::Y; 
            let rotation = Quat::from_rotation_arc(forward, orientation);

            let capsule_depth = length - 0.05 * 2.0;
            let capsule_radius = 0.05;

            spawn_speed_marker(&mut commands, &mut meshes, &mut materials,
                capsule_depth, capsule_radius,
                shifted_middle,
                rotation,
            );
        }
    }
}


//Is active when in EditUpsertSetSpeed state and when left mouse button is just released
pub fn finalize_upsert_ball_on_globe(
    mut commands: Commands,
    ball_mesh_resource: Res<HandleForBallMesh>,
    ball_material_resource: Res<HandleForBallMaterial>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    rapier_context: Res<RapierContext>,
    mouse: Res<Input<MouseButton>>,
    windows: Query<&mut Window>,
    mut query_upsert_ball: Query<(Entity, &Transform, &Handle<StandardMaterial>, &BallUuid), With<Upserted>>,
    query_speed_marker: Query<(Entity, &CapsuleDepth, &CapsuleRotation), With<SpeedMarker>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut send_insert_ball_events: EventWriter<crate::query_server::SendInsertBallEvent>,
) {
    if !mouse.just_released(MouseButton::Left) {
        return
    }
    
    let window = windows.single();
    let Some(cursor_position) = window.cursor_position() else { return; };

    //Do raycast to check pointer is on globe
    for (camera, camera_transform) in &cameras {
        //Find ball marked for upsert, if not found something is wrong so go to EditUpsertState
        //println!("gvtest1");
        let upsert_ball = if let Ok(result) = query_upsert_ball.get_single_mut() {
            result
        } else {
            next_state.set(AppState::EditUpsert);
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

        let (speed_marker, capsule_depth, capsule_rotation) = query_speed_marker.single();
        commands.entity(speed_marker).despawn();

        if let Some((_, _)) = rapier_context.cast_ray(
            ray.origin,
            ray.direction,
            f32::MAX,
            true,
            filter,
        ){
            let ball_position = upsert_ball.1.translation;
            
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
                    (ball_position.x, ball_position.y, ball_position.z),
                    impulse,
                    Some(upsert_ball.3.0) );
                send_insert_ball_event(&mut send_insert_ball_events, upsert_ball.3.0, ball_position, Some(impulse));
            }
            else{
                //Remove Upsert component on ball. The ball is then permanent static.
                commands.entity(upsert_ball.0).remove::<Upserted>();
                send_insert_ball_event(&mut send_insert_ball_events, upsert_ball.3.0, ball_position, None);
            }
        }
        else{
            //Mouse did not hit globe so ball will be fixed.
            commands.entity(upsert_ball.0).remove::<Upserted>();
            send_insert_ball_event(&mut send_insert_ball_events, upsert_ball.3.0, upsert_ball.1.translation, None);
        }
    }
    next_state.set(AppState::EditUpsert);

}


pub fn edit_delete_ball(
    mut commands: Commands,
    cameras: Query<(&Camera, &GlobalTransform)>,
    rapier_context: Res<RapierContext>,
    mouse: Res<Input<MouseButton>>,
    windows: Query<&mut Window>,
    query_globe: Query<Entity, With<globe::Globe>>,
    mut send_delete_ball_events: EventWriter<crate::query_server::SendDeleteBallEvent>,
    query_balls: Query<(Entity, &BallUuid)>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return
    }
    
    let window = windows.single();
    let Some(cursor_position) = window.cursor_position() else { return; };

    //Do raycast to check pointer is on globe
    for (camera, camera_transform) in &cameras {
        // First, compute a ray from the mouse position.
        let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else { return; };
        //Only hit ball, ball is only member of CollisionGroup GROUP_2
        let filter = QueryFilter {
            groups: Some(
                CollisionGroups {
                    memberships: Group::GROUP_2,
                    filters: (Group::GROUP_1 | Group::GROUP_2)
                }
            ),
            ..default()
        };

        if let Some((entity, _)) = rapier_context.cast_ray(
            ray.origin,
            ray.direction,
            f32::MAX,
            true,
            filter,
        ){
            let entity_globe = query_globe.single();
            if entity_globe != entity {
                //Despawn if not globe, then it should be a ball
                commands.entity(entity).despawn();

                for (entity_ball, uuid_ball) in query_balls.iter() {
                    if entity == entity_ball {
                        send_delete_ball_events.send(crate::query_server::SendDeleteBallEvent {uuid: uuid_ball.0});
                    }
                }  
            }
        }
    }
}

pub fn handle_delete_state(
    keyboard_input: Res<Input<KeyCode>>,
    current_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.pressed(KeyCode::Delete) {
        if *current_state == AppState::EditUpsert {
            next_state.set(AppState::EditDelete);
        }
    } else {
        if *current_state == AppState::EditDelete {
            next_state.set(AppState::EditUpsert);
        }
    }
}

fn send_insert_ball_event(
    send_insert_ball_events: &mut EventWriter<crate::query_server::SendInsertBallEvent>,
    ball_uuid: Uuid,
    ball_position: Vec3,
    ball_impulse: Option<Vec3>,
) {
    // is_fixed is true if ball_impulse is None, false otherwise
    let is_fixed = ball_impulse.is_none();
    send_insert_ball_events.send(crate::query_server::SendInsertBallEvent {
        ball: BallDto {
            is_fixed,
            is_insert: true,
            uuid: ball_uuid,
            color: Some("#ff0000".to_string()),
            position: Some(PositionDto {
                x: ball_position.x,
                y: ball_position.y,
                z: ball_position.z,
            }),
            impulse: ball_impulse.map(|impulse| ImpulseDto { // Use map to convert Option<Vec3> to Option<ImpulseDto>
                x: impulse.x,
                y: impulse.y,
                z: impulse.z,
            }),
        }
    });
}

pub fn receive_ball_transactions_event_listener(
    mut commands: Commands, 
    mut events: EventReader<crate::query_server::ReceiveBallTransactionsEvent>, 
    ball_mesh_resource: Res<HandleForBallMesh>,
    ball_material_resource: Res<HandleForBallMaterial>,
    query_balls: Query<(Entity, &BallUuid)>
) {
    for event in events.iter() {
        let mut balls_to_insert = HashSet::new();
        let mut balls_to_delete = HashSet::new();

        // First pass: Determine which balls to insert and delete
        for ball_transaction in &event.ball_transactions.ball_transactions {
            let uuid = ball_transaction.ball_dto.uuid;
            if ball_transaction.ball_dto.is_insert {
                balls_to_insert.insert(uuid);
            } else {
                balls_to_insert.remove(&uuid);
                balls_to_delete.insert(uuid);
            }
        }

        // Second pass: Handle insertions
        for uuid in balls_to_insert {
            // Check if a ball with this UUID already exists
            let uuid_strings: Vec<String> = query_balls.iter()
            .map(|(_, ball_uuid)| ball_uuid.0.to_string())
            .collect();
            bevy::log::info!("Collected UUIDs: {:?}", uuid_strings);
            let ball_already_exists = query_balls.iter().any(|(_, uuid_ball)| uuid_ball.0 == uuid);

            if !ball_already_exists {
                // Find the first transaction with the matching UUID
                // This line searches through all transactions in the current event
                // for the first one where the transaction's UUID matches the UUID in the balls_to_insert set.
                // 'if let Some(ball_transaction)' is used to safely handle the case where
                // such a transaction is found (Some) or not found (None).
                if let Some(ball_transaction) = event.ball_transactions.ball_transactions.iter().find(|bt| bt.ball_dto.uuid == uuid) {
                    handle_insert_ball_transaction(
                        &mut commands,
                        &ball_mesh_resource,
                        &ball_material_resource,
                        ball_transaction,
                    );
                }
            }
        }

        // Handle deletions
        for uuid in balls_to_delete {
            if let Some((entity_ball, _)) = query_balls.iter().find(|(_, uuid_ball)| uuid_ball.0 == uuid) {
                commands.entity(entity_ball).despawn();
            }
        }
    }
}


fn handle_insert_ball_transaction(
    commands: &mut Commands,
    ball_mesh_resource: &Res<HandleForBallMesh>,
    ball_material_resource: &Res<HandleForBallMaterial>,
    ball_transaction: &BallTransactionDto,
) {
    let position = match &ball_transaction.ball_dto.position {
        Some(pos) => pos,
        None => {
            // Log error or handle the case of missing position
            bevy::log::error!("Missing position! Not good.");
            return;
        },
    };

    if ball_transaction.ball_dto.is_fixed {
        spawn_static_ball(
            commands, 
            ball_mesh_resource,
            ball_material_resource,
            (position.x, position.y, position.z),
            false,
            Some(ball_transaction.ball_dto.uuid)
        );
    } else {
        let impulse = match &ball_transaction.ball_dto.impulse {
            Some(imp) => imp,
            None => {
                bevy::log::error!("Missing impulse! Not good on a moving ball.");
                return;
            },
        };

        spawn_moving_ball(
            commands, 
            ball_mesh_resource,
            ball_material_resource,
            (position.x, position.y, position.z),
            Vec3::new(impulse.x, impulse.y, impulse.z),
            Some(ball_transaction.ball_dto.uuid)
        );
    }
}
