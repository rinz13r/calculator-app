mod utils;
use utils::utils::add;

#[test]
fn test_add() {
    assert_eq!(12, add(8, 4));
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
    let a = f64::from_str (&input ("Enter the first number: ")).unwrap_or_default ();
    let b = f64::from_str (&input ("Enter the second number: ")).unwrap_or_default ();
    let op = input ("Enter the operation: ");

    match op.as_str () {
        "+" => (println! ("{}", add (a, b))),
        _ => (println! ("Unknown operation"))
    }
}
