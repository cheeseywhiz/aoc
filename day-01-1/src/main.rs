use std::io::{self, Read};

fn main() {
    let mut lines = String::new();
    io::stdin().read_to_string(&mut lines).unwrap();
    let mut depth: Option<usize> = None;
    let mut answer = 0;

    for line in lines.split('\n').filter(|line| !line.is_empty()) {
        let depth2: usize = str::parse(line).unwrap();

        if let Some(depth) = depth {
            if depth2 > depth {
                answer += 1;
            }
        }

        depth = Some(depth2)
    }

    println!("answer: {answer}");
}
