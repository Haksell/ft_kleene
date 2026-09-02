use itertools::Itertools as _;

pub fn concat(s1: &str, s2: &str) -> String {
    format!("{s1}{s2}")
}

pub fn power_concat(s: &str, m: u32) -> String {
    s.repeat(m as usize)
}

pub fn kleene_star(sigma: &str, m: u32) -> Vec<String> {
    fn dfs(alphabet: &[char], depth: u32, s: &mut String, res: &mut Vec<String>) {
        res.push(s.clone());

        if depth == 0 {
            return;
        }

        for &c in alphabet {
            s.push(c);
            dfs(alphabet, depth - 1, s, res);
            s.pop();
        }
    }

    let alphabet = sigma.chars().collect_vec();
    // TODO: test with should_panic
    assert!(alphabet.iter().all_unique(), "alphabet should consist of unique characters");

    let mut res = Vec::new();
    dfs(&alphabet, m, &mut String::new(), &mut res);
    res
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

    #[test]
    fn kleene_star_works() {
        assert_eq!(kleene_star("ab", 0), vec![""]);
        assert_eq!(kleene_star("ab", 1), vec!["", "a", "b"]);
        assert_eq!(kleene_star("ab", 2), vec!["", "a", "aa", "ab", "b", "ba", "bb"]);
        assert_eq!(
            kleene_star("abc", 2),
            vec!["", "a", "aa", "ab", "ac", "b", "ba", "bb", "bc", "c", "ca", "cb", "cc"]
        );
    }

    #[test]
    #[should_panic(expected = "alphabet should consist of unique characters")]
    fn kleene_star_duplicate_symbols() {
        kleene_star("abb", 2);
    }
}
