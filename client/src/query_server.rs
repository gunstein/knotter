use bevy::{prelude::*, utils::Uuid};
use bevy_mod_reqwest::bevy_eventlistener::callbacks::ListenerInput;
use bevy_mod_reqwest::{*, reqwest::Url};
use shared::domain::dtos::ball_dto::BallDto;
use shared::domain::dtos::ball_transaction_dto::BallTransactionDto;
use url::ParseError;
use crate::ball::components::{MovingBall, StaticBall};
use crate::globe::GlobeName;

pub struct QueryServerPlugin;

impl Plugin for QueryServerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(ReqwestPlugin::default()) 
        .add_event::<SendInsertBallEvent>()
        .add_event::<SendDeleteBallEvent>()
        .add_event::<SendCreateNewGlobeEvent>()
        .add_event::<SendTransactionsRequestEvent>()
        .add_event::<ReceivedTransactionsEvent>()
        .add_event::<ReceivedGetNewGlobeIdResponseEvent>()
        .add_systems(Update, send_transactions_requests)
        .add_systems(Update, (insert_ball_event_listener, delete_ball_event_listener))
        .add_systems(Update, create_new_globe_event_listener)
        .add_systems(Update, handle_received_new_globe_id_response_events)
        .add_systems(Update, send_transactions_request)
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

#[derive(Event)]
pub struct SendDeleteBallEvent {
    pub uuid: Uuid,
}

#[derive(Resource)]
struct ReqTimer(pub Timer);

#[derive(Resource)]
pub struct LastReceivedTransaction(pub String);


#[derive(Event)]
pub struct SendTransactionsRequestEvent;

#[derive(Event)]
pub struct ReceiveNewGlobeCreatedEvent {
    pub globe_name: String,
}

#[derive(serde::Deserialize, Debug, Event)]
pub struct ReceivedTransactionsEvent {
    pub ball_transactions: Vec<BallTransactionDto>
}

impl From<ListenerInput<ReqResponse>> for ReceivedTransactionsEvent {
    fn from(value: ListenerInput<ReqResponse>) -> Self {
        let s = value.deserialize_json().unwrap();
        s
    }
}

#[derive(serde::Deserialize, Debug, Event)]
pub struct ReceivedGetNewGlobeIdResponseEvent {
    pub new_globe_id: String,
}

impl From<ListenerInput<ReqResponse>> for ReceivedGetNewGlobeIdResponseEvent {
    fn from(value: ListenerInput<ReqResponse>) -> Self {
        let s = value.deserialize_json().unwrap();
        s
    }
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

fn insert_ball_event_listener(
    mut events: EventReader<SendInsertBallEvent>, 
    mut client: BevyReqwest,
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
                let req = client.post(url)
                .header("Content-Type", "application/json")
                .body(body).build().unwrap();
                client.send(
                    req,
                    On::run(|req: Listener<ReqResponse>| {
                        if let Ok(string) = req.as_str() {
                            bevy::log::info!("handle_insert_ball_responses: {string}");
                        }
                        else{
                            bevy::log::error!("handle_insert_ball_responses: Received !Ok instead of a string.");
                        }
                    }),
                );
            }
        }
    }
}


fn delete_ball_event_listener(
    mut events: EventReader<SendDeleteBallEvent>, 
    mut client: BevyReqwest,
    globe_name: Res<GlobeName>,
    api_url: Res<crate::ApiURL>,    
) {
    for event in events.read() {
        if let Some(globe_name) = &globe_name.0 {
            let url_string = build_url(api_url.0.as_str(), &globe_name).unwrap().to_string();
            if let Ok(url) = Url::parse(&format!("{}/{}", url_string, event.uuid)) {
                let req = client.delete(url).build().unwrap();
                client.send(
                    req,
                    On::run(|req: Listener<ReqResponse>| {
                        if let Ok(string) = req.as_str() {
                            bevy::log::info!("handle_delete_ball_responses: {string}");
                        }
                        else{
                            bevy::log::error!("handle_delete_ball_responses: Received !Ok instead of a string.");
                        }
                    }),
                );
            }
        }
    }
}

fn send_transactions_request(
    mut events: EventReader<SendTransactionsRequestEvent>,
    api_url: Res<crate::ApiURL>,
    mut client: BevyReqwest,
    globe_name_res: Res<GlobeName>,
    last_trans: Res<LastReceivedTransaction>,
    mut send_create_new_globe_event: EventWriter<crate::query_server::SendCreateNewGlobeEvent>,
) {
    for _event in events.read() {
        if let Some(globe_name) = &globe_name_res.0 {
            let url_string = build_url(api_url.0.as_str(), &globe_name)
                .unwrap()
                .to_string();
            let request_url = format!("{}/{}", url_string, last_trans.0);
            bevy::log::info!("Sending transaction request to URL: {request_url}");
            
            if let Ok(url) = Url::parse(&request_url) {
                let req = client.get(url).build().unwrap();
                client.send(
                    req,
                    On::send_event::<ReceivedTransactionsEvent>());
            } else {
                bevy::log::error!("Failed to parse URL: {request_url}");
            }
        }
        else {
            //If no globe_name is set, send event to create new globe
            send_create_new_globe_event.send(crate::query_server::SendCreateNewGlobeEvent);
        }
    }
}

fn send_transactions_requests(
    time: Res<Time>,
    mut timer: ResMut<ReqTimer>,
    mut send_transactions_request_event: EventWriter<SendTransactionsRequestEvent>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        send_transactions_request_event.send(SendTransactionsRequestEvent);
        //send_transactions_request(&mut commands, &api_url, &mut client, &globe_name, &last_trans.0, &mut send_create_new_globe_event, &mut last_received_transaction, &mut send_receive_ball_transactions_events);
    }
}

fn create_new_globe_event_listener(
    mut events: EventReader<SendCreateNewGlobeEvent>, 
    api_url: Res<crate::ApiURL>,
    mut client: BevyReqwest,
) {
    for _event in events.read() {
        bevy::log::info!("create_new_globe_event_listener");

        let url_string = build_url(api_url.0.as_str(), "new_globe_id").unwrap().to_string();
        bevy::log::info!("url_string: {}", url_string);
        if let Ok(url) = Url::parse(url_string.as_str()) {
            let req = client.get(url).build().unwrap();
            client.send(
                req,
                On::send_event::<ReceivedGetNewGlobeIdResponseEvent>());
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn handle_received_new_globe_id_response_events(
    mut events: EventReader<ReceivedGetNewGlobeIdResponseEvent>,
    query_moving_balls: Query<Entity, With<MovingBall>>,
    query_static_balls: Query<Entity, With<StaticBall>>,
    mut globe_name: ResMut<GlobeName>,
    mut last_received_transaction: ResMut<LastReceivedTransaction>,
    mut commands: Commands
) {
    for ev in events.read() {
        if !ev.new_globe_id.is_empty(){
            for entity_moving_ball in query_moving_balls.iter() {
                commands.entity(entity_moving_ball).despawn();
            }
            for entity_static_ball in query_static_balls.iter() {
                commands.entity(entity_static_ball).despawn();
            }
            globe_name.0 = Some(ev.new_globe_id.clone());
            last_received_transaction.0 = "0".to_string();
        }
        else{
            bevy::log::error!("handle_create_new_globe_responses: Received empty new globe_id.");
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn handle_received_new_globe_id_response_events(
    mut events: EventReader<ReceivedGetNewGlobeIdResponseEvent>,
    query_moving_balls: Query<Entity, With<MovingBall>>,
    query_static_balls: Query<Entity, With<StaticBall>>,
    mut globe_name: ResMut<GlobeName>,
    mut last_received_transaction: ResMut<LastReceivedTransaction>,
    mut commands: Commands
) {
    let mut new_globe_id = String::new();
    for ev in events.read() {
        if !ev.new_globe_id.is_empty(){
            new_globe_id = ev.new_globe_id.clone();
        }
        else{
            bevy::log::error!("handle_create_new_globe_responses: Received empty new globe_id.");
        }
    }
    if !new_globe_id.is_empty(){
        crate::navigate_to_globe(&new_globe_id);
    }
}
