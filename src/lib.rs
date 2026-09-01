pub fn concat(s1: &str, s2: &str) -> String {
    format!("{s1}{s2}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concat_works() {
        assert_eq!(concat("hello", ""), "hello");
        assert_eq!(concat("", "world"), "world");
        assert_eq!(concat("hello", "world"), "helloworld");
    }
}
