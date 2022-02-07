/// Returns the factrial of an integer.
/// ```
/// use rust_cavemen::factrial;
/// assert_eq!(factrial(&4), 24);
/// ```
pub fn factrial(n: &u32) -> u32 {
    let mut result: u32 = 1;
    for i in 1..n + 1 {
        result *= i;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factrial() {
        assert_eq!(factrial(&0), 1);
        assert_eq!(factrial(&1), 1);
        assert_eq!(factrial(&2), 2);
        assert_eq!(factrial(&3), 6);
        assert_eq!(factrial(&4), 24);
        assert_eq!(factrial(&5), 120);
    }
}