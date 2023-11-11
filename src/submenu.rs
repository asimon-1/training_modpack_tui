use serde::ser::Serializer;
use serde::Serialize;

use crate::{InputControl, Slider, StatefulTable, Toggle};

#[derive(Clone)]
pub struct SubMenu<'a> {
    pub title: &'a str,
    pub id: &'a str,
    pub help_text: &'a str,
    pub submenu_type: SubMenuType,
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

impl<'a> InputControl for SubMenu<'a> {
    fn on_a(&mut self) {
        match self.submenu_type {
            SubMenuType::ToggleSingle => {
                // Set all values to 0 first before incrementing the selected toggle
                // This ensure that exactly one toggle has a nonzero value
                for ind in 0..self.toggles.len() {
                    self.toggles.get_by_idx_mut(ind).unwrap().value = 0;
                }
                self.selected_toggle().increment();
            }
            SubMenuType::ToggleMultiple => self.selected_toggle().increment(),
            SubMenuType::Slider => {}
            SubMenuType::None => {}
        }
    }
    fn on_b(&mut self) {}
    fn on_x(&mut self) {}
    fn on_y(&mut self) {}
    fn on_up(&mut self) {
        match self.submenu_type {
            SubMenuType::ToggleSingle => self.toggles.prev_row_checked(),
            SubMenuType::ToggleMultiple => self.toggles.prev_row_checked(),
            SubMenuType::Slider => {}
            SubMenuType::None => {}
        }
    }
    fn on_down(&mut self) {
        match self.submenu_type {
            SubMenuType::ToggleSingle => self.toggles.next_row_checked(),
            SubMenuType::ToggleMultiple => self.toggles.next_row_checked(),
            SubMenuType::Slider => {}
            SubMenuType::None => {}
        }
    }
    fn on_left(&mut self) {
        match self.submenu_type {
            SubMenuType::ToggleSingle => self.toggles.prev_col_checked(),
            SubMenuType::ToggleMultiple => self.toggles.prev_col_checked(),
            SubMenuType::Slider => {}
            SubMenuType::None => {}
        }
    }
    fn on_right(&mut self) {
        match self.submenu_type {
            SubMenuType::ToggleSingle => self.toggles.next_col_checked(),
            SubMenuType::ToggleMultiple => self.toggles.next_col_checked(),
            SubMenuType::Slider => {}
            SubMenuType::None => {}
        }
    }
    fn on_start(&mut self) {}
    fn on_l(&mut self) {}
    fn on_r(&mut self) {}
    fn on_zl(&mut self) {}
    fn on_zr(&mut self) {}
}

impl<'a> SubMenu<'a> {
    fn selected_toggle(&mut self) -> &mut Toggle<'a> {
        self.toggles.get_selected().expect("No toggle selected!")
    }
}

#[derive(Clone, Copy, Serialize)]
pub enum SubMenuType {
    ToggleSingle,
    ToggleMultiple,
    Slider,
    None,
}
