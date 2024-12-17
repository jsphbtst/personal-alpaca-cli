use reqwest::blocking::Client;

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

  fn base_request(&self, url: String) -> Result<serde_json::Value, reqwest::Error> {
    let response = self.client.get(url)
      .header("Accept", "application/json")
      .header("APCA-API-KEY-ID", &self.api_key)
      .header("APCA-API-SECRET-KEY", &self.api_secret)
      .send()?
      .json()?;

    Ok(response)
  }

  pub fn fetch_asset(&self, symbol: &str) -> Result<serde_json::Value, reqwest::Error> {
    let url = format!("{}/v2/assets/{}", &self.base_url, symbol);
    self.base_request(url)
  }

  pub fn fetch_positions(&self) -> Result<serde_json::Value, reqwest::Error> {
    let url = format!("{}/v2/positions", &self.base_url);
    self.base_request(url)
  }

  pub fn fetch_positions_by_symbol(&self, symbol: String) -> Result<serde_json::Value, reqwest::Error> {
    let url = format!("{}/v2/positions/{}", &self.base_url, symbol);
    self.base_request(url)
  }
}