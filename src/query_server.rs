use bevy::prelude::*;
use bevy_mod_reqwest::*;

pub struct QueryServerPlugin;

impl Plugin for QueryServerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(ReqwestPlugin)
        //.add_plugins(LogPlugin::default())
        .add_systems(Update, send_transactions_requests)
        .add_systems(Update, handle_transactions_responses)
        .add_systems(Update, send_test_requests)
        .add_systems(Update, handle_test_responses)
        .insert_resource(ReqTimer(Timer::new(
            std::time::Duration::from_secs(2),//Check if server has new data every 2 seconds
            TimerMode::Repeating,
        )))
        .insert_resource(TestTimer(Timer::new(
            std::time::Duration::from_secs(2),//Check if server has new data every 2 seconds
            TimerMode::Repeating,
        )));
    }
}

#[derive(Resource)]
struct ReqTimer(pub Timer);

#[derive(Resource)]
struct TestTimer(pub Timer);

#[derive(Component)]
pub struct TransactionsQuery;

#[derive(Component)]
pub struct TestQuery;

fn send_transactions_requests(mut commands: Commands, time: Res<Time>, mut timer: ResMut<ReqTimer>) {
    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        if let Ok(url) = "http://localhost:3000/api/resource".try_into() {
            let req = reqwest::Request::new(reqwest::Method::GET, url);
            let req = ReqwestRequest::new(req);
            commands.spawn(req).insert(TransactionsQuery);
        }
    }
}

fn send_test_requests(mut commands: Commands, time: Res<Time>, mut timer: ResMut<TestTimer>) {
    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        if let Ok(url) = "http://localhost:3000/api/test".try_into() {
            let req = reqwest::Request::new(reqwest::Method::GET, url);
            let req = ReqwestRequest::new(req);
            commands.spawn(req).insert(TestQuery);
        }
    }
}

fn handle_transactions_responses(mut commands: Commands, results: Query<(Entity, &ReqwestBytesResult), With<TransactionsQuery>>) {
    for (e, res) in results.iter() {
        let string = res.as_str().unwrap();
        bevy::log::info!("{string}");

        // Done with this entity
        commands.entity(e).despawn_recursive();
    }
}

fn handle_test_responses(mut commands: Commands, results: Query<(Entity, &ReqwestBytesResult), With<TestQuery>>) {
    for (e, res) in results.iter() {
        let string = res.as_str().unwrap();
        bevy::log::info!("{string}");

        // Done with this entity
        commands.entity(e).despawn_recursive();
    }
}