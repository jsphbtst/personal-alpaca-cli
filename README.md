# Joseph's Personal Alpaca CLI Tool

A command-line interface tool for interacting with Alpaca Markets API, allowing you to manage orders, check prices, and handle authentication.

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

Get price information for a specific stock:

```bash
stock-trader prices --symbol AAPL
# Alternative: Use -s or --ticker
stock-trader prices -s AAPL
```

### Managing Positions

View positions for a specific stock:

```bash
stock-trader positions --symbol AAPL
# Alternative: Use -s or --ticker
stock-trader positions -s AAPL
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
# Alternative shorter version
stock-trader orders execute -s AAPL --side buy -n 1000

# Sell order
stock-trader orders execute --symbol AAPL --side sell --notional 1000
```

#### Cancel Orders

Cancel a specific order using its ID:

```bash
stock-trader orders cancel --order-id ORDER_UUID
```

## Command Reference

| Command | Description |
|---------|-------------|
| `prices` | Get price information for a stock |
| `positions` | View positions for a stock |
| `auth set` | Set API credentials |
| `auth reset` | Reset API credentials |
| `orders list` | List orders with optional status filter |
| `orders execute` | Execute buy/sell orders |
| `orders cancel` | Cancel a specific order |

## Options

### Global Options

- `-s, --symbol, --ticker`: Stock ticker symbol
- `--side`: Type of order (buy/sell)
- `-n, --notional, --value, --dollars`: Dollar amount for orders

### Order Status Options

- `--status`: Filter orders by status (open/closed/all)

## Authentication

The tool requires Alpaca API credentials to function. You can obtain these from your Alpaca dashboard:
1. API Key ID
2. Secret Key ID

Set these using the `auth set` command before using other features.

## Notes

- All monetary values (notional) should be provided as decimal numbers
- Order IDs must be in UUID v4 format
- The default order side is "buy" if not specified
- The default order list status is "all" if not specified
