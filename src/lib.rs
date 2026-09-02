mod nfa;

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

/// Accepts strings of the form ab(a|b)aaa(a|b)*ab.
pub fn my_rg_acceptor(s: &str) -> bool {
    #[derive(Debug, PartialEq)]
    #[expect(clippy::upper_case_acronyms)]
    enum State {
        Empty,
        A,
        AB,
        ABX,
        ABXA,
        ABXAA,
        ABXAAAS,
        ABXAAASA,
        ABXAAASAB,
        Sink,
    }

    assert!(
        s.chars().all(|c| c == 'a' || c == 'b'),
        "my_rg_acceptor is defined over the alphabet Σ = ab"
    );

    let mut state = State::Empty;

    if *DEBUG {
        println!("State::{state:?}");
    }

    for c in s.chars() {
        state = match (state, c) {
            (State::Empty, 'a') => State::A,
            (State::A, 'b') => State::AB,
            (State::AB, _) => State::ABX,
            (State::ABX, 'a') => State::ABXA,
            (State::ABXA, 'a') => State::ABXAA,
            (State::ABXAA, 'a') | (State::ABXAAAS | State::ABXAAASAB, 'b') => State::ABXAAAS,
            (State::ABXAAAS | State::ABXAAASA | State::ABXAAASAB, 'a') => State::ABXAAASA,
            (State::ABXAAASA, 'b') => State::ABXAAASAB,
            _ => State::Sink,
        };

        if *DEBUG {
            println!("State::{state:?}");
        }
    }

    state == State::ABXAAASAB
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

    #[test]
    fn my_rg_acceptor_accepts() {
        assert!(my_rg_acceptor("abaaaaab"));
        assert!(my_rg_acceptor("abbaaaab"));

        assert!(my_rg_acceptor("abaaaaaab"));
        assert!(my_rg_acceptor("abbaaaaab"));
        assert!(my_rg_acceptor("abaaaabab"));
        assert!(my_rg_acceptor("abbaaabab"));

        assert!(my_rg_acceptor("abaaaaaaab"));
        assert!(my_rg_acceptor("abbaaaaaab"));
        assert!(my_rg_acceptor("abaaaaabab"));
        assert!(my_rg_acceptor("abbaaaabab"));
        assert!(my_rg_acceptor("abaaaabaab"));
        assert!(my_rg_acceptor("abbaaabaab"));
        assert!(my_rg_acceptor("abaaaabbab"));
        assert!(my_rg_acceptor("abbaaabbab"));

        assert!(my_rg_acceptor("abaaaaabbbbbbbbbab"));
        assert!(my_rg_acceptor("abaaaaabababababab"));
    }

    #[test]
    fn my_rg_acceptor_rejects() {
        assert!(!my_rg_acceptor(""));
        assert!(!my_rg_acceptor("a"));
        assert!(!my_rg_acceptor("ab"));
        assert!(!my_rg_acceptor("aba"));
        assert!(!my_rg_acceptor("abaa"));
        assert!(!my_rg_acceptor("abaaa"));
        assert!(!my_rg_acceptor("abaaaa"));
        assert!(!my_rg_acceptor("abaaaaa"));

        assert!(!my_rg_acceptor("bbaaaaab"));
        assert!(!my_rg_acceptor("aaaaaaab"));
        assert!(!my_rg_acceptor("ababaaab"));
        assert!(!my_rg_acceptor("abaabaab"));
        assert!(!my_rg_acceptor("abaaabab"));

        assert!(!my_rg_acceptor("abaaaaaaabb"));
        assert!(!my_rg_acceptor("abaaaaaaaba"));
    }

    #[test]
    #[should_panic(expected = "alphabet")]
    fn my_rg_acceptor_panics() {
        my_rg_acceptor("abaaaaabcab");
    }
}
