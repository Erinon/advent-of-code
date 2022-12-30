fn main() {
    let mut sum = 0;
    let mut total_count = u64::MAX;

    for (i, line) in std::io::stdin().lines().enumerate() {
        let line = line.unwrap();

        let mut count = 0u64;

        for c in line.chars() {
            let priority = 1 + c as u8 - if c >= 'a' { b'a' } else { b'A' - 26 };

            if i % 3 != 2 {
                count |= 1 << priority;
            } else if total_count & 1 << priority != 0 {
                sum += priority as u32;
                count = u64::MAX;
                total_count = u64::MAX;
                break;
            }
        }

        total_count &= count;
    }

    println!("{}", sum);
}
