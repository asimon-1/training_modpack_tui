use ratatui::widgets::ListState;
use serde::ser::{Serialize, SerializeSeq, Serializer};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct StatefulList<T: Serialize> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T: Serialize> IntoIterator for StatefulList<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<T: Serialize> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        let mut state = ListState::default();
        // Enforce state as first of list
        state.select(Some(0));
        StatefulList { state, items }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}

impl<T: Serialize> Serialize for StatefulList<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.items.len()))?;
        for e in self.items.iter() {
            seq.serialize_element(e)?;
        }
        seq.end()
    }
}
