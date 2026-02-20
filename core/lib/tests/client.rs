use std::time::Duration;

use line_bot_sdk_rust::client::LINE;

#[test]
fn line_client_new_creates_all_clients() {
    let line = LINE::new("test_token".to_string());

    // Verify all clients are accessible (they compile and are initialized)
    let _ = &line.channel_access_token_api_client;
    let _ = &line.insight_api_client;
    let _ = &line.liff_api_client;
    let _ = &line.manage_audience_api_client;
    let _ = &line.manage_audience_blob_api_client;
    let _ = &line.messaging_api_client;
    let _ = &line.messaging_api_blob_client;
    let _ = &line.module_api_client;
    let _ = &line.module_attach_api_client;
    let _ = &line.shop_api_client;
    let _ = &line.webhook_dummy_api_client;
}

#[test]
fn line_client_clone() {
    let line = LINE::new("test_token".to_string());
    let cloned = line.clone();
    // Cloned client should be independent
    let _ = &cloned.messaging_api_client;
}

#[test]
fn line_client_builder_default() {
    let line = LINE::builder("test_token".to_string()).build();
    let _ = &line.messaging_api_client;
}

#[test]
fn line_client_builder_with_timeout() {
    let line = LINE::builder("test_token".to_string())
        .timeout(Duration::from_secs(30))
        .build();
    let _ = &line.messaging_api_client;
}
