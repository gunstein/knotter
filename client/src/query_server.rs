use bevy::{prelude::*, utils::Uuid};
use bevy_mod_reqwest::{*, reqwest::Url};
use reqwest::Body;
use serde::{Serialize, Deserialize};
use shared::domain::dtos::ball_dto::BallDto;
use shared::domain::dtos::get_ball_transactions_by_globeid_response_dto::GetBallTransactionsByGlobeIdResponseDto;
use shared::domain::dtos::position_dto::PositionDto;
use shared::domain::dtos::impulse_dto::ImpulseDto;

pub struct QueryServerPlugin;

impl Plugin for QueryServerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(ReqwestPlugin) 
        .add_event::<SendInsertBallEvent>()
        .add_event::<SendDeleteBallEvent>()
        .add_event::<ReceiveBallTransactionsEvent>()
        //.add_plugins(LogPlugin::default())
        .add_systems(Update, send_transactions_requests)
        .add_systems(Update, handle_transactions_responses)
        .add_systems(Update, handle_insert_ball_responses)
        .add_systems(Update, handle_delete_ball_responses)
        .add_systems(Update, (insert_ball_event_listener, delete_ball_event_listener))
        .insert_resource(ReqTimer(Timer::new(
            std::time::Duration::from_secs(2),//Check if server has new data every 2 seconds
            TimerMode::Repeating,
        )))
        .insert_resource(LastReceivedTransaction("0".to_string()))
        ;
    }
}

#[derive(Event)]
pub struct SendInsertBallEvent {
    pub ball:BallDto,
}

#[derive(Component)]
pub struct InsertBallQuery;

#[derive(Event)]
pub struct SendDeleteBallEvent {
    pub uuid: Uuid,
}

#[derive(Component)]
pub struct DeleteBallQuery;

#[derive(Resource)]
struct ReqTimer(pub Timer);

#[derive(Component)]
pub struct TransactionsQuery;

#[derive(Resource)]
pub struct LastReceivedTransaction(pub String);

#[derive(Event)]
pub struct ReceiveBallTransactionsEvent {
    pub ball_transactions: GetBallTransactionsByGlobeIdResponseDto,
}

fn insert_ball_event_listener(mut commands: Commands, mut events: EventReader<SendInsertBallEvent>, reqwest: Res<ReqwestClient>) {
    for event in events.iter() {
        if let Ok(url) = Url::parse("http://127.0.0.1:8080/globe1") {
            let body = serde_json::to_string(&event.ball).unwrap();
            bevy::log::info!("insert body: {body}");

            //let req = reqwest.0.post(url).json(&body).build().unwrap();
            let req = reqwest.0.post(url)
            .header("Content-Type", "application/json")
            .body(body).build().unwrap();

            let req = ReqwestRequest::new(req);
            commands.spawn(req).insert(InsertBallQuery);
        }
    }
}

fn handle_insert_ball_responses(
    mut commands: Commands, 
    results: Query<(Entity, &ReqwestBytesResult), With<InsertBallQuery>>
) {
    for (e, res) in results.iter() {
        match res.as_str() {
            Some(string) => {
                bevy::log::info!("handle_insert_ball_responses: {string}");
            }
            None => {
                bevy::log::error!("handle_insert_ball_responses: Received None instead of a string.");
            }
        }

        // Done with this entity
        commands.entity(e).despawn_recursive();
    }
}

fn delete_ball_event_listener(mut commands: Commands, mut events: EventReader<SendDeleteBallEvent>, reqwest: Res<ReqwestClient>) {
    for event in events.iter() {

        if let Ok(url) = Url::parse(&format!("http://127.0.0.1:8080/globe1/{}", event.uuid)) {
            let req = reqwest.0.delete(url).build().unwrap();
    
            let req = ReqwestRequest::new(req);
            commands.spawn(req).insert(DeleteBallQuery);
        }
    }
}

fn handle_delete_ball_responses(
    mut commands: Commands, 
    results: Query<(Entity, &ReqwestBytesResult), With<DeleteBallQuery>>
) {
    for (e, res) in results.iter() {
        match res.as_str() {
            Some(string) => {
                bevy::log::info!("handle_delete_ball_responses: {string}");
            }
            None => {
                bevy::log::error!("handle_delete_ball_responses: Received None instead of a string.");
            }
        }

        // Done with this entity
        commands.entity(e).despawn_recursive();
    }
}

fn send_transactions_requests(mut commands: Commands, time: Res<Time>, mut timer: ResMut<ReqTimer>, last_trans: Res<LastReceivedTransaction>) {
    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        if let Ok(url) = Url::parse(&format!("http://127.0.0.1:8080/globe1/{}", last_trans.0)) {
            bevy::log::info!("get transactions url: {url}");
            let req = reqwest::Request::new(reqwest::Method::GET, url);
            let req = ReqwestRequest::new(req);
            commands.spawn(req).insert(TransactionsQuery);
        }
    }
}

fn handle_transactions_responses(
    mut commands: Commands, 
    results: Query<(Entity, &ReqwestBytesResult), With<TransactionsQuery>>,
    mut last_received_transaction: ResMut<LastReceivedTransaction>,
    mut send_receive_ball_transactions_events: EventWriter<ReceiveBallTransactionsEvent>,
) {
    for (e, res) in results.iter() {
        match res.as_str() {
            Some(string) => {
                bevy::log::info!("handle_transactions_responses: {string}");

                // Deserialize to GetBallTransactionsByGlobeIdResponseDto
                match serde_json::from_str::<GetBallTransactionsByGlobeIdResponseDto>(&string) {
                    Ok(deserialized) => {
                        // Successfully deserialized, use `deserialized` here
                        bevy::log::info!("Deserialized response: {:?}", deserialized);
                        if deserialized.ball_transactions.len() > 0 {
                            // Get last transactions received from response and update resource LastReceivedTransaction
                            match deserialized.ball_transactions.last() {
                                Some(last_element) => {
                                    last_received_transaction.0 = last_element.transaction_id.to_string();
                                }
                                None => bevy::log::info!("handle_transactions_responses: The vector is empty"),
                            }

                            // Make event and send
                            send_receive_ball_transactions_events.send(ReceiveBallTransactionsEvent {
                                ball_transactions: deserialized,
                            });
                        }
                    }
                    Err(err) => {
                        bevy::log::error!("Failed to deserialize: {}", err);
                    }
                }
            }
            None => {
                bevy::log::error!("handle_transactions_responses: Received None instead of a string.");
            }
        }

        // Done with this entity
        commands.entity(e).despawn_recursive();
    }
}

