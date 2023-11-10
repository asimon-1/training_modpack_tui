use crate::{App, AppPage};
use ratatui::{backend::Backend, layout::Rect, prelude::*, widgets::Paragraph, Frame};

#[allow(unused_variables)]
pub fn render_ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
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
            f.render_widget(Paragraph::new("Toggle!"), tab_area);
        }
        AppPage::SLIDER => {
            f.render_widget(Paragraph::new("Slider!"), tab_area);
        }
        AppPage::CONFIRMATION => {
            f.render_widget(Paragraph::new("Confirmation!"), tab_area);
        }
    }
}

#[allow(dead_code, unused_variables)]
fn render_submenu_page<B: Backend>(
    frame: &mut Frame<B>,
    app: &mut App,
    area: Rect,
    help_chunk: Rect,
) {
    let t = app
        .tabs
        .get_selected()
        .unwrap()
        .clone();
    frame.render_widget(t, area);
}

#[allow(dead_code, unused_variables)]
fn render_slider_page<B: Backend>(
    frame: &mut Frame<B>,
    app: &mut App,
    area: Rect,
    help_chunk: Rect,
) {
}

#[allow(dead_code, unused_variables)]
fn render_toggle_page<B: Backend>(
    frame: &mut Frame<B>,
    app: &mut App,
    area: Rect,
    help_chunk: Rect,
) {
    let t = app
        .tabs
        .get_selected()
        .unwrap()
        .submenus
        .get_selected()
        .unwrap()
        .clone();
    frame.render_widget(t, area);
}
