use std::collections::HashMap;
use std::io::{self, Write};
use std::time::Duration;

use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::time::{sleep, timeout};
use tokio_tungstenite::{connect_async, tungstenite::Message};

use tokio::sync::mpsc;

use crate::error::{AppError, AppResult};
use crate::tui::PriceUpdate;

const STREAM_URL: &str = "wss://stream.data.alpaca.markets/v2/iex";
const MAX_RECONNECT_ATTEMPTS: u32 = 5;
const INITIAL_BACKOFF_MS: u64 = 1000;
const HEARTBEAT_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Serialize)]
struct AuthMessage<'a> {
    action: &'static str,
    key: &'a str,
    secret: &'a str,
}

#[derive(Serialize)]
struct SubscribeMessage {
    action: &'static str,
    trades: Vec<String>,
    quotes: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct StreamMessage {
    #[serde(rename = "T")]
    msg_type: String,
    msg: Option<String>,
    #[serde(rename = "S")]
    symbol: Option<String>,
    // #[serde(rename = "p")]
    // price: Option<f64>,
    // #[serde(rename = "s")]
    // size: Option<u64>,
    #[serde(rename = "bp")]
    bid_price: Option<f64>,
    #[serde(rename = "ap")]
    ask_price: Option<f64>,
}

#[derive(Clone)]
struct QuoteState {
    bid: f64,
    ask: f64,
}

struct DisplayState {
    quotes: HashMap<String, QuoteState>,
    symbols: Vec<String>,
    lines_printed: usize,
}

impl DisplayState {
    fn new(symbols: Vec<String>) -> Self {
        Self {
            quotes: HashMap::new(),
            symbols,
            lines_printed: 0,
        }
    }

    fn update_quote(&mut self, symbol: String, bid: f64, ask: f64) {
        self.quotes.insert(symbol, QuoteState { bid, ask });
        self.redraw();
    }

    fn redraw(&mut self) {
        // Move cursor up to overwrite previous output
        if self.lines_printed > 0 {
            print!("\x1B[{}A", self.lines_printed);
        }

        let mut lines = 0;
        for sym in &self.symbols {
            // Clear line and print
            print!("\x1B[K");  // Clear from cursor to end of line
            if let Some(q) = self.quotes.get(sym) {
                println!("[{sym}] ${:.2} bid / ${:.2} ask", q.bid, q.ask);
            } else {
                println!("[{sym}] waiting...");
            }
            lines += 1;
        }

        self.lines_printed = lines;
        io::stdout().flush().ok();
    }
}

enum StreamExit {
    Shutdown,
    Disconnected,
}

pub async fn stream_trades(api_key: &str, api_secret: &str, symbols: Vec<String>) -> AppResult<()> {
    let mut attempt = 0;

    loop {
        attempt += 1;

        if attempt > MAX_RECONNECT_ATTEMPTS {
            return Err(AppError::Api(format!(
                "Failed to connect after {MAX_RECONNECT_ATTEMPTS} attempts"
            )));
        }

        if attempt > 1 {
            let backoff = INITIAL_BACKOFF_MS * 2_u64.pow(attempt - 2);
            println!("Reconnecting in {backoff}ms (attempt {attempt}/{MAX_RECONNECT_ATTEMPTS})...");
            sleep(Duration::from_millis(backoff)).await;
        }

        println!("Connecting to...");

        let ws_stream = match connect_async(STREAM_URL).await {
            Ok((stream, _)) => stream,
            Err(e) => {
                eprintln!("Connection failed: {e}");
                continue;
            }
        };

        let (mut write, mut read) = ws_stream.split();

        match read.next().await {
            Some(Ok(msg)) => println!("Connected: {msg}"),
            Some(Err(e)) => {
                eprintln!("Read error: {e}");
                continue;
            }
            None => {
                eprintln!("Connection closed unexpectedly");
                continue;
            }
        }

        let auth = AuthMessage {
            action: "auth",
            key: api_key,
            secret: api_secret,
        };
        if let Err(e) = write.send(Message::Text(serde_json::to_string(&auth)?)).await {
            eprintln!("Auth send failed: {e}");
            continue;
        }

        match read.next().await {
            Some(Ok(msg)) => println!("Auth response: {msg}"),
            Some(Err(e)) => {
                eprintln!("Auth read error: {e}");
                continue;
            }
            None => {
                eprintln!("Connection closed during auth");
                continue;
            }
        }

        let subscribe = SubscribeMessage {
            action: "subscribe",
            trades: symbols.clone(),
            quotes: symbols.clone(),
        };
        if let Err(e) = write.send(Message::Text(serde_json::to_string(&subscribe)?)).await {
            eprintln!("Subscribe send failed: {e}");
            continue;
        }

        println!("Subscribed to: {:?}", symbols);
        println!("Streaming... (Ctrl+C to stop)\n");

        attempt = 0;

        let mut display = DisplayState::new(symbols.clone());

        let exit_reason = process_messages(&mut read, &mut display).await;

        match exit_reason {
            StreamExit::Shutdown => {
                println!("\n\nShutting down gracefully...");
                let _ = write.send(Message::Close(None)).await;
                return Ok(());
            }
            StreamExit::Disconnected => {
                eprintln!("\nConnection lost. Reconnecting...");
            }
        }
    }
}

async fn process_messages<S>(read: &mut S, display: &mut DisplayState) -> StreamExit
where
    S: StreamExt<Item = Result<Message, tokio_tungstenite::tungstenite::Error>> + Unpin,
{
    loop {
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {
                return StreamExit::Shutdown;
            }

            result = timeout(HEARTBEAT_TIMEOUT, read.next()) => {
                match result {
                    // Got message within timeout
                    Ok(Some(Ok(Message::Text(text)))) => {
                        handle_text_message(&text, display);
                    }
                    Ok(Some(Ok(Message::Ping(_)))) => {}
                    Ok(Some(Ok(Message::Close(_)))) => {
                        return StreamExit::Disconnected;
                    }
                    Ok(Some(Ok(_))) => {}
                    Ok(Some(Err(_))) | Ok(None) => {
                        return StreamExit::Disconnected;
                    }
                    // Timeout elapsed - no message received
                    Err(_) => {
                        eprintln!("\n[heartbeat timeout - no data for 30s]");
                        return StreamExit::Disconnected;
                    }
                }
            }
        }
    }
}

fn handle_text_message(text: &str, display: &mut DisplayState) {
    if let Ok(messages) = serde_json::from_str::<Vec<StreamMessage>>(text) {
        for m in messages {
            match m.msg_type.as_str() {
                "q" => {
                    if let (Some(sym), Some(bp), Some(ap)) =
                        (m.symbol, m.bid_price, m.ask_price)
                    {
                        display.update_quote(sym, bp, ap);
                    }
                }
                "t" => {
                    // Trades could be shown separately or update a "last trade" field
                }
                "error" => {
                    if let Some(msg) = &m.msg {
                        eprintln!("[error: {msg}]");
                    }
                }
                _ => {}
            }
        }
    }
}

/// Stream prices to a channel (for TUI mode)
pub async fn stream_to_channel(
    api_key: &str,
    api_secret: &str,
    symbols: Vec<String>,
    tx: mpsc::Sender<PriceUpdate>,
) -> AppResult<()> {
    let mut attempt = 0;

    loop {
        attempt += 1;

        if attempt > MAX_RECONNECT_ATTEMPTS {
            return Err(AppError::Api(format!(
                "Failed to connect after {MAX_RECONNECT_ATTEMPTS} attempts"
            )));
        }

        if attempt > 1 {
            let backoff = INITIAL_BACKOFF_MS * 2_u64.pow(attempt - 2);
            sleep(Duration::from_millis(backoff)).await;
        }

        let ws_stream = match connect_async(STREAM_URL).await {
            Ok((stream, _)) => stream,
            Err(_) => continue,
        };

        let (mut write, mut read) = ws_stream.split();

        if read.next().await.is_none() {
            continue;
        }

        let auth = AuthMessage {
            action: "auth",
            key: api_key,
            secret: api_secret,
        };

        if write.send(Message::Text(serde_json::to_string(&auth)?)).await.is_err() {
            continue;
        }

        if read.next().await.is_none() {
            continue;
        }

        let subscribe = SubscribeMessage {
            action: "subscribe",
            trades: symbols.clone(),
            quotes: symbols.clone(),
        };

        if write.send(Message::Text(serde_json::to_string(&subscribe)?)).await.is_err() {
            continue;
        }

        attempt = 0;

        loop {
            let result = timeout(HEARTBEAT_TIMEOUT, read.next()).await;

            match result {
                Ok(Some(Ok(Message::Text(text)))) => {
                    if let Ok(messages) = serde_json::from_str::<Vec<StreamMessage>>(&text) {
                        for m in messages {
                            if m.msg_type == "q" {
                                if let (Some(sym), Some(bp), Some(ap)) =
                                    (m.symbol, m.bid_price, m.ask_price)
                                {
                                    let mid_price = (bp + ap) / 2.0;
                                    let _ = tx.send(PriceUpdate {
                                        symbol: sym,
                                        price: mid_price,
                                    }).await;
                                }
                            }
                        }
                    }
                }
                Ok(Some(Ok(Message::Close(_)))) | Ok(Some(Err(_))) | Ok(None) | Err(_) => {
                    break; // Reconnect
                }
                _ => {}
            }
        }
    }
}
