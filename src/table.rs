use itertools::Itertools;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    prelude::*,
    widgets::{Paragraph, Widget},
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Allows a snake-filled table of arbitrary size
/// The final row does not need to be filled
/// [ a , b , c , d ]
/// [ e, f, g, h, i ]
/// [ j, k          ]
#[derive(Debug, Eq, PartialEq, Clone, Copy, Deserialize)]
pub struct TableState {
    pub row: usize,
    pub column: usize,
}

impl TableState {
    pub fn new() -> TableState {
        TableState { row: 0, column: 0 }
    }
    pub fn new_with(row: usize, column: usize) -> TableState {
        TableState { row, column }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct StatefulTable<T: Copy + Clone + Serialize, const ROWS: usize, const COLS: usize> {
    pub state: TableState,
    pub items: [[Option<T>; COLS]; ROWS],
}

// Size-related functions
impl<T: Copy + Clone + Serialize, const ROWS: usize, const COLS: usize>
    StatefulTable<T, ROWS, COLS>
{
    pub fn rows(&self) -> usize {
        ROWS
    }
    pub fn cols(&self) -> usize {
        COLS
    }
    pub fn len(&self) -> usize {
        self.items
            .iter()
            .map(|row| {
                row.iter()
                    .map(|item| if item.is_some() { 1 } else { 0 })
                    .sum::<usize>()
            })
            .sum()
    }
    pub fn full_len(&self) -> usize {
        ROWS * COLS
    }
    pub fn as_vec(&self) -> Vec<T> {
        let mut v = Vec::new();
        for row in self.items.iter() {
            for item in row.iter() {
                if let Some(i) = item {
                    v.push(*i);
                }
            }
        }
        v
    }
}

// Associated Functions
impl<T: Copy + Clone + Serialize, const ROWS: usize, const COLS: usize>
    StatefulTable<T, ROWS, COLS>
{
    pub fn new() -> Self {
        Self {
            state: TableState::new(),
            items: [[None; COLS]; ROWS],
        }
    }
    pub fn with_items(v: Vec<T>) -> Self {
        let mut table: Self = Self::new();
        if v.len() > ROWS * COLS {
            panic!(
                "Cannot create StatefulTable; too many items for size {}x{}: {}",
                ROWS,
                COLS,
                v.len()
            );
        } else {
            for (i, item) in v.iter().enumerate() {
                table.items[i.div_euclid(COLS)][i.rem_euclid(COLS)] = Some(*item);
            }
            table
        }
    }
}

// State Functions
impl<T: Copy + Clone + Serialize, const ROWS: usize, const COLS: usize>
    StatefulTable<T, ROWS, COLS>
{
    pub fn select(&mut self, row: usize, column: usize) {
        assert!(column < COLS);
        assert!(row < ROWS);
        self.state = TableState { row, column };
    }

    pub fn get_selected(&self) -> Option<&T> {
        self.items[self.state.row][self.state.column].as_ref()
    }

    pub fn get(&self, row: usize, column: usize) -> Option<&T> {
        if row >= ROWS || column >= COLS {
            None
        } else {
            self.items[row][column].as_ref()
        }
    }

    pub fn get_by_idx(&self, idx: usize) -> Option<&T> {
        let row = idx.div_euclid(COLS);
        let col = idx.rem_euclid(COLS);
        self.get(row, col)
    }

    pub fn next_row(&mut self) {
        if self.state.row == ROWS - 1 {
            // Wrap around
            self.state.row = 0;
        } else {
            self.state.row += 1;
        }
    }

    pub fn next_row_checked(&mut self) {
        self.next_row();
        if self.get_selected().is_none() {
            self.next_row_checked();
        }
    }

    pub fn prev_row(&mut self) {
        if self.state.row == 0 {
            self.state.row = ROWS - 1;
        } else {
            self.state.row += 1;
        }
        if self.state.row >= ROWS {
            self.state.row -= ROWS;
        }
    }

    pub fn prev_row_checked(&mut self) {
        self.prev_row();
        if self.get_selected().is_none() {
            self.prev_row_checked();
        }
    }

    pub fn next_col(&mut self) {
        if self.state.column == COLS - 1 {
            self.state.column = 0;
        } else {
            self.state.column += 1;
        }
    }

    pub fn next_col_checked(&mut self) {
        self.next_col();
        if self.get_selected().is_none() {
            self.state.column = 0;
        }
    }

    pub fn prev_col(&mut self) {
        if self.state.column == 0 {
            self.state.column = COLS - 1;
        } else {
            self.state.column -= 1;
        }
    }

    pub fn prev_col_checked(&mut self) {
        self.prev_col();
        self.carriage_return();
    }

    /// If the selected cell is None, move selection to the left until you get Some.
    /// No-op if the selected cell is Some.
    /// For example, a 2x3 table with 4 elements would shift the selection from 1,2 to 1,0
    ///
    /// [ a ,  b ,  c ]
    /// [ d ,  e , [ ]]
    ///
    ///        |
    ///        V
    ///
    /// [ a ,  b ,  c ]
    /// [[d],    ,    ]
    pub fn carriage_return(&mut self) {
        assert!(
            self.items[self.state.row].iter().any(|x| x.is_some()),
            "Carriage return called on an empty row!"
        );
        if self.get_selected().is_none() {
            self.prev_col();
            self.carriage_return();
        }
    }
}

impl<T: Copy + Clone + Serialize, const ROWS: usize, const COLS: usize> Serialize
    for StatefulTable<T, ROWS, COLS>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let flat: Vec<T> = self.as_vec();
        flat.serialize(serializer)
    }
}

impl<'de, T: Copy + Clone + Serialize + Deserialize<'de>, const ROWS: usize, const COLS: usize>
    Deserialize<'de> for StatefulTable<T, ROWS, COLS>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let items: Vec<T> = Vec::deserialize(deserializer)?;
        Ok(StatefulTable::with_items(items))
    }
}

impl<T: Copy + Clone + Serialize, const ROWS: usize, const COLS: usize> IntoIterator
    for StatefulTable<T, ROWS, COLS>
{
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.as_vec().into_iter()
    }
}

impl<T: Copy + Clone + Serialize, const ROWS: usize, const COLS: usize> Widget
    for StatefulTable<T, ROWS, COLS>
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        // TODO!()
        let grid = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3); ROWS])
            .split(area)
            .iter()
            .map(|&area| {
                Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Ratio(1, COLS as u32); COLS])
                    .split(area)
                    .to_vec()
            })
            .collect_vec();
        for (x, row) in grid.iter().enumerate() {
            for (y, rect) in row.iter().enumerate() {
                let item_opt = self.get(x, y);
                if let Some(item) = item_opt {
                    Paragraph::new("Some").render(*rect, buf);
                } else {
                    Paragraph::new("None").render(*rect, buf);
                }
            }

        }
    }
}
