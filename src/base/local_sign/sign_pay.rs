use crate::base::data::constants::{
    TX_DESTINATION, 
    TX_ACCOUNT, 
    TX_SIGNING_PUB_KEY, 
    TX_FEE,
    TX_AMOUNT, 
    TX_SEQUENCE, 
    TX_TRANSACTION_TYPE, 
    TX_FLAGS, 
    TX_MEMOS,
};
use std::rc::Rc;
use crate::base::serialize::signed_obj::*;
use crate::wallet::keypair::Keypair;
use crate::api::payment::data::{TxJson};
use crate::base::{G_TRANSACTION_TYPE_MAP, TWHashMap};
use crate::base::local_sign::sign_tx::{SignTx, PRE_FIELDS};

pub trait FormatSignTxJson {
    fn prepare(&mut self, sign_tx: &SignTx);
    fn format(&mut self);
}

pub struct SignTxPay <'a> {
    pub fields  : Vec<&'a str>,
    pub keypair : &'a Keypair,
    pub tx_json : &'a TxJson,
    pub sequence: u32,

    pub output  : SignedTxJson<'a>,
}

impl <'a> SignTxPay <'a> {
    pub fn with_params(keypair: &'a Keypair, tx_json: &'a TxJson, sequence: u32) -> Self {
        let mut pre = vec![];
        pre.extend_from_slice(&PRE_FIELDS);

        SignTxPay {
            fields  : pre,
            keypair : keypair,
            tx_json : tx_json,
            sequence: sequence,
            output  : SignedTxJson::new(),
        }
    }

    pub fn build(&mut self, sign_tx: &SignTx) -> String {
        //Step 1
        self.prepare(&sign_tx);

        //Step 2
        self.format();

        //Step 3
        sign_tx.get_txn_signature(&mut self.fields, &mut self.output);

        //Step 4
        sign_tx.get_blob(&mut self.output)
    }
}
impl <'a> SignTxPay <'a> {
    pub fn prepare(&mut self, sign_tx: &SignTx) {
        sign_tx.update(&mut self.fields, TX_AMOUNT);
        sign_tx.update(&mut self.fields, TX_DESTINATION);
        sign_tx.update(&mut self.fields, TX_MEMOS);
    }

    fn format(&mut self) {
        let tx_json_rc = Rc::new (self.tx_json);

        let mut index = 0;
        for &key in &self.fields {
            let tx_json = tx_json_rc.clone();
            match key {
                TX_FLAGS => {
                    let value = tx_json.flags;
                    let flags = TxJsonFlagsBuilder::new(value).build();
                    self.output.insert(index, flags);
                },
                TX_FEE => {
                    let value = tx_json.fee;
                    let fee = TxJsonFeeBuilder::new(value).build();
                    self.output.insert(index, fee);
                },
                TX_TRANSACTION_TYPE => {
                    let value = *G_TRANSACTION_TYPE_MAP.get_value_from_key(&tx_json.transaction_type).unwrap();
                    let transaction_type = TxJsonTransactionTypeBuilder::new(value).build();
                    self.output.insert(index, transaction_type);
                },
                TX_ACCOUNT => {
                    let value = String::from(tx_json.account.as_str());
                    let account = TxJsonAccountBuilder::new(value).build();
                    self.output.insert(index, account);
                },
                TX_AMOUNT => {
                    let value = &tx_json.amount;
                    let amount = TxJsonAmountBuilder::new(&value).build();
                    self.output.insert(index, amount);
                },
                TX_DESTINATION => {
                    let value = String::from(tx_json.destination.as_str());
                    let destination = TxJsonDestinationBuilder::new(value).build();
                    self.output.insert(index, destination);
                },
                TX_SEQUENCE => {
                    let value = tx_json.sequence;
                    let sequence = TxJsonSequenceBuilder::new(value).build();
                    self.output.insert(index, sequence);
                },

                TX_SIGNING_PUB_KEY => {
                    let value = String::from( self.keypair.public_key.as_str() );
                    let signing_pub_key = TxJsonSigningPubKeyBuilder::new(value).build();
                    self.output.insert(index, signing_pub_key);
                },

                TX_MEMOS => {
                    if let Some(value) = &tx_json.memo {
                        let mut v: Vec<String> = vec![];
                        let mut i = 0;
                        while i < value.len() {
                            let x = String::from( value[i].memo.memo_data.memo_data.as_str() );
                            v.push( x );

                            i += 1;
                        }

                        let tx_memos = TxJsonMemosBuilder::new(v).build();
                        self.output.insert(index, tx_memos);
                    }
                },
                _ => {
                    panic!("pppppppppppppppppppppppnic.................");
                }
            }

            index += 1;
        }
    }
}
