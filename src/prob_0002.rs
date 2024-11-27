pub fn solve() -> String {
    let mut sum = 0;

    let mut a = 0;
    let mut b = 1;
    while b < 4_000_000 {
        let c = a + b;
        a = b;
        b = c;

        if b % 2 == 0 {
            sum += b;
        }
    }

    sum.to_string()
}
