mod alpaca_api;
mod credentials;
mod cli;

use cli::{get_cli_matches, handle_prices_cmd, handle_auth_cmd, handle_positions_cmd};
use credentials::get_credentials;

fn main() {
  let match_result = get_cli_matches();

  // Writing this nonverbosely for learning purposes - J
  let auth_args_opt = match_result.subcommand_matches("auth");
  if auth_args_opt.is_some() {
    let auth_args = auth_args_opt.unwrap();
    handle_auth_cmd(auth_args);
    return;
  }

  let credentials = match get_credentials() {
    Ok(c) => c,
    Err(e) => {
      eprintln!("Failed to write credentials: {}", e);
      std::process::exit(1);
    }
  };

  if let Some(prices_args) = match_result.subcommand_matches("prices") {
    handle_prices_cmd(prices_args, credentials.apca_api_key, credentials.apca_secret_key);
    return;
  }

  if let Some(positions_args) = match_result.subcommand_matches("positions") {
    handle_positions_cmd(positions_args, credentials.apca_api_key, credentials.apca_secret_key);
    return;
  }
}
