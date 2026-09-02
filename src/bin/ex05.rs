use ft_kleene::my_rg_acceptor;
use itertools::Itertools as _;

fn main() {
    let args = std::env::args().skip(1).collect_vec();
    let [s] = args.as_slice() else {
        eprintln!("Usage: cargo run --bin ex05 s");
        std::process::exit(1);
    };
    let result = my_rg_acceptor(s);
    println!("my_rg_acceptor({s:?}) = {result}");
}
