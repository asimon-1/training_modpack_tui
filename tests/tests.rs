use training_mod_tui_2::{StatefulTable, TableState};

fn initialize_table(row: usize, col: usize) -> StatefulTable<u8> {
    let mut s = StatefulTable::with_items(2, 3, vec![0, 1, 2, 3, 4]);
    s.select(row, col);
    s
}

#[test]
fn test_next_col_full() {
    let mut t = initialize_table(0, 0);
    assert_eq!(t.get_selected(), Some(0).as_ref());
    t.next_col();
    assert_eq!(t.get_selected(), Some(1).as_ref());
    t.next_col();
    assert_eq!(t.get_selected(), Some(2).as_ref());
    t.next_col();
    assert_eq!(t.get_selected(), Some(0).as_ref());
}

#[test]
fn test_next_col_checked_full() {
    let mut t = initialize_table(0, 0);
    assert_eq!(t.get_selected(), Some(0).as_ref());
    t.next_col_checked();
    assert_eq!(t.get_selected(), Some(1).as_ref());
    t.next_col_checked();
    assert_eq!(t.get_selected(), Some(2).as_ref());
    t.next_col_checked();
    assert_eq!(t.get_selected(), Some(0).as_ref());
}

#[test]
fn test_prev_col_full() {
    let mut t = initialize_table(0, 0);
    assert_eq!(t.get_selected(), Some(0).as_ref());
    t.prev_col();
    assert_eq!(t.get_selected(), Some(2).as_ref());
    t.prev_col();
    assert_eq!(t.get_selected(), Some(1).as_ref());
    t.prev_col();
    assert_eq!(t.get_selected(), Some(0).as_ref());
}

#[test]
fn test_prev_col_checked_full() {
    let mut t = initialize_table(0, 0);
    assert_eq!(t.get_selected(), Some(0).as_ref());
    t.prev_col_checked();
    assert_eq!(t.get_selected(), Some(2).as_ref());
    t.prev_col_checked();
    assert_eq!(t.get_selected(), Some(1).as_ref());
    t.prev_col_checked();
    assert_eq!(t.get_selected(), Some(0).as_ref());
}

#[test]
fn test_next_col_short() {
    let mut t = initialize_table(1, 0);
    assert_eq!(t.get_selected(), Some(3).as_ref());
    t.next_col();
    assert_eq!(t.get_selected(), Some(4).as_ref());
    t.next_col();
    assert_eq!(t.get_selected(), None.as_ref());
    t.next_col();
    assert_eq!(t.get_selected(), Some(3).as_ref());
}

#[test]
fn test_next_col_checked_short() {
    let mut t = initialize_table(1, 0);
    assert_eq!(t.get_selected(), Some(3).as_ref());
    t.next_col_checked();
    assert_eq!(t.get_selected(), Some(4).as_ref());
    t.next_col_checked();
    assert_eq!(t.get_selected(), Some(3).as_ref());
}

#[test]
fn test_prev_col_short() {
    let mut t = initialize_table(1, 0);
    assert_eq!(t.get_selected(), Some(3).as_ref());
    t.prev_col();
    assert_eq!(t.get_selected(), None.as_ref());
    t.prev_col();
    assert_eq!(t.get_selected(), Some(4).as_ref());
    t.prev_col();
    assert_eq!(t.get_selected(), Some(3).as_ref());
}

#[test]
fn test_carriage_return_none() {
    let mut t = initialize_table(1, 2);
    t.carriage_return();
    assert_eq!(t.state, TableState::new_with(1, 1));
}

#[test]
fn test_carriage_return_some() {
    let mut t = initialize_table(1, 1);
    t.carriage_return();
    assert_eq!(t.state, TableState::new_with(1, 1));
}

#[test]
fn test_table_with_items() {
    let items: Vec<u8> = vec![0, 1, 2, 3, 4];
    let t: StatefulTable<u8> = StatefulTable::with_items(2, 3, items);
    let u = initialize_table(0, 0);
    assert_eq!(t, u);
}

#[test]
pub fn test_get_selected() {
    let t = initialize_table(1, 1);
    assert_eq!(t.get_selected(), Some(4).as_ref());
}

#[test]
pub fn test_get() {
    let t = initialize_table(1, 1);
    assert_eq!(t.get(0, 0), Some(0).as_ref());
    assert_eq!(t.get(0, 1), Some(1).as_ref());
    assert_eq!(t.get(0, 2), Some(2).as_ref());
    assert_eq!(t.get(1, 0), Some(3).as_ref());
    assert_eq!(t.get(1, 1), Some(4).as_ref());
    assert_eq!(t.get(1, 2), None.as_ref());
    assert_eq!(t.get(10, 0), None.as_ref());
    assert_eq!(t.get(0, 10), None.as_ref());
}

#[test]
pub fn test_get_by_idx() {
    let t = initialize_table(1, 1);
    assert_eq!(t.get_by_idx(0), Some(0).as_ref());
    assert_eq!(t.get_by_idx(1), Some(1).as_ref());
    assert_eq!(t.get_by_idx(2), Some(2).as_ref());
    assert_eq!(t.get_by_idx(3), Some(3).as_ref());
    assert_eq!(t.get_by_idx(4), Some(4).as_ref());
    assert_eq!(t.get_by_idx(5), None.as_ref());
    assert_eq!(t.get_by_idx(50), None.as_ref());
}

#[test]
pub fn test_len() {
    let t = initialize_table(1, 1);
    assert_eq!(t.len(), 5);
}

#[test]
pub fn test_serialize() {
    let t = initialize_table(1, 1);
    let t_ser = serde_json::to_string(&t).unwrap();
    assert_eq!(&t_ser, "[0,1,2,3,4]");
}

// #[test]
// pub fn test_deserialize() {
//     let t_ser = "[0,1,2,3,4]";
//     let t = serde_json::from_str::<StatefulTable<u8, 2, 3>>(t_ser).unwrap();
//     let u = initialize_table(0, 0);
//     assert_eq!(t, u);
// }

#[test]
pub fn test_new() {
    let t: StatefulTable<u8> = StatefulTable::new(2,3);
    let u: StatefulTable<u8> = StatefulTable::with_items(2,3,vec![]);
    let v: StatefulTable<u8> = StatefulTable {
        state: TableState::new(),
        items: vec![vec![None; 3]; 2],
        rows: 2,
        cols: 3,
    };
    assert_eq!(t, u);
    assert_eq!(t, v);
}
