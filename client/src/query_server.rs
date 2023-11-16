use bevy::{prelude::*, utils::Uuid};
use bevy_mod_reqwest::{*, reqwest::Url};
use reqwest::Body;
use serde::{Serialize, Deserialize};

pub struct QueryServerPlugin;

impl Plugin for QueryServerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(ReqwestPlugin) 
        .add_event::<SendInsertBallEvent>()
        .add_event::<SendDeleteBallEvent>()
        //.add_plugins(LogPlugin::default())
        .add_systems(Update, (send_transactions_requests, handle_transactions_responses))
        .add_systems(Update, handle_insert_ball_responses)
        .add_systems(Update, (insert_ball_event_listener, delete_ball_event_listener))
        .insert_resource(ReqTimer(Timer::new(
            std::time::Duration::from_secs(2),//Check if server has new data every 2 seconds
            TimerMode::Repeating,
        )))
        .insert_resource(LastReceivedTransaction("0".to_string()))
        ;
    }
}


#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct BallDto {
    pub is_fixed: bool,
    pub is_insert: bool,
    pub uuid: Uuid,
    pub color: Option<String>, 
    pub position: Option<PositionDto>,
    pub impulse: Option<ImpulseDto>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PositionDto {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ImpulseDto {
    pub x: f32,
    pub y: f32,
    pub z: f32,
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

#[derive(Resource)]
struct ReqTimer(pub Timer);


#[derive(Component)]
pub struct TransactionsQuery;

#[derive(Resource)]
pub struct LastReceivedTransaction(pub String);

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
                bevy::log::info!("{string}");
            }
            None => {
                bevy::log::error!("Received None instead of a string.");
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
            commands.spawn(req);
        }
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
    results: Query<(Entity, &ReqwestBytesResult), With<TransactionsQuery>>
) {
    for (e, res) in results.iter() {
        match res.as_str() {
            Some(string) => {
                bevy::log::info!("{string}");
            }
            None => {
                bevy::log::error!("Received None instead of a string.");
            }
        }

        // Done with this entity
        commands.entity(e).despawn_recursive();
    }
}
