use crate::base::serialize::signed_obj::{
    SignedTxJson, 
    TxJsonBuilder,  
    TxJsonSigningPubKeyBuilder, 
    TxJsonRelationTypeBuilder,
    TxJsonFlagsBuilder, 
    TxJsonFeeBuilder, 
    TxJsonTransactionTypeBuilder, 
    TxJsonAccountBuilder, 
    TxJsonSequenceBuilder,
    TxJsonTargetBuilder, 
    TxJsonLimitAmountBuilder,
};
use crate::base::data::constants::{
    TX_FLAGS, 
    TX_FEE, 
    TX_ACCOUNT, 
    TX_TRANSACTION_TYPE, 
    TX_SEQUENCE, 
    TX_SIGNING_PUB_KEY,
    TX_RELATION_TYPE, 
    TX_TARGET, 
    TX_LIMIT_AMOUNT
};
use std::rc::Rc;
use crate::base::wallet::keypair::*;
use crate::base::local_sign::sign_tx::{SignTx, PRE_FIELDS};
use crate::base::{G_TRANSACTION_TYPE_MAP, TWHashMap};
use crate::message::transaction::relation::RelationTxJson;

pub trait FormatSignTxJson {
    fn prepare(&mut self, sign_tx: &SignTx);
    fn format(&mut self);
}

pub struct SignTxRelate <'a> {
    pub fields  : Vec<&'a str>,
    pub keypair : &'a Keypair,
    pub tx_json : &'a RelationTxJson,
    pub sequence: u32,
    pub output  : SignedTxJson<'a>,
}

impl <'a> SignTxRelate <'a> {
    pub fn with_params(keypair: &'a Keypair, tx_json: &'a RelationTxJson, sequence: u32) -> Self {
        let mut pre = vec![];
        pre.extend_from_slice(&PRE_FIELDS);

        SignTxRelate {
            fields  : pre,
            keypair : keypair,
            tx_json : tx_json,
            sequence: sequence,
            output  : SignedTxJson::new(),
        }
    }

    //output blob which is signed.
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

impl <'a> FormatSignTxJson for SignTxRelate <'a> {
    fn prepare(&mut self, sign_tx: &SignTx) {
        sign_tx.update(&mut self.fields, TX_RELATION_TYPE);
        sign_tx.update(&mut self.fields, TX_TARGET);
        sign_tx.update(&mut self.fields, TX_LIMIT_AMOUNT);
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
                TX_SIGNING_PUB_KEY => {
                    let value = String::from( self.keypair.public_key.as_str() );
                    let signing_pub_key = TxJsonSigningPubKeyBuilder::new(value).build();
                    self.output.insert(index, signing_pub_key);
                },
                TX_ACCOUNT => {
                    let value = String::from(tx_json.account.as_str());
                    let account = TxJsonAccountBuilder::new(value).build();
                    self.output.insert(index, account);
                },
                TX_SEQUENCE => {
                    let value = self.sequence;
                    let amount = TxJsonSequenceBuilder::new(value).build();
                    self.output.insert(index, amount);
                },
                TX_LIMIT_AMOUNT => {
                    let value = &tx_json.limit_amount;
                    let amount = TxJsonLimitAmountBuilder::new(&value).build();
                    self.output.insert(index, amount);
                },
                TX_TARGET => {
                    let value = String::from( tx_json.target.as_str() );
                    let account = TxJsonTargetBuilder::new(value).build();
                    self.output.insert(index, account);
                },
                TX_RELATION_TYPE => {
                    let value = tx_json.relation_type;
                    let account = TxJsonRelationTypeBuilder::new(value).build();
                    self.output.insert(index, account);
                },

                _ => {
                    panic!("pppppppppppppppppppppppnic.................");
                }
            }

            index += 1;
        }
    }
}
