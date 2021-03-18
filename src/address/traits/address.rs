// ---------------------------------------------------------------------------------------------------------
// address trait
// 
// 1、不同加密解决方案在生成地址的时候，需要实现以下方法
// 2、该 trait 是对外的 public 接口
// 
// ---------------------------------------------------------------------------------------------------------
pub trait AddressI {
    /*
        需要 seed

        【计算过程】
        透明调用 generater 的 human_readable_public_key 方法

        【输出】
        对应 wallet 结构中的 `account_id` 字段
    */
    fn human_account_id(&self) -> String;

    /*
        需要 seed

        【计算过程】
        1、透明调用 generater 的 generate_public_key 方法
        2、checksum(35 + public_key)
        3、base58(0 + public_key + checksum)

        【输出】
        对应 wallet 结构中的 `public_key` 字段
    */
    fn public_key(&self) -> String;

    /*
        需要 public_key

        【计算过程】
        1、透明调用 generater 的 generate_public_key 方法
        2、大写的十六进制编码

        【输出】
        对应 wallet 结构中的 `public_key_hex` 字段
    */
    fn public_key_hex(&self) -> String;

    /*
        需要 seed

        【计算过程】
        1、透明调用 generater 的 generate_private_key 方法
        2、大写的十六进制编码

        【输出】
        导出 私钥 
    */
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