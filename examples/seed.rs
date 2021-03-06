extern crate jlib;
// use jlib::base::seed::seed_trait::SeedI;
use jlib::base::seed::seed_guomi::SeedGuomi;

fn main() {
    let seed_guomi = SeedGuomi::new();
    
    let passphrase = Some("Masterphrase");
    let seed = seed_guomi.build(passphrase);
    println!("readable seed : {}", seed);
}
