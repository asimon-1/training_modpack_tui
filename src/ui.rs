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

    match app.page {
        AppPage::SUBMENU => render_submenu_page(frame, app, menu_area, help_area),
        AppPage::TOGGLE => render_toggle_page(frame, app, menu_area, help_area),
        AppPage::SLIDER => {
            frame.render_widget(Paragraph::new("Slider!"), menu_area);
        }
        AppPage::CONFIRMATION => {
            frame.render_widget(Paragraph::new("Confirmation!"), menu_area);
        }
        AppPage::CLOSE => {}
    }
}

#[allow(dead_code, unused_variables)]
fn render_submenu_page(frame: &mut Frame, app: &mut App, area: Rect, help_chunk: Rect) {
    // Convert the currently selected tab's grid of Option<SubMenu>'s
    // into a Vec<Row<Cell>> so that we can pass it into Table::new()
    let submenus = &mut app.selected_tab().submenus;
    let submenus_unwrapped = submenus.as_vec_vec_t();
    let rows = submenus_unwrapped
        .iter()
        .map(|row| row.iter().map(|submenu| Cell::from(submenu.title)))
        .map(|row| Row::new(row));

    let table = Table::new(rows)
        .block(Block::default().borders(Borders::ALL).title("Submenus:"))
        .cell_highlight_style(Style::default().bg(Color::Gray))
        .widths(&[Constraint::Ratio(1, NX_SUBMENU_COLUMNS as u32); NX_SUBMENU_COLUMNS]);

    frame.render_stateful_widget(table, area, &mut submenus.state);
}

#[allow(dead_code, unused_variables)]
fn render_toggle_page(frame: &mut Frame, app: &mut App, area: Rect, help_chunk: Rect) {
    let toggles = &mut app.selected_submenu().toggles;
    let toggles_unwrapped = toggles.as_vec_vec_t();
    let rows = toggles_unwrapped
        .iter()
        .map(|row| row.iter().map(|toggle| Cell::from(toggle.title)))
        .map(|row| Row::new(row));

    let table = Table::new(rows)
        .block(Block::default().borders(Borders::ALL).title("Submenus:"))
        .cell_highlight_style(Style::default().bg(Color::Gray))
        .widths(&[Constraint::Ratio(1, NX_SUBMENU_COLUMNS as u32); NX_SUBMENU_COLUMNS]);

    frame.render_stateful_widget(table, area, &mut toggles.state);
}

#[allow(dead_code, unused_variables)]
fn render_slider_page(frame: &mut Frame, app: &mut App, area: Rect, help_chunk: Rect) {}
