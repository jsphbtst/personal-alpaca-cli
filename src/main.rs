mod alpaca_api;
mod credentials;
mod cli;

fn main() {
  let result = cli::matches::capture();

  // Writing this nonverbosely for learning purposes - J
  let auth_args_opt = result.subcommand_matches("auth");
  if auth_args_opt.is_some() {
    let auth_args = auth_args_opt.unwrap();
    cli::cmd::handle_auth(auth_args);
    return;
  }

  let credentials = match credentials::get_credentials() {
    Ok(c) => c,
    Err(e) => {
      eprintln!("Failed to write credentials: {}", e);
      std::process::exit(1);
    }
  };

  let api_key = credentials.apca_api_key;
  let api_secret = credentials.apca_secret_key;

  if let Some(prices_args) = result.subcommand_matches("prices") {
    cli::cmd::handle_prices(prices_args, api_key, api_secret);
    return;
  }

  if let Some(positions_args) = result.subcommand_matches("positions") {
    cli::cmd::handle_positions(positions_args, api_key, api_secret);
    return;
  }

  if let Some(orders_args) = result.subcommand_matches("orders") {
    cli::cmd::handle_orders(orders_args, api_key, api_secret);
    return;
  }
}
