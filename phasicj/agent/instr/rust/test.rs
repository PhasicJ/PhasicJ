#![cfg(test)]

static SYNCHBLOCK_CLASS_DATA: &[u8] = include_bytes!(env!("SYNCHBLOCK_CLASS"));

#[test]
fn smoke_test() {
    println!("Hello, test.");
}
