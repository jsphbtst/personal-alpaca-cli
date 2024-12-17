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
    .subcommand(Command::new("positions"))
    .get_matches()
}