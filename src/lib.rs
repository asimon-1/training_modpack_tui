use ratatui::{backend::Backend, layout::Rect, Frame};
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
//       └─ Option<Slider>

#[derive(Serialize)]
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

    pub fn get_menu_selections(self: &App) -> String {
        let mut ret = serde_json::to_string(self).unwrap();
        // for tab in self.tabs.clone().into_iter() {
        //     for submenu in tab.submenus.into_iter() {
        //         ret += &submenu.id;
        //     }
        // }
        ret
    }
}

#[allow(dead_code)]
#[derive(Clone, Serialize)]
pub struct Tab {
    title: String,
    id: String,
    submenus: StatefulTable<SubMenu>,
}

#[allow(dead_code)]
#[derive(Clone, Serialize)]
pub struct SubMenu {
    title: String,
    id: String,
    help_text: String,
    is_single_option: bool,
    type_: SubMenuType,
    toggles: StatefulTable<Toggle>,
    slider: Option<Slider>,
}

#[derive(Clone, Serialize)]
pub enum SubMenuType {
    TOGGLE,
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

#[derive(Clone, Serialize)]
pub struct Toggle {
    pub toggle_title: String,
    pub toggle_value: u8,
    pub toggle_max: u8,
}

#[derive(Clone, Serialize)]
pub struct Slider {
    pub selected_min: u32,
    pub selected_max: u32,
    pub abs_min: u32,
    pub abs_max: u32,
}

pub fn create_app() -> App {
    let mut app = App::new();
    let mut button_tab_submenus: Vec<SubMenu> = Vec::new();
    button_tab_submenus.push(SubMenu {
        title: "Menu Open Start Press".to_string(),
        id: "menu_open_start_press".to_string(),
        help_text: "Help".to_string(),
        is_single_option: true,
        type_: SubMenuType::TOGGLE,
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
