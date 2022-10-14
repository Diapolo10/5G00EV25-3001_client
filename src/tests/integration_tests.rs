// Integration tests go here
use client;

// We don’t need to annotate any code in the tests folder with #[cfg(test)]
// Cargo treats the tests directory specially and compiles files in this directory only when we run cargo test.
#[test]
fn it_works() {
    assert!(true);
}
