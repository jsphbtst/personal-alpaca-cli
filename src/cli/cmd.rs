use clap::ArgMatches;

use crate::alpaca_api::AlpacaClient;
use crate::credentials::{write_credentials, Credentials};
use crate::error::{AppError, AppResult};

pub fn handle_auth(auth_args: &ArgMatches) -> AppResult<()> {
  if let Some(set_args) = auth_args.subcommand_matches("set") {
    let credentials = Credentials {
      apca_api_key: set_args
        .get_one::<String>("api-key")
        .cloned()
        .unwrap_or_default(),
      apca_secret_key: set_args
        .get_one::<String>("secret-key")
        .cloned()
        .unwrap_or_default(),
    };
    write_credentials(&credentials)?;
    return Ok(());
  }

  if auth_args.subcommand_matches("reset").is_some() {
    let empty_credentials = Credentials {
      apca_api_key: String::new(),
      apca_secret_key: String::new(),
    };
    write_credentials(&empty_credentials)?;
    return Ok(());
  }

  Ok(())
}

pub async fn handle_prices(prices_args: &ArgMatches, api_key: &str, api_secret: &str) -> AppResult<()> {
  let client = AlpacaClient::new(api_key.to_string(), api_secret.to_string());

  if let Some(symbols) = prices_args.get_many::<String>("symbols") {
    let symbols: Vec<String> = symbols.map(|s| s.to_uppercase()).collect();

    println!("Fetching {} symbols concurrently...\n", symbols.len());

    let futures: Vec<_> = symbols
      .iter()
      .map(|symbol| client.fetch_asset(symbol))
      .collect();

    let results = futures::future::join_all(futures).await;

    for (symbol, result) in symbols.iter().zip(results) {
      match result {
        Ok(asset) => {
          println!("{}:", symbol);
          println!("  Name: {}", asset.name);
          println!("  Exchange: {}", asset.exchange);
          println!("  Tradable: {}", asset.tradable);
          println!();
        }
        Err(e) => {
          println!("{}: Error - {}\n", symbol, e);
        }
      }
    }

    return Ok(());
  }

  if let Some(symbol) = prices_args.get_one::<String>("symbol") {
    let asset = client.fetch_asset(&symbol.to_uppercase()).await?;
    println!("{}", serde_json::to_string_pretty(&asset)?);
    return Ok(());
  }

  Err(AppError::MissingArgument("symbol or symbols".into()))
}

pub async fn handle_positions(positions_args: &ArgMatches, api_key: &str, api_secret: &str) -> AppResult<()> {
  let client = AlpacaClient::new(api_key.to_string(), api_secret.to_string());

  if let Some(symbols) = positions_args.get_many::<String>("symbols") {
    let symbols: Vec<String> = symbols.map(|s| s.to_uppercase()).collect();

    println!("Fetching {} symbols concurrently...\n", symbols.len());

    let futures: Vec<_> = symbols
      .iter()
      .map(|symbol| client.fetch_positions_by_symbol(symbol.to_string()))
      .collect();

    let results = futures::future::join_all(futures).await;

    for (symbol, result) in symbols.iter().zip(results) {
      match result {
        Ok(position) => {
          println!("Symbol: {}", position.symbol);
          println!("Current price: {}", position.current_price);
          println!("Qty: {}", position.qty);
        }
        Err(e) => {
          println!("{}: Error - {}\n", symbol, e)
        }
      }
    }
    return Ok(());
  }

  match positions_args.get_one::<String>("symbol") {
    Some(s) => {
      let position = client.fetch_positions_by_symbol(s.to_uppercase()).await?;
      println!("{}", serde_json::to_string_pretty(&position)?);
    }
    None => {
      let positions = client.fetch_positions().await?;
      println!("{}", serde_json::to_string_pretty(&positions)?);
    }
  };

  Ok(())
}

pub async fn handle_orders(orders_args: &ArgMatches, api_key: &str, api_secret: &str) -> AppResult<()> {
  let client = AlpacaClient::new(api_key.to_string(), api_secret.to_string());

  if let Some(list_args) = orders_args.subcommand_matches("list") {
    let status = list_args
      .get_one::<String>("status")
      .ok_or_else(|| AppError::MissingArgument("status".into()))?;

    let json = client.fetch_orders(status.to_lowercase()).await?;
    println!("{}", serde_json::to_string_pretty(&json)?);

    return Ok(());
  }

  if let Some(execute_args) = orders_args.subcommand_matches("execute") {
    let side = execute_args
      .get_one::<String>("side")
      .ok_or_else(|| AppError::MissingArgument("side".into()))?;

    let symbol = execute_args
      .get_one::<String>("symbol")
      .ok_or_else(|| AppError::MissingArgument("symbol".into()))?;

    let notional = execute_args
      .get_one::<f64>("notional")
      .copied()
      .unwrap_or(5.0);

    let json = client.create_order(side.to_lowercase(), symbol.to_uppercase(), notional).await?;
    println!("{}", serde_json::to_string_pretty(&json)?);

    return Ok(());
  }

  if let Some(cancel_args) = orders_args.subcommand_matches("cancel") {
    let order_id = cancel_args
      .get_one::<String>("order_id")
      .ok_or_else(|| AppError::MissingArgument("order_id".into()))?;

    let json = client.cancel_order(order_id.to_string()).await?;
    println!("{}", serde_json::to_string_pretty(&json)?);

    return Ok(());
  }

  if let Some(pick_args) = orders_args.subcommand_matches("randombuy") {
    let positions = client.fetch_positions().await?;

    let existing: std::collections::HashSet<String> = positions
      .iter()
      .map(|p| p.symbol.clone())
      .collect();

    let candidates: Vec<String> = client
      .base_stocks
      .iter()
      .filter(|&&s| !existing.contains(s))
      .map(|&s| s.to_string())
      .collect();

    println!("Picking from {} candidates...", candidates.len());

    let symbol = crate::cli::utils::select_random_stock(
      candidates,
      crate::cli::utils::generate_random_number,
    )
    .ok_or_else(|| AppError::Config("No stocks available to buy".into()))?;

    let notional = pick_args
      .get_one::<f64>("notional")
      .copied()
      .unwrap_or(5.0);

    println!("Picked {}. Executing order...", symbol);

    let order = client.create_order("buy".to_string(), symbol, notional).await?;
    println!("{}", serde_json::to_string_pretty(&order)?);

    return Ok(());
  }

  Ok(())
}
