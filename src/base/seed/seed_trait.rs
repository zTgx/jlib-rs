pub trait SeedI {
    fn generate_seed(&self, passphrase: Option<&str>) -> Vec<u8>;
    fn human_readable_seed(&self, seed: &Vec<u8>) -> String;
    fn checksum(&self, digest: &Vec<u8>) -> Vec<u8>;
    fn is_valid(&self, readable_seed: &String) -> bool;
}
