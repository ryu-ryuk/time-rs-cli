use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{
        EnterAlternateScreen, LeaveAlternateScreen, SetSize, disable_raw_mode, enable_raw_mode,
    },
};
#[cfg(target_os = "linux")]
use notify_rust::Notification;
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
    io,
    time::{Duration, Instant},
};

#[derive(Parser, Debug)]
#[command(name = "timer-cli", disable_help_flag = true)]
#[command(about = "A terminal-based countdown timer", long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 120)]
    duration: u64,
    #[arg(short, long, default_value = "timer-cli")]
    title: String,
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
    let mut already_notified = false;
    let mut paused = false;
    let mut paused_at = Duration::ZERO;

    loop {
        let now = Instant::now();
        let elapsed = if paused { paused_at } else { now - start };
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
            let is_done = remaining.is_zero();

            let time_style = if is_done {
                Style::default()
                    .fg(Color::Rgb(243, 139, 168))
                    .add_modifier(Modifier::RAPID_BLINK | Modifier::BOLD)
            } else {
                Style::default()
                    .fg(theme.text)
                    .add_modifier(Modifier::BOLD | Modifier::SLOW_BLINK)
            };

            let progress = if duration.as_secs() > 0 {
                (elapsed.as_secs_f64() / duration.as_secs_f64()).clamp(0.0, 1.0)
            } else {
                1.0
            };

            let total_blocks = 20;
            let filled = (progress * total_blocks as f64).round() as usize;
            let empty = total_blocks - filled;

            let bar = format!(
                "⏳ [{}{}] {:>3}% {}",
                "■".repeat(filled),
                "・".repeat(empty),
                (progress * 100.0).round() as u8,
                match (progress * 100.0) as u8 {
                    100 => "(๑•̀ㅂ•́)و✧ 完了！",
                    80..=99 => "٩(｡•́‿•̀｡)۶ Almost there",
                    50..=79 => "( •̀ ω •́ )✧ 半分だ！",
                    20..=49 => "(・・;) まだ...",
                    _ => "(´・ω・｀) Just Starting",
                }
            );

            let bar_style = if is_done {
                Style::default()
                    .fg(Color::Rgb(243, 139, 168))
                    .add_modifier(Modifier::RAPID_BLINK | Modifier::BOLD)
            } else {
                Style::default()
                    .fg(Color::Rgb(180, 190, 254))
                    .bg(Color::Rgb(49, 50, 68))
                    .add_modifier(Modifier::BOLD)
            };

            let content = if show_help {
                vec![
                    Line::from(""),
                    Line::from(Span::styled(
                        "╭─[ Control Panel: 操作一覧 ]─╮",
                        theme_style(&theme),
                    )),
                    Line::from(Span::styled(
                        "│ q: quit      r: restart       │",
                        theme_style(&theme),
                    )),
                    Line::from(Span::styled(
                        "│ j: +10s      k: -10s          │",
                        theme_style(&theme),
                    )),
                    Line::from(Span::styled(
                        "│ p: pomodoro  ␣: pause/resume │",
                        theme_style(&theme),
                    )),
                    Line::from(Span::styled(
                        "│ h: toggle help  esc: close    │",
                        theme_style(&theme),
                    )),
                    Line::from(Span::styled(
                        "╰──────────────────────────────╯",
                        theme_style(&theme),
                    )),
                    Line::from(""),
                    Line::from(Span::styled(format!("(；・∀・)  {time_str}"), time_style)),
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
                    Line::from(Span::styled(time_str, time_style)),
                    Line::from(Span::styled(bar, bar_style)),
                    Line::from(""),
                    Line::from(Span::styled(
                        "press 'h' to open control panel ( ＾◡＾)っ ♨",
                        Style::default()
                            .fg(Color::DarkGray)
                            .add_modifier(Modifier::ITALIC),
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
                        paused = false;
                        paused_at = Duration::ZERO;
                        show_help = false;
                        already_notified = false;
                    }
                    KeyCode::Char(' ') => {
                        paused = !paused;
                        if paused {
                            paused_at = Instant::now() - start;
                        } else {
                            start = Instant::now() - paused_at;
                        }
                        show_help = false;
                    }
                    KeyCode::Char('h') => show_help = !show_help,
                    KeyCode::Esc => show_help = false,
                    KeyCode::Char('j') => {
                        duration += Duration::from_secs(10);
                        show_help = false;
                        already_notified = false;
                    }
                    KeyCode::Char('k') => {
                        if duration > Duration::from_secs(10) {
                            duration -= Duration::from_secs(10);
                        }
                        show_help = false;
                        already_notified = false;
                    }
                    KeyCode::Char('p') => {
                        duration = Duration::from_secs(1500);
                        start = Instant::now();
                        paused = false;
                        paused_at = Duration::ZERO;
                        show_help = false;
                        already_notified = false;
                    }
                    _ => {}
                }
            }
        }

        if remaining.is_zero() && !already_notified {
            already_notified = true;
            #[cfg(target_os = "linux")]
            Notification::new()
                .summary("⌛ Timer Done")
                .body(&format!("{} is over!", args.title))
                .hint(notify_rust::Hint::Resident(true))
                .show()
                .ok();
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
        bg: Color::Rgb(24, 24, 37),
        border: Color::Rgb(48, 45, 65),
        text: Color::Rgb(205, 214, 244),
        title: Color::Rgb(180, 190, 254),
    }
}

fn theme_style(theme: &Theme) -> Style {
    Style::default()
        .fg(theme.text)
        .bg(theme.bg)
        .add_modifier(Modifier::DIM)
}
