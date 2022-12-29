fn main() {
    let mut score = 0;

    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let mut chars = line.chars();

        score += match (chars.next().unwrap(), chars.nth(1).unwrap()) {
            ('A', 'X') => 3,
            ('B', 'X') => 1,
            ('C', 'X') => 2,
            ('A', 'Y') => 4,
            ('B', 'Y') => 5,
            ('C', 'Y') => 6,
            ('A', 'Z') => 8,
            ('B', 'Z') => 9,
            ('C', 'Z') => 7,
            _ => panic!(),
        }
    }

    println!("{}", score);
}
