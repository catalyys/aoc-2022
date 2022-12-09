use std::borrow::BorrowMut;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() {
    let filename = "input.txt";

    let mut stack1 = vec!["f", "c", "j", "p", "h", "t", "w"];
    let mut stack2 = vec!["g", "r", "v", "f", "z", "j", "b", "h"];
    let mut stack3 = vec!["h", "p", "t", "r"];
    let mut stack4 = vec!["z", "s", "n", "p", "h", "t"];
    let mut stack5 = vec!["n", "v", "f", "z", "h", "j", "d", "d"];
    let mut stack6 = vec!["p", "m", "g", "f", "w", "d", "z"];
    let mut stack7 = vec!["m", "v", "z", "w", "s", "j", "d", "p"];
    let mut stack8 = vec!["n", "d", "s"];
    let mut stack9 = vec!["d", "z", "s", "f", "m"];

    let mut crates = HashMap::from([
        (1, stack1),
        (2, stack2),
        (3, stack3),
    ]);

    let stack = crates.get(&1);
    if let Some(val) = stack {
        let mut asdkl = val.get_mut(1);
    } else {
        println!("Key is missing!");
    }

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
    }
}
