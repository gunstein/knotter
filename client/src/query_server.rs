use bevy::{prelude::*, utils::Uuid};
use bevy_mod_reqwest::{*, reqwest::Url};
use shared::domain::dtos::ball_dto::BallDto;
use shared::domain::dtos::get_ball_transactions_by_globeid_response_dto::GetBallTransactionsByGlobeIdResponseDto;
use url::ParseError;
use crate::ball::components::{MovingBall, StaticBall};
use crate::globe::GlobeName;
use shared::domain::dtos::get_new_globe_id_response_dto::GetNewGlobeIdResponse;

pub struct QueryServerPlugin;

impl Plugin for QueryServerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(ReqwestPlugin) 
        .add_event::<SendInsertBallEvent>()
        .add_event::<SendDeleteBallEvent>()
        .add_event::<ReceiveBallTransactionsEvent>()
        .add_event::<SendCreateNewGlobeEvent>()
        //.add_plugins(LogPlugin::default())
        .add_systems(Update, send_transactions_requests)
        .add_systems(Update, handle_transactions_responses)
        .add_systems(Update, handle_insert_ball_responses)
        .add_systems(Update, handle_delete_ball_responses)
        .add_systems(Update, (insert_ball_event_listener, delete_ball_event_listener))
        .add_systems(Update, (create_new_globe_event_listener, handle_get_new_globe_responses))
        .insert_resource(ReqTimer(Timer::new(
            std::time::Duration::from_secs(1),//Check if server has new data every second
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

#[derive(Event)]
pub struct SendCreateNewGlobeEvent;

#[derive(Component)]
pub struct GetNewGlobeIdQuery;

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

#[derive(Event)]
pub struct ReceiveNewGlobeCreatedEvent {
    pub globe_name: String,
}

fn build_url(base_url: &str, path: &str) -> Result<Url, ParseError> {
    bevy::log::info!("build_url base_url: {}", base_url);
    bevy::log::info!("build_url path: {}", path);

    let mut base = Url::parse(base_url)?;

    // Ensure that the base URL ends with a '/'
    let mut base_path = base.path().to_owned();
    if !base_path.ends_with('/') {
        base_path.push('/');
    }

    // Append the path
    base_path.push_str(path);
    base.set_path(&base_path);

    let full_url = base;

    //bevy::log::info!("Full URL: {}", full_url);

    Ok(full_url)
}

fn insert_ball_event_listener(mut commands: Commands, 
    mut events: EventReader<SendInsertBallEvent>, 
    reqwest: Res<ReqwestClient>,
    globe_name: Res<GlobeName>,
    api_url: Res<crate::ApiURL>,
) {
    for event in events.read() {
        //if let Ok(url) = Url::parse("http://127.0.0.1:8080/globe1") {
        //bevy::log::info!("Base URL: {}",api_url.0.as_str() );
        if let Some(the_globe_name) = &globe_name.0 {
            let url_string = build_url(api_url.0.as_str(), &the_globe_name).unwrap().to_string();
            bevy::log::info!("insert_ball_event_listener url_string: {url_string}");
            if let Ok(url) = Url::parse(url_string.as_str()) {
                let body = serde_json::to_string(&event.ball).unwrap();
                //bevy::log::info!("insert body: {body}");

                //let req = reqwest.0.post(url).json(&body).build().unwrap();
                let req = reqwest.0.post(url)
                .header("Content-Type", "application/json")
                .body(body).build().unwrap();

                let req = ReqwestRequest::new(req);
                commands.spawn(req).insert(InsertBallQuery);
            }
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
                //bevy::log::info!("handle_insert_ball_responses: {string}");
            }
            None => {
                bevy::log::error!("handle_insert_ball_responses: Received None instead of a string.");
            }
        }

        // Done with this entity
        commands.entity(e).despawn_recursive();
    }
}

fn delete_ball_event_listener(mut commands: Commands, mut events: EventReader<SendDeleteBallEvent>, 
    reqwest: Res<ReqwestClient>,
    globe_name: Res<GlobeName>,
    api_url: Res<crate::ApiURL>,    
) {
    for event in events.read() {
        if let Some(globe_name) = &globe_name.0 {
            let url_string = build_url(api_url.0.as_str(), &globe_name).unwrap().to_string();
            if let Ok(url) = Url::parse(&format!("{}/{}", url_string, event.uuid)) {
                let req = reqwest.0.delete(url).build().unwrap();
        
                let req = ReqwestRequest::new(req);
                commands.spawn(req).insert(DeleteBallQuery);
            }
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
                //bevy::log::info!("handle_delete_ball_responses: {string}");
            }
            None => {
                bevy::log::error!("handle_delete_ball_responses: Received None instead of a string.");
            }
        }

        // Done with this entity
        commands.entity(e).despawn_recursive();
    }
}

fn send_transactions_request(
    commands: &mut Commands,
    api_url: &Res<crate::ApiURL>,
    globe_name: &Res<GlobeName>,
    last_trans_id: &str,
    send_create_new_globe_event: &mut EventWriter<crate::query_server::SendCreateNewGlobeEvent>,
) {
    if let Some(globe_name) = &globe_name.0 {
        let url_string = build_url(api_url.0.as_str(), &globe_name)
            .unwrap()
            .to_string();
        let request_url = format!("{}/{}", url_string, last_trans_id);
        bevy::log::info!("Sending transaction request to URL: {request_url}");
        
        if let Ok(url) = Url::parse(&request_url) {
            let req = reqwest::Request::new(reqwest::Method::GET, url);
            let req = ReqwestRequest::new(req);
            commands.spawn(req).insert(TransactionsQuery);
        } else {
            bevy::log::error!("Failed to parse URL: {request_url}");
        }
    }
    else {
            //If no globe_name is set, send event to create new globe
            send_create_new_globe_event.send(crate::query_server::SendCreateNewGlobeEvent);
    }
}

fn send_transactions_requests(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<ReqTimer>,
    last_trans: Res<LastReceivedTransaction>,
    globe_name: Res<GlobeName>,
    api_url: Res<crate::ApiURL>,
    mut send_create_new_globe_event: EventWriter<crate::query_server::SendCreateNewGlobeEvent>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        send_transactions_request(&mut commands, &api_url, &globe_name, &last_trans.0, &mut send_create_new_globe_event);
    }
}

fn handle_transactions_responses(
    mut commands: Commands,
    results: Query<(Entity, &ReqwestBytesResult), With<TransactionsQuery>>,
    mut last_received_transaction: ResMut<LastReceivedTransaction>,
    mut send_receive_ball_transactions_events: EventWriter<ReceiveBallTransactionsEvent>,
    api_url: Res<crate::ApiURL>,
    globe_name: Res<crate::globe::GlobeName>,
    mut send_create_new_globe_event: EventWriter<crate::query_server::SendCreateNewGlobeEvent>,
) {
    for (e, res) in results.iter() {
        match res.as_str() {
            Some(string) => {
                match serde_json::from_str::<GetBallTransactionsByGlobeIdResponseDto>(&string) {
                    Ok(deserialized) => {
                        if !deserialized.ball_transactions.is_empty() {
                            if let Some(last_element) = deserialized.ball_transactions.last() {
                                last_received_transaction.0 = last_element.transaction_id.to_string();
                                send_receive_ball_transactions_events.send(ReceiveBallTransactionsEvent {
                                    ball_transactions: deserialized,
                                });

                                // Send a new request immediately
                                send_transactions_request(&mut commands, &api_url, &globe_name, &last_received_transaction.0, &mut send_create_new_globe_event);
                            }
                        }
                    }
                    Err(err) => bevy::log::error!("Failed to deserialize: {}", err),
                }
            }
            None => bevy::log::error!("Received None instead of a string."),
        }
        commands.entity(e).despawn_recursive();
    }
}

fn create_new_globe_event_listener(mut commands: Commands, 
    mut events: EventReader<SendCreateNewGlobeEvent>, 
    api_url: Res<crate::ApiURL>,
) {
    for _event in events.read() {
        bevy::log::info!("create_new_globe_event_listener");

        let url_string = build_url(api_url.0.as_str(), "new_globe_id").unwrap().to_string();
        bevy::log::info!("url_string: {}", url_string);
        if let Ok(url) = Url::parse(url_string.as_str()) {
            let req = reqwest::Request::new(reqwest::Method::GET, url);
            let req = ReqwestRequest::new(req);
            commands.spawn(req).insert(GetNewGlobeIdQuery);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn handle_get_new_globe_responses(
    mut commands: Commands, 
    results: Query<(Entity, &ReqwestBytesResult), With<GetNewGlobeIdQuery>>,
    query_moving_balls: Query<Entity, With<MovingBall>>,
    query_static_balls: Query<Entity, With<StaticBall>>,
    mut globe_name: ResMut<GlobeName>,
    mut last_received_transaction: ResMut<LastReceivedTransaction>,
) {
    for (e, res) in results.iter() {
        match res.as_str() {
            Some(string) => {
                bevy::log::info!("handle_get_new_globe_responses: {string}");
                match serde_json::from_str::<GetNewGlobeIdResponse>(&string) {
                    Ok(deserialized) => {
                        for entity_moving_ball in query_moving_balls.iter() {
                            commands.entity(entity_moving_ball).despawn();
                        }
                        for entity_static_ball in query_static_balls.iter() {
                            commands.entity(entity_static_ball).despawn();
                        }
                        globe_name.0 = Some(deserialized.new_globe_id);
                        last_received_transaction.0 = "0".to_string();
                    }
                    Err(err) => bevy::log::error!("Failed to deserialize: {}", err),
                }
            }
            None => {
                bevy::log::error!("handle_create_new_globe_responses: Received None instead of a string.");
            }
        }

        // Done with this entity
        commands.entity(e).despawn_recursive();
    }
}

#[cfg(target_arch = "wasm32")]
fn handle_get_new_globe_responses(
    mut commands: Commands, 
    results: Query<(Entity, &ReqwestBytesResult), With<GetNewGlobeIdQuery>>,
) {
    let mut new_globe_id = String::new();
    for (e, res) in results.iter() {
        match res.as_str() {
            Some(string) => {
                bevy::log::info!("handle_get_new_globe_responses: {string}");
                match serde_json::from_str::<GetNewGlobeIdResponse>(&string) {
                    Ok(deserialized) => {
                        new_globe_id = deserialized.new_globe_id;
                    }
                    Err(err) => bevy::log::error!("Failed to deserialize: {}", err),
                }
            }
            None => {
                bevy::log::error!("handle_create_new_globe_responses: Received None instead of a string.");
            }
        }

        // Done with this entity
        commands.entity(e).despawn_recursive();
    }
    if !new_globe_id.is_empty(){
        crate::navigate_to_globe(&new_globe_id);
    }
}
