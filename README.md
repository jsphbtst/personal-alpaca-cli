# Joseph's Personal Alpaca CLI Tool

A command-line interface tool for interacting with Alpaca Markets API, allowing you to manage orders, check prices, and handle authentication.

Built with async Rust using `tokio` and `reqwest`.

## Installation

Just run `./install.sh` and you should be good to go.

## Usage

### Authentication

Before using the tool, you need to set up your Alpaca API credentials:

```bash
# Set API credentials
stock-trader auth set --api-key YOUR_API_KEY --secret-key YOUR_SECRET_KEY

# Reset credentials
stock-trader auth reset
```

### Checking Prices

Get price information for stocks:

```bash
# Single stock
stock-trader prices --symbol AAPL
stock-trader prices -s AAPL

# Multiple stocks (fetched concurrently)
stock-trader prices --symbols AAPL,GOOGL,MSFT,AMZN,TSLA
```

### Managing Positions

View your positions:

```bash
# View all positions
stock-trader positions

# View specific position
stock-trader positions --symbol AAPL
stock-trader positions -s AAPL

# View multiple positions (fetched concurrently)
stock-trader positions --symbols AAPL,GOOGL,MSFT
```

### Orders

#### List Orders

View your orders with optional status filtering:

```bash
# List all orders (default)
stock-trader orders list

# List only open orders
stock-trader orders list --status open

# List only closed orders
stock-trader orders list --status closed
```

#### Execute Orders

Place buy or sell orders:

```bash
# Buy order
stock-trader orders execute --symbol AAPL --side buy --notional 1000
stock-trader orders execute -s AAPL --side buy -n 1000

# Sell order
stock-trader orders execute --symbol AAPL --side sell --notional 1000
```

#### Cancel Orders

Cancel a specific order using its ID:

```bash
stock-trader orders cancel --order-id ORDER_UUID
```

#### Random Buy

Randomly pick and buy a stock from the S&P 500 that you don't already own:

```bash
stock-trader orders randombuy --notional 100
stock-trader orders randombuy -n 100
```

## Command Reference

| Command | Description |
|---------|-------------|
| `prices` | Get price information for stocks |
| `positions` | View positions |
| `auth set` | Set API credentials |
| `auth reset` | Reset API credentials |
| `orders list` | List orders with optional status filter |
| `orders execute` | Execute buy/sell orders |
| `orders cancel` | Cancel a specific order |
| `orders randombuy` | Randomly buy a stock from S&P 500 |

## Options

### Symbol Options

- `-s, --symbol`: Single stock ticker symbol
- `--symbols`: Multiple symbols, comma-separated (fetched concurrently)

### Order Options

- `--side`: Type of order (buy/sell)
- `-n, --notional`: Dollar amount for orders
- `--status`: Filter orders by status (open/closed/all)

## Authentication

The tool requires Alpaca API credentials. Get these from your Alpaca dashboard:
1. API Key ID
2. Secret Key ID

Set these using the `auth set` command before using other features.

## Notes

- All monetary values (notional) should be provided as decimal numbers
- Order IDs must be in UUID v4 format
- The default order side is "buy" if not specified
- The default order list status is "all" if not specified
- Multiple symbol queries are fetched concurrently for performance
