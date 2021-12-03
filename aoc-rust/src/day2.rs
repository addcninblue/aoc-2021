use std::fs;

#[derive(Debug)]
enum Direction {
    Forward,
    Down,
    Up,
    Unknown
}

fn read_file() -> Vec<(Direction, i32)> {
    let filename = "../inputs/day2";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    contents.split("\n").map(|line| {
        let line_parts: Vec<&str> = line.split(" ").collect();
        if line_parts.len() < 2 {
            (Direction::Unknown, 0)
        } else {
            let direction = match line_parts[0] {
                "forward" => Direction::Forward,
                "down" => Direction::Down,
                "up" => Direction::Up,
                _ => Direction::Unknown,
            };
            let num = match line_parts[1].parse::<i32>() {
                Ok(n) => n,
                Err(_) => 0,
            };

            (direction, num)
        }
    }).collect()
}

pub fn sol1() -> i32 {
    let res = read_file();
    let (horizontal, depth) = res.iter().fold((0, 0), |(horizontal, depth), (direction, amount)| {
        match direction {
            Direction::Forward => (horizontal + amount, depth),
            Direction::Down => (horizontal, depth + amount),
            Direction::Up => (horizontal, depth - amount),
            Direction::Unknown => (horizontal, depth),
        }
    });
    horizontal * depth
}

pub fn sol2() -> i32 {
    let res = read_file();
    let (horizontal, depth, _aim) = res.iter().fold((0, 0, 0), |(horizontal, depth, aim), (direction, amount)| {
        match direction {
            Direction::Forward => (horizontal + amount, depth + aim * amount, aim),
            Direction::Down => (horizontal, depth, aim + amount),
            Direction::Up => (horizontal, depth, aim - amount),
            Direction::Unknown => (horizontal, depth, aim),
        }
    });
    horizontal * depth
}
