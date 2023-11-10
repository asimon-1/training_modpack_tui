use crate::{App, AppPage};
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
    }
}

#[allow(dead_code, unused_variables)]
fn render_submenu_page(frame: &mut Frame, app: &mut App, area: Rect, help_chunk: Rect) {}

#[allow(dead_code, unused_variables)]
fn render_slider_page(frame: &mut Frame, app: &mut App, area: Rect, help_chunk: Rect) {}

#[allow(dead_code, unused_variables)]
fn render_toggle_page(frame: &mut Frame, app: &mut App, area: Rect, help_chunk: Rect) {}

// fn toggle_table(rows: Vec<Vec<Option<Toggle>>>) -> Table<'a> {
//     Table::new(rows.try_into().unwrap())
//         .block(Block::default().borders(Borders::ALL).title("Submenus:"))
//         .highlight_symbol("-- ")
//         .highlight_spacing(HighlightSpacing::Never)
//         .widths(&[Constraint::Ratio(1, NX_SUBMENU_COLUMNS as u32); NX_SUBMENU_COLUMNS])
// }
