use ft_kleene::levenshtein_distance;
use itertools::Itertools as _;

fn main() {
    let args = std::env::args().skip(1).collect_vec();
    let [s1, s2] = args.as_slice() else {
        eprintln!("Usage: cargo run --bin ex03 s1 s2");
        std::process::exit(1);
    };
    let result = levenshtein_distance(s1, s2);
    println!("levenshtein_distance({s1:?}, {s2:?}) = {result}");
}
