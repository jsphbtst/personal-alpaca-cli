use serde_json;
use clap::ArgMatches;

use crate::alpaca_api::AlpacaClient;
use crate::credentials::{write_credentials, Credentials};

pub fn handle_prices(prices_args: &ArgMatches, api_key: String, api_secret: String) {
  let client = AlpacaClient::new(api_key, api_secret);

  let result = match prices_args.get_one::<String>("symbol") {
    Some(s) => client.fetch_asset(&s.to_uppercase()),
    None => {
      eprintln!("Symbol is required");
      std::process::exit(1);
    }
  };

  match result {
    Ok(json) => println!("{}", serde_json::to_string_pretty(&json).unwrap()),
    Err(e) => {
      eprintln!("Error fetching asset details: {}", e);
      std::process::exit(1);
    }
  }
}

pub fn handle_auth(auth_args: &ArgMatches) {
  let mut new_credentials = Credentials {
    apca_api_key: "".to_string(),
    apca_secret_key: "".to_string()
  };

  let auth_set_opts = auth_args.subcommand_matches("set");
  if auth_set_opts.is_some() {
    let auth_set_args = auth_set_opts.unwrap();

    let empty_string = "".to_string();
    let apca_api_key = auth_set_args.get_one::<String>("api-key")
      .unwrap_or(&empty_string)
      .to_string();
    let apca_secret_key = auth_set_args.get_one::<String>("secret-key")
      .unwrap_or(&empty_string)
      .to_string();

    new_credentials.apca_api_key = apca_api_key;
    new_credentials.apca_secret_key = apca_secret_key;

    if let Err(e) = write_credentials(&new_credentials) {
      eprintln!("Failed to write credentials: {}", e);
      std::process::exit(1);
    }
  }

  if let Some(_) = auth_args.subcommand_matches("reset") {
    if let Err(e) = write_credentials(&new_credentials) {
      eprintln!("Failed to write credentials: {}", e);
      std::process::exit(1);
    }
  }
}

pub fn handle_positions(positions_args: &ArgMatches, api_key: String, api_secret: String) {
  let client = AlpacaClient::new(api_key, api_secret);

  let result = match positions_args.get_one::<String>("symbol") {
    Some(s) => client.fetch_positions_by_symbol(s.to_uppercase()),
    None => client.fetch_positions(),
  };

  match result {
    Ok(json) => println!("{}", serde_json::to_string_pretty(&json).unwrap()),
    Err(e) => {
      eprintln!("Error fetching asset details: {}", e);
      std::process::exit(1);
    }
  }
}

pub fn handle_orders(orders_args: &ArgMatches, api_key: String, api_secret: String) {
  let client = AlpacaClient::new(api_key, api_secret);

  match orders_args.subcommand_matches("list") {
    Some(list_args) => {
      let result = match list_args.get_one::<String>("status") {
        Some(s) => client.fetch_orders(s.to_lowercase()),
        None => {
          eprintln!("Status is required");
          std::process::exit(1);
        }
      };

      match result {
        Ok(json) => println!("{}", serde_json::to_string_pretty(&json).unwrap()),
        Err(e) => {
          eprintln!("Error fetching asset details: {}", e);
          std::process::exit(1);
        }
      }
    },
    None => {}
  }

  match orders_args.subcommand_matches("execute") {
    Some(execute_args) => {
      let side = match execute_args.get_one::<String>("side") {
        Some(s) => s.to_lowercase(),
        None => {
          eprintln!("Error fetching asset details");
          std::process::exit(1);
        }
      };

      let symbol = match execute_args.get_one::<String>("symbol") {
        Some(s) => s.to_uppercase(),
        None => {
          eprintln!("Symbol is required");
          std::process::exit(1);
        }
      };

      let notional = match execute_args.get_one::<f64>("notional") {
        Some(s) => *s,
        None => 5.0,
      };

      match client.create_order(side, symbol, notional) {
        Ok(json) => println!("{}", serde_json::to_string_pretty(&json).unwrap()),
        Err(e) => {
          eprintln!("Error executing order: {}", e);
          std::process::exit(1);
        }
      }
    },
    None => {}
  }

  match orders_args.subcommand_matches("cancel") {
    Some(cancel_args) => {
      let order_id = match cancel_args.get_one::<String>("order_id") {
        Some(s) => s.to_string(),
        None => {
          eprintln!("Order ID is required");
          std::process::exit(1);
        }
      };

      match client.cancel_order(order_id) {
        Ok(json) => println!("{}", serde_json::to_string_pretty(&json).unwrap()),
        Err(e) => {
          eprintln!("Error canceling order: {}", e);
          std::process::exit(1);
        }
      }
    },
    None => {}
  }

  match orders_args.subcommand_matches("randombuy") {
    Some(pick_args) => {
      let result = match client.get_positions_tickers() {
        Ok(p) => p,
        Err(_e) => Vec::new()
      };

      let existing_tickers: std::collections::HashSet<String> = result
          .iter()
          .map(|position| position.symbol.clone())
          .collect();

      let mut new_stocks: Vec<String> = client.base_stocks
        .iter()
        .filter(|&&stock| !existing_tickers.contains(&String::from(stock)))
        .map(|&s| String::from(s))
        .collect();

      println!("Picking...");

      while new_stocks.len() > 2 {
        let random_num = crate::cli::utils::generate_random_number_api();
        let mid = new_stocks.len() / 2;
        if random_num % 2 == 0 {
          new_stocks = new_stocks[..mid].to_vec();
        } else {
          new_stocks = new_stocks[mid..].to_vec();
        }
      }

      let symbol: String;
      if new_stocks.len() == 2 {
        let final_num = crate::cli::utils::generate_random_number_api();
        if final_num % 2 == 0 {
          symbol = new_stocks[0].clone()
        } else {
          symbol = new_stocks[1].clone()
        }
      } else {
        symbol = new_stocks[0].clone()
      }

      let notional = match pick_args.get_one::<f64>("notional") {
        Some(s) => *s,
        None => 5.0,
      };

      println!("Picked {}. Executing order...", symbol);

      let side: String = "buy".to_string();
      match client.create_order(side, symbol, notional) {
        Ok(json) => println!("{}", serde_json::to_string_pretty(&json).unwrap()),
        Err(e) => {
          eprintln!("Error executing order: {}", e);
          std::process::exit(1);
        }
      }
    },
    None => {}
  }
}