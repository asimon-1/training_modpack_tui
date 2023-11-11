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

use training_mod_tui_2::{
    App, InputControl, StatefulTable, SubMenu, SubMenuType, Tab, Toggle, NX_SUBMENU_COLUMNS,
    NX_SUBMENU_ROWS, NX_TAB_COLUMNS, NX_TAB_ROWS,
};

fn main() -> Result<(), Box<dyn Error>> {
    let app = create_app();
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

pub fn create_app<'a>() -> App<'a> {
    let a_button = Toggle {
        toggle_title: "A Button",
        toggle_value: 0,
        toggle_max: 1,
    };
    let b_button = Toggle {
        toggle_title: "B Button",
        toggle_value: 1,
        toggle_max: 1,
    };

    let mut app = App::new(NX_TAB_ROWS, NX_TAB_COLUMNS);
    let mut button_tab_submenus: Vec<SubMenu> = Vec::new();
    button_tab_submenus.push(SubMenu {
        title: "Menu Open Start Press",
        id: "menu_open_start_press",
        help_text: "Help",
        type_: SubMenuType::ToggleSingle,
        toggles: StatefulTable::with_items(
            NX_SUBMENU_ROWS,
            NX_SUBMENU_COLUMNS,
            vec![a_button, b_button],
        ),
        slider: None,
    });
    button_tab_submenus.push(SubMenu {
        title: "Save State Save",
        id: "save_state_save",
        help_text: "Save State Save: Hold any one button and press the others to trigger",
        type_: SubMenuType::ToggleMultiple,
        toggles: StatefulTable::with_items(NX_SUBMENU_ROWS, NX_SUBMENU_COLUMNS, Vec::new()),
        slider: None,
    });

    button_tab_submenus.push(SubMenu {
        title: "Menu Open Start Press",
        id: "menu_open_start_press",
        help_text: "Menu Open Start Press: Hold start or press minus to open the mod menu. To open the original menu, press start.\nThe default menu open option is always available as Hold DPad Up + Press B.",
        type_: SubMenuType::ToggleSingle,
        toggles: StatefulTable::with_items(NX_SUBMENU_ROWS, NX_SUBMENU_COLUMNS, Vec::new()),
        slider: None,
    });
    button_tab_submenus.push(SubMenu {
        title: "Save State Save",
        id: "save_state_save",
        help_text: "Save State Save: Hold any one button and press the others to trigger",
        type_: SubMenuType::ToggleMultiple,
        toggles: StatefulTable::with_items(NX_SUBMENU_ROWS, NX_SUBMENU_COLUMNS, Vec::new()),
        slider: None,
    });
    button_tab_submenus.push(SubMenu {
        title: "Save State Load",
        id: "save_state_load",
        help_text: "Save State Load: Hold any one button and press the others to trigger",
        type_: SubMenuType::ToggleMultiple,
        toggles: StatefulTable::with_items(NX_SUBMENU_ROWS, NX_SUBMENU_COLUMNS, Vec::new()),
        slider: None,
    });
    button_tab_submenus.push(SubMenu {
        title: "Input Record",
        id: "input_record",
        help_text: "Input Record: Hold any one button and press the others to trigger",
        type_: SubMenuType::ToggleMultiple,
        toggles: StatefulTable::with_items(NX_SUBMENU_ROWS, NX_SUBMENU_COLUMNS, Vec::new()),
        slider: None,
    });
    button_tab_submenus.push(SubMenu {
        title: "Input Playback",
        id: "input_playback",
        help_text: "Input Playback: Hold any one button and press the others to trigger",
        type_: SubMenuType::ToggleMultiple,
        toggles: StatefulTable::with_items(NX_SUBMENU_ROWS, NX_SUBMENU_COLUMNS, Vec::new()),
        slider: None,
    });

    let button_tab = Tab {
        id: "button",
        title: "Button Config",
        submenus: StatefulTable::with_items(
            NX_SUBMENU_ROWS,
            NX_SUBMENU_COLUMNS,
            button_tab_submenus,
        ),
    };
    app.tabs = StatefulTable::with_items(NX_TAB_ROWS, NX_TAB_COLUMNS, vec![button_tab]);
    app
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    mut app: training_mod_tui_2::App,
    tick_rate: Duration,
) -> io::Result<String> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| training_mod_tui_2::render_ui(f, &mut app))?;
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
                    KeyCode::Left => app.on_left(),
                    KeyCode::Right => app.on_right(),
                    KeyCode::Down => app.on_down(),
                    KeyCode::Up => app.on_up(),
                    KeyCode::Enter => app.on_a(),
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
