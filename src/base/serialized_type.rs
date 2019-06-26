use typename::TypeName;

pub trait SerializedType {
  fn serialize(&self) -> Vec<u8>;
  fn parse(&self);
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

impl SerializedType for STInt8 {
  fn serialize(&self) -> Vec<u8> {
    self.value.to_be_bytes().to_vec()
  }

  fn parse(&self) {}
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

impl SerializedType for STInt16 {
  fn serialize(&self) -> Vec<u8> {
    self.value.to_be_bytes().to_vec()
  }

  fn parse(&self) {}
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

impl SerializedType for STInt32 {
  fn serialize(&self) -> Vec<u8> {
    self.value.to_be_bytes().to_vec()
  }

  fn parse(&self) {}
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


