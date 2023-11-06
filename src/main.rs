use crossterm::{
    event::{self, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::error::Error;
use std::{
    io,
    time::{Duration, Instant},
};

fn main() -> Result<(), Box<dyn Error>> {
    let app = training_mod_tui_2::create_app();
    let mut terminal = setup_terminal()?;

    let tick_rate = Duration::from_millis(250);
    let res = run_app(&mut terminal, app, tick_rate);
    restore_terminal(terminal)?;

    if let Err(err) = res {
        println!("Error: {:?}", err)
    } else {
        println!("JSON: {:#?}", res.as_ref().unwrap());
    }

    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>, Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal(
    mut terminal: Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    mut app: training_mod_tui_2::App,
    tick_rate: Duration,
) -> io::Result<String> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| training_mod_tui_2::ui(f, &mut app))?;
        let menu_json = app.to_json();

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(menu_json),
                    // KeyCode::Char('x') => app.save_defaults(),
                    // KeyCode::Char('p') => app.reset_current_submenu(),
                    // KeyCode::Char('o') => app.reset_all_submenus(),
                    // KeyCode::Char('r') => app.next_tab(),
                    // KeyCode::Char('l') => app.previous_tab(),
                    // KeyCode::Left => app.on_left(),
                    // KeyCode::Right => app.on_right(),
                    // KeyCode::Down => app.on_down(),
                    // KeyCode::Up => app.on_up(),
                    // KeyCode::Enter => app.on_a(),
                    // KeyCode::Backspace => app.on_b(),
                    _ => {}
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}
