mod utils;
use utils::utils::add;

#[test]
fn test_add() {
    assert_eq!(12, add(8, 4));
}

fn main() {
    add(1, 2);
    println!("Hello, world!");
}
