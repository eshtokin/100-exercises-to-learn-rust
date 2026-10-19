/// Return `12` if `n` is even,
/// `13` if `n` is divisible by `3`,
/// `17` otherwise.
fn magic_number(n: u32) -> u32 {
    let last_even: u32 = n % 2;
    let last_third: u32 = n % 3;

    if (last_even == 0) {
        return 12
    } else if (last_third == 0) {
        return 13
    }

    return 17
        
}


#[cfg(test)]
mod tests {
    use crate::magic_number;

    #[test]
    fn one() {
        assert_eq!(magic_number(1), 17);
    }

    #[test]
    fn two() {
        assert_eq!(magic_number(2), 12);
    }

    #[test]
    fn six() {
        assert_eq!(magic_number(6), 12);
    }

    #[test]
    fn nine() {
        assert_eq!(magic_number(9), 13);
    }

    #[test]
    fn high() {
        assert_eq!(magic_number(233), 17);
    }
}
