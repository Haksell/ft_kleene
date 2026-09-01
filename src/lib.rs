pub fn concat(s1: &str, s2: &str) -> String {
    format!("{s1}{s2}")
}

pub fn power_concat(s: &str, m: u32) -> String {
    s.repeat(m as usize)
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

    #[test]
    fn power_concat_works() {
        assert_eq!(power_concat("sup", 0), "");
        assert_eq!(power_concat("sup", 1), "sup");
        assert_eq!(power_concat("sup", 2), "supsup");
        assert_eq!(power_concat("sup", 3), "supsupsup");
        assert_eq!(power_concat("", 42), "");
    }
}
