/// Adds two
///
/// # Examples
///
/// ```
/// let x = 5;
/// assert_eq!(x + 2, add_two::add_two(x));
/// ```
pub fn add_two(x: i32) -> i32 {
    x + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_exists() {
        add_two(7);
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(3, add_two(1));
    }

}
