fn main() {
    let mut score = 0;

    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let mut chars = line.chars();

        score += match (chars.next().unwrap(), chars.nth(1).unwrap()) {
            ('A', 'X') => 4,
            ('B', 'X') => 1,
            ('C', 'X') => 7,
            ('A', 'Y') => 8,
            ('B', 'Y') => 5,
            ('C', 'Y') => 2,
            ('A', 'Z') => 3,
            ('B', 'Z') => 9,
            ('C', 'Z') => 6,
            _ => panic!(),
        }
    }

    println!("{}", score);
}
