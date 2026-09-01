use ft_kleene::power_concat;
use itertools::Itertools as _;

fn main() {
    let args = std::env::args().skip(1).collect_vec();
    let [s, m] = args.as_slice() else {
        usage();
    };
    let m = m.parse::<u32>().unwrap_or_else(|_| usage());
    let result = power_concat(s, m);
    println!("{s:?}^{m} = {result:?}");
}

fn usage() -> ! {
    eprintln!("Usage: cargo run --bin ex01 s m");
    std::process::exit(1);
}
