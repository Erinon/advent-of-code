fn main() {
    let mut sums = Vec::new();
    let mut current_sum = 0;

    for line in std::io::stdin().lines() {
        let line = line.unwrap();

        if line.is_empty() {
            sums.push(current_sum);
            current_sum = 0;
            continue;
        }

        current_sum += line.parse::<u64>().unwrap();
    }

    sums.push(current_sum);

    sums.sort_unstable_by(|a, b| b.cmp(a));

    let max_sum: u64 = sums.iter().take(3).sum();

    println!("{}", max_sum);
}
