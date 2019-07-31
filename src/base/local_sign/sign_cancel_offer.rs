use std::rc::Rc;

use crate::message::transaction::offer_cancel::{OfferCancelTxJson};

use crate::base::serialize::signed_obj::{
    SignedTxJson, TxJsonBuilder,  TxJsonSigningPubKeyBuilder, TxJsonOfferSequenceBuilder,
    TxJsonFlagsBuilder, TxJsonFeeBuilder, TxJsonTransactionTypeBuilder, TxJsonAccountBuilder, TxJsonSequenceBuilder,
};

use crate::base::data::constants::{
    TX_FLAGS, TX_FEE, TX_ACCOUNT, TX_TRANSACTION_TYPE, TX_SEQUENCE, TX_SIGNING_PUB_KEY, TX_OFFER_SEQUENCE,
};

use crate::base::wallet::keypair::*;

use crate::base::local_sign::sign_tx::{SignTx, PRE_FIELDS};
use crate::base::{G_TRANSACTION_TYPE_MAP, TWHashMap};

pub trait FormatSignTxJson {
    fn prepare(&mut self, sign_tx: &SignTx);
    fn format(&mut self, tx: &mut SignedTxJson);
}

pub struct SignTxCancelOffer <'a> {
    pub fields : Vec<&'a str>,
    pub keypair: &'a Keypair,
    pub tx_json: &'a OfferCancelTxJson,

    pub sequence: u32,
}

impl <'a> SignTxCancelOffer <'a> {
    pub fn with_params(keypair: &'a Keypair, tx_json: &'a OfferCancelTxJson, sequence: u32) -> Self {
        let mut pre = vec![];
        pre.extend_from_slice(&PRE_FIELDS);

        SignTxCancelOffer {
            fields : pre,
            keypair: keypair,
            tx_json: tx_json,

            sequence: sequence,
        }
    }

    //output blob which is signed.
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

impl <'a> FormatSignTxJson for SignTxCancelOffer <'a> {
    fn prepare(&mut self, sign_tx: &SignTx) {
        sign_tx.update(&mut self.fields, TX_OFFER_SEQUENCE)
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
                TX_SIGNING_PUB_KEY => {
                    let value = String::from( self.keypair.public_key.as_str() );
                    let signing_pub_key = TxJsonSigningPubKeyBuilder::new(value).build();
                    output.insert(index, signing_pub_key);
                },
                TX_ACCOUNT => {
                    let value = String::from(tx_json.account.as_str());
                    let account = TxJsonAccountBuilder::new(value).build();
                    output.insert(index, account);
                },
                TX_SEQUENCE => {
                    let value = self.sequence;
                    let amount = TxJsonSequenceBuilder::new(value).build();
                    output.insert(index, amount);
                },
                TX_OFFER_SEQUENCE => {
                    let value = tx_json.offer_sequence as u32;
                    let amount = TxJsonOfferSequenceBuilder::new(value).build();
                    output.insert(index, amount);
                }
                _ => {
                    panic!("pppppppppppppppppppppppnic.................");
                }
            }

            index += 1;
        }
    }









}
