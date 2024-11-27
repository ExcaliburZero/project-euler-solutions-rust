pub fn solve() -> String {
    (1..1000)
        .filter(|x| x % 3 == 0 || x % 5 == 0)
        .sum::<i64>()
        .to_string()
}
