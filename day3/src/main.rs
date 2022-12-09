use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn main() {
    let file_path = "input";

    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let mut sum = 0;

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        // Show the line and its number.
        let (part1, part2) = line.split_at(line.len()/2);
        println!("{}. {} {} len: {}", index + 1, part1, part2, line.len());

        for chars in part1.chars(){
            if part2.contains(chars) {
                println!("{}", chars);
                sum += 1;
                break;
            }
        }
    }
    let a = 'a'
    int num = a - 1;
    println!("{sum}");
}
