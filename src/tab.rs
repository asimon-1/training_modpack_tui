use itertools::Itertools;
use ratatui::{
    prelude::*,
    widgets::{Paragraph, Widget},
};
use serde::ser::{SerializeMap, Serializer};
use serde::Serialize;

use crate::{InputControl, StatefulTable, SubMenu};

#[derive(Clone)]
pub struct Tab<'a> {
    pub title: &'a str,
    pub id: &'a str,
    pub submenus: StatefulTable<SubMenu<'a>>,
}

impl<'a> Serialize for Tab<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.submenus.len()))?;
        for submenu in self.submenus.as_vec().iter() {
            map.serialize_entry(&submenu.title, &submenu)?;
        }
        map.end()
    }
}

impl<'a> InputControl for Tab<'a> {
    fn on_a(&mut self) {}
    fn on_b(&mut self) {}
    fn on_x(&mut self) {}
    fn on_y(&mut self) {}
    fn on_up(&mut self) {
        self.submenus.prev_row_checked()
    }
    fn on_down(&mut self) {
        self.submenus.next_row_checked()
    }
    fn on_left(&mut self) {
        self.submenus.prev_col_checked()
    }
    fn on_right(&mut self) {
        self.submenus.next_col_checked()
    }
    fn on_start(&mut self) {}
    fn on_l(&mut self) {}
    fn on_r(&mut self) {}
    fn on_zl(&mut self) {}
    fn on_zr(&mut self) {}
}

impl<'a> Widget for Tab<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        let grid = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(3); self.submenus.rows])
            .split(area)
            .iter()
            .map(|&area| {
                Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(vec![
                        Constraint::Ratio(1, self.submenus.cols as u32);
                        self.submenus.cols
                    ])
                    .split(area)
                    .to_vec()
            })
            .collect_vec();
        for (x, row) in grid.iter().enumerate() {
            for (y, rect) in row.iter().enumerate() {
                let item_opt = self.submenus.get(x, y);
                if let Some(item) = item_opt {
                    Paragraph::new(item.title).render(*rect, buf);
                } else {
                    Paragraph::new("").render(*rect, buf);
                }
            }
        }
    }
}
