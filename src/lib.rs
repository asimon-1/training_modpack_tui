use itertools::Itertools;
use ratatui::{backend::Backend, layout::Rect, prelude::*, widgets::Paragraph, Frame};
use serde::ser::{SerializeMap, Serializer};
use serde::Serialize;

mod gauge;
mod list;
mod table;

pub use crate::gauge::*;
pub use crate::list::*;
pub use crate::table::*;

#[allow(dead_code)]
// const NX_TUI_WIDTH: u16 = 240;
// const NX_TAB_COLUMNS: usize = 4;
const NX_SUBMENU_ROWS: usize = 8;
const NX_SUBMENU_COLUMNS: usize = 4;

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
    pub tabs: StatefulTable<Tab<'a>, 1, 10>, // Note that if you go to 11 there's a stack overflow...
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
    title: &'a str,
    id: &'a str,
    submenus: StatefulTable<SubMenu<'a>, 1, 15>,
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
    title: &'a str,
    id: &'a str,
    help_text: &'a str,
    type_: SubMenuType,
    toggles: StatefulTable<Toggle<'a>, NX_SUBMENU_ROWS, NX_SUBMENU_COLUMNS>,
    slider: Option<Slider>,
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

#[allow(unused_variables)]
pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(0)])
        .split(f.size());
    let tab_area = layout[0];
    let menu_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3); 5]) // TODO
        .split(layout[1])
        .iter()
        .map(|&area| {
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(25); 4]) // TODO
                .split(area)
                .to_vec()
        })
        .collect_vec();
    f.render_widget(Paragraph::new("Title!"), tab_area);
}

#[allow(dead_code, unused_variables)]
fn render_submenu_page<B: Backend>(
    _f: &mut Frame<B>,
    app: &mut App,
    list_chunks: Vec<Rect>,
    help_chunk: Rect,
) {
}

#[allow(dead_code, unused_variables)]
fn render_slider_page<B: Backend>(
    f: &mut Frame<B>,
    app: &mut App,
    vertical_chunk: Rect,
    help_chunk: Rect,
) {
}

#[allow(dead_code, unused_variables)]
fn render_toggle_page<B: Backend>(
    f: &mut Frame<B>,
    app: &mut App,
    list_chunks: Vec<Rect>,
    help_chunk: Rect,
) {
}

/////////////////////////// TODO!() remove stuff below this line

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

    let mut app = App::new();
    let mut button_tab_submenus: Vec<SubMenu> = Vec::new();
    button_tab_submenus.push(SubMenu {
        title: "Menu Open Start Press",
        id: "menu_open_start_press",
        help_text: "Help",
        type_: SubMenuType::ToggleSingle,
        toggles: StatefulTable::with_items(vec![a_button, b_button]),
        slider: None,
    });
    button_tab_submenus.push(SubMenu {
        title: "Save State Save",
        id: "save_state_save",
        help_text: "Save State Save: Hold any one button and press the others to trigger",
        type_: SubMenuType::ToggleMultiple,
        toggles: StatefulTable::with_items(Vec::new()),
        slider: None,
    });

    button_tab_submenus.push(SubMenu {
        title: "Menu Open Start Press",
        id: "menu_open_start_press",
        help_text: "Menu Open Start Press: Hold start or press minus to open the mod menu. To open the original menu, press start.\nThe default menu open option is always available as Hold DPad Up + Press B.",
        type_: SubMenuType::ToggleSingle,
        toggles: StatefulTable::with_items(Vec::new()),
        slider: None,
    });
    button_tab_submenus.push(SubMenu {
        title: "Save State Save",
        id: "save_state_save",
        help_text: "Save State Save: Hold any one button and press the others to trigger",
        type_: SubMenuType::ToggleMultiple,
        toggles: StatefulTable::with_items(Vec::new()),
        slider: None,
    });
    button_tab_submenus.push(SubMenu {
        title: "Save State Load",
        id: "save_state_load",
        help_text: "Save State Load: Hold any one button and press the others to trigger",
        type_: SubMenuType::ToggleMultiple,
        toggles: StatefulTable::with_items(Vec::new()),
        slider: None,
    });
    button_tab_submenus.push(SubMenu {
        title: "Input Record",
        id: "input_record",
        help_text: "Input Record: Hold any one button and press the others to trigger",
        type_: SubMenuType::ToggleMultiple,
        toggles: StatefulTable::with_items(Vec::new()),
        slider: None,
    });
    button_tab_submenus.push(SubMenu {
        title: "Input Playback",
        id: "input_playback",
        help_text: "Input Playback: Hold any one button and press the others to trigger",
        type_: SubMenuType::ToggleMultiple,
        toggles: StatefulTable::with_items(Vec::new()),
        slider: None,
    });

    let button_tab = Tab {
        id: "button",
        title: "Button Config",
        submenus: StatefulTable::with_items(button_tab_submenus),
    };
    app.tabs = StatefulTable::with_items(vec![button_tab]);
    app
}
