use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        BarChart, Block, BorderType, Borders, Cell, Gauge, Paragraph, Row, Scrollbar,
        ScrollbarOrientation, ScrollbarState, Table, TableState,
    },
    Frame, Terminal,
};
use sysinfo::{Pid, Process, ProcessStatus, System};
use std::{
    io,
    time::{Duration, Instant},
};

// ─── Color Palette ──────────────────────────────────────────────────────────
const CLR_BG: Color = Color::Rgb(10, 10, 20);
const CLR_ACCENT: Color = Color::Rgb(0, 200, 180);
const CLR_ACCENT2: Color = Color::Rgb(255, 80, 120);
const CLR_MUTED: Color = Color::Rgb(80, 90, 110);
const CLR_TEXT: Color = Color::Rgb(200, 210, 220);
const CLR_WARN: Color = Color::Rgb(255, 180, 0);
const CLR_CRIT: Color = Color::Rgb(255, 60, 80);
const CLR_OK: Color = Color::Rgb(50, 220, 140);
const CLR_SELECTED: Color = Color::Rgb(20, 30, 50);

// ─── App State ───────────────────────────────────────────────────────────────
#[derive(PartialEq, Clone, Copy)]
enum SortBy {
    Name,
    Ram,
    Cpu,
    Pid,
}

#[derive(PartialEq, Clone, Copy)]
enum Tab {
    Processes,
    Overview,
}

struct App {
    sys: System,
    processes: Vec<ProcessInfo>,
    table_state: TableState,
    scroll_state: ScrollbarState,
    sort_by: SortBy,
    sort_desc: bool,
    filter: String,
    filter_mode: bool,
    tab: Tab,
    last_update: Instant,
    cpu_history: Vec<f64>,
    ram_history: Vec<f64>,
}

#[derive(Clone)]
struct ProcessInfo {
    pid: u32,
    name: String,
    ram_mb: f64,
    cpu_pct: f32,
    status: String,
    status_color: Color,
}

impl App {
    fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        let mut app = Self {
            sys,
            processes: vec![],
            table_state: TableState::default(),
            scroll_state: ScrollbarState::default(),
            sort_by: SortBy::Ram,
            sort_desc: true,
            filter: String::new(),
            filter_mode: false,
            tab: Tab::Processes,
            last_update: Instant::now(),
            cpu_history: vec![0.0; 60],
            ram_history: vec![0.0; 60],
        };
        app.refresh();
        app.table_state.select(Some(0));
        app
    }

    fn refresh(&mut self) {
        self.sys.refresh_all();

        // Update history
        let cpu = self.sys.global_cpu_usage() as f64;
        let total_ram = self.sys.total_memory() as f64;
        let used_ram = self.sys.used_memory() as f64;
        let ram_pct = if total_ram > 0.0 { used_ram / total_ram * 100.0 } else { 0.0 };
        self.cpu_history.push(cpu);
        self.ram_history.push(ram_pct);
        if self.cpu_history.len() > 60 { self.cpu_history.remove(0); }
        if self.ram_history.len() > 60 { self.ram_history.remove(0); }

        // Collect processes
        let filter_lower = self.filter.to_lowercase();
        let mut procs: Vec<ProcessInfo> = self
            .sys
            .processes()
            .values()
            .filter(|p| {
                filter_lower.is_empty()
                    || p.name().to_string_lossy().to_lowercase().contains(&filter_lower)
            })
            .map(|p| {
                let (status, color) = status_info(p.status());
                ProcessInfo {
                    pid: p.pid().as_u32(),
                    name: p.name().to_string_lossy().to_string(),
                    ram_mb: p.memory() as f64 / 1024.0 / 1024.0,
                    cpu_pct: p.cpu_usage(),
                    status,
                    status_color: color,
                }
            })
            .collect();

        // Sort
        match self.sort_by {
            SortBy::Name => procs.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase())),
            SortBy::Ram  => procs.sort_by(|a, b| a.ram_mb.partial_cmp(&b.ram_mb).unwrap()),
            SortBy::Cpu  => procs.sort_by(|a, b| a.cpu_pct.partial_cmp(&b.cpu_pct).unwrap()),
            SortBy::Pid  => procs.sort_by(|a, b| a.pid.cmp(&b.pid)),
        }
        if self.sort_desc { procs.reverse(); }

        let len = procs.len();
        self.processes = procs;
        self.scroll_state = self.scroll_state.content_length(len);

        // Clamp selection
        if let Some(sel) = self.table_state.selected() {
            if sel >= len && len > 0 {
                self.table_state.select(Some(len - 1));
            }
        }

        self.last_update = Instant::now();
    }

    fn move_selection(&mut self, delta: i32) {
        let len = self.processes.len();
        if len == 0 { return; }
        let cur = self.table_state.selected().unwrap_or(0) as i32;
        let next = (cur + delta).clamp(0, len as i32 - 1) as usize;
        self.table_state.select(Some(next));
        self.scroll_state = self.scroll_state.position(next);
    }

    fn toggle_sort(&mut self, s: SortBy) {
        if self.sort_by == s {
            self.sort_desc = !self.sort_desc;
        } else {
            self.sort_by = s;
            self.sort_desc = true;
        }
        self.refresh();
    }

    fn total_ram_gb(&self) -> f64 { self.sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0 }
    fn used_ram_gb(&self)  -> f64 { self.sys.used_memory()  as f64 / 1024.0 / 1024.0 / 1024.0 }
    fn ram_pct(&self) -> f64 {
        let t = self.sys.total_memory() as f64;
        if t == 0.0 { 0.0 } else { self.sys.used_memory() as f64 / t * 100.0 }
    }
    fn cpu_pct(&self) -> f64 { self.sys.global_cpu_usage() as f64 }
}

fn status_info(status: ProcessStatus) -> (String, Color) {
    match status {
        ProcessStatus::Run    => ("● RUN".into(),   CLR_OK),
        ProcessStatus::Sleep  => ("◌ SLP".into(),   CLR_ACCENT),
        ProcessStatus::Idle   => ("○ IDL".into(),   CLR_MUTED),
        ProcessStatus::Stop   => ("■ STP".into(),   CLR_WARN),
        ProcessStatus::Zombie => ("✖ ZMB".into(),   CLR_CRIT),
        ProcessStatus::Dead   => ("✕ DED".into(),   CLR_CRIT),
        _                     => ("? UNK".into(),   CLR_MUTED),
    }
}

// ─── Main ────────────────────────────────────────────────────────────────────
fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let tick = Duration::from_millis(1500);

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick
            .checked_sub(app.last_update.elapsed())
            .unwrap_or_default();

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if app.filter_mode {
                    match key.code {
                        KeyCode::Esc | KeyCode::Enter => { app.filter_mode = false; app.refresh(); }
                        KeyCode::Backspace => { app.filter.pop(); app.refresh(); }
                        KeyCode::Char(c)   => { app.filter.push(c); app.refresh(); }
                        _ => {}
                    }
                } else {
                    match (key.code, key.modifiers) {
                        (KeyCode::Char('q'), _) | (KeyCode::Char('c'), KeyModifiers::CONTROL) => break,
                        (KeyCode::Down  | KeyCode::Char('j'), _) => app.move_selection(1),
                        (KeyCode::Up    | KeyCode::Char('k'), _) => app.move_selection(-1),
                        (KeyCode::PageDown, _) => app.move_selection(10),
                        (KeyCode::PageUp,   _) => app.move_selection(-10),
                        (KeyCode::Tab,      _) => {
                            app.tab = if app.tab == Tab::Processes { Tab::Overview } else { Tab::Processes };
                        }
                        (KeyCode::Char('1'), _) => app.toggle_sort(SortBy::Name),
                        (KeyCode::Char('2'), _) => app.toggle_sort(SortBy::Ram),
                        (KeyCode::Char('3'), _) => app.toggle_sort(SortBy::Cpu),
                        (KeyCode::Char('4'), _) => app.toggle_sort(SortBy::Pid),
                        (KeyCode::Char('/'), _) => { app.filter_mode = true; }
                        (KeyCode::Char('x'), _) => { app.filter.clear(); app.refresh(); }
                        (KeyCode::Char('r'), _) => app.refresh(),
                        _ => {}
                    }
                }
            }
        } else {
            app.refresh();
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}

// ─── UI ──────────────────────────────────────────────────────────────────────
fn ui(f: &mut Frame, app: &mut App) {
    let area = f.area();

    // Root background
    f.render_widget(
        Block::default().style(Style::default().bg(CLR_BG)),
        area,
    );

    let root = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // header
            Constraint::Length(5),  // system bars
            Constraint::Min(0),     // body
            Constraint::Length(2),  // footer
        ])
        .split(area);

    render_header(f, app, root[0]);
    render_system_bars(f, app, root[1]);

    match app.tab {
        Tab::Processes => render_process_table(f, app, root[2]),
        Tab::Overview  => render_overview(f, app, root[2]),
    }

    render_footer(f, app, root[3]);
}

// ─── Header ──────────────────────────────────────────────────────────────────
fn render_header(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Fill(1), Constraint::Length(30)])
        .split(area);

    let title_line = Line::from(vec![
        Span::styled("⬡ ", Style::default().fg(CLR_ACCENT)),
        Span::styled("PROCWATCH", Style::default().fg(CLR_TEXT).add_modifier(Modifier::BOLD)),
        Span::styled("  ", Style::default()),
        Span::styled(
            format!("{} processes", app.processes.len()),
            Style::default().fg(CLR_MUTED),
        ),
        if !app.filter.is_empty() {
            Span::styled(
                format!("  filter: \"{}\"", app.filter),
                Style::default().fg(CLR_WARN),
            )
        } else { Span::raw("") },
    ]);

    f.render_widget(
        Paragraph::new(title_line).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(CLR_ACCENT))
                .style(Style::default().bg(CLR_BG)),
        ),
        chunks[0],
    );

    // Tab indicator
    let tabs = Line::from(vec![
        Span::styled(
            " Processes ",
            if app.tab == Tab::Processes {
                Style::default().fg(CLR_BG).bg(CLR_ACCENT).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(CLR_MUTED)
            },
        ),
        Span::styled(" │ ", Style::default().fg(CLR_MUTED)),
        Span::styled(
            " Overview ",
            if app.tab == Tab::Overview {
                Style::default().fg(CLR_BG).bg(CLR_ACCENT2).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(CLR_MUTED)
            },
        ),
    ]);
    f.render_widget(
        Paragraph::new(tabs)
            .alignment(Alignment::Right)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(CLR_MUTED))
                    .style(Style::default().bg(CLR_BG)),
            ),
        chunks[1],
    );
}

// ─── System Bars ─────────────────────────────────────────────────────────────
fn render_system_bars(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // CPU Gauge
    let cpu = app.cpu_pct();
    let cpu_color = gauge_color(cpu);
    let cpu_gauge = Gauge::default()
        .block(
            Block::default()
                .title(Line::from(vec![
                    Span::styled(" CPU ", Style::default().fg(CLR_ACCENT).add_modifier(Modifier::BOLD)),
                    Span::styled(format!("{:.1}%", cpu), Style::default().fg(CLR_TEXT)),
                    Span::raw(" "),
                ]))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(CLR_ACCENT))
                .style(Style::default().bg(CLR_BG)),
        )
        .gauge_style(Style::default().fg(cpu_color).bg(Color::Rgb(20, 25, 35)))
        .percent(cpu as u16)
        .label(Span::styled(
            bar_fill(cpu, 20),
            Style::default().fg(cpu_color),
        ));
    f.render_widget(cpu_gauge, chunks[0]);

    // RAM Gauge
    let ram_pct = app.ram_pct();
    let ram_color = gauge_color(ram_pct);
    let ram_gauge = Gauge::default()
        .block(
            Block::default()
                .title(Line::from(vec![
                    Span::styled(" RAM ", Style::default().fg(CLR_ACCENT2).add_modifier(Modifier::BOLD)),
                    Span::styled(
                        format!("{:.1}/{:.1} GB  ({:.0}%)", app.used_ram_gb(), app.total_ram_gb(), ram_pct),
                        Style::default().fg(CLR_TEXT),
                    ),
                    Span::raw(" "),
                ]))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(CLR_ACCENT2))
                .style(Style::default().bg(CLR_BG)),
        )
        .gauge_style(Style::default().fg(ram_color).bg(Color::Rgb(20, 25, 35)))
        .percent(ram_pct as u16)
        .label(Span::styled(
            bar_fill(ram_pct, 20),
            Style::default().fg(ram_color),
        ));
    f.render_widget(ram_gauge, chunks[1]);
}

// ─── Process Table ───────────────────────────────────────────────────────────
fn render_process_table(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(area);

    let sort_arrow = |s: SortBy| -> &'static str {
        if app.sort_by == s { if app.sort_desc { " ▼" } else { " ▲" } } else { "" }
    };

    let header_style = Style::default().fg(CLR_BG).bg(CLR_ACCENT).add_modifier(Modifier::BOLD);
    let header = Row::new([
        Cell::from(format!("PID{}", sort_arrow(SortBy::Pid))).style(header_style),
        Cell::from(format!("NAME{}", sort_arrow(SortBy::Name))).style(header_style),
        Cell::from(format!("RAM (MB){}", sort_arrow(SortBy::Ram))).style(header_style),
        Cell::from("RAM BAR").style(header_style),
        Cell::from(format!("CPU%{}", sort_arrow(SortBy::Cpu))).style(header_style),
        Cell::from("CPU BAR").style(header_style),
        Cell::from("STATE").style(header_style),
    ]);

    // Find max RAM for bar scaling
    let max_ram = app.processes.iter().map(|p| p.ram_mb as u64).max().unwrap_or(1).max(1);
    let max_cpu = app.processes.iter().map(|p| p.cpu_pct as u64).max().unwrap_or(1).max(1) as f64;

    let selected_style = Style::default().bg(CLR_SELECTED).add_modifier(Modifier::BOLD);
    let normal_style   = Style::default().bg(CLR_BG);

    let rows: Vec<Row> = app.processes.iter().enumerate().map(|(i, p)| {
        let is_selected = app.table_state.selected() == Some(i);
        let row_style = if is_selected { selected_style } else { normal_style };

        let ram_bar = mini_bar(p.ram_mb as u64, max_ram, 12);
        let cpu_bar = mini_bar(p.cpu_pct as u64, max_cpu as u64, 10);
        let ram_color = gauge_color(p.ram_mb / max_ram as f64 * 100.0);
        let cpu_color = gauge_color(p.cpu_pct as f64 / max_cpu * 100.0);

        Row::new([
            Cell::from(p.pid.to_string()).style(Style::default().fg(CLR_MUTED)),
            Cell::from(truncate(&p.name, 22)).style(Style::default().fg(CLR_TEXT).add_modifier(Modifier::BOLD)),
            Cell::from(format!("{:>9.1}", p.ram_mb)).style(Style::default().fg(ram_color)),
            Cell::from(ram_bar).style(Style::default().fg(ram_color)),
            Cell::from(format!("{:>5.1}%", p.cpu_pct)).style(Style::default().fg(cpu_color)),
            Cell::from(cpu_bar).style(Style::default().fg(cpu_color)),
            Cell::from(p.status.clone()).style(Style::default().fg(p.status_color).add_modifier(Modifier::BOLD)),
        ])
        .style(row_style)
        .height(1)
    }).collect();

    let table = Table::new(
        rows,
        [
            Constraint::Length(7),   // PID
            Constraint::Length(23),  // NAME
            Constraint::Length(10),  // RAM MB
            Constraint::Length(14),  // RAM BAR
            Constraint::Length(7),   // CPU%
            Constraint::Length(12),  // CPU BAR
            Constraint::Length(8),   // STATE
        ],
    )
    .header(header)
    .block(
        Block::default()
            .title(Line::from(vec![
                Span::styled(" Processes ", Style::default().fg(CLR_ACCENT).add_modifier(Modifier::BOLD)),
                Span::styled(
                    format!("({}/{} shown)", app.processes.len(), app.sys.processes().len()),
                    Style::default().fg(CLR_MUTED),
                ),
                Span::raw(" "),
            ]))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(CLR_ACCENT))
            .style(Style::default().bg(CLR_BG)),
    )
    .row_highlight_style(selected_style)
    .highlight_symbol("▶ ");

    f.render_stateful_widget(table, chunks[0], &mut app.table_state);

    // Scrollbar
    let scrollbar = Scrollbar::default()
        .orientation(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("▲"))
        .end_symbol(Some("▼"))
        .track_symbol(Some("│"))
        .thumb_symbol("█");
    f.render_stateful_widget(scrollbar, chunks[1], &mut app.scroll_state);
}

// ─── Overview ────────────────────────────────────────────────────────────────
fn render_overview(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // CPU History chart (as BarChart)
    let cpu_data: Vec<(&str, u64)> = app
        .cpu_history
        .iter()
        .map(|&v| ("", v as u64))
        .collect();
    let cpu_chart = BarChart::default()
        .block(
            Block::default()
                .title(Line::from(vec![
                    Span::styled(" CPU History ", Style::default().fg(CLR_ACCENT).add_modifier(Modifier::BOLD)),
                    Span::styled(format!("(current: {:.1}%)", app.cpu_pct()), Style::default().fg(CLR_TEXT)),
                    Span::raw(" "),
                ]))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(CLR_ACCENT))
                .style(Style::default().bg(CLR_BG)),
        )
        .data(&cpu_data)
        .bar_width(1)
        .bar_gap(0)
        .max(100)
        .bar_style(Style::default().fg(CLR_ACCENT))
        .value_style(Style::default().fg(CLR_BG));
    f.render_widget(cpu_chart, chunks[0]);

    // RAM History chart
    let ram_data: Vec<(&str, u64)> = app
        .ram_history
        .iter()
        .map(|&v| ("", v as u64))
        .collect();
    let ram_chart = BarChart::default()
        .block(
            Block::default()
                .title(Line::from(vec![
                    Span::styled(" RAM History ", Style::default().fg(CLR_ACCENT2).add_modifier(Modifier::BOLD)),
                    Span::styled(
                        format!("(current: {:.1}/{:.1} GB)", app.used_ram_gb(), app.total_ram_gb()),
                        Style::default().fg(CLR_TEXT),
                    ),
                    Span::raw(" "),
                ]))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(CLR_ACCENT2))
                .style(Style::default().bg(CLR_BG)),
        )
        .data(&ram_data)
        .bar_width(1)
        .bar_gap(0)
        .max(100)
        .bar_style(Style::default().fg(CLR_ACCENT2))
        .value_style(Style::default().fg(CLR_BG));
    f.render_widget(ram_chart, chunks[1]);
}

// ─── Footer ──────────────────────────────────────────────────────────────────
fn render_footer(f: &mut Frame, app: &App, area: Rect) {
    let (filter_indicator, filter_style) = if app.filter_mode {
        (
            format!(" 🔍 Filter: {}█ ", app.filter),
            Style::default().fg(CLR_WARN).add_modifier(Modifier::BOLD),
        )
    } else {
        (String::new(), Style::default())
    };

    let line = if app.filter_mode {
        Line::from(vec![
            Span::styled(filter_indicator, filter_style),
            Span::styled("  [Esc] done  [Backspace] delete", Style::default().fg(CLR_MUTED)),
        ])
    } else {
        Line::from(vec![
            Span::styled(" [q]", Style::default().fg(CLR_ACCENT).add_modifier(Modifier::BOLD)),
            Span::styled(" quit  ", Style::default().fg(CLR_MUTED)),
            Span::styled("[↑↓]", Style::default().fg(CLR_ACCENT).add_modifier(Modifier::BOLD)),
            Span::styled(" navigate  ", Style::default().fg(CLR_MUTED)),
            Span::styled("[Tab]", Style::default().fg(CLR_ACCENT).add_modifier(Modifier::BOLD)),
            Span::styled(" switch view  ", Style::default().fg(CLR_MUTED)),
            Span::styled("[/]", Style::default().fg(CLR_ACCENT).add_modifier(Modifier::BOLD)),
            Span::styled(" filter  ", Style::default().fg(CLR_MUTED)),
            Span::styled("[1]", Style::default().fg(CLR_ACCENT2).add_modifier(Modifier::BOLD)),
            Span::styled(" name  ", Style::default().fg(CLR_MUTED)),
            Span::styled("[2]", Style::default().fg(CLR_ACCENT2).add_modifier(Modifier::BOLD)),
            Span::styled(" ram  ", Style::default().fg(CLR_MUTED)),
            Span::styled("[3]", Style::default().fg(CLR_ACCENT2).add_modifier(Modifier::BOLD)),
            Span::styled(" cpu  ", Style::default().fg(CLR_MUTED)),
            Span::styled("[4]", Style::default().fg(CLR_ACCENT2).add_modifier(Modifier::BOLD)),
            Span::styled(" pid  ", Style::default().fg(CLR_MUTED)),
            Span::styled("[x]", Style::default().fg(CLR_CRIT).add_modifier(Modifier::BOLD)),
            Span::styled(" clear filter  ", Style::default().fg(CLR_MUTED)),
            Span::styled("[r]", Style::default().fg(CLR_OK).add_modifier(Modifier::BOLD)),
            Span::styled(" refresh", Style::default().fg(CLR_MUTED)),
        ])
    };

    f.render_widget(
        Paragraph::new(line).style(Style::default().bg(Color::Rgb(15, 15, 28))),
        area,
    );
}

// ─── Helpers ─────────────────────────────────────────────────────────────────
fn gauge_color(pct: f64) -> Color {
    if pct >= 90.0      { CLR_CRIT }
    else if pct >= 70.0 { CLR_WARN }
    else if pct >= 40.0 { CLR_ACCENT }
    else                { CLR_OK }
}

fn mini_bar(val: u64, max: u64, width: usize) -> String {
    let blocks = ['░', '▒', '▓', '█'];
    // let blocks = ['\u{2591}', '\u{2592}', '\u{2593}', '\u{2588}'];
    if max == 0 { return " ".repeat(width); }
    let filled = (val as f64 / max as f64 * width as f64).round() as usize;
    let filled = filled.min(width);
    format!(
        "{}{}",
        "█".repeat(filled),
        "░".repeat(width - filled),
    )
}

fn bar_fill(pct: f64, width: usize) -> String {
    let filled = (pct / 100.0 * width as f64).round() as usize;
    let filled = filled.min(width);
    format!("{}{}", "█".repeat(filled), "░".repeat(width - filled))
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}…", &s[..max - 1])
    }
}