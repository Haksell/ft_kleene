use ft_kleene::kleene_star;
use itertools::Itertools as _;

fn main() {
    let args = std::env::args().skip(1).collect_vec();
    let [sigma, m] = args.as_slice() else {
        usage();
    };
    let m = m.parse::<u32>().unwrap_or_else(|_| usage());
    let result = kleene_star(sigma, m);
    println!("{result:?}");
}

fn usage() -> ! {
    eprintln!("Usage: cargo run --bin ex02 sigma m");
    std::process::exit(1);
}
