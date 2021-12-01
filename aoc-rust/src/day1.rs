// use std::env;
use std::fs;

pub fn solution() -> i32 {
    let filename = "../inputs/day1";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let lines: Vec<i32> = contents.split("\n").map(|line| {
        match line.parse::<i32>() {
            Ok(n) => n,
            Err(_) => 0,
        }
    }).collect();

    let iter1 = lines.iter();

    let mut iter2 = lines.iter();
    iter2.next();

    let mut iter3 = lines.iter();
    iter3.next();
    iter3.next();

    let sums: Vec<i32> = iter1.zip(iter2).zip(iter3).map(|((item1, item2), item3)| item1 + item2 + item3).collect();

    let sums_iter1 = sums.iter();
    let mut sums_iter2 = sums.iter();
    sums_iter2.next();

    let mut increments = 0;

    for (item1, item2) in sums_iter1.zip(sums_iter2) {
        if item1 < item2 {
            increments += 1;
        }
    }

    increments
}
