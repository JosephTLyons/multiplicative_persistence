fn multiplicative_persistence(n: u32) -> Vec<u32> {
    let mut num_string = n.to_string();
    let mut step_vec: Vec<u32> = Vec::new();
    step_vec.push(n);

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

fn main() {
    for i in 1..=100_000 {
        let result: Vec<u32> = multiplicative_persistence(i);

        if result.len() >= 8 {
            println!("{}:", i);

            for (i, value) in result.iter().enumerate() {
                println!("    {}) {}", i + 1, value)
            }

            println!();
        }
    }
}
