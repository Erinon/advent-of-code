fn main() {
    let mut max_sum = 0;
    let mut current_sum = 0;

    for line in std::io::stdin().lines() {
        let line = line.unwrap();

        if line.is_empty() {
            max_sum = max_sum.max(current_sum);
            current_sum = 0;
            continue;
        }

        current_sum += line.parse::<u64>().unwrap();
    }

    max_sum = max_sum.max(current_sum);

    println!("{}", max_sum);
}
