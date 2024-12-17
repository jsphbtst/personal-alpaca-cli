mod alpaca_api;
mod credentials;
mod cli;

use cli::{get_cli_matches, handle_prices_cmd, handle_auth_cmd, handle_positions_cmd};
use credentials::get_credentials;

fn main() {
  let match_result = get_cli_matches();

  let credentials = match get_credentials() {
    Ok(c) => c,
    Err(e) => {
      eprintln!("Failed to write credentials: {}", e);
      std::process::exit(1);
    }
  };

  // Writing this nonverbosely for learning purposes - J
  let prices_args_opt = match_result.subcommand_matches("prices");
  if prices_args_opt.is_some() {
    handle_prices_cmd(prices_args_opt, credentials.apca_api_key, credentials.apca_secret_key);
    return;
  }

  if let Some(auth_args) = match_result.subcommand_matches("auth") {
    handle_auth_cmd(auth_args);
    return;
  }

  if let Some(positions_args) = match_result.subcommand_matches("positions") {
    handle_positions_cmd(positions_args, credentials.apca_api_key, credentials.apca_secret_key);
    return;
  }
}
