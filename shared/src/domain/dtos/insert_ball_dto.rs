use crate::domain::dtos::ball_dto::BallDto;
use log::debug;
use env_logger;

pub type InsertBallDto = BallDto;

#[test]
fn test_deserialization_insertballdto() {
    // Initialize the logger
    let _ = env_logger::builder().is_test(true).try_init();

    let payload = "{\"is_fixed\":true,\"is_insert\":true,\"uuid\":\"a55018a3-a5fd-408b-876b-7bec638cdda1\",\"color\":\"#ff0000\",\"position\":{\"x\":-0.06181187,\"y\":-0.061811965,\"z\":1.0463562},\"impulse\":null}".to_string();
    debug!("payload: {}", payload);
    let deserialized: Result<InsertBallDto, _> = serde_json::from_str(&payload);  
    debug!("deserialized: {:?}", deserialized);
    println!("deserialized: {:?}", deserialized);
    assert!(deserialized.is_ok());
}