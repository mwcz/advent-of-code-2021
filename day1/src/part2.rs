use std::io;

fn to_int(line: &Result<String, io::Error>) -> i64 {
    match line {
        Ok(ref s) => s.parse::<i64>().unwrap(),
        Err(_) => 0,
    }
}

fn main() {
    let lines: Vec<Result<String, io::Error>> = io::stdin().lines().collect();

    let mut incs = 0;
    let mut last = i64::MAX;

    for win in lines.windows(3) {
        let sum: i64 = win.iter().map(to_int).sum();

        if sum > last {
            incs += 1;
        }

        last = sum;
    }

    println!("Increments: {}", incs);
}
