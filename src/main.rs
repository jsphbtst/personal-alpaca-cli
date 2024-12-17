mod alpaca_api;
mod credentials;
mod cli;

use cli::{get_cli_matches, handle_prices_cmd, handle_auth_cmd};
use credentials::read_credentials;

fn main() {
  let match_result = get_cli_matches();

  let credentials_result = read_credentials();
  if !credentials_result.is_ok() {
    eprintln!("Error fetching credentials: {:#?}", credentials_result.err());
    std::process::exit(1);
  }

  let credentials = credentials_result.unwrap();
  let api_key = credentials.apca_api_key.to_string();
  let api_secret = credentials.apca_secret_key.to_string();

  let prices_args_opt = match_result.subcommand_matches("prices");
  if prices_args_opt.is_some() {
    handle_prices_cmd(prices_args_opt, api_key, api_secret);
  }

  // Writing this verbosely for learning purposes - J
  if let Some(auth_args) = match_result.subcommand_matches("auth") {
    handle_auth_cmd(auth_args);
  }
}
