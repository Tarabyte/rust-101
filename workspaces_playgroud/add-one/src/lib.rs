///
/// # Examples
///
/// ```
/// let nine = 9;
/// assert_eq!(10, add_one::add_one(9));
/// ```
///
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_one() {
        assert_eq!(2, add_one(1));
    }
}
