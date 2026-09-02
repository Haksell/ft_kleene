use ft_kleene::kleene_star_acceptor;
use itertools::Itertools as _;

fn main() {
    let args = std::env::args().skip(1).collect_vec();
    let [sigma, s] = args.as_slice() else {
        eprintln!("Usage: cargo run --bin ex04 sigma s");
        std::process::exit(1);
    };
    let result = kleene_star_acceptor(sigma, s);
    println!("kleene_star_acceptor({sigma:?}, {s:?}) = {result}");
}
