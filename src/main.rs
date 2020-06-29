mod utils;
use utils::utils::{add, sub};

#[test]
fn test_add() {
    assert_eq!(12, add(8, 4));
}

#[test]
fn test_sub () {
    assert_eq! (1, sub (-3, -4));
}

use std::io::*;

fn input (text: &'static str) -> String {
    // let stdin = std::io::stdin ();
    let mut buffer = String::new ();

    print! ("{}", text);
    std::io::stdout().flush ().expect ("flush failed");
    std::io::stdin ().read_line (&mut buffer).expect ("unable to perform read");
    buffer.pop ();
    buffer
}

use std::str::FromStr;

fn main() {
    println! ("{}+{}={}", 1, 2, add (1, 2));
}
