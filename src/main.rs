fn multiplicative_persistence(n: u32) -> Vec<u32> {
    let mut num_string = n.to_string();
    let mut step_vec: Vec<u32> = Vec::new();

    while num_string.len() > 1 {
        let mut result: u32 = 1;

        for i in num_string.chars() {
            result *= i.to_digit(10).expect("Could not parse char to u32");
        }

        step_vec.push(result);
        num_string = result.to_string()
    }

    step_vec
}

fn run(numbers_to_check: u32, print_threshold: usize) {
    for i in 1..=numbers_to_check {
        let result: Vec<u32> = multiplicative_persistence(i);

        if result.len() >= print_threshold {
            println!("{}:", i);

            for (i, value) in result.iter().enumerate() {
                println!("    {}) {}", i + 1, value)
            }

            println!();
        }
    }
}

fn main() {
    run(100_000, 7);
}
