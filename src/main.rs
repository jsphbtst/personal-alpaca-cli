mod alpaca_api;
mod cli;
mod credentials;
mod error;
mod websocket;

use error::AppResult;

async fn run() -> AppResult<()> {
  let matches = cli::matches::capture();

  if let Some(auth_args) = matches.subcommand_matches("auth") {
    return cli::cmd::handle_auth(auth_args);
  }

  let credentials = credentials::get_credentials()?;
  let api_key = credentials.apca_api_key;
  let api_secret = credentials.apca_secret_key;

  if let Some(prices_args) = matches.subcommand_matches("prices") {
    return cli::cmd::handle_prices(prices_args, &api_key, &api_secret).await;
  }

  if let Some(positions_args) = matches.subcommand_matches("positions") {
    return cli::cmd::handle_positions(positions_args, &api_key, &api_secret).await;
  }

  if let Some(orders_args) = matches.subcommand_matches("orders") {
    return cli::cmd::handle_orders(orders_args, &api_key, &api_secret).await;
  }

  if let Some(stream_args) = matches.subcommand_matches("stream") {
    let symbols: Vec<String> = stream_args
      .get_many::<String>("symbols")
      .unwrap()
      .cloned()
      .collect();
    return websocket::stream_trades(&api_key, &api_secret, symbols).await;
  }

  Ok(())
}

#[tokio::main]
async fn main() {
  if let Err(e) = run().await {
    eprintln!("Error: {e}");
    std::process::exit(1);
  }
}
