pub mod app;
pub use crate::app::*;
pub mod gauge;
pub use crate::gauge::*;
pub mod list;
pub use crate::list::*;
pub mod slider;
pub use crate::slider::*;
pub mod submenu;
pub use crate::submenu::*;
pub mod tab;
pub use crate::tab::*;
pub mod table;
pub use crate::table::*;
pub mod toggle;
pub use crate::toggle::*;
pub mod ui;
pub use crate::ui::*;

pub const NX_TAB_ROWS: usize = 1;
pub const NX_TAB_COLUMNS: usize = 15;
pub const NX_SUBMENU_ROWS: usize = 8;
pub const NX_SUBMENU_COLUMNS: usize = 4;

pub trait InputControl {
    fn on_a(&mut self);
    fn on_b(&mut self);
    fn on_x(&mut self);
    fn on_y(&mut self);
    fn on_up(&mut self);
    fn on_down(&mut self);
    fn on_left(&mut self);
    fn on_right(&mut self);
    fn on_start(&mut self);
    fn on_l(&mut self);
    fn on_r(&mut self);
    fn on_zl(&mut self);
    fn on_zr(&mut self);
}
