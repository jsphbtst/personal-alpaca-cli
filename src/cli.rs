use clap::{command, Arg, Command};

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
            .required(true)
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
    .subcommand(Command::new("positions"))
    .get_matches()
}