extern crate jlib;
use jlib::base::seed::seed_trait::SeedI;
use jlib::base::seed::seed_guomi::SeedGuomi;

fn main() {
    let seed_guomi = SeedGuomi::new();
    let seed = seed_guomi.generate_masterphrase(Some("Masterphrase"));
    let seed_readable = seed_guomi.human_readable_seed(&seed);
    println!("readable seed : {}", seed_readable);
}
