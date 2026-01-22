use clap::{command, Arg, Command, value_parser};

pub fn capture() -> clap::ArgMatches {
  command!()
    .about("This is a CLI tool for Alpaca-related actions")
    .subcommand(
      Command::new("prices")
        .arg(
          Arg::new("symbol")
            .short('s')
            .long("symbol")
            .aliases(["ticker", "tcker"])
            .help("Single stock ticker symbol")
        )
        .arg(
          Arg::new("symbols")
            .long("symbols")
            .value_delimiter(',')  // Allows: --symbols AAPL,GOOGL,MSFT
            .num_args(1..)         // One or more values
            .help("Multiple stock symbols (comma-separated) — fetched concurrently")
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
        .arg(
          Arg::new("symbols")
            .long("symbols")
            .value_delimiter(',')  // Allows: --symbols AAPL,GOOGL,MSFT
            .num_args(1..)         // One or more values
            .help("Multiple stock symbols (comma-separated) — fetched concurrently")
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
        .subcommand(
          Command::new("randombuy")
            .arg(
              Arg::new("notional")
                .short('n')
                .long("notional")
                .required(true)
                .value_parser(value_parser!(f64))
                .aliases(["value", "dollars"])
                .help("Dollar amount of the stock order")
            )
        )
    )
    .get_matches()
}