use serde::{Deserialize, Serialize, Serializer};

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

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct StatefulTable<T: Clone + Serialize> {
    pub state: TableState,
    pub items: Vec<Vec<Option<T>>>,
    pub rows: usize,
    pub cols: usize,
}

// Size-related functions
impl<T: Clone + Serialize>
    StatefulTable<T>
{
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
        self.rows * self.cols
    }
    pub fn as_vec(&self) -> Vec<T> {
        let mut v = Vec::with_capacity(self.len());
        for row in self.items.iter() {
            for item in row.iter() {
                if let Some(i) = item {
                    v.push(i.clone());
                }
            }
        }
        v
    }
}

// Associated Functions
impl<T: Clone + Serialize>
    StatefulTable<T>
{
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            state: TableState::new(),
            items: vec![vec![None; cols]; rows],
            rows: rows,
            cols: cols,
        }
    }
    pub fn with_items(rows: usize, cols: usize, v: Vec<T>) -> Self {
        let mut table: Self = Self::new(rows, cols);
        if v.len() > rows * cols {
            panic!(
                "Cannot create StatefulTable; too many items for size {}x{}: {}",
                rows,
                cols,
                v.len()
            );
        } else {
            for (i, item) in v.iter().enumerate() {
                table.items[i.div_euclid(cols)][i.rem_euclid(cols)] = Some(item.clone());
            }
            table
        }
    }
}

// State Functions
impl<T: Clone + Serialize>
    StatefulTable<T>
{
    pub fn select(&mut self, row: usize, column: usize) {
        assert!(column < self.cols);
        assert!(row < self.rows);
        self.state = TableState { row, column };
    }

    pub fn get_selected(&self) -> Option<&T> {
        self.items[self.state.row][self.state.column].as_ref()
    }

    pub fn get(&self, row: usize, column: usize) -> Option<&T> {
        if row >= self.rows || column >= self.cols {
            None
        } else {
            self.items[row][column].as_ref()
        }
    }

    pub fn get_by_idx(&self, idx: usize) -> Option<&T> {
        let row = idx.div_euclid(self.cols);
        let col = idx.rem_euclid(self.cols);
        self.get(row, col)
    }

    pub fn next_row(&mut self) {
        if self.state.row == self.rows - 1 {
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
            self.state.row = self.rows - 1;
        } else {
            self.state.row += 1;
        }
        if self.state.row >= self.rows {
            self.state.row -= self.rows;
        }
    }

    pub fn prev_row_checked(&mut self) {
        self.prev_row();
        if self.get_selected().is_none() {
            self.prev_row_checked();
        }
    }

    pub fn next_col(&mut self) {
        if self.state.column == self.cols - 1 {
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
            self.state.column = self.cols - 1;
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

impl<T: Clone + Serialize> Serialize
    for StatefulTable<T>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let flat: Vec<T> = self.as_vec();
        flat.serialize(serializer)
    }
}

// impl<'de, T: Clone + Serialize + Deserialize<'de>>
//     Deserialize<'de> for StatefulTable<T>
// {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let items: Vec<T> = Vec::deserialize(deserializer)?;
//         Ok(StatefulTable::with_items(rows????, cols????, items))
//     }
// }

impl<T: Clone + Serialize> IntoIterator
    for StatefulTable<T>
{
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.as_vec().into_iter()
    }
}
