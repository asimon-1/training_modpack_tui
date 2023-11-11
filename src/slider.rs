use serde::Serialize;

#[derive(Clone, Copy, Serialize)]
pub struct Slider {
    pub selected_min: u32,
    pub selected_max: u32,
    pub abs_min: u32,
    pub abs_max: u32,
}
