pub trait AddressI {
    fn human_account_id(&self) -> String;
    fn public_key(&self) -> String;
    fn public_key_hex(&self) -> String;
}