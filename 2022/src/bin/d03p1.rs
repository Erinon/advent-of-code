fn main() {
    let mut sum = 0u32;

    for line in std::io::stdin().lines() {
        let line = line.unwrap();

        let comp_size = line.len() / 2;
        let mut count = 0u64;

        for (i, c) in line.chars().enumerate() {
            let priority = 1 + c as u8 - if c >= 'a' { b'a' } else { b'A' - 26 };

            if i < comp_size {
                count |= 1 << priority;
            } else if count & 1 << priority != 0 {
                sum += priority as u32;
                break;
            }
        }
    }

    println!("{}", sum);
}
