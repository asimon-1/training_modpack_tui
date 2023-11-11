use itertools::Itertools;
use ratatui::{
    prelude::*,
    widgets::{Paragraph, Widget},
};
use serde::ser::Serializer;
use serde::Serialize;

use crate::{InputControl, Slider, StatefulTable, Toggle};

#[derive(Clone)]
pub struct SubMenu<'a> {
    pub title: &'a str,
    pub id: &'a str,
    pub help_text: &'a str,
    pub type_: SubMenuType,
    pub toggles: StatefulTable<Toggle<'a>>,
    pub slider: Option<Slider>,
}

impl<'a> Serialize for SubMenu<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // TODO! Match on SubMenuType and Impl for Slider
        self.toggles.serialize(serializer)
    }
}

impl<'a> Widget for SubMenu<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        let grid = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(3); self.toggles.rows])
            .split(area)
            .iter()
            .map(|&area| {
                Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(vec![
                        Constraint::Ratio(1, self.toggles.cols as u32);
                        self.toggles.cols
                    ])
                    .split(area)
                    .to_vec()
            })
            .collect_vec();
        for (x, row) in grid.iter().enumerate() {
            for (y, rect) in row.iter().enumerate() {
                let item_opt = self.toggles.get(x, y);
                if let Some(item) = item_opt {
                    Paragraph::new(item.toggle_title).render(*rect, buf);
                } else {
                    Paragraph::new("").render(*rect, buf);
                }
            }
        }
    }
}

impl<'a> InputControl for SubMenu<'a> {
    fn on_a(&mut self) {}
    fn on_b(&mut self) {}
    fn on_x(&mut self) {}
    fn on_y(&mut self) {}
    fn on_up(&mut self) {}
    fn on_down(&mut self) {}
    fn on_left(&mut self) {}
    fn on_right(&mut self) {}
    fn on_start(&mut self) {}
    fn on_l(&mut self) {}
    fn on_r(&mut self) {}
    fn on_zl(&mut self) {}
    fn on_zr(&mut self) {}
}

#[derive(Clone, Copy, Serialize)]
pub enum SubMenuType {
    ToggleSingle,
    ToggleMultiple,
    Slider,
    None,
}
