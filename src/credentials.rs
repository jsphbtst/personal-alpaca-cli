use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Credentials {
  #[serde(rename = "apcaApiKey")]
  pub apca_api_key: String,
  #[serde(rename = "apcaSecretKey")]
  pub apca_secret_key: String,
}

pub fn read_credentials() -> Result<Credentials, Box<dyn std::error::Error>> {
  let home = dirs::home_dir().expect("Failed to get home directory");
  let cred_path = home.join(".config").join("stock-trader").join("credentials.json");

  let contents = fs::read_to_string(cred_path)?;
  let credentials: Credentials = serde_json::from_str(&contents)?;

  Ok(credentials)
}