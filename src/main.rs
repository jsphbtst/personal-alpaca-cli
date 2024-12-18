mod alpaca_api;
mod credentials;
mod cli;

fn main() {
  let match_result = cli::get_cli_matches();

  // Writing this nonverbosely for learning purposes - J
  let auth_args_opt = match_result.subcommand_matches("auth");
  if auth_args_opt.is_some() {
    let auth_args = auth_args_opt.unwrap();
    cli::handle_auth_cmd(auth_args);
    return;
  }

  let credentials = match credentials::get_credentials() {
    Ok(c) => c,
    Err(e) => {
      eprintln!("Failed to write credentials: {}", e);
      std::process::exit(1);
    }
  };

  if let Some(prices_args) = match_result.subcommand_matches("prices") {
    cli::handle_prices_cmd(prices_args, credentials.apca_api_key, credentials.apca_secret_key);
    return;
  }

  if let Some(positions_args) = match_result.subcommand_matches("positions") {
    cli::handle_positions_cmd(positions_args, credentials.apca_api_key, credentials.apca_secret_key);
    return;
  }

  if let Some(orders_args) = match_result.subcommand_matches("orders") {
    cli::handle_orders_cmd(orders_args, credentials.apca_api_key, credentials.apca_secret_key);
    return;
  }
}
