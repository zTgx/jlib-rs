use crate::message::transaction::set_brokerage::{SetBrokerageTxJson};
use crate::base::serialize::signed_obj::*;

use crate::base::data::constants::{
    TX_ACCOUNT, TX_SIGNING_PUB_KEY, TX_FEE,
    TX_AMOUNT, TX_SEQUENCE, TX_TRANSACTION_TYPE, TX_FLAGS,
    TX_RATE_DEN, TX_RATE_NUM, TX_FEE_ACCOUNT,
};

use std::rc::Rc;
use crate::base::wallet::keypair::{Keypair};
use crate::base::local_sign::sign_tx::{SignTx, PRE_FIELDS};
use crate::base::{G_TRANSACTION_TYPE_MAP, TWHashMap};

pub trait FormatSignTxJson {
    fn prepare(&mut self, sign_tx: &SignTx);
    fn format(&mut self);
}

pub struct SignTxBrokerage <'a> {
    pub fields : Vec<&'a str>,
    pub keypair: &'a Keypair,
    pub tx_json: &'a SetBrokerageTxJson,

    pub sequence: u32,

    pub output: SignedTxJson<'a>,
}

impl <'a> SignTxBrokerage <'a> {
    pub fn with_params(keypair: &'a Keypair, tx_json: &'a SetBrokerageTxJson, sequence: u32) -> Self {
        let mut pre = vec![];
        pre.extend_from_slice(&PRE_FIELDS);

        SignTxBrokerage {
            fields : pre,
            keypair: keypair,
            tx_json: tx_json,

            sequence: sequence,

            output: SignedTxJson::new(),
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
impl <'a> SignTxBrokerage <'a> {
    pub fn prepare(&mut self, sign_tx: &SignTx) {
        //Pay specific fields
        sign_tx.update(&mut self.fields, TX_AMOUNT);
        sign_tx.update(&mut self.fields, TX_RATE_DEN);
        sign_tx.update(&mut self.fields, TX_RATE_NUM);
        sign_tx.update(&mut self.fields, TX_FEE_ACCOUNT);
    }

    fn format(&mut self) {
        let tx_json_rc = Rc::new ( self.tx_json );

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
                    let value = &tx_json.manage_account;
                    let account = TxJsonAccountBuilder::new(value.to_string()).build();
                    self.output.insert(index, account);
                },
                TX_AMOUNT => {
                    let value = &tx_json.amount;
                    let amount = TxJsonAmountBuilder::new(&value).build();
                    self.output.insert(index, amount);
                },
                TX_RATE_DEN => {
                    let value = tx_json.offer_feerate_den;
                    let den = TxJsonBrokerageDenBuilder::new(value).build();
                    self.output.insert(index, den);
                },
                TX_RATE_NUM => {
                    let value = tx_json.offer_feerate_num;
                    let num = TxJsonBrokerageNumBuilder::new(value).build();
                    self.output.insert(index, num);
                },
                TX_FEE_ACCOUNT => {
                    let value = &tx_json.fee_account;
                    let account = TxJsonFeeAccountBuilder::new(value).build();
                    self.output.insert(index, account);
                },
                TX_SEQUENCE => {
                    let value = tx_json.sequence;
                    let sequence = TxJsonSequenceBuilder::new(value).build();
                    self.output.insert(index, sequence);
                },

                TX_SIGNING_PUB_KEY => {
                    let value = &self.keypair.public_key;
                    let signing_pub_key = TxJsonSigningPubKeyBuilder::new(value.to_string()).build();
                    self.output.insert(index, signing_pub_key);
                },

                _ => {
                    panic!("pppppppppppppppppppppppnic.................");
                },
            }

            index += 1;
        }
    }
}
