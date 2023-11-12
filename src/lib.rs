mod containers;
pub use containers::*;
mod structures;
pub use structures::*;

pub mod ui;
pub use crate::ui::*;

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
