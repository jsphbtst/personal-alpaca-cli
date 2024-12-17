use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Credentials {
  #[serde(rename = "apcaApiKey")]
  pub apca_api_key: String,
  #[serde(rename = "apcaSecretKey")]
  pub apca_secret_key: String,
}

fn read_credentials() -> Result<Credentials, Box<dyn std::error::Error>> {
  let home = dirs::home_dir().expect("Failed to get home directory");
  let cred_path = home.join(".config").join("stock-trader").join("credentials.json");

  let contents = fs::read_to_string(cred_path)?;
  let credentials: Credentials = serde_json::from_str(&contents)?;

  Ok(credentials)
}

pub fn write_credentials(credentials: &Credentials) -> Result<(), Box<dyn std::error::Error>> {
  let home = dirs::home_dir().expect("Failed to get home directory");
  let cred_path = home.join(".config").join("stock-trader").join("credentials.json");

  let json = serde_json::to_string_pretty(credentials)?;
  fs::write(cred_path, json)?;

  Ok(())
}

pub fn get_credentials() -> Result<Credentials, Box<dyn std::error::Error>> {
  let credentials_result = read_credentials();
  let credentials = credentials_result?;
  Ok(credentials)
}