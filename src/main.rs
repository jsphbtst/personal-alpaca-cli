use serde_json;

mod cli;
use cli::get_cli_matches;

mod api;
use api::AlpacaClient;

mod credentials;
use credentials::read_credentials;


fn main() {
  let match_result = get_cli_matches();

  let credentials_result = read_credentials();
  if !credentials_result.is_ok() {
    eprintln!("Error fetching credentials: {:#?}", credentials_result.err());
    std::process::exit(1);
  }

  let credentials = credentials_result.unwrap();
  let api_key = credentials.apca_api_key.to_string();
  let api_secret = credentials.apca_secret_key.to_string();

  // Shorter way: if let Some(price_args) = match_result.subcommand_matches("prices") {
  let prices_args_opt = match_result.subcommand_matches("prices");
  if prices_args_opt.is_some() {
    let price_args = prices_args_opt.unwrap();
    let symbol = price_args.get_one::<String>("symbol")
      .unwrap_or(&"NONE".to_string())
      .to_uppercase();

    let client = AlpacaClient::new(api_key, api_secret);
    match client.fetch_asset(&symbol) {
      Ok(json) => println!("{}", serde_json::to_string_pretty(&json).unwrap()),
      Err(e) => {
        eprintln!("Error fetching asset details: {}", e);
        std::process::exit(1);
      }
    }
  }
}
