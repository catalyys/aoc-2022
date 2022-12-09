use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let filename = "input.txt";

    // let stack1 = vec!["Z", "N"];
    // let stack2 = vec!["M", "C", "D"];
    // let stack3 = vec!["P"];
    let stack1 = vec!["F", "C", "J", "P", "H", "T", "W"];
    let stack2 = vec!["G", "R", "V", "F", "Z", "J", "B", "H"];
    let stack3 = vec!["H", "P", "T", "R"];
    let stack4 = vec!["Z", "S", "N", "P", "H", "T"];
    let stack5 = vec!["N", "V", "F", "Z", "H", "J", "D", "D"];
    let stack6 = vec!["P", "M", "G", "F", "W", "D", "Z"];
    let stack7 = vec!["M", "V", "Z", "W", "S", "J", "D", "P"];
    let stack8 = vec!["N", "D", "S"];
    let stack9 = vec!["D", "Z", "S", "F", "M"];

    let mut crates = HashMap::from([
        (1, stack1),
        (2, stack2),
        (3, stack3),
        (4, stack4),
        (5, stack5),
        (6, stack6),
        (7, stack7),
        (8, stack8),
        (9, stack9),
    ]);

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let re = Regex::new(r"(\d+).*(\d).*(\d)").unwrap();

        for cap in re.captures_iter(&line) {
            // println!("amount: {} from: {} to: {}", &cap[1], &cap[2], &cap[3]);

            let amount: i32 = cap[1].parse().unwrap();
            let from: i32 = cap[2].parse().unwrap();
            let to: i32 = cap[3].parse().unwrap();

            for _ in 0..amount {
                let from_stack = crates.get_mut(&from);

                if let Some(val) = from_stack {
                    let crate_char = val.pop();
                    // println!("moving {}", crate_char.unwrap());

                    let to_stack = crates.get_mut(&to);

                    if let Some(val2) = to_stack {
                        val2.push(crate_char.unwrap())
                    }
                }
            }
        }
    }

    let mut answer_a = "".to_owned();
    for i in 1..10{
        let temp_stack = crates.get_mut(&i);
        if let Some(val3) = temp_stack {
            answer_a.push_str(val3[val3.len()-1]);
        }
    }

    println!("{}", answer_a)
}
