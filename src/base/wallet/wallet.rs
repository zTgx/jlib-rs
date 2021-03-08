
/*
    钱包的类型

    1、创建钱包需要指定一种类型。
    2、没有默认类型。
    3、对应的值是椭圆曲线。
*/
#[derive(Debug, Copy, Clone)]
pub enum WalletType {
    SECP256K1,
    ED25519,
    SM2P256V1,
}

/*
    钱包的数据结构
*/
#[derive(Debug)]
pub struct Wallet {
    pub key_type        : WalletType,
    pub account_id      : String,
    pub master_key      : String,
    pub master_seed     : String,
    pub master_seed_hex : String,
    pub public_key      : String,
    pub public_key_hex  : String,
}