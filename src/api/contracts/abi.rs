use hex;
use sha3::{Digest, Keccak256};

/*
    【输入】方法签名
    【输出】有效性检查
*/
fn is_valid_function_sig(sig: &str) {
    //判断 fn_sig 是否包含了空格
    if let Some(idx) = sig.find(char::is_whitespace) {
        assert!(idx == 0, "方法签名中【不能】包含空格！");
    }

    //判断是否包含了左右括号

    //判断方法名是否有效
}

/*
    【输入】待计算的方法签名，如 calc(uint256)
    1、不带返回值
    2、方法名 + 参数类型
    3、如果存在多个参数，参数类型用逗号分开
    4、不许有空格的存在

    【输出】函数签名的 Keccak（SHA-3）哈希的前 4 字节（高位在左的大端序）
*/
pub fn function_hash(sig: &str) -> String {
    is_valid_function_sig(&sig);

    let mut hasher = Keccak256::new();
    // write input message
    hasher.update(sig);

    // read hash digest
    let result = hasher.finalize();

    let checksum: &[u8] = &result[..4];

    hex::encode(checksum)
}

// ---------------------------------------------------------------------------------------------------------
//
// abi 测试用例
//
// 运行测试用例：
// cargo test api::contracts::abi::tests::abi
// ---------------------------------------------------------------------------------------------------------
// 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abi() {
        // 不带参数的方法名
        let hashed = function_hash("retrieve()");
        assert_eq!(hashed, "2e64cec1".to_owned());

        //带参数的方法名
        let hashed = function_hash("store(uint256)");
        assert_eq!(hashed, "6057361d".to_owned());

        //带空格的无效方法名
        function_hash("calc");
    }
}