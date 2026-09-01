use ft_kleene::concat;
use itertools::Itertools as _;

fn main() {
    let args = std::env::args().skip(1).collect_vec();
    let [s1, s2] = args.as_slice() else {
        eprintln!("Usage: cargo run --bin ex00 s1 s2");
        std::process::exit(1);
    };
    let result = concat(s1, s2);
    println!("{s1:?} + {s2:?} = {result:?}");
}
