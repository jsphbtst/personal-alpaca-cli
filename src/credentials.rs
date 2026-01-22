use serde::{Deserialize, Serialize};
use std::fs;

use crate::error::{AppError, AppResult};

#[derive(Serialize, Deserialize, Debug)]
pub struct Credentials {
  #[serde(rename = "apcaApiKey")]
  pub apca_api_key: String,
  #[serde(rename = "apcaSecretKey")]
  pub apca_secret_key: String,
}

fn get_credentials_path() -> AppResult<std::path::PathBuf> {
  let home = dirs::home_dir()
    .ok_or_else(|| AppError::Config("Could not determine home directory".into()))?;

  Ok(home.join(".config").join("stock-trader").join("credentials.json"))
}

fn read_credentials() -> AppResult<Credentials> {
  let cred_path = get_credentials_path()?;
  let contents = fs::read_to_string(cred_path)?;
  let credentials: Credentials = serde_json::from_str(&contents)?;

  Ok(credentials)
}

pub fn write_credentials(credentials: &Credentials) -> AppResult<()> {
  let cred_path = get_credentials_path()?;
  let json = serde_json::to_string_pretty(credentials)?;
  fs::write(cred_path, json)?;

  Ok(())
}

pub fn get_credentials() -> AppResult<Credentials> {
  read_credentials()
}
