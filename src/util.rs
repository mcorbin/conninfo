
/// Split `s` with " " and remove all empty (= "") values.
///
/// # Example
///
/// let result = split_and_remove_empty("foo bar")
///
pub fn split_and_remove_empty(s: &str) -> Vec<&str> {
    let result: Vec<&str> = s.trim().split(' ').filter(|&v| v != "").collect();
    result
}
