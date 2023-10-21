use training_mod_tui_2::{StatefulTable, TableSize, TableState};

fn initialize_table(row: usize, column: usize) -> StatefulTable<u8> {
    StatefulTable {
        state: TableState::new_with(row, column),
        rows: vec![
            vec![Some(0), Some(1), Some(2)],
            vec![Some(3), Some(4), None],
        ],
        size: TableSize::new_with(2, 3),
    }
}

#[test]
fn test_next_col_full() -> Result<(), String> {
    let mut t = initialize_table(0, 0);
    assert_eq!(t.get_selected(), Some(0).as_ref());
    t.next_col();
    assert_eq!(t.get_selected(), Some(1).as_ref());
    t.next_col();
    assert_eq!(t.get_selected(), Some(2).as_ref());
    t.next_col();
    assert_eq!(t.get_selected(), Some(0).as_ref());
    Ok(())
}

#[test]
fn test_next_col_checked_full() -> Result<(), String> {
    let mut t = initialize_table(0, 0);
    assert_eq!(t.get_selected(), Some(0).as_ref());
    t.next_col_checked();
    assert_eq!(t.get_selected(), Some(1).as_ref());
    t.next_col_checked();
    assert_eq!(t.get_selected(), Some(2).as_ref());
    t.next_col_checked();
    assert_eq!(t.get_selected(), Some(0).as_ref());
    Ok(())
}

#[test]
fn test_prev_col_full() -> Result<(), String> {
    let mut t = initialize_table(0, 0);
    assert_eq!(t.get_selected(), Some(0).as_ref());
    t.prev_col();
    assert_eq!(t.get_selected(), Some(2).as_ref());
    t.prev_col();
    assert_eq!(t.get_selected(), Some(1).as_ref());
    t.prev_col();
    assert_eq!(t.get_selected(), Some(0).as_ref());
    Ok(())
}

#[test]
fn test_prev_col_checked_full() -> Result<(), String> {
    let mut t = initialize_table(0, 0);
    assert_eq!(t.get_selected(), Some(0).as_ref());
    t.prev_col_checked();
    assert_eq!(t.get_selected(), Some(2).as_ref());
    t.prev_col_checked();
    assert_eq!(t.get_selected(), Some(1).as_ref());
    t.prev_col_checked();
    assert_eq!(t.get_selected(), Some(0).as_ref());
    Ok(())
}

#[test]
fn test_next_col_short() -> Result<(), String> {
    let mut t = initialize_table(1, 0);
    assert_eq!(t.get_selected(), Some(3).as_ref());
    t.next_col();
    assert_eq!(t.get_selected(), Some(4).as_ref());
    t.next_col();
    assert_eq!(t.get_selected(), None.as_ref());
    t.next_col();
    assert_eq!(t.get_selected(), Some(3).as_ref());
    Ok(())
}

#[test]
fn test_next_col_checked_short() -> Result<(), String> {
    let mut t = initialize_table(1, 0);
    assert_eq!(t.get_selected(), Some(3).as_ref());
    t.next_col_checked();
    assert_eq!(t.get_selected(), Some(4).as_ref());
    t.next_col_checked();
    assert_eq!(t.get_selected(), Some(3).as_ref());
    Ok(())
}

#[test]
fn test_prev_col_short() -> Result<(), String> {
    let mut t = initialize_table(1, 0);
    assert_eq!(t.get_selected(), Some(3).as_ref());
    t.prev_col();
    assert_eq!(t.get_selected(), None.as_ref());
    t.prev_col();
    assert_eq!(t.get_selected(), Some(4).as_ref());
    t.prev_col();
    assert_eq!(t.get_selected(), Some(3).as_ref());
    Ok(())
}

#[test]
fn test_carriage_return_none() -> Result<(), String> {
    let mut t = initialize_table(1, 2);
    t.carriage_return();
    assert_eq!(t.state, TableState::new_with(1, 1));
    Ok(())
}

#[test]
fn test_carriage_return_some() -> Result<(), String> {
    let mut t = initialize_table(1, 1);
    t.carriage_return();
    assert_eq!(t.state, TableState::new_with(1, 1));
    Ok(())
}

#[test]
fn test_table_with_items() -> Result<(), String> {
    let items: Vec<u8> = vec![0, 1, 2, 3, 4];
    let t: StatefulTable<u8> = StatefulTable::with_items(items, 3);
    let u: StatefulTable<u8> = initialize_table(0, 0);
    assert_eq!(t,u);
    Ok(())
}

#[test]
pub fn test_get_selected() -> Result<(), String> {
    let t = initialize_table(1, 1);
    assert_eq!(t.get_selected(), Some(4).as_ref());
    Ok(())
}

#[test]
pub fn test_get() -> Result<(), String> {
    let t = initialize_table(1, 1);
    assert_eq!(t.get(0, 0), Some(0).as_ref());
    assert_eq!(t.get(0, 1), Some(1).as_ref());
    assert_eq!(t.get(0, 2), Some(2).as_ref());
    assert_eq!(t.get(1, 0), Some(3).as_ref());
    assert_eq!(t.get(1, 1), Some(4).as_ref());
    assert_eq!(t.get(1, 2), None.as_ref());
    assert_eq!(t.get(10, 0), None.as_ref());
    assert_eq!(t.get(0, 10), None.as_ref());
    Ok(())
}

#[test]
pub fn test_get_by_idx() -> Result<(), String> {
    let t = initialize_table(1, 1);
    assert_eq!(t.get_by_idx(0), Some(0).as_ref());
    assert_eq!(t.get_by_idx(1), Some(1).as_ref());
    assert_eq!(t.get_by_idx(2), Some(2).as_ref());
    assert_eq!(t.get_by_idx(3), Some(3).as_ref());
    assert_eq!(t.get_by_idx(4), Some(4).as_ref());
    assert_eq!(t.get_by_idx(5), None.as_ref());
    assert_eq!(t.get_by_idx(50), None.as_ref());
    Ok(())
}

#[test]
pub fn test_len() -> Result<(), String> {
    let t = initialize_table(1, 1);
    assert_eq!(t.len(), 5);
    Ok(())
}
