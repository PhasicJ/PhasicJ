#![cfg(test)]

#[test]
fn instrument_test_class() {
    // NOTE(dwtj): We go up four directories because this macro searches
    //  for the file to include relative to this source file.
    let test_class = include_bytes!(concat!("../../../../", env!("TEST_CLASS_EXEC_PATH")));
    assert!(test_class.len() > 0);
}
