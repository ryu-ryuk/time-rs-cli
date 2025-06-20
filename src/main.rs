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
#[command(name = "timeRS", disable_help_flag = true)]
#[command(about = "A terminal-based countdown timer", long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 120)]
    duration: u64,
    #[arg(short, long, default_value = "time-RS")]
    title: String,
    #[arg(short, long, default_value = "mocha")]
    style: String,
}

enum AppState {
    Normal,
    Help,
    Input,
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
    let mut app_state = AppState::Normal;
    let mut already_notified = false;
    let mut paused = false;
    let mut paused_at = Duration::ZERO;
    let mut tagline_index = 0;
    let mut last_tagline_change = Instant::now();
    let tagline_change_interval = Duration::from_secs(5);
    let mut input_buffer = String::new();

    let taglines = [
        "press 'h' for control panel (＾ｖ＾)ノ",
        "set custom time with 'm' key ⌛",
        "pomodoro mode? hit 'p' 💡",
        "press 'space' to pause/resume ⏸️",
    ];

    let reset_timer_state = |new_duration: Option<Duration>,
                             reset_paused_state: bool,
                             reset_elapsed_time: bool,
                             start_time: &mut Instant,
                             current_duration: &mut Duration,
                             is_paused: &mut bool,
                             paused_duration: &mut Duration,
                             notified_flag: &mut bool| {
        if let Some(d) = new_duration {
            *current_duration = d;
        }
        if reset_elapsed_time {
            *start_time = Instant::now();
            *notified_flag = false;
        }
        if reset_paused_state {
            *is_paused = false;
            *paused_duration = Duration::ZERO;
        }
        if *current_duration == Duration::ZERO {
            *notified_flag = true;
        }
    };

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
            let build_status = if is_done {
                "[OK]"
            } else if paused {
                "[||]"
            } else {
                "[..]"
            };
            let total_blocks = 12;
            let filled = (progress * total_blocks as f64).round() as usize;
            let bar = format!(
                "{} [{}{}] {:>3}% {}",
                build_status,
                "█".repeat(filled),
                "░".repeat(total_blocks - filled),
                (progress * 100.0).round() as u8,
                time_str
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

            let content = match app_state {
                AppState::Help => {
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
                            "│ m: manual set (mins) esc: ✕  │",
                            theme_style(&theme),
                        )),
                        Line::from(Span::styled(
                            "╰──────────────────────────────╯",
                            theme_style(&theme),
                        )),
                        Line::from(""),
                        Line::from(Span::styled(format!("(；・∀・)  {time_str}"), time_style)),
                    ]
                }
                AppState::Input => {
                    vec![
                        Line::from(""),
                        Line::from(Span::styled(
                            "⏱️  Enter duration in minutes:",
                            theme_style(&theme),
                        )),
                        Line::from(""),
                        Line::from(Span::styled(
                            format!(">> {}", input_buffer),
                            Style::default()
                                .fg(Color::Rgb(137, 180, 250))
                                .add_modifier(Modifier::BOLD),
                        )),
                        Line::from(""),
                        Line::from(""),
                        Line::from(Span::styled(format!("(；・∀・)  {time_str}"), time_style)),
                    ]
                }
                AppState::Normal => {
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
                            taglines[tagline_index % taglines.len()],
                            Style::default()
                                .fg(Color::DarkGray)
                                .add_modifier(Modifier::ITALIC),
                        )),
                    ]
                }
            };

            let block = Paragraph::new(content)
                .alignment(Alignment::Center)
                .block(Block::default().style(Style::default().fg(theme.border).bg(theme.bg)));

            f.render_widget(block, layout[0]);
        })?;

        if now.duration_since(last_tagline_change) >= tagline_change_interval {
            tagline_index = tagline_index.wrapping_add(1);
            last_tagline_change = now;
        }

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match app_state {
                    AppState::Normal => match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('r') => {
                            reset_timer_state(
                                None,
                                true,
                                true,
                                &mut start,
                                &mut duration,
                                &mut paused,
                                &mut paused_at,
                                &mut already_notified,
                            );
                        }
                        KeyCode::Char(' ') => {
                            paused = !paused;
                            if paused {
                                paused_at = Instant::now() - start;
                            } else {
                                start = Instant::now() - paused_at;
                            }
                        }
                        KeyCode::Char('h') => app_state = AppState::Help,
                        KeyCode::Esc => app_state = AppState::Normal,
                        KeyCode::Char('j') => {
                            reset_timer_state(
                                Some(duration + Duration::from_secs(10)),
                                false,
                                false,
                                &mut start,
                                &mut duration,
                                &mut paused,
                                &mut paused_at,
                                &mut already_notified,
                            );
                        }
                        KeyCode::Char('k') => {
                            if duration > Duration::from_secs(10) {
                                reset_timer_state(
                                    Some(duration - Duration::from_secs(10)),
                                    false,
                                    false,
                                    &mut start,
                                    &mut duration,
                                    &mut paused,
                                    &mut paused_at,
                                    &mut already_notified,
                                );
                            }
                        }
                        KeyCode::Char('p') => {
                            reset_timer_state(
                                Some(Duration::from_secs(1500)),
                                true,
                                true,
                                &mut start,
                                &mut duration,
                                &mut paused,
                                &mut paused_at,
                                &mut already_notified,
                            );
                        }
                        KeyCode::Char('m') => {
                            app_state = AppState::Input;
                            input_buffer.clear();
                        }
                        _ => {}
                    },

                    AppState::Help => match key.code {
                        KeyCode::Char('q') | KeyCode::Char('h') | KeyCode::Esc => {
                            app_state = AppState::Normal;
                        }
                        _ => {}
                    },

                    AppState::Input => match key.code {
                        KeyCode::Char(c) if c.is_ascii_digit() => input_buffer.push(c),
                        KeyCode::Backspace => {
                            input_buffer.pop();
                        }
                        KeyCode::Enter => {
                            if let Ok(mins) = input_buffer.trim().parse::<u64>() {
                                reset_timer_state(
                                    Some(Duration::from_secs(mins * 60)),
                                    true,
                                    true,
                                    &mut start,
                                    &mut duration,
                                    &mut paused,
                                    &mut paused_at,
                                    &mut already_notified,
                                );
                            }
                            app_state = AppState::Normal;
                            input_buffer.clear();
                        }
                        KeyCode::Esc | KeyCode::Char('q') => {
                            app_state = AppState::Normal;
                            input_buffer.clear();
                        }
                        _ => {}
                    },
                }
            }
        }

        // Recompute state after event handling for accurate notification check
        let now_after_events = Instant::now();
        let elapsed_after_events = if paused {
            paused_at
        } else {
            now_after_events - start
        };
        let remaining_after_events = if duration > elapsed_after_events {
            duration - elapsed_after_events
        } else {
            Duration::ZERO
        };

        if remaining_after_events.is_zero() && !already_notified {
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
