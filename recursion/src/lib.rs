pub fn factorial(n: u64) -> u64 {
    if n == 0 {
        return 1;
    } else {
        return n * factorial(n - 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_5() {
        assert_eq!(120, factorial(5));
    }
}