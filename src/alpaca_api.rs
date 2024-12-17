use reqwest::blocking::Client;

pub struct AlpacaClient {
  api_key: String,
  api_secret: String,
  client: Client,
}

impl AlpacaClient {
  pub fn new(api_key: String, api_secret: String) -> Self {
    Self {
      api_key,
      api_secret,
      client: Client::new(),
    }
  }

  pub fn fetch_asset(&self, symbol: &str) -> Result<serde_json::Value, reqwest::Error> {
    let url = format!("https://api.alpaca.markets/v2/assets/{}", symbol);

    let response = self.client.get(url)
      .header("Accept", "application/json")
      .header("APCA-API-KEY-ID", &self.api_key)
      .header("APCA-API-SECRET-KEY", &self.api_secret)
      .send()?
      .json()?;

    Ok(response)
  }

  pub fn fetch_positions(&self) -> Result<serde_json::Value, reqwest::Error> {
    let url = "https://api.alpaca.markets/v2/positions";

    let response = self.client.get(url)
      .header("Accept", "application/json")
      .header("APCA-API-KEY-ID", &self.api_key)
      .header("APCA-API-SECRET-KEY", &self.api_secret)
      .send()?
      .json()?;

    Ok(response)
  }

  pub fn fetch_positions_by_symbol(&self, symbol: String) -> Result<serde_json::Value, reqwest::Error> {
    let url = format!("https://api.alpaca.markets/v2/positions/{}", symbol);

    let response = self.client.get(url)
      .header("Accept", "application/json")
      .header("APCA-API-KEY-ID", &self.api_key)
      .header("APCA-API-SECRET-KEY", &self.api_secret)
      .send()?
      .json()?;

    Ok(response)
  }
}