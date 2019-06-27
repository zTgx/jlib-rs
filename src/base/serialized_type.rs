use crate::base::amount::*;
use crate::base::*; //util
use typename::TypeName;

pub trait SerializedType {
  fn serialize(&self) -> Vec<u8>;
  fn parse(&self);
}

pub trait SerializedSTInt8 {
  fn serialize(value: u8) -> Vec<u8>;
  fn parse();
}

//STInt8
#[derive(TypeName, Debug)]
pub struct STInt8 {
  pub id: i32,
  pub value: u8,
}
impl STInt8 {
  pub fn new(value: u8) -> Self {
    STInt8 {
      id: 16, //default value
      value: value,
    }
  }
}

impl SerializedSTInt8 for STInt8 {
  fn serialize(value: u8) -> Vec<u8> {
     value.to_be_bytes().to_vec()
  }

  fn parse() {}
}

//----------------------------------------------------
pub trait SerializedSTInt16 {
  fn serialize(value: u16) -> Vec<u8>;
  fn parse();
}

//STInt16
#[derive(TypeName, Debug)]
pub struct STInt16 {
  pub id: i32,
  pub value: u16,
}
impl STInt16 {
  pub fn new(value: u16) -> Self {
    STInt16 {
      id: 1, //default value
      value: value,
    }
  }
}

impl SerializedSTInt16 for STInt16 {
  fn serialize(value: u16) -> Vec<u8> {
     value.to_be_bytes().to_vec()
  }

  fn parse() {}
}

//----------------------------------------------------
pub trait SerializedSTInt32 {
  fn serialize(value: u32) -> Vec<u8>;
  fn parse();
}
//STInt32
#[derive(TypeName, Debug)]
pub struct STInt32 {
  pub id: i32,
  pub value: u32,
}
impl STInt32 {
  pub fn new(value: u32) -> Self {
    STInt32 {
      id: 2, //default value
      value: value,
    }
  }
}

impl SerializedSTInt32 for STInt32 {
  fn serialize(value: u32) -> Vec<u8> {
    value.to_be_bytes().to_vec()
  }

  fn parse() {}
}

//STInt64
#[derive(TypeName, Debug)]
pub struct STInt64 {
  pub id: i32,
  pub value: u64,
}
impl STInt64 {
  pub fn new(value: u64) -> Self {
    STInt64 {
      id: 3, //default value
      value: value,
    }
  }
}

impl SerializedType for STInt64 {
  fn serialize(&self) -> Vec<u8> {
    self.value.to_be_bytes().to_vec()
  }

  fn parse(&self) {}
}

//STHash128
#[derive(TypeName, Debug)]
pub struct STHash128 {
  pub id: i32,
  pub value: u128,
}
impl STHash128 {
  pub fn new(value: u128) -> Self {
    STHash128 {
      id: 4, //default value
      value: value,
    }
  }
}

impl SerializedType for STHash128 {
  fn serialize(&self) -> Vec<u8> {
    self.value.to_be_bytes().to_vec()
  }

  fn parse(&self) {}
}

//STHash256
#[derive(TypeName, Debug)]
pub struct STHash256 {
  pub id: i32,
  pub value: u128,
}
impl STHash256 {
  pub fn new(value: u128) -> Self {
    STHash256 {
      id: 5, //default value
      value: value,
    }
  }
}

impl SerializedType for STHash256 {
  fn serialize(&self) -> Vec<u8> {
    self.value.to_be_bytes().to_vec()
  }

  fn parse(&self) {}
}

//STHash160
#[derive(TypeName, Debug)]
pub struct STHash160 {
  pub id: i32,
  pub value: u128,
}
impl STHash160 {
  pub fn new(value: u128) -> Self {
    STHash160 {
      id: 17, //default value
      value: value,
    }
  }
}

impl SerializedType for STHash160 {
  fn serialize(&self) -> Vec<u8> {
    self.value.to_be_bytes().to_vec()
  }

  fn parse(&self) {}
}

//STCurrency
// #[derive(TypeName, Debug)]
// pub struct STCurrency {
//     pub id: i32,
// }
// impl STCurrency {
//     pub fn new() -> Self {
//         STCurrency {
//             id: 
//         }
//     }
// }



pub trait SerializedSTAmount {
  fn serialize(value: Amount) -> Vec<u8>;
  fn parse();
}

//STAmount
pub struct STAmount {
    pub id: i32,
}
impl STAmount {
    pub fn new() -> Self {
        STAmount {
            id: 6,
        }
    }
}
impl SerializedSTAmount for STAmount {
    fn serialize(amount: Amount) -> Vec<u8> {

        // var valueBytes = arraySet(8, 0);

        //SWTC
        if amount.is_native {
            // var bn = new BN(amount._value, 10);
            // var valueHex = bn.toString(16);
            let mut value_hex = amount.value.unwrap().to_str_radix(16);
            println!("value_hex : {}", value_hex);

            // Enforce correct length (64 bits)
            if value_hex.len() > 16 {
                // throw new Error('Amount Value out of bounds');
            }

            while value_hex.len() < 16 {
                // valueHex = '0' + valueHex;
                value_hex.insert(0, '0');
            }

            //Convert the HEX value to bytes array
            // valueBytes = convertHexToByteArray(valueHex);//bytes.fromBits(hex.toBits(valueHex));
            unsafe {

                let mut value_bytes = hex::decode(value_hex).unwrap();


                // Clear most significant two bits - these bits should already be 0 if
                // Amount enforces the range correctly, but we'll clear them anyway just
                // so this code can make certain guarantees about the encoded value.
                value_bytes[0] &= 0x3f;

                if !amount.is_negative {
                    value_bytes[0] |= 0x40;
                }

                return value_bytes.to_vec();

            }
        }

        vec![]
    }

    fn parse() {}
}

pub trait SerializedSTVL {
  fn serialize(value: String) -> Vec<u8>;
  fn parse();
}
pub fn serialize_varint(byte_data: &mut Vec<u8>) -> Vec<u8> {
    let mut val = byte_data.len();
    let mut v: Vec<u8> = vec![];
    if (val <= 192) {
        let mut t = vec![val as u8];
        v.append(&mut t);
    } else if (val <= 12480) {
        val -= 193;
        let mut t = [(193 + (val >> 8)) as u8, (val & 0xff) as u8].to_vec();
        v.append(&mut t);
    } else if (val <= 918744) {
        val -= 12481;

        let mut t = [(241 + (val >> 16)) as u8, (val >> 8 & 0xff) as u8, (val & 0xff) as u8].to_vec();
        v.append(&mut t);
    } 

    v.append(byte_data);

    v
}

//STVL
pub struct STVL {
    pub id: i32,
}
impl STVL {
    pub fn new() -> Self {
        STVL {
            id: 7,
        }
    }

}
impl SerializedSTVL for STVL {
    fn serialize(value: String) -> Vec<u8> {
        let mut byte_data = hex::decode(value).unwrap();
        serialize_varint(&mut byte_data)
    }

    fn parse() {

    }
}

///////////////////////////////////////////
pub trait SerializedSTAccount {
  fn serialize(value: String) -> Vec<u8>;
  fn parse();
}
//STAccount
pub struct STAccount {
    pub id: i32,
}
impl STAccount {
    pub fn new() -> Self {
        STAccount {
            id: 8,
        }
    }
}
impl SerializedSTAccount for STAccount {
  fn serialize(value: String) -> Vec<u8> {
      let mut byte_data = util::decode_j_address(value).unwrap();
      println!("STAccount : {:?}", byte_data);
      serialize_varint(&mut byte_data)
  }

  fn parse() {}
}


//STPathSet
pub struct STPathSet {
    pub id: i32,
    //
}
impl STPathSet {
    pub fn new() -> Self {
        STPathSet {
            id: 18,
        }
    }
}

//STVector256
pub struct STVector256 {
    pub id: i32,
}
impl STVector256 {
    pub fn new() -> Self {
        STVector256 {
            id: 19,
        }
    }
}

//STMemo
pub struct STMemo {
    // pub id: i32,
}
impl STMemo {
    pub fn new() -> Self {
        STMemo {
            // id: 20,
        }
    }
}

// lazy_static! {

//     pub struct TransactionTypes {
//         // pub map: HashMap<&'static str, i32> 
//     }
//     impl TransactionTypes {
//         pub fn get() {
//             println!("get");
//         }
//     }
//     pub static ref TRANSACTION_TYPES_G: TransactionTypes = {
//         TransactionTypes {
//         }
//     }

// }

/////////////////////////////////////////
//convert These three functions to global HashMap<&'static str, i8> .!!!
/*
 * return the transaction type in string
 * Data defined in the TRANSACTION_TYPES
*/
// pub fn get_transaction_type(tx_type: &str) -> i8 {
//     match tx_type {
//         "Payment"         => { return 0;   },
//         "AccountSet"      => { return 3;   },
//         "SetRegularKey"   => { return 5;   },
//         "OfferCreate"     => { return 7;   },
//         "OfferCancel"     => { return 8;   },
//         "Contract"        => { return 9;   },
//         "RemoveContract"  => { return 10;  },
//         "TrustSet"        => { return 20;  },
//         "RelationSet"     => { return 21;  },
//         "RelationDel"     => { return 22;  },
//         "ConfigContract"  => { return 30;  },
//         "EnableFeature"   => { return 100; },
//         "SetFee"          => { return 101; },

//         //Invalid transaction type!
//         _ => { return -1; }
//     }
// }



// /*
//  * return the transaction result in string
//  * Data defined in the TRANSACTION_RESULTS
//  *  tesSUCCESS               : 0,
//   tecCLAIM                 : 100,
//   tecPATH_PARTIAL          : 101,
//   tecUNFUNDED_ADD          : 102,
//   tecUNFUNDED_OFFER        : 103,
//   tecUNFUNDED_PAYMENT      : 104,
//   tecFAILED_PROCESSING     : 105,
//   tecDIR_FULL              : 121,
//   tecINSUF_RESERVE_LINE    : 122,
//   tecINSUF_RESERVE_OFFER   : 123,
//   tecNO_DST                : 124,
//   tecNO_DST_INSUF_SWT      : 125,
//   tecNO_LINE_INSUF_RESERVE : 126,
//   tecNO_LINE_REDUNDANT     : 127,
//   tecPATH_DRY              : 128,
//   tecMASTER_DISABLED       : 130,
//   tecNO_REGULAR_KEY        : 131,
//   tecOWNERS                : 132,
//   tecNO_ISSUER             : 133,
//   tecNO_AUTH               : 134,
//   tecNO_LINE               : 135,
//   tecINSUFF_FEE            : 136,
//   tecFROZEN                : 137,
//   tecNO_TARGET             : 138,
//   tecNO_PERMISSION         : 139,
//   tecNO_ENTRY              : 140,
//   tecINSUFFICIENT_RESERVE  : 141
// */
// pub fn get_transaction_result(result: &str) -> i32 {
//     match result {
//         "tesSUCCESS"                => { return 0;   },
//         "tecCLAIM"                  => { return 100; },
//         "tecPATH_PARTIAL"           => { return 101; },
//         "tecUNFUNDED_ADD"           => { return 102; },
//         "tecUNFUNDED_OFFER"         => { return 103; },
//         "tecUNFUNDED_PAYMENT"       => { return 104; },
//         "tecFAILED_PROCESSING"      => { return 105; },
//         "tecDIR_FULL"               => { return 121; },
//         "tecINSUF_RESERVE_LINE"     => { return 122; },
//         "tecINSUFFICIENT_RESERVE"   => { return 141; },
        
//         //Invalid transaction result
//         _ => { return -1; },
//     }
// }


/*
 * return the transaction type in string
 * Data defined in the ledger entry:
  AccountRoot: [97].concat(sleBase,[
  Contract: [99].concat(sleBase,[
  DirectoryNode: [100].concat(sleBase,[
  EnabledFeatures: [102].concat(sleBase,[
  FeeSettings: [115].concat(sleBase,[
  GeneratorMap: [103].concat(sleBase,[
  LedgerHashes: [104].concat(sleBase,[
  Nickname: [110].concat(sleBase,[
  Offer: [111].concat(sleBase,[
  SkywellState: [114].concat(sleBase,[

  TODO: add string input handles
*/
// pub fn get_ledger_entry_type(entry_type: i8) -> &'static str {
//     match entry_type {
//         97  => { return "AccountRoot";      },
//         99  => { return "Contract";         },
//         100 => { return "DirectoryNode";    },
//         102 => { return "EnabledFeatures";  },
//         115 => { return "FeeSettings";      },
//         103 => { return "GeneratorMap";     },
//         104 => { return "LedgerHashes";     },
//         110 => { return "Nickname";         },
//         111 => { return "Offer";            },
//         114 => { return "SkywellState";     },

//         //Invalid input type for ransaction result!
//         _ => { return "Invalid"; },
//     }   
// }


