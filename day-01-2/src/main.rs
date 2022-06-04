use std::io::{self, Read};

fn main() {
    let mut lines = String::new();
    io::stdin().read_to_string(&mut lines).unwrap();
    let depths = lines
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| str::parse(line).unwrap())
        .collect::<Vec<usize>>();
    eprintln!("{depths:#?}");
    let windows = depths.windows(3).collect::<Vec<_>>();
    eprintln!("{windows:#?}");
    let mut sum: Option<usize> = None;
    let mut answer = 0;

    for window in windows {
        eprintln!("{window:#?}");
        let sum2 = window.iter().sum();
        eprintln!("{sum2}");

        if let Some(sum) = sum {
            if sum2 > sum {
                answer += 1;
            }

        }

        sum = Some(sum2);
    }

    println!("answer: {answer}");
}
