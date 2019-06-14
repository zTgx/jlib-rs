// examples/hello.rs
fn main() {
    extern crate bs58;

    pub const JTM: &'static [u8; 58] = b"jpshnaf39wBUDNEGHJKLM4PQRST7VWXYZ2bcdeCg65rkm8oFqi1tuvAxyz";

    let input: String = "ssndDcNoc4FvwVPveY3KbfWh8fNh3".to_string();
    let encoded = bs58::encode(input).with_alphabet(JTM).into_string();
    println!("encoded: {}", encoded);

    let decoded = bs58::decode("ssndDcNoc4FvwVPveY3KbfWh8fNh3")
            .with_alphabet(JTM)
            .into_vec().unwrap();
    println!("decoded: {:?}", decoded);
}