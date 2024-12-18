use serde_json;
use clap::{command, Arg, ArgMatches, Command, value_parser};

use crate::alpaca_api::AlpacaClient;
use crate::credentials::{write_credentials, Credentials};

pub fn get_cli_matches() -> clap::ArgMatches {
  command!()
    .about("This is a CLI tool for Alpaca-related actions")
    .subcommand(
      Command::new("prices")
        .arg(
          Arg::new("symbol")
            .short('s')
            .long("symbol")
            .aliases(["ticker", "tcker"])
            .help("Stock ticker symbol")
        )
    )
    .subcommand(
      Command::new("positions")
        .arg(
          Arg::new("symbol")
            .short('s')
            .long("symbol")
            .aliases(["ticker", "tcker"])
            .help("Stock ticker symbol")
        )
    )
    .subcommand(
      Command::new("auth")
        .subcommand(
          Command::new("set")
            .arg(
              Arg::new("api-key")
                .long("api-key")
                .aliases(["apikey"])
                .required(true)
                .help("Your APCA API Key ID from Alpaca")
            )
            .arg(
              Arg::new("secret-key")
                .long("secret-key")
                .aliases(["secretkey"])
                .required(true)
                .help("Your APCA Secret Key ID from Alpaca")
            )
        )
        .subcommand(Command::new("reset"))
        // TODO: rm
    )
    .subcommand(
      Command::new("orders")
        .subcommand(
          Command::new("list")
            .arg(
              Arg::new("status")
                .long("status")
                .value_parser(["open", "closed", "all"])
                .default_value("all")
            )
        )
        .subcommand(
          Command::new("execute")
            .arg(
              Arg::new("side")
                .long("side")
                .value_parser(["buy", "sell"])
                .default_value("buy")
                .help("Type of order execution: buy or sell")
            )
            .arg(
              Arg::new("symbol")
                .short('s')
                .long("symbol")
                .aliases(["ticker", "tcker"])
                .required(true)
                .help("Stock ticker symbol")
            )
            .arg(
              Arg::new("notional")
                .short('n')
                .long("notional")
                .value_parser(value_parser!(f64))
                .aliases(["value", "dollars"])
                .help("Dollar amount of the stock order")
            )
        )
        .subcommand(
          Command::new("cancel")
            .arg(
              Arg::new("order_id")
                .long("order_id")
                .aliases(["orderid", "orderId", "order-id"])
                .required(true)
                .help("Order ID to be cancelled (uuid v4 format)")
            )
        )
    )
    .get_matches()
}

pub fn handle_prices_cmd(prices_args: &ArgMatches, api_key: String, api_secret: String) {
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

pub fn handle_auth_cmd(auth_args: &ArgMatches) {
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

pub fn handle_positions_cmd(positions_args: &ArgMatches, api_key: String, api_secret: String) {
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

pub fn handle_orders_cmd(orders_args: &ArgMatches, api_key: String, api_secret: String) {
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
}