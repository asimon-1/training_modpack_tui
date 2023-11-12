use crate::{App, AppPage, NX_SUBMENU_COLUMNS};
use ratatui::{layout::Rect, prelude::*, widgets::*, Frame};

#[allow(unused_variables)]
pub fn render_ui(frame: &mut Frame, app: &mut App) {
    // Set up Layout
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(frame.size());

    // Define Areas
    // tab_area: list across the top
    // menu_area: menu entries
    let tab_area = layout[0];
    let menu_area = layout[1];
    let help_area = layout[2];

    render_tabs(frame, app, tab_area);
    match app.page {
        AppPage::SUBMENU => render_submenu_page(frame, app, menu_area),
        AppPage::TOGGLE => render_toggle_page(frame, app, menu_area),
        AppPage::SLIDER => {
            frame.render_widget(Paragraph::new("Slider!"), menu_area);
        }
        AppPage::CONFIRMATION => {
            frame.render_widget(Paragraph::new("Confirmation!"), menu_area);
        }
        AppPage::CLOSE => {}
    }
    render_help_text(frame, app, help_area);
}

fn render_submenu_page(frame: &mut Frame, app: &mut App, area: Rect) {
    let submenus = &mut app.selected_tab().submenus;
    // Convert the currently selected tab's grid of Option<SubMenu>'s
    // into an Iter<Row<Cell>> so that we can pass it into Table::new()
    let rows = submenus
        .items
        .iter()
        .map(|row| {
            row.iter()
                .filter(|submenu| submenu.is_some())
                .map(|submenu| {
                    let s = submenu.clone().unwrap();
                    Cell::from(s.title.to_string())
                })
        })
        .map(|row| Row::new(row));

    let table = Table::new(rows)
        .block(Block::default().borders(Borders::ALL).title("Submenus:"))
        .cell_highlight_style(Style::default().bg(Color::Gray))
        .widths(&[Constraint::Ratio(1, NX_SUBMENU_COLUMNS as u32); NX_SUBMENU_COLUMNS]);

    frame.render_stateful_widget(table, area, &mut submenus.state);
}

fn render_toggle_page(frame: &mut Frame, app: &mut App, area: Rect) {
    let toggles = &mut app.selected_submenu().toggles;
    // Convert the currently selected submenu's grid of Option<Toggle>'s
    // into an Inter<Row<Cell>> so that we can pass it into Table::new()
    let rows = toggles
        .items
        .iter()
        .map(|row| {
            row.iter().filter(|x| x.is_some()).map(|toggle| {
                // Display both the title and the value
                // Don't need to clone() here because toggle is Copy
                let t = toggle.unwrap();
                Cell::from(t.title.to_string() + "  -  " + &t.value.to_string())
            })
        })
        .map(|row| Row::new(row));

    let table = Table::new(rows)
        .block(Block::default().borders(Borders::ALL).title("Submenus:"))
        .cell_highlight_style(Style::default().bg(Color::Gray))
        .widths(&[Constraint::Ratio(1, NX_SUBMENU_COLUMNS as u32); NX_SUBMENU_COLUMNS]);

    frame.render_stateful_widget(table, area, &mut toggles.state);
}

#[allow(dead_code, unused_variables)]
fn render_slider_page(frame: &mut Frame, app: &mut App, area: Rect) {}

fn render_tabs(frame: &mut Frame, app: &mut App, area: Rect) {
    let titles = app.tabs.iter().map(|tab| tab.title).collect();
    let tabs = Tabs::new(titles)
        .select(app.tabs.state.selected().unwrap())
        .highlight_style(Style::default().bg(Color::Gray));
    frame.render_widget(tabs, area);
}

fn render_help_text(frame: &mut Frame, app: &mut App, area: Rect) {
    frame.render_widget(Paragraph::new(app.selected_submenu().help_text), area);
}
