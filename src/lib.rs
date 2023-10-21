use ratatui::{backend::Backend, layout::Rect, Frame};
use serde::ser::{SerializeSeq, SerializeStruct, Serializer};
use serde::Serialize;

mod gauge;
mod list;
mod table;

pub use crate::gauge::*;
pub use crate::list::*;
pub use crate::table::*;

#[allow(dead_code)]
static NX_TUI_WIDTH: u16 = 240;
static NX_TAB_COLUMNS: usize = 4; // TODO!() I guessed on these, need to verify
static NX_SUBMENU_COLUMNS: usize = 4; // TODO!() I guessed on these, need to verify

#[derive(PartialEq, Serialize)]
pub enum AppPage {
    SUBMENU,
    TOGGLE,
    SLIDER,
    CONFIRMATION,
}

// Menu structure is:
// App <StatefulList<Tab>>
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

pub struct App {
    pub tabs: StatefulList<Tab>,
    pub page: AppPage,
}

impl App {
    pub fn new() -> App {
        App {
            tabs: StatefulList::with_items(Vec::new()),
            page: AppPage::SUBMENU,
        }
    }

    pub fn to_json(self: &App) -> String {
        serde_json::to_string(self).expect("Could not serialize the menu to JSON!")
    }
}

impl Serialize for App {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.tabs.serialize(serializer)
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct Tab {
    title: String,
    id: String,
    submenus: StatefulTable<SubMenu>,
}

impl Serialize for Tab {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.submenus.serialize(serializer)
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct SubMenu {
    title: String,
    id: String,
    help_text: String,
    type_: SubMenuType,
    toggles: StatefulTable<Toggle>,
    slider: Option<Slider>,
}

impl Serialize for SubMenu {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // TODO! Match on SubMenuType and Impl for Slider
        let mut seq = serializer.serialize_seq(Some(self.toggles.len()))?;
        for e in self.toggles.flatten().iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}

#[derive(Clone, Serialize)]
pub enum SubMenuType {
    TOGGLE_SINGLE,
    TOGGLE_MULTIPLE,
    SLIDER,
    NONE,
}

#[allow(unused_variables)]
pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {}

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

#[derive(Clone)]
pub struct Toggle {
    pub toggle_title: String,
    pub toggle_value: u8,
    pub toggle_max: u8,
}

impl Serialize for Toggle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(self.toggle_value)
    }
}

#[derive(Clone, Serialize)]
pub struct Slider {
    pub selected_min: u32,
    pub selected_max: u32,
    pub abs_min: u32,
    pub abs_max: u32,
}

pub fn create_app() -> App {
    let a_button = Toggle {
        toggle_title: "A Button".to_string(),
        toggle_value: 0,
        toggle_max: 1,
    };
    let b_button = Toggle {
        toggle_title: "B Button".to_string(),
        toggle_value: 1,
        toggle_max: 1,
    };

    let mut app = App::new();
    let mut button_tab_submenus: Vec<SubMenu> = Vec::new();
    button_tab_submenus.push(SubMenu {
        title: "Menu Open Start Press".to_string(),
        id: "menu_open_start_press".to_string(),
        help_text: "Help".to_string(),
        type_: SubMenuType::TOGGLE_SINGLE,
        toggles: StatefulTable::with_items(vec![a_button, b_button], NX_SUBMENU_COLUMNS),
        slider: None,
    });
    button_tab_submenus.push(SubMenu {
        title: "Save State Save".to_string(),
        id: "save_state_save".to_string(),
        help_text: "Save State Save: Hold any one button and press the others to trigger"
            .to_string(),
        type_: SubMenuType::TOGGLE_MULTIPLE,
        toggles: StatefulTable::with_items(Vec::new(), NX_SUBMENU_COLUMNS),
        slider: None,
    });
    let mut button_tab = Tab {
        id: "button".to_string(),
        title: "Button Config".to_string(),
        submenus: StatefulTable::with_items(button_tab_submenus, NX_TAB_COLUMNS),
    };
    app.tabs = StatefulList::with_items(vec![button_tab]);
    app
}
