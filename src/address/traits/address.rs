pub trait AddressI {
    fn human_account_id(&self) -> String;
    fn public_key(&self) -> String;
    fn public_key_hex(&self) -> String;
    fn private_key(&self) -> String;
}

// ---------------------------------------------------------------------------------------------------------
// address 的有效性检查
// ---------------------------------------------------------------------------------------------------------
pub trait AddressCheckI {
    /*
        需要地址

        Wallet数据结构中的account_id字段。
    */
    fn check(&self, address: &String) -> bool;
}