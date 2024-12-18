use reqwest::blocking::Client;
use serde_json::json;

pub struct AlpacaClient {
  api_key: String,
  api_secret: String,
  base_url: String,
  client: Client,
}

impl AlpacaClient {
  pub fn new(api_key: String, api_secret: String) -> Self {
    let url = "https://api.alpaca.markets";
    Self {
      api_key,
      api_secret,
      base_url: url.to_string(),
      client: Client::new(),
    }
  }

  fn get_request(&self, url: String) -> Result<serde_json::Value, reqwest::Error> {
    let response = self.client.get(url)
      .header("Accept", "application/json")
      .header("APCA-API-KEY-ID", &self.api_key)
      .header("APCA-API-SECRET-KEY", &self.api_secret)
      .send()?
      .json()?;

    Ok(response)
  }

  fn post_request(&self, url: String, body: serde_json::Value) -> Result<serde_json::Value, reqwest::Error> {
    let response = self.client.post(url)
      .header("Accept", "application/json")
      .header("APCA-API-KEY-ID", &self.api_key)
      .header("APCA-API-SECRET-KEY", &self.api_secret)
      .json(&body)
      .send()?
      .json()?;

    Ok(response)
  }

  fn delete_request(&self, url: String) -> Result<serde_json::Value, reqwest::Error> {
    let response = self.client.delete(url)
      .header("Accept", "application/json")
      .header("APCA-API-KEY-ID", &self.api_key)
      .header("APCA-API-SECRET-KEY", &self.api_secret)
      .send()?;

    if response.status() == reqwest::StatusCode::NO_CONTENT {
      return Ok(serde_json::json!({}));
    }

    let json = response.json()?;
    Ok(json)
  }

  pub fn fetch_asset(&self, symbol: &str) -> Result<serde_json::Value, reqwest::Error> {
    let url = format!("{}/v2/assets/{}", &self.base_url, symbol);
    self.get_request(url)
  }

  pub fn fetch_positions(&self) -> Result<serde_json::Value, reqwest::Error> {
    let url = format!("{}/v2/positions", &self.base_url);
    self.get_request(url)
  }

  pub fn fetch_positions_by_symbol(&self, symbol: String) -> Result<serde_json::Value, reqwest::Error> {
    let url = format!("{}/v2/positions/{}", &self.base_url, symbol);
    self.get_request(url)
  }

  pub fn fetch_orders(&self, status: String) -> Result<serde_json::Value, reqwest::Error> {
    let url = format!("{}/v2/orders?status={}", &self.base_url, status);
    self.get_request(url)
  }

  pub fn create_order(&self, side: String, symbol: String, notional: f64) -> Result<serde_json::Value, reqwest::Error> {
    let url = format!("{}/v2/orders", &self.base_url);
    let body = json!({
      "symbol": symbol,
      "notional": notional,
      "side": side,
      "type": "market",
      "time_in_force": "day"
    });

    self.post_request(url, body)
  }

  pub fn cancel_order(&self, order_id: String) -> Result<serde_json::Value, reqwest::Error> {
    let url = format!("{}/v2/orders/{}", &self.base_url, order_id);
    self.delete_request(url)
  }
}