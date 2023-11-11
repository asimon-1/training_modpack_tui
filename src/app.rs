use serde::ser::Serializer;
use serde::Serialize;

use crate::{InputControl, StatefulTable, Tab};

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

impl<'a> InputControl for App<'a> {
    fn on_a(&mut self) {
        match self.page {
            AppPage::SUBMENU => self.tabs.get_selected().expect("No tab selected!").on_a(),
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
        }
    }
    fn on_b(&mut self) {
        match self.page {
            AppPage::SUBMENU => self.tabs.get_selected().expect("No tab selected!").on_b(),
            AppPage::TOGGLE => self
                .tabs
                .get_selected()
                .expect("No tab selected!")
                .submenus
                .get_selected()
                .expect("No submenu selected!")
                .on_b(),
            AppPage::SLIDER => self
                .tabs
                .get_selected()
                .expect("No tab selected!")
                .submenus
                .get_selected()
                .expect("No submenu selected!")
                .on_b(),
            AppPage::CONFIRMATION => {}
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
        }
    }
    fn on_zl(&mut self) {
        match self.page {
            AppPage::SUBMENU => self.tabs.get_selected().expect("No tab selected!").on_zl(),
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
        }
    }
    fn on_zr(&mut self) {
        match self.page {
            AppPage::SUBMENU => self.tabs.get_selected().expect("No tab selected!").on_zr(),
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
        }
    }
}
