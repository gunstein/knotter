//use actix_web::{web, test, App};
//use knotter_api::setup_database;

/* 
#[actix_rt::test]
async fn test_set_and_retrieve_data() {
    // Initialize the database
    let db = setup_database(true).expect("Failed to set up test database");

    // Setup the mock server
    let mut app = test::init_service(
        App::new()
            .service(web::resource("/set_data").route(web::post().to(set_data_endpoint)))
            // Include any other routes necessary for testing
    ).await;

    // Create a mock Transaction
    let json = r#"{ 
        "Insert": {
            "is_fixed": true,
            "is_insert": true,
            "object_uuid": "4d3cbd35-41e8-40be-96d2-ac0c4b9f4f26",
            "color": "blue",
            "position": {
                "x": -1.05,
                "y": 0.0,
                "z": 0.0
            },
            "velocity": null
        }
    }"#;

    let transaction = Transaction::Insert(TransactionData::new(json));
    let req = test::TestRequest::post()
        .uri("/set_data")
        .set_json(&transaction)
        .to_request();

    // Send the request and verify the response
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Here, retrieve the data using the appropriate endpoint and compare with the mock Transaction
    // For example:
    // let retrieved_data: TransactionData = test::read_body_json(resp).await;
    // assert_eq!(transaction, retrieved_data);
}
*/

use reqwest;
use reqwest::StatusCode;
use std::process::Command;
use tokio;

const BASE_URL: &str = "http://127.0.0.1:8080";

struct TestServer {
    process: std::process::Child,
}

impl Drop for TestServer {
    fn drop(&mut self) {
        let _ = self.process.kill(); // Attempt to kill the server. We ignore errors here.
    }
}

#[derive(Debug)]
enum WaitForServerError {
    ReqwestError(reqwest::Error),
    Timeout,
}

impl From<reqwest::Error> for WaitForServerError {
    fn from(error: reqwest::Error) -> Self {
        WaitForServerError::ReqwestError(error)
    }
}


async fn wait_for_server_ready(base_url: &str, retries: usize) -> Result<(), WaitForServerError> {
    let client = reqwest::Client::new();
    for _ in 0..retries {
        match client.get(base_url).send().await {
            Ok(response) if response.status().is_success() => return Ok(()),
            Ok(_) | Err(_) => tokio::time::sleep(std::time::Duration::from_millis(500)).await,
        }
    }
    Err(WaitForServerError::Timeout)
}

#[tokio::test]
async fn test_healthcheck() {
    // Start the service in a test mode
    let server_process = Command::new("cargo")
        .args(&["run", "--", "--test-mode"])
        .spawn()
        .expect("Failed to start the server");

    let server = TestServer { process: server_process };

    wait_for_server_ready(&format!("{}/health", BASE_URL), 10).await.expect("Server not ready");

    let client = reqwest::Client::new();
    let resp = client.get(&format!("{}/health", BASE_URL))
        .send()
        .await
        .expect("Failed to send request");

    assert!(resp.status().is_success());

    let health_response: serde_json::Value = resp.json().await.expect("Failed to deserialize response");
    assert_eq!(health_response["message"], "Everything is working fine");
}

#[tokio::test]
async fn test_set_and_retrieve_data() {
    // Start the service in a test mode (assuming you've set up the server to run in test mode with a command-line arg)
    let server_process = Command::new("cargo")
        .args(&["run", "--", "--test-mode"])
        .spawn()
        .expect("Failed to start the server");

    let server = TestServer { process: server_process };

    wait_for_server_ready(&format!("{}/health", BASE_URL), 10).await.expect("Server not ready");

    let client = reqwest::Client::new();

    // Create mock Transaction data
    let globe_id = "some_unique_globe_id".to_string();
    let json_data = r#"{ 
        "Insert": {
            "is_fixed": true,
            "is_insert": true,
            "object_uuid": "4d3cbd35-41e8-40be-96d2-ac0c4b9f4f26",
            "color": "blue",
            "position": {
                "x": -1.05,
                "y": 0.0,
                "z": 0.0
            },
            "velocity": null
        }
    }"#;

    // Send POST request to set data
    let resp = client.post(&format!("{}/{globe_id}", BASE_URL, globe_id = globe_id))
        .json(&json_data)
        .send()
        .await
        .expect("Failed to send POST request");

    if resp.status() != StatusCode::OK {
        // Extract error message from the response body
        let error_message: String = resp.text().await.expect("Failed to read response text");
        panic!("Received an error: {}", error_message);
    }

    // Now, retrieve the data
    let resp = client.get(&format!("{}/{globe_id}/{transaction_id}", BASE_URL, globe_id = globe_id, transaction_id = "0"))
        .send()
        .await
        .expect("Failed to send GET request");

    assert_eq!(resp.status(), StatusCode::OK);

    // Parse the response and compare to original data
    let bytes = resp.bytes().await.expect("Failed to read response bytes");
    let retrieved_data = String::from_utf8_lossy(&bytes).to_string();
    assert_eq!(retrieved_data, "test123".to_string() );

    // Optionally, terminate the server at the end of the test
    //server.kill().expect("Failed to kill the server");
}