use std::collections::{HashMap, VecDeque};
use std::io;
use std::time::Duration;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    symbols::Marker,
    text::{Line, Span},
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, Paragraph},
    Frame, Terminal,
};
use tokio::sync::mpsc;

use crate::error::AppResult;

const MAX_POINTS: usize = 100;
const COLORS: [Color; 6] = [
    Color::Cyan,
    Color::Yellow,
    Color::Magenta,
    Color::Green,
    Color::Red,
    Color::Blue,
];

#[derive(Debug, Clone)]
pub struct PriceUpdate {
    pub symbol: String,
    pub price: f64,
}

pub struct App {
    prices: HashMap<String, VecDeque<f64>>,
    symbols: Vec<String>,
    y_min: f64,
    y_max: f64,
    chart_data: Vec<Vec<(f64, f64)>>,
}

impl App {
    pub fn new(symbols: Vec<String>) -> Self {
        let mut prices = HashMap::new();
        let mut chart_data = Vec::new();
        for sym in &symbols {
            prices.insert(sym.clone(), VecDeque::with_capacity(MAX_POINTS));
            chart_data.push(Vec::new());
        }
        Self {
            prices,
            symbols,
            y_min: 0.0,
            y_max: 100.0,
            chart_data,
        }
    }

    pub fn update(&mut self, update: PriceUpdate) {
        if let Some(history) = self.prices.get_mut(&update.symbol) {
            history.push_back(update.price);
            if history.len() > MAX_POINTS {
                history.pop_front();
            }
            self.recalculate_bounds();
            self.rebuild_chart_data();
        }
    }

    fn recalculate_bounds(&mut self) {
        self.y_min = f64::MAX;
        self.y_max = f64::MIN;

        for history in self.prices.values() {
            for &price in history {
                if price < self.y_min {
                    self.y_min = price;
                }
                if price > self.y_max {
                    self.y_max = price;
                }
            }
        }

        if self.y_min == f64::MAX {
            self.y_min = 0.0;
            self.y_max = 100.0;
        } else {
            let padding = (self.y_max - self.y_min) * 0.05;
            self.y_min -= padding;
            self.y_max += padding;

            if (self.y_max - self.y_min).abs() < 0.01 {
                self.y_min -= 1.0;
                self.y_max += 1.0;
            }
        }
    }

    fn rebuild_chart_data(&mut self) {
        for (i, sym) in self.symbols.iter().enumerate() {
            if let Some(history) = self.prices.get(sym) {
                self.chart_data[i] = history
                    .iter()
                    .enumerate()
                    .map(|(x, &price)| (x as f64, price))
                    .collect();
            }
        }
    }
}

pub async fn run(symbols: Vec<String>, mut rx: mpsc::Receiver<PriceUpdate>) -> AppResult<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(symbols);

    loop {
        terminal.draw(|f| draw_ui(f, &app))?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        _ => {}
                    }
                }
            }
        }

        while let Ok(update) = rx.try_recv() {
            app.update(update);
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn draw_ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    draw_chart(f, app, chunks[0]);
    draw_legend(f, app, chunks[1]);
}

fn draw_chart(f: &mut Frame, app: &App, area: Rect) {
    let datasets: Vec<Dataset> = app
        .symbols
        .iter()
        .enumerate()
        .filter_map(|(i, sym)| {
            let data = &app.chart_data[i];
            if data.is_empty() {
                return None;
            }

            let color = COLORS[i % COLORS.len()];

            Some(
                Dataset::default()
                    .name(sym.clone())
                    .marker(Marker::Braille)
                    .graph_type(GraphType::Line)
                    .style(Style::default().fg(color))
                    .data(data),
            )
        })
        .collect();

    let y_labels = vec![
        Span::raw(format!("${:.2}", app.y_min)),
        Span::raw(format!("${:.2}", (app.y_min + app.y_max) / 2.0)),
        Span::raw(format!("${:.2}", app.y_max)),
    ];

    let chart = Chart::new(datasets)
        .block(
            Block::default()
                .title(" Live Prices ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White)),
        )
        .x_axis(
            Axis::default()
                .style(Style::default().fg(Color::DarkGray))
                .bounds([0.0, MAX_POINTS as f64]),
        )
        .y_axis(
            Axis::default()
                .style(Style::default().fg(Color::DarkGray))
                .labels(y_labels)
                .bounds([app.y_min, app.y_max]),
        );

    f.render_widget(chart, area);
}

fn draw_legend(f: &mut Frame, app: &App, area: Rect) {
    let legend_items: Vec<Span> = app
        .symbols
        .iter()
        .enumerate()
        .flat_map(|(i, sym)| {
            let color = COLORS[i % COLORS.len()];
            let price = app.prices.get(sym)
                .and_then(|h| h.back())
                .map(|p| format!("${:.2}", p))
                .unwrap_or_else(|| "---".to_string());

            vec![
                Span::styled(format!("■ {}: ", sym), Style::default().fg(color)),
                Span::raw(format!("{}  ", price)),
            ]
        })
        .collect();

    let legend = Paragraph::new(Line::from(legend_items))
        .block(Block::default().borders(Borders::ALL).title(" Legend "))
        .style(Style::default().fg(Color::White));

    f.render_widget(legend, area);
}
