mod alpaca_api;
mod credentials;
mod cli;

use cli::{get_cli_matches, handle_prices_cmd, handle_auth_cmd, handle_positions_cmd};
use credentials::read_credentials;

fn main() {
  let match_result = get_cli_matches();

  let credentials_result = read_credentials();
  if !credentials_result.is_ok() {
    eprintln!("Error fetching credentials: {:#?}", credentials_result.err());
    std::process::exit(1);
  }

  // TODO: implement retrieve api_keys method later - J
  let credentials = credentials_result.unwrap();
  let api_key = credentials.apca_api_key.to_string();
  let api_secret = credentials.apca_secret_key.to_string();

  // Writing this nonverbosely for learning purposes - J
  let prices_args_opt = match_result.subcommand_matches("prices");
  if prices_args_opt.is_some() {
    handle_prices_cmd(prices_args_opt, api_key, api_secret);
    return;
  }

  if let Some(auth_args) = match_result.subcommand_matches("auth") {
    handle_auth_cmd(auth_args);
    return;
  }

  if let Some(positions_args) = match_result.subcommand_matches("positions") {
    handle_positions_cmd(positions_args, api_key, api_secret);
    return;
  }
}
