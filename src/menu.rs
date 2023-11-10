use itertools::Itertools;
use ratatui::{
    prelude::*,
    widgets::{Paragraph, Widget},
};
use serde::ser::{SerializeMap, Serializer};
use serde::Serialize;

use crate::StatefulTable;

#[allow(dead_code)]
// const NX_TUI_WIDTH: u16 = 240;
pub const NX_TAB_ROWS: usize = 1;
pub const NX_TAB_COLUMNS: usize = 15;
pub const NX_SUBMENU_ROWS: usize = 8;
pub const NX_SUBMENU_COLUMNS: usize = 4;

#[derive(PartialEq, Serialize)]
pub enum AppPage {
    SUBMENU,
    TOGGLE,
    SLIDER,
    CONFIRMATION,
}

// Menu structure is:
// App <StatefulTable<Tab>>
// │
// └─ Tab <StatefulTable<Submenu>>
//    │
//    └─ Submenu <Struct>
//       │
//       ├─ StatefulTable<Toggle>
//       │
//       │  OR
//       │
//       └─ Option<Slider>

pub struct App<'a> {
    pub tabs: StatefulTable<Tab<'a>>,
    pub page: AppPage,
}

impl<'a> App<'a> {
    pub fn new(rows: usize, cols: usize) -> App<'a> {
        App {
            tabs: StatefulTable::new(rows, cols),
            page: AppPage::SUBMENU,
        }
    }

    pub fn to_json(self: &App<'a>) -> String {
        serde_json::to_string(&self).expect("Could not serialize the menu to JSON!")
    }
}

impl<'a> Serialize for App<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.tabs.serialize(serializer)
    }
}

#[allow(dead_code)]
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

impl<'a> Widget for Tab<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
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

#[allow(dead_code)]
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
    fn render(self, area: Rect, buf: &mut Buffer) {
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

#[derive(Clone, Copy, Serialize)]
pub enum SubMenuType {
    ToggleSingle,
    ToggleMultiple,
    Slider,
    None,
}

#[derive(Clone, Copy)]
pub struct Toggle<'a> {
    pub toggle_title: &'a str,
    pub toggle_value: u8,
    pub toggle_max: u8,
}

impl Serialize for Toggle<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(self.toggle_value)
    }
}

#[derive(Clone, Copy, Serialize)]
pub struct Slider {
    pub selected_min: u32,
    pub selected_max: u32,
    pub abs_min: u32,
    pub abs_max: u32,
}
