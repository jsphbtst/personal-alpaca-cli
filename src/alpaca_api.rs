use reqwest::blocking::Client;
use serde_json::{json, from_value};

pub struct AlpacaClient {
  api_key: String,
  api_secret: String,
  base_url: String,
  client: Client,
  pub base_stocks: [&'static str; 500]
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Position {
  pub symbol: String
}

pub const SP500_STOCKS: [&str; 500] = [
  "MMM", "AOS", "ABT", "ABBV", "ACN", "ATVI", "ADM", "ADBE", "ADP", "AAP",
  "AES", "AFL", "A", "APD", "AKAM", "ALK", "ALB", "ARE", "ALGN", "ALLE",
  "LNT", "ALL", "GOOGL", "GOOG", "MO", "AMZN", "AMCR", "AMD", "AEE", "AAL",
  "AEP", "AXP", "AIG", "AMT", "AWK", "AMP", "ABC", "AME", "AMGN", "APH",
  "ADI", "ANSS", "AON", "APA", "AAPL", "AMAT", "APTV", "ACGL", "ANET", "AJG",
  "AIZ", "T", "ATO", "ADSK", "AZO", "AVB", "AVY", "AXON", "BKR", "BALL",
  "BAC", "BBWI", "BAX", "BDX", "WRB", "BRK.B", "BBY", "BIO", "TECH", "BIIB",
  "BLK", "BK", "BA", "BKNG", "BWA", "BXP", "BSX", "BMY", "AVGO", "BR",
  "BRO", "BF.B", "BG", "CHRW", "CDNS", "CZR", "CPT", "CPB", "COF", "CAH",
  "KMX", "CCL", "CARR", "CTLT", "CAT", "CBOE", "CBRE", "CDW", "CE", "CNC",
  "CNP", "CDAY", "CF", "CRL", "SCHW", "CHTR", "CVX", "CMG", "CB", "CHD",
  "CI", "CINF", "CTAS", "CSCO", "C", "CFG", "CLX", "CME", "CMS", "KO",
  "CTSH", "CL", "CMCSA", "CMA", "CAG", "COP", "ED", "STZ", "CEG", "COO",
  "CPRT", "GLW", "CTVA", "CSGP", "COST", "CTRA", "CCI", "CSX", "CMI", "CVS",
  "DHI", "DHR", "DRI", "DVA", "DE", "DAL", "XRAY", "DVN", "DXCM", "FANG",
  "DLR", "DFS", "DIS", "DG", "DLTR", "D", "DPZ", "DOV", "DOW", "DTE",
  "DUK", "DD", "EMN", "ETN", "EBAY", "ECL", "EIX", "EW", "EA", "ELV",
  "LLY", "EMR", "ENPH", "ETR", "EOG", "EPAM", "EQT", "EFX", "EQIX", "EQR",
  "ESS", "EL", "ETSY", "EG", "EVRG", "ES", "EXC", "EXPE", "EXPD", "EXR",
  "XOM", "FFIV", "FDS", "FICO", "FAST", "FRT", "FDX", "FITB", "FSLR", "FE",
  "FIS", "FI", "FLT", "FMC", "F", "FTNT", "FTV", "FOXA", "FOX", "BEN",
  "FCX", "GRMN", "IT", "GEHC", "GEN", "GNRC", "GD", "GE", "GIS", "GM",
  "GPC", "GILD", "GL", "GPN", "GS", "HAL", "HIG", "HAS", "HCA", "PEAK",
  "HSIC", "HSY", "HES", "HPE", "HLT", "HOLX", "HD", "HON", "HRL", "HST",
  "HWM", "HPQ", "HUM", "HBAN", "HII", "IBM", "IEX", "IDXX", "ITW", "ILMN",
  "INCY", "IR", "PODD", "INTC", "ICE", "IFF", "IP", "IPG", "INTU", "ISRG",
  "IVZ", "INVH", "IQV", "IRM", "JBHT", "JKHY", "J", "JNJ", "JCI", "JPM",
  "JNPR", "K", "KDP", "KEY", "KEYS", "KMB", "KIM", "KMI", "KLAC", "KHC",
  "KR", "LHX", "LH", "LRCX", "LW", "LVS", "LDOS", "LEN", "LNC", "LIN",
  "LYV", "LKQ", "LMT", "L", "LOW", "LYB", "MTB", "MRO", "MPC", "MKTX",
  "MAR", "MMC", "MLM", "MAS", "MA", "MTCH", "MKC", "MCD", "MCK", "MDT",
  "MRK", "META", "MET", "MTD", "MGM", "MCHP", "MU", "MSFT", "MAA", "MRNA",
  "MHK", "MOH", "TAP", "MDLZ", "MPWR", "MNST", "MCO", "MS", "MOS", "MSI",
  "MSCI", "NDAQ", "NTAP", "NFLX", "NEM", "NWSA", "NWS", "NEE", "NKE", "NI",
  "NDSN", "NSC", "NTRS", "NOC", "NCLH", "NRG", "NUE", "NVDA", "NVR", "NXPI",
  "ORLY", "OXY", "ODFL", "OMC", "ON", "OKE", "ORCL", "OGN", "OTIS", "PCAR",
  "PKG", "PARA", "PH", "PAYX", "PAYC", "PYPL", "PNR", "PEP", "PKI", "PFE",
  "PCG", "PM", "PSX", "PNW", "PXD", "PNC", "POOL", "PPG", "PPL", "PFG",
  "PG", "PGR", "PLD", "PRU", "PEG", "PTC", "PSA", "PHM", "QRVO", "PWR",
  "QCOM", "DGX", "RL", "RJF", "RTX", "O", "REG", "REGN", "RF", "RSG",
  "RMD", "RHI", "ROK", "ROL", "ROP", "ROST", "RCL", "SPGI", "CRM", "SBAC",
  "SLB", "STX", "SEE", "SRE", "NOW", "SHW", "SPG", "SWKS", "SJM", "SNA",
  "SEDG", "SO", "LUV", "SWK", "SBUX", "STT", "STLD", "STE", "SYK", "SYF",
  "SNPS", "SYY", "TMUS", "TROW", "TTWO", "TPR", "TRGP", "TGT", "TEL", "TDY",
  "TFX", "TER", "TSLA", "TXN", "TXT", "TMO", "TJX", "TSCO", "TT", "TDG",
  "TRV", "TRMB", "TFC", "TYL", "TSN", "USB", "UDR", "ULTA", "UNP", "UAL",
  "UPS", "URI", "UNH", "UHS", "VLO", "VTR", "VRSN", "VRSK", "VZ", "VRTX",
  "VFC", "VTRS", "VICI", "V", "VMC", "WAB", "WBA", "WMT", "WBD", "WM",
  "WAT", "WEC", "WFC", "WELL", "WST", "WDC", "WRK", "WY", "WHR", "WMB",
  "WTW", "GWW", "WYNN", "XEL", "XYL", "YUM", "ZBRA", "ZBH", "ZION", "ZTS"
];

impl AlpacaClient {
  pub fn new(api_key: String, api_secret: String) -> Self {
    let url = "https://api.alpaca.markets";
    Self {
      api_key,
      api_secret,
      base_url: url.to_string(),
      client: Client::new(),
      base_stocks: SP500_STOCKS,
    }
  }

  fn get_request(&self, url: String) -> Result<serde_json::Value, reqwest::Error> {
    let response = self.client.get(url)
      .header("Accept", "application/json")
      .header("APCA-API-KEY-ID", &self.api_key)
      .header("APCA-API-SECRET-KEY", &self.api_secret)
      .send()?
      .json()?;

    Ok(response)
  }

  fn post_request(&self, url: String, body: serde_json::Value) -> Result<serde_json::Value, reqwest::Error> {
    let response = self.client.post(url)
      .header("Accept", "application/json")
      .header("APCA-API-KEY-ID", &self.api_key)
      .header("APCA-API-SECRET-KEY", &self.api_secret)
      .json(&body)
      .send()?
      .json()?;

    Ok(response)
  }

  fn delete_request(&self, url: String) -> Result<serde_json::Value, reqwest::Error> {
    let response = self.client.delete(url)
      .header("Accept", "application/json")
      .header("APCA-API-KEY-ID", &self.api_key)
      .header("APCA-API-SECRET-KEY", &self.api_secret)
      .send()?;

    if response.status() == reqwest::StatusCode::NO_CONTENT {
      return Ok(serde_json::json!({}));
    }

    let json = response.json()?;
    Ok(json)
  }

  pub fn fetch_asset(&self, symbol: &str) -> Result<serde_json::Value, reqwest::Error> {
    let url = format!("{}/v2/assets/{}", &self.base_url, symbol);
    self.get_request(url)
  }

  pub fn fetch_positions(&self) -> Result<serde_json::Value, reqwest::Error> {
    let url = format!("{}/v2/positions", &self.base_url);
    self.get_request(url)
  }

  pub fn fetch_positions_by_symbol(&self, symbol: String) -> Result<serde_json::Value, reqwest::Error> {
    let url = format!("{}/v2/positions/{}", &self.base_url, symbol);
    self.get_request(url)
  }

  pub fn fetch_orders(&self, status: String) -> Result<serde_json::Value, reqwest::Error> {
    let url = format!("{}/v2/orders?status={}", &self.base_url, status);
    self.get_request(url)
  }

  pub fn create_order(&self, side: String, symbol: String, notional: f64) -> Result<serde_json::Value, reqwest::Error> {
    let url = format!("{}/v2/orders", &self.base_url);
    let body = json!({
      "symbol": symbol,
      "notional": notional,
      "side": side,
      "type": "market",
      "time_in_force": "day"
    });

    self.post_request(url, body)
  }

  pub fn cancel_order(&self, order_id: String) -> Result<serde_json::Value, reqwest::Error> {
    let url = format!("{}/v2/orders/{}", &self.base_url, order_id);
    self.delete_request(url)
  }

  pub fn get_positions_tickers(&self) -> Result<Vec<Position>, Box<dyn std::error::Error>> {
    let result = self.fetch_positions()?; // Gets us the serde_json::Value - J
    let positions: Vec<Position> = from_value(result)?;
    Ok(positions)
  }
}