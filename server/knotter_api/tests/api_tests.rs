use reqwest;
use reqwest::StatusCode;
use std::process::Command;
use tokio;
use knotter_api::domain::dtos::insert_ball_response_dto::InsertBallResponseDto;
use knotter_api::domain::dtos::get_ball_transactions_by_globeid_response_dto::GetBallTransactionsByGlobeIdResponseDto;

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
    let globe_id = "a_globe_id".to_string();
    let json_data = serde_json::json!({
        "is_fixed": true,
        "is_insert": true,
        "uuid": "4d3cbd35-41e8-40be-96d2-ac0c4b9f4f26",
        "color": "#ff0000",
        "position": {
            "x": -1.05,
            "y": 0.0,
            "z": 0.0
        },
        "velocity": serde_json::Value::Null
    });
    
    println!("json_data: {:?}", json_data.to_string());

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

    let insert_response_data: InsertBallResponseDto = resp.json().await.expect("Failed to deserialize response");
    assert_eq!(insert_response_data.message, "Successfully inserted.".to_string());

    // Now, retrieve the data
    let query_resp = client.get(&format!("{}/{globe_id}/{transaction_id}", BASE_URL, globe_id = globe_id, transaction_id = "0"))
        .send()
        .await
        .expect("Failed to send GET request");

    assert_eq!(query_resp.status(), StatusCode::OK);

    // Parse the response and compare to original data
    let query_response_data: GetBallTransactionsByGlobeIdResponseDto = query_resp.json().await.expect("Failed to deserialize response");
 
    assert_eq!(query_response_data.ball_transactions.len(), 1);
}

#[tokio::test]
async fn test_delete_data() {
    // Start the service in a test mode (assuming you've set up the server to run in test mode with a command-line arg)
    let server_process = Command::new("cargo")
        .args(&["run", "--", "--test-mode"])
        .spawn()
        .expect("Failed to start the server");

    let server = TestServer { process: server_process };

    wait_for_server_ready(&format!("{}/health", BASE_URL), 10).await.expect("Server not ready");

    let client = reqwest::Client::new();

    // Create mock Transaction data
    let globe_id = "a_globe_id".to_string();
    let uuid = "4d3cbd35-41e8-40be-96d2-ac0c4b9f4f26".to_string();
    let json_data = serde_json::json!({
        "is_fixed": true,
        "is_insert": true,
        "uuid": uuid,
        "color": "#ff0000",
        "position": {
            "x": -1.05,
            "y": 0.0,
            "z": 0.0
        },
        "velocity": serde_json::Value::Null
    });
    
    println!("json_data: {:?}", json_data.to_string());

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

    let insert_response_data: InsertBallResponseDto = resp.json().await.expect("Failed to deserialize response");
    assert_eq!(insert_response_data.message, "Successfully inserted.".to_string());

    // retrieve the data
    let query_resp = client.get(&format!("{}/{globe_id}/{transaction_id}", BASE_URL, globe_id = globe_id, transaction_id = "0"))
        .send()
        .await
        .expect("Failed to send GET request");

    assert_eq!(query_resp.status(), StatusCode::OK);

    // Parse the response and compare to original data
    let query_response_data: GetBallTransactionsByGlobeIdResponseDto = query_resp.json().await.expect("Failed to deserialize response");
 
    assert_eq!(query_response_data.ball_transactions.len(), 1);

    //delete object
    let query_resp = client.delete(&format!("{}/{globe_id}/{uuid}", BASE_URL, globe_id = globe_id, uuid = uuid))
        .send()
        .await
        .expect("Failed to send DELETE request");

    assert_eq!(query_resp.status(), StatusCode::OK);

    // retrieve the data
    let query_resp = client.get(&format!("{}/{globe_id}/{transaction_id}", BASE_URL, globe_id = globe_id, transaction_id = "0"))
        .send()
        .await
        .expect("Failed to send GET request");

    assert_eq!(query_resp.status(), StatusCode::OK);

    // Parse the response and compare to original data
    let query_response_data: GetBallTransactionsByGlobeIdResponseDto = query_resp.json().await.expect("Failed to deserialize response");
 
    assert_eq!(query_response_data.ball_transactions.len(), 2);    
}