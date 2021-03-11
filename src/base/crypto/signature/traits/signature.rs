pub trait SignatureI {
    fn sign(&self, message: &[u8]) -> String;
    fn verify(&self, message: &[u8], signature: &[u8], key: &[u8]) -> bool;
    fn sign_txn_signature(&self, so: &Vec<u8>) -> String;
}