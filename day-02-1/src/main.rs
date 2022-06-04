use std::io::{self, Read};

fn split_lines(lines: &str) -> Vec<&str> {
    lines
        .split_inclusive('\n')
        .map(|line| line.rsplit_once('\n').unwrap().0)
        .collect()
}

fn main() {
    // read stdin
    let mut lines = String::new();
    io::stdin().read_to_string(&mut lines).unwrap();
    let lines = split_lines(&lines);

    // calculate position
    eprintln!("{lines:#?}");
    let (mut distance, mut depth) = (0, 0);

    for line in lines {
        let (direction, amount) = line.split_once(' ').unwrap();
        let amount: usize = amount.parse().unwrap();

        match direction {
            "forward" => distance += amount,
            "up" => depth -= amount,
            "down" => depth += amount,
            _ => panic!("unexpected direction: {direction}")
        }
    }

    println!("distance: {distance}");
    println!("depth: {depth}");
    let answer = distance * depth;
    println!("answer: {answer}")
}
