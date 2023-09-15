use itertools::Itertools;
use serde::ser::{Serialize, Serializer, SerializeSeq};

/// Allows a snake-filled table of arbitrary size
/// The final row does not need to be filled
/// [ a , b , c , d ]
/// [ e, f, g, h, i ]
/// [ j, k          ]
#[derive(Debug, Eq, PartialEq, Clone)]
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
pub struct TableSize {
    pub rows: usize,
    pub columns: usize,
}

impl TableSize {
    pub fn new() -> TableSize {
        TableSize {
            rows: 0,
            columns: 0,
        }
    }
    pub fn new_with(rows: usize, columns: usize) -> TableSize {
        TableSize { rows, columns }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct StatefulTable<T: Clone + Serialize> {
    pub state: TableState,
    pub rows: Vec<Vec<Option<T>>>,
    pub size: TableSize,
}

impl<T: Clone + Serialize> IntoIterator for StatefulTable<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.flatten().into_iter()
    }
}

impl<T: Clone + Serialize> StatefulTable<T> {
    /// Turn a vec like [a, b, c] into a table like
    /// [a, b]
    /// [c,  ]
    pub fn with_items(i: Vec<T>, columns: usize) -> StatefulTable<T> {
        let rows: Vec<Vec<Option<T>>> = i
            .into_iter()
            .chunks(columns)
            .into_iter()
            .map(|chunk| chunk.into_iter().map(|y| Some(y)).collect_vec())
            .map(|mut row| {
                // Fill the final row with None's until its len is `columns`
                for _ in 0..(columns - row.len()) {
                    row.push(None);
                }
                row
            })
            .collect_vec();
        let size = TableSize {
            rows: rows.len(),
            columns: columns,
        };
        StatefulTable {
            state: TableState::new(),
            rows: rows,
            size: size,
        }
    }
    pub fn select(&mut self, row: usize, column: usize) {
        assert!(column < self.size.columns);
        assert!(row < self.size.rows);
        self.state = TableState { row, column };
    }

    pub fn get_selected(&self) -> Option<&T> {
        self.rows[self.state.row][self.state.column].as_ref()
    }

    pub fn get(&self, row: usize, column: usize) -> Option<&T> {
        if row >= self.size.rows || column >= self.size.columns {
            None
        } else {
            self.rows[row][column].as_ref()
        }
    }

    pub fn get_by_idx(&self, idx: usize) -> Option<&T> {
        let row = idx.div_euclid(self.size.columns);
        let col = idx.rem_euclid(self.size.columns);
        self.get(row, col)
    }

    pub fn next_row(&mut self) {
        if self.state.row == self.size.rows - 1 {
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
            self.state.row = self.size.rows - 1;
        } else {
            self.state.row += 1;
        }
        if self.state.row >= self.size.rows {
            self.state.row -= self.size.rows;
        }
    }

    pub fn prev_row_checked(&mut self) {
        self.prev_row();
        if self.get_selected().is_none() {
            self.prev_row_checked();
        }
    }

    pub fn next_col(&mut self) {
        if self.state.column == self.size.columns - 1 {
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
            self.state.column = self.size.columns - 1;
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
            self.rows[self.state.row].iter().any(|x| x.is_some()),
            "Carriage return called on an empty row!"
        );
        if self.get_selected().is_none() {
            self.prev_col();
            self.carriage_return();
        }
    }

    pub fn flatten(&self) -> Vec<T> {
        let mut ret: Vec<T> = Vec::new();
        for row in 0..self.size.rows {
            for column in 0..self.size.columns {
                if let Some(x) = self.get(row, column) {
                    ret.push(x.clone())
                }
            }
        }
        ret
    }
}

impl<T: Clone + Serialize> Serialize for StatefulTable<T> {
    fn serialize<S>(&self, serializer:S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let flat = self.flatten();
        let mut seq = serializer.serialize_seq(Some(flat.len()))?;
        for e in flat.iter() {
            seq.serialize_element(e)?;
        }
        seq.end()
    }
}