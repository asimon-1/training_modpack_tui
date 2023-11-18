use serde::ser::{SerializeMap, Serializer};
use serde::Serialize;
use std::collections::HashMap;

use crate::{InputControl, StatefulList, SubMenu, SubMenuType, Tab};

#[derive(PartialEq, Serialize)]
pub enum AppPage {
    SUBMENU,
    TOGGLE,
    SLIDER,
    CONFIRMATION,
    CLOSE,
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
    pub tabs: StatefulList<Tab<'a>>,
    pub page: AppPage,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App {
            tabs: StatefulList::new(),
            page: AppPage::SUBMENU,
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).expect("Could not serialize the menu to JSON!")
    }

    pub fn update_from_json(&mut self, json: &str) {
        let all_settings: HashMap<String, Vec<u8>> =
            serde_json::from_str(json).expect("Could not parse the json!");
        for tab in self.tabs.iter_mut() {
            for idx in 0..tab.submenus.len() {
                // I don't like iterating by index here but implementation of iter_mut() is tough for StatefulTable
                let submenu_opt = tab.submenus.get_by_idx_mut(idx);
                if let Some(submenu) = submenu_opt {
                    if let Some(val) = all_settings.get(submenu.title) {
                        submenu.update_from_vec(val.clone());
                    }
                }
            }
        }
    }

    pub fn selected_tab(&mut self) -> &mut Tab<'a> {
        self.tabs.get_selected().expect("No tab selected!")
    }

    pub fn selected_submenu(&mut self) -> &mut SubMenu<'a> {
        self.selected_tab()
            .submenus
            .get_selected()
            .expect("No submenu selected!")
    }
}

impl<'a> Serialize for App<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serializes as a mapping between submenu titles and values
        // Need to iterate through tabs to avoid making a list of mappings
        let len: usize = self.tabs.iter().map(|tab| tab.len()).sum();
        let mut map = serializer.serialize_map(Some(len))?;
        for tab in self.tabs.iter() {
            for submenu in tab.submenus.iter() {
                map.serialize_entry(submenu.title, submenu)?;
            }
        }
        map.end()
    }
}

impl<'a> InputControl for App<'a> {
    fn on_a(&mut self) {
        match self.page {
            AppPage::SUBMENU => {
                let tab = self.tabs.get_selected().expect("No tab selected!");
                let submenu_type = tab
                    .submenus
                    .get_selected()
                    .expect("No submenu selected!")
                    .submenu_type;
                self.page = match submenu_type {
                    SubMenuType::ToggleSingle => AppPage::TOGGLE,
                    SubMenuType::ToggleMultiple => AppPage::TOGGLE,
                    SubMenuType::Slider => AppPage::SLIDER,
                    SubMenuType::None => AppPage::SUBMENU,
                };
                tab.on_a()
            }
            AppPage::TOGGLE => self
                .tabs
                .get_selected()
                .expect("No tab selected!")
                .submenus
                .get_selected()
                .expect("No submenu selected!")
                .on_a(),
            AppPage::SLIDER => self
                .tabs
                .get_selected()
                .expect("No tab selected!")
                .submenus
                .get_selected()
                .expect("No submenu selected!")
                .on_a(),
            AppPage::CONFIRMATION => {}
            AppPage::CLOSE => {}
        }
    }
    fn on_b(&mut self) {
        match self.page {
            AppPage::SUBMENU => {
                // Exit the app
                self.page = AppPage::CLOSE;
            }
            AppPage::TOGGLE => {
                // Return to the list of submenus
                self.page = AppPage::SUBMENU;
            }
            AppPage::SLIDER => {
                // Return to the list of submenus if we don't have a slider handle selected
                let slider = self
                    .selected_submenu()
                    .slider
                    .as_mut()
                    .expect("No slider selected!");
                if !slider.is_handle_selected() {
                    self.page = AppPage::SUBMENU;
                } else {
                    self.selected_submenu().on_b();
                }
            }
            AppPage::CONFIRMATION => {
                // Return to the list of submenus
                self.page = AppPage::SUBMENU;
            }
            AppPage::CLOSE => {}
        }
    }
    fn on_x(&mut self) {
        match self.page {
            AppPage::SUBMENU => self.tabs.get_selected().expect("No tab selected!").on_x(),
            AppPage::TOGGLE => self
                .tabs
                .get_selected()
                .expect("No tab selected!")
                .submenus
                .get_selected()
                .expect("No submenu selected!")
                .on_x(),
            AppPage::SLIDER => self
                .tabs
                .get_selected()
                .expect("No tab selected!")
                .submenus
                .get_selected()
                .expect("No submenu selected!")
                .on_x(),
            AppPage::CONFIRMATION => {}
            AppPage::CLOSE => {}
        }
    }
    fn on_y(&mut self) {
        match self.page {
            AppPage::SUBMENU => self.tabs.get_selected().expect("No tab selected!").on_y(),
            AppPage::TOGGLE => self
                .tabs
                .get_selected()
                .expect("No tab selected!")
                .submenus
                .get_selected()
                .expect("No submenu selected!")
                .on_y(),
            AppPage::SLIDER => self
                .tabs
                .get_selected()
                .expect("No tab selected!")
                .submenus
                .get_selected()
                .expect("No submenu selected!")
                .on_y(),
            AppPage::CONFIRMATION => {}
            AppPage::CLOSE => {}
        }
    }
    fn on_up(&mut self) {
        match self.page {
            AppPage::SUBMENU => self.tabs.get_selected().expect("No tab selected!").on_up(),
            AppPage::TOGGLE => self
                .tabs
                .get_selected()
                .expect("No tab selected!")
                .submenus
                .get_selected()
                .expect("No submenu selected!")
                .on_up(),
            AppPage::SLIDER => self
                .tabs
                .get_selected()
                .expect("No tab selected!")
                .submenus
                .get_selected()
                .expect("No submenu selected!")
                .on_up(),
            AppPage::CONFIRMATION => {}
            AppPage::CLOSE => {}
        }
    }
    fn on_down(&mut self) {
        match self.page {
            AppPage::SUBMENU => self
                .tabs
                .get_selected()
                .expect("No tab selected!")
                .on_down(),
            AppPage::TOGGLE => self
                .tabs
                .get_selected()
                .expect("No tab selected!")
                .submenus
                .get_selected()
                .expect("No submenu selected!")
                .on_down(),
            AppPage::SLIDER => self
                .tabs
                .get_selected()
                .expect("No tab selected!")
                .submenus
                .get_selected()
                .expect("No submenu selected!")
                .on_down(),
            AppPage::CONFIRMATION => {}
            AppPage::CLOSE => {}
        }
    }
    fn on_left(&mut self) {
        match self.page {
            AppPage::SUBMENU => self
                .tabs
                .get_selected()
                .expect("No tab selected!")
                .on_left(),
            AppPage::TOGGLE => self
                .tabs
                .get_selected()
                .expect("No tab selected!")
                .submenus
                .get_selected()
                .expect("No submenu selected!")
                .on_left(),
            AppPage::SLIDER => self
                .tabs
                .get_selected()
                .expect("No tab selected!")
                .submenus
                .get_selected()
                .expect("No submenu selected!")
                .on_left(),
            AppPage::CONFIRMATION => {}
            AppPage::CLOSE => {}
        }
    }
    fn on_right(&mut self) {
        match self.page {
            AppPage::SUBMENU => self
                .tabs
                .get_selected()
                .expect("No tab selected!")
                .on_right(),
            AppPage::TOGGLE => self
                .tabs
                .get_selected()
                .expect("No tab selected!")
                .submenus
                .get_selected()
                .expect("No submenu selected!")
                .on_right(),
            AppPage::SLIDER => self
                .tabs
                .get_selected()
                .expect("No tab selected!")
                .submenus
                .get_selected()
                .expect("No submenu selected!")
                .on_right(),
            AppPage::CONFIRMATION => {}
            AppPage::CLOSE => {}
        }
    }
    fn on_start(&mut self) {
        match self.page {
            AppPage::SUBMENU => self
                .tabs
                .get_selected()
                .expect("No tab selected!")
                .on_start(),
            AppPage::TOGGLE => self
                .tabs
                .get_selected()
                .expect("No tab selected!")
                .submenus
                .get_selected()
                .expect("No submenu selected!")
                .on_start(),
            AppPage::SLIDER => self
                .tabs
                .get_selected()
                .expect("No tab selected!")
                .submenus
                .get_selected()
                .expect("No submenu selected!")
                .on_start(),
            AppPage::CONFIRMATION => {}
            AppPage::CLOSE => {}
        }
    }
    fn on_l(&mut self) {
        match self.page {
            AppPage::SUBMENU => self.tabs.get_selected().expect("No tab selected!").on_l(),
            AppPage::TOGGLE => self
                .tabs
                .get_selected()
                .expect("No tab selected!")
                .submenus
                .get_selected()
                .expect("No submenu selected!")
                .on_l(),
            AppPage::SLIDER => self
                .tabs
                .get_selected()
                .expect("No tab selected!")
                .submenus
                .get_selected()
                .expect("No submenu selected!")
                .on_l(),
            AppPage::CONFIRMATION => {}
            AppPage::CLOSE => {}
        }
    }
    fn on_r(&mut self) {
        match self.page {
            AppPage::SUBMENU => self.tabs.get_selected().expect("No tab selected!").on_r(),
            AppPage::TOGGLE => self
                .tabs
                .get_selected()
                .expect("No tab selected!")
                .submenus
                .get_selected()
                .expect("No submenu selected!")
                .on_r(),
            AppPage::SLIDER => self
                .tabs
                .get_selected()
                .expect("No tab selected!")
                .submenus
                .get_selected()
                .expect("No submenu selected!")
                .on_r(),
            AppPage::CONFIRMATION => {}
            AppPage::CLOSE => {}
        }
    }
    fn on_zl(&mut self) {
        match self.page {
            AppPage::SUBMENU => {
                self.tabs.previous();
            }
            AppPage::TOGGLE => self
                .tabs
                .get_selected()
                .expect("No tab selected!")
                .submenus
                .get_selected()
                .expect("No submenu selected!")
                .on_zl(),
            AppPage::SLIDER => self
                .tabs
                .get_selected()
                .expect("No tab selected!")
                .submenus
                .get_selected()
                .expect("No submenu selected!")
                .on_zl(),
            AppPage::CONFIRMATION => {}
            AppPage::CLOSE => {}
        }
    }
    fn on_zr(&mut self) {
        match self.page {
            AppPage::SUBMENU => self.tabs.next(),
            AppPage::TOGGLE => self
                .tabs
                .get_selected()
                .expect("No tab selected!")
                .submenus
                .get_selected()
                .expect("No submenu selected!")
                .on_zr(),
            AppPage::SLIDER => self
                .tabs
                .get_selected()
                .expect("No tab selected!")
                .submenus
                .get_selected()
                .expect("No submenu selected!")
                .on_zr(),
            AppPage::CONFIRMATION => {}
            AppPage::CLOSE => {}
        }
    }
}
