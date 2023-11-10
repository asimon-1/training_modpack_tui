use serde::ser::{SerializeMap, Serializer};
use serde::Serialize;

use crate::StatefulTable;

#[allow(dead_code)]
// const NX_TUI_WIDTH: u16 = 240;
// const NX_TAB_COLUMNS: usize = 4;
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
    pub tabs: StatefulTable<Tab<'a>, 1, 2>, // Can't be too big otherwise stack overflow TODO!()
    pub page: AppPage,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App {
            tabs: StatefulTable::new(),
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
#[derive(Clone, Copy)]
pub struct Tab<'a> {
    pub title: &'a str,
    pub id: &'a str,
    pub submenus: StatefulTable<SubMenu<'a>, NX_SUBMENU_ROWS, NX_SUBMENU_COLUMNS>,
}

impl<'a> Serialize for Tab<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.submenus.len()))?;
        for submenu in self.submenus {
            map.serialize_entry(&submenu.title, &submenu)?;
        }
        map.end()
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct SubMenu<'a> {
    pub title: &'a str,
    pub id: &'a str,
    pub help_text: &'a str,
    pub type_: SubMenuType,
    pub toggles: StatefulTable<Toggle<'a>, NX_SUBMENU_ROWS, NX_SUBMENU_COLUMNS>,
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