pub trait ChecksumI {
    fn checksum(&self, digest: &Vec<u8>) -> Vec<u8>;
}
