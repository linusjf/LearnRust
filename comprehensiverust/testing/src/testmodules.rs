#[allow(dead_code)]
fn helper(a: &str, b: &str) -> String {
    format!("{a} {b}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_helper() {
        assert_eq!(helper("foo", "bar"), "foo bar");
    }
}
