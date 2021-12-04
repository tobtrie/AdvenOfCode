fn main() {
    println!("Hello, world!");

    let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    let measurement_increases = count_measurement_increases(&input);
    println!("measurement increases: {}", measurement_increases);
}

fn count_measurement_increases(input: &Vec<u64>) -> u64 {
    let mut counter = 0;
    for idx in 1..input.len() {
        let curr = input.get(idx);
        let prev = input.get(idx - 1);

        match (curr, prev) {
            (Some(curr), Some(prev)) => {
                if curr > prev {
                    counter += 1;
                }
            },
            _ => {/*do nothing*/}
        };
    }

    counter
}
