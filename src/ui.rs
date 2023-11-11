use crate::{App, AppPage, NX_SUBMENU_COLUMNS};
use ratatui::{layout::Rect, prelude::*, widgets::*, Frame};

#[allow(unused_variables)]
pub fn render_ui(f: &mut Frame, app: &mut App) {
    // Set up Layout
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(f.size());

    // Define Areas
    // tab_area: list across the top
    // menu_area: menu entries
    let tab_area = layout[0];
    let menu_area = layout[1];
    let help_area = layout[2];

    match app.page {
        AppPage::SUBMENU => render_submenu_page(f, app, menu_area, help_area),
        AppPage::TOGGLE => {
            f.render_widget(Paragraph::new("Toggle!"), menu_area);
        }
        AppPage::SLIDER => {
            f.render_widget(Paragraph::new("Slider!"), menu_area);
        }
        AppPage::CONFIRMATION => {
            f.render_widget(Paragraph::new("Confirmation!"), menu_area);
        }
        AppPage::CLOSE => {}
    }
}

#[allow(dead_code, unused_variables)]
fn render_submenu_page(frame: &mut Frame, app: &mut App, area: Rect, help_chunk: Rect) {
    // Convert the currently selected tab's grid of Option<SubMenu>'s
    // into a Vec<Row<Cell>> so that we can pass it into Table::new()
    let mut submenus = app
        .tabs
        .get_selected()
        .expect("No tab selected in render_submenu_page()!")
        .submenus
        .clone();
    let vec_vec_t = submenus.as_vec_vec_t();
    let rows = vec_vec_t
        .iter()
        .map(|row| row.iter().map(|submenu| Cell::from(submenu.title)))
        .map(|row| Row::new(row));

    let table = Table::new(rows)
        .block(Block::default().borders(Borders::ALL).title("Submenus:"))
        // .highlight_symbol("-- ")
        // .highlight_spacing(HighlightSpacing::WhenSelected)
        .cell_highlight_style(Style::default().bg(Color::Gray))
        .widths(&[Constraint::Ratio(1, NX_SUBMENU_COLUMNS as u32); NX_SUBMENU_COLUMNS]);

    frame.render_stateful_widget(table, area, &mut submenus.state);
}

#[allow(dead_code, unused_variables)]
fn render_slider_page(frame: &mut Frame, app: &mut App, area: Rect, help_chunk: Rect) {}

#[allow(dead_code, unused_variables)]
fn render_toggle_page(frame: &mut Frame, app: &mut App, area: Rect, help_chunk: Rect) {}
