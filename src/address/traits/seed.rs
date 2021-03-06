//----------------------------------------------------------------------------------------------------------------
//    生成seed必需要实现的trait
//----------------------------------------------------------------------------------------------------------------
pub trait SeedI {
    /*
        需要passphrase(可选)

        1、如果passphrase没有指定， 则随机生成16个字节的bytes。
        2、【摘要计算】，得32字节的bytes。
        3、取前128位。
        4、全部大写的hex编码。
    */
    fn get_seed(&self, passphrase: Option<&str>) -> Vec<u8>;

    /*
        需要seed

        1、SEEDPREFIX(0x21) 追加到 seed 前，共17字节，【摘要计算】。
        2、计算checksum。
        3、SEEDPREFIX(0x21)，seed，checksum合并, 做base58计算。
        4、结果为29字节字符串。（如：shQRzBzq9akA2C2o4MKt1fM51WWs9）
    */
    fn human_seed(&self, seed: &Vec<u8>) -> String;

    /*
        需要 secret

        1、secret 的 base58 解码。
        2、截取 1..17 区间的16个字节
    */
    // fn secret_to_seed(secret: &String) -> Vec<u8> {
    //     let seed = BaseX::new(SKYWELL).decode((&secret).to_string()).unwrap();
    //     seed[1..17].to_vec()
    // }

    /*
        需要seed

        1、计算rfc1751
        2、结果为大写字符串
    */
    fn human_seed_rfc1751(&self, seed: &Vec<u8>) -> String;
}

//----------------------------------------------------------------------------------------------------------------
//    Seed的有效性检查trait
//----------------------------------------------------------------------------------------------------------------

pub trait SeedCheckI {
    /*
        需要seed
        
        * wallet数据结构中的 master_seed字段
        * ssyMgBnugfGybuKvzwqnfTNE773Eo
    */
    fn check(seed: &String) -> bool;
}