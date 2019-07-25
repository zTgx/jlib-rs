use crate::message::transaction::transaction::{TxJson};
use crate::base::signed_obj::*;

use crate::base::constants::{
    TX_DESTINATION, TX_ACCOUNT, TX_SIGNING_PUB_KEY, TX_FEE,
    TX_AMOUNT, TX_SEQUENCE, TX_TRANSACTION_TYPE, TX_FLAGS, TX_MEMOS,
};
use std::rc::Rc;
use crate::base::keypair::{Keypair};
use crate::base::sign_tx::{SignTx, PRE_FIELDS};
use crate::base::{G_TRANSACTION_TYPE_MAP, TWHashMap};

pub trait FormatSignTxJson {
    fn prepare(&mut self, sign_tx: &SignTx);
    fn format(&mut self, tx: &mut SignedTxJson);
}

pub struct SignTxPay <'a> {
    pub fields : Vec<&'a str>,
    pub keypair: &'a Keypair,
    pub tx_json: &'a TxJson,

    pub sequence: u32,
}

impl <'a> SignTxPay <'a> {
    pub fn with_params(keypair: &'a Keypair, tx_json: &'a TxJson, sequence: u32) -> Self {
        let mut pre = vec![];
        pre.extend_from_slice(&PRE_FIELDS);

        SignTxPay {
            fields : pre,
            keypair: keypair,
            tx_json: tx_json,

            sequence: sequence,
        }
    }

    pub fn build(&mut self, sign_tx: &SignTx) -> String {
        //Step 1
        self.prepare(&sign_tx);

        //Step 2
        let mut output: SignedTxJson = SignedTxJson::new();
        self.format(&mut output);

        //Step 3
        sign_tx.get_txn_signature(&mut self.fields, &mut output);

        //Step 4
        sign_tx.get_blob(&mut output)
    }
}
impl <'a> SignTxPay <'a> {
    pub fn prepare(&mut self, sign_tx: &SignTx) {
        //Pay specific fields
        sign_tx.update(&mut self.fields, TX_AMOUNT);
        sign_tx.update(&mut self.fields, TX_DESTINATION);
        sign_tx.update(&mut self.fields, TX_MEMOS);
    }

    fn format(&mut self, output: &mut SignedTxJson) {
        let tx_json_rc = Rc::new ( self.tx_json );

        let mut index = 0;
        for &key in &self.fields {
            let tx_json = tx_json_rc.clone();
            println!("key : {} ", key);
            match key {
                TX_FLAGS => {
                    let value = tx_json.flags;
                    let flags = TxJsonFlagsBuilder::new(value).build();
                    output.insert(index, flags);
                },
                TX_FEE => {
                    let value = tx_json.fee;
                    let fee = TxJsonFeeBuilder::new(value.to_string()).build();
                    output.insert(index, fee);
                },
                TX_TRANSACTION_TYPE => {
                    let value = *G_TRANSACTION_TYPE_MAP.get_value_from_key(&tx_json.transaction_type).unwrap();
                    let transaction_type = TxJsonTransactionTypeBuilder::new(value).build();
                    output.insert(index, transaction_type);
                },
                TX_ACCOUNT => {
                    let value = String::from(tx_json.account.as_str());
                    let account = TxJsonAccountBuilder::new(value).build();
                    output.insert(index, account);
                },
                TX_AMOUNT => {
                    let value = String::from(tx_json.amount.as_str());

                    let amount = TxJsonAmountBuilder::new(value).build();
                    output.insert(index, amount);
                },
                TX_DESTINATION => {
                    let value = String::from(tx_json.destination.as_str());
                    let destination = TxJsonDestinationBuilder::new(value).build();
                    output.insert(index, destination);
                },
                TX_SEQUENCE => {
                    let value = tx_json.sequence;
                    let sequence = TxJsonSequenceBuilder::new(value).build();
                    output.insert(index, sequence);
                },

                TX_SIGNING_PUB_KEY => {
                    let value = String::from( self.keypair.property.public_key.as_str() );
                    let signing_pub_key = TxJsonSigningPubKeyBuilder::new(value).build();
                    output.insert(index, signing_pub_key);
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
                        output.insert(index, tx_memos);
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
