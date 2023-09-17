use bevy::prelude::*;
use bevy_mod_reqwest::*;

pub struct QueryServerPlugin;

impl Plugin for QueryServerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(ReqwestPlugin)
        //.add_plugins(LogPlugin::default())
        .add_systems(Update, send_requests)
        .add_systems(Update, handle_responses)
        .insert_resource(ReqTimer(Timer::new(
            std::time::Duration::from_secs(2),//Check if server has new data every 2 seconds
            TimerMode::Repeating,
        )));
    }
}

#[derive(Resource)]
struct ReqTimer(pub Timer);

fn send_requests(mut commands: Commands, time: Res<Time>, mut timer: ResMut<ReqTimer>) {
    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        if let Ok(url) = "http://localhost:3000/api/resource".try_into() {
            let req = reqwest::Request::new(reqwest::Method::GET, url);
            let req = ReqwestRequest::new(req);
            commands.spawn(req);
        }
    }
}

fn handle_responses(mut commands: Commands, results: Query<(Entity, &ReqwestBytesResult)>) {
    for (e, res) in results.iter() {
        let string = res.as_str().unwrap();
        bevy::log::info!("{string}");

        // Done with this entity
        commands.entity(e).despawn_recursive();
    }
}