pub trait SignatureI {
    /*
    @sign
    message: [u8]   /   message bytes needed to be sign.
    key    : [u8]   /   secret key bytes array.
    Output : signed hex string
    */
    fn sign(&self, message: &[u8]) -> String;

    /*
    @verify
    message  : [u8]    /  raw message bytes.
    signature: [u8]    /  signed bytes array.
    Output   : bool    /  verify success or not.
    */
    fn verify(&self, message: &[u8], signature: &[u8], key: &[u8]) -> bool;


    fn sign_txn_signature(&self, so: &Vec<u8>) -> String;
}