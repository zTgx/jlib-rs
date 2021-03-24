// 基础币
pub const CURRENCY     : &'static str = "SWT"; 

//手续费
pub const FEE          : u64 = 10000; 

//SECP256K1 加密算法对应的零号、一号地址
pub const ACCOUNT_ZERO : &'static str = "jjjjjjjjjjjjjjjjjjjjjhoLvTp";
pub const ACCOUNT_ONE  : &'static str = "jjjjjjjjjjjjjjjjjjjjBZbvri";

//SM2P256V1 加密算法对应的零号、一号地址
pub const ACCOUNT_ZERO_SM2P256V1 : &'static str = "jjjjjjjjjjjjjjjjjjjjjn1TT5q"; //swt的issuer
pub const ACCOUNT_ONE_SM2P256V1  : &'static str = "jjjjjjjjjjjjjjjjjjjjwVBfmE";  //占位地址