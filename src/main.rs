use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{
        EnterAlternateScreen, LeaveAlternateScreen, SetSize, disable_raw_mode, enable_raw_mode,
    },
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

use std::{
    error::Error,
    io::{self},
    time::{Duration, Instant},
};

#[derive(Parser, Debug)]
#[command(name = "timer-cli")]
#[command(about = "A terminal-based countdown timer", long_about = None)]
struct Args {
    /// duration in seconds
    #[arg(short, long, default_value_t = 120)]
    duration: u64,

    /// title
    #[arg(short, long, default_value = "timer-cli")]
    title: String,

    /// mocha
    #[arg(short, long, default_value = "mocha")]
    style: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    run(args)
}

fn run(args: Args) -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    // try resizing terminal to 50x8 cells for best preview
    // TODO: doesn't work
    execute!(stdout, SetSize(50, 8)).ok();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = timer_loop(&mut terminal, args);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    res
}

fn timer_loop(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    args: Args,
) -> Result<(), Box<dyn Error>> {
    let mut start = Instant::now();
    let mut duration = Duration::from_secs(args.duration);
    let theme = mocha_theme();
    let mut show_help = false;

    loop {
        let elapsed = Instant::now() - start;
        let remaining = if duration > elapsed {
            duration - elapsed
        } else {
            Duration::ZERO
        };

        terminal.draw(|f| {
            let area = f.area();
            let layout = Layout::default()
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(area);

            let mins = remaining.as_secs() / 60;
            let secs = remaining.as_secs() % 60;
            let time_str = format!("{:02}:{:02}", mins, secs);

            let content = if show_help {
                vec![
                    Line::from(""),
                    Line::from(Span::styled(
                        "┌──────────── Help ────────────┐",
                        theme_style(&theme),
                    )),
                    Line::from(Span::styled(
                        "│ q: quit   r: restart   h: help │",
                        theme_style(&theme),
                    )),
                    Line::from(Span::styled(
                        "│ j: +10s   k: -10s    esc: close │",
                        theme_style(&theme),
                    )),
                    Line::from(Span::styled(
                        "└───────────────────────────────┘",
                        theme_style(&theme),
                    )),
                    Line::from(""),
                    Line::from(Span::styled(
                        format!("(＾＾；) {time_str}"),
                        Style::default()
                            .fg(theme.text)
                            .add_modifier(Modifier::BOLD | Modifier::SLOW_BLINK),
                    )),
                ]
            } else {
                vec![
                    Line::from(Span::styled(
                        &args.title,
                        Style::default()
                            .fg(theme.title)
                            .add_modifier(Modifier::BOLD | Modifier::ITALIC),
                    )),
                    Line::from(""),
                    Line::from(Span::styled(
                        time_str,
                        Style::default()
                            .fg(theme.text)
                            .add_modifier(Modifier::BOLD | Modifier::SLOW_BLINK),
                    )),
                ]
            };

            let block = Paragraph::new(content)
                .alignment(Alignment::Center)
                .block(Block::default().style(Style::default().fg(theme.border).bg(theme.bg)));

            f.render_widget(block, layout[0]);
        })?;

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('r') => {
                        start = Instant::now();
                        show_help = false;
                    }
                    KeyCode::Char('h') => show_help = !show_help,
                    KeyCode::Esc => show_help = false,
                    KeyCode::Char('j') => {
                        duration += Duration::from_secs(10);
                        show_help = false;
                    }
                    KeyCode::Char('k') => {
                        if duration > Duration::from_secs(10) {
                            duration -= Duration::from_secs(10);
                        }
                        show_help = false;
                    }
                    _ => {}
                }
            }
        }

        if remaining.is_zero() {
            continue;
        }
    }

    Ok(())
}

struct Theme {
    bg: Color,
    border: Color,
    text: Color,
    title: Color,
}

fn mocha_theme() -> Theme {
    Theme {
        bg: Color::Rgb(24, 24, 37),       // base
        border: Color::Rgb(48, 45, 65),   // overlay
        text: Color::Rgb(205, 214, 244),  // text
        title: Color::Rgb(180, 190, 254), // blue/lavender
    }
}

fn theme_style(theme: &Theme) -> Style {
    Style::default()
        .fg(theme.text)
        .bg(theme.bg)
        .add_modifier(Modifier::DIM)
}
