mod alpaca_api;
mod cli;
mod credentials;
mod error;
mod tui;
mod websocket;

use error::AppResult;
use tokio::sync::mpsc;

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

  if let Some(chart_args) = matches.subcommand_matches("chart") {
    let symbols: Vec<String> = chart_args
      .get_many::<String>("symbols")
      .unwrap()
      .cloned()
      .collect();

    // Create channel for websocket -> TUI communication
    let (tx, rx) = mpsc::channel(100);

    // Spawn websocket task
    let ws_symbols = symbols.clone();
    let ws_key = api_key.clone();
    let ws_secret = api_secret.clone();
    tokio::spawn(async move {
      let _ = websocket::stream_to_channel(&ws_key, &ws_secret, ws_symbols, tx).await;
    });

    // Run TUI (blocks until user quits)
    return tui::run(symbols, rx).await;
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
