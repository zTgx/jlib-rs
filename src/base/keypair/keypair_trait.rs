pub trait KeypairI {
    /*
        需要：seed
        1.	seed (16bytes)+ seq (4bytes) seq=0
        2.	SHA512Half
        3.	与椭圆最大数比，如果大于，seq+1，重复步骤1
        4.	获得private generator 32字节
    */
    fn private_generator(&self, masterphrase: &Vec<u8>)  -> Vec<u8>;

    /*
        需要：private generator，椭圆常数G
        1.	Private generator与椭圆常数G进行乘法运算
        2.	获得椭圆点（x，y）
        3.	压缩成(02+X 如Y 偶), 或(03+X 如 Y 奇)
        4.	获得public generator 33字节
    */
    fn public_generator(&self, private_generator: &Vec<u8>) -> Vec<u8>;
    
    /*
        需要：private generator， public generator
        1.	Public generator + 4字节seq + 4 字节 subseq， seq=subseq=0
        2.	SHA512Half
        3.	与椭圆最大数比，如果大于，subseq+1，重复步骤1
        4.	获得hash，32字节
        5.	与private generator相加，获得结果，32字节
    */
    fn generate_private_key(&self, private_generator: &Vec<u8>, public_generator: &Vec<u8>) -> Vec<u8>;

    /*
        需要：public generator
        1.	Public generator + 4字节seq + 4 字节 subseq， seq=subseq=0
        2.	SHA512Half
        3.	与椭圆最大数比，如果大于，subseq+1，重复步骤1
        4.	获得hash，32字节
        5.	与椭圆常数G进行乘法运算
        6.	获得椭圆点A（x，y）
        7.	将public generator解压为椭圆点B(x,y)
        8.	A+B = C
        9.	压缩成public key 33字节
    */
    fn generate_public_key(&self, public_generator: &Vec<u8>) -> Vec<u8>;

    /*
        需要：public key
        1.	对public key进行SHA256
        2.	再进行RIPEMD160，获得20字节
        3.	前置‘0’+20字节
        4.	SHA256 获得hash
        5.	取前4字节为校验
        6.	合并 ‘0’+20字节+4字节
        7.	加前置0变成正数
        8.	映射到base58，34字节
    */
    fn human_readable_public_key(&self, public_key: &Vec<u8>) -> String;
}