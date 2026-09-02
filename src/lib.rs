use itertools::Itertools as _;
use std::{cmp::min, collections::HashSet, sync::LazyLock};

static DEBUG: LazyLock<bool> = LazyLock::new(|| {
    std::env::var("FT_KLEENE_DEBUG")
        .is_ok_and(|k| matches!(k.to_lowercase().as_str(), "1" | "true"))
});

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

    let alphabet = sigma.chars().unique().collect_vec();

    let mut res = Vec::new();
    dfs(&alphabet, m, &mut String::new(), &mut res);
    res
}

#[expect(clippy::needless_range_loop)]
pub fn levenshtein_distance(s1: &str, s2: &str) -> u32 {
    let n1 = s1.chars().count();
    let n2 = s2.chars().count();

    let mut dp = vec![vec![0; n2 + 1]; n1 + 1];
    for i in 1..=n1 {
        dp[i][0] = i as u32;
    }
    for i in 1..=n2 {
        dp[0][i] = i as u32;
    }

    for (i1, c1) in s1.chars().enumerate() {
        for (i2, c2) in s2.chars().enumerate() {
            dp[i1 + 1][i2 + 1] = if c1 == c2 {
                dp[i1][i2]
            } else {
                1 + min(min(dp[i1 + 1][i2], dp[i1][i2 + 1]), dp[i1][i2])
            };
        }
    }

    if *DEBUG {
        for row in &dp {
            println!("{row:?}");
        }
    }

    dp[n1][n2]
}

pub fn kleene_star_acceptor(sigma: &str, s: &str) -> bool {
    let alphabet = sigma.chars().collect::<HashSet<_>>();
    s.chars().all(|c| alphabet.contains(&c))
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
        assert_eq!(kleene_star("ba", 2), vec!["", "b", "bb", "ba", "a", "ab", "aa"]);
        assert_eq!(kleene_star("abb", 2), vec!["", "a", "aa", "ab", "b", "ba", "bb"]);
        assert_eq!(
            kleene_star("abc", 2),
            vec!["", "a", "aa", "ab", "ac", "b", "ba", "bb", "bc", "c", "ca", "cb", "cc"]
        );
    }

    #[test]
    fn levenshtein_distance_works() {
        assert_eq!(levenshtein_distance("", ""), 0);
        assert_eq!(levenshtein_distance("", "a"), 1);
        assert_eq!(levenshtein_distance("", "ab"), 2);
        assert_eq!(levenshtein_distance("", "abc"), 3);

        assert_eq!(levenshtein_distance("", ""), 0);
        assert_eq!(levenshtein_distance("a", ""), 1);
        assert_eq!(levenshtein_distance("ab", ""), 2);
        assert_eq!(levenshtein_distance("abc", ""), 3);

        assert_eq!(levenshtein_distance("aba", "aa"), 1);
        assert_eq!(levenshtein_distance("abba", "aa"), 2);
        assert_eq!(levenshtein_distance("abbba", "aa"), 3);

        assert_eq!(levenshtein_distance("sus", "sussy"), 2);
        assert_eq!(levenshtein_distance("overreliance", "reliance"), 4);

        // https://www.youtube.com/watch?v=9120Php3Kh4
        assert_eq!(levenshtein_distance("pted", "pernoctated"), 7);
        assert_eq!(levenshtein_distance("interviewer", "ewer"), 7);
        assert_eq!(levenshtein_distance("electricity", "city"), 7);
        assert_eq!(levenshtein_distance("yhenuton", "oxyphenbutaxone"), 7);

        assert_eq!(
            levenshtein_distance("anticonstitutionnellement", "anticonstitutionnellement"),
            0
        );

        assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
        assert_eq!(levenshtein_distance("ooga", "chaka"), 4);
        assert_eq!(levenshtein_distance("levenshtein", "berenstain"), 4);
    }

    #[test]
    fn kleene_star_acceptor_works() {
        assert!(kleene_star_acceptor("hello world!", "hl"));
        assert!(!kleene_star_acceptor("hello world!", "ec"));

        assert!(kleene_star_acceptor("", ""));
        assert!(kleene_star_acceptor("a", ""));
        assert!(kleene_star_acceptor("ab", ""));

        assert!(kleene_star_acceptor("a", "aaaaaaaaaa"));
        assert!(kleene_star_acceptor("ab", "ababababa"));
        assert!(kleene_star_acceptor("ab", "aaaaaaaaa"));
        assert!(kleene_star_acceptor("ab", "bbbbbbbbb"));

        assert!(!kleene_star_acceptor("", "a"));
        assert!(!kleene_star_acceptor("a", "b"));
    }
}
