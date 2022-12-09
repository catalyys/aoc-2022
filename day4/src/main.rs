use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "input";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut sum1 = 0;
    let mut sum2 = 0;


    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        let split = line.split(",");

        let vec: Vec<&str> = split.collect();

        let mut nums: Vec<&str> = vec[0].split("-").collect();

        nums.append(&mut vec[1].split("-").collect());

        let num1: i32 = nums[0].parse().unwrap();
        let num2: i32 = nums[1].parse().unwrap();
        let num3: i32 = nums[2].parse().unwrap();
        let num4: i32 = nums[3].parse().unwrap();

        if (num1 <= num3 && num2 >= num4) || (num1 >= num3 && num2 <= num4) {
            //println!("{} - {} --- {} - {}", nums[0], nums[1], nums[2], nums[3]);

            sum1 += 1;
        }

        if num2 < num3 || num4 < num1 {
            println!("{} - {} --- {} - {}", nums[0], nums[1], nums[2], nums[3]);

            sum2 += 1;
        }
        //println!("{} {} {} {}", nums[0], nums[1], nums[2], nums[3]);

        //println!("{}. {}", index + 1, line);
    }

    println!("sum1: {}  sum2: {}", sum1, 1000-sum2)
}
