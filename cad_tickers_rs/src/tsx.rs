use reqwest::Client;
use serde_json::json;
use crate::gql_data::GQL;
use rand::seq::SliceRandom;
use rand::thread_rng;
use ua_generator::UserAgent;

pub async fn get_ticker_filings(symbol: &str) -> Result<String, Box<dyn std::error::Error>> {
    let gql = GQL::new();
    let mut payload = gql.get_company_filings_payload.clone();
    payload["variables"]["symbol"] = json!(symbol);
    let response = send_request(payload).await?;
    Ok(response)
}

pub async fn get_news_and_events(symbol: &str) -> Result<String, Box<dyn std::error::Error>> {
    let gql = GQL::new();
    let mut payload = gql.get_company_news_events_payload.clone();
    payload["variables"]["symbol"] = json!(symbol);
    let response = send_request(payload).await?;
    Ok(response)
}

async fn send_request(payload: serde_json::Value) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let user_agent = UserAgent::random();
    let res = client.post(&format!("https://money.tmx.com/en/quote/{}", payload["variables"]["symbol"].as_str().unwrap().to_uppercase()))
        .header("User-Agent", user_agent.to_string())
        .json(&payload)
        .send()
        .await?;
    let body = res.text().await?;
    Ok(body)
}