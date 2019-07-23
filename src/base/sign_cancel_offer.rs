// use crate::base::sign::*;
// use ring::{digest};

use std::rc::Rc;

use crate::base::sign_tx::{SignTx};
use crate::message::transaction::offer_cancel::{OfferCancelTxJson};
use crate::base::signed_obj::{
    SignedTxJson, TxJsonBuilder, 
    TxJsonFlagsBuilder, TxJsonFeeBuilder, TxJsonTransactionTypeBuilder, TxJsonAccountBuilder, TxJsonAmountBuilder, TxJsonTxnSignatureBuilder,
};

use crate::base::constants::{
    TX_FLAGS, TX_FEE, TX_ACCOUNT, TX_TRANSACTION_TYPE, TX_SEQUENCE, 
};

use crate::base::keypair::*;
use crate::base::sign::{SignatureX};

pub trait FormatSignTxJson {
    fn prepare(&mut self);
    fn format(&mut self, tx: &mut SignedTxJson);
}

static PRE_FIELDS: [&'static str; 4] = ["Flags", "Fee", "TransactionType", "Account"];

pub struct SignTxCancelOffer <'a> {
    pub fields : Vec<&'a str>,
    pub keypair: &'a Keypair,
    pub tx_json: &'a OfferCancelTxJson,
}

impl <'a> SignTxCancelOffer <'a> {
    pub fn with_params(keypair: &'a Keypair, tx_json: &'a OfferCancelTxJson) -> Self {
        let mut pre = vec![];
        pre.extend_from_slice(&PRE_FIELDS);
        SignTxCancelOffer {
            fields : pre,
            keypair: keypair,
            tx_json: tx_json,
        }
    }

    //output blob which is signed.
    pub fn build(&mut self) -> String {
        //Step 1
        self.prepare();

        //Step 2
        let mut output: SignedTxJson = SignedTxJson::new();
        self.format(&mut output);

        //Step 3
        self.update_txn_signature(&mut output);

        //Step 4
        SignTxCancelOffer::calc_blob(&mut output)
    }

    pub fn update_txn_signature(&mut self, signed_tx_json: &mut SignedTxJson) {
        let output: Vec<u8> = signed_tx_json.serialize();

        let signature_x = SignatureX::new(&self.keypair);
        let txn_signature= signature_x.sign_txn_signature(&output);
        println!("txn_signature: {}", txn_signature);

        self.update("TxnSignature");

        let mut index = 0;
        for x in &self.fields {
            if *x == "TxnSignature" {
                break;
            }

            index += 1;
        }

        let txn_signature = TxJsonTxnSignatureBuilder::new(txn_signature).build();
        signed_tx_json.insert(index, txn_signature);
    }

    pub fn calc_blob(signed_tx_json: &mut SignedTxJson) -> String {
        let output: Vec<u8> = signed_tx_json.serialize();
        hex::encode(&output).to_ascii_uppercase()
    }

    pub fn update(&mut self, field: &'a str) {
        self.fields.push(field);
        SignTx::sort_fields(&mut self.fields);
    }
}

impl <'a> FormatSignTxJson for SignTxCancelOffer <'a> {
    fn prepare(&mut self) {
        if self.tx_json.offer_sequence != 0 {
            self.update("Sequence")
        }
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
                    println!("value: {}", tx_json.transaction_type);

                    let value = 0u16;//tx_json.transaction_type;
                    let transaction_type = TxJsonTransactionTypeBuilder::new(value).build();
                    output.insert(index, transaction_type);
                },
                TX_ACCOUNT => {
                    let value = String::from(tx_json.account.as_str());
                    let account = TxJsonAccountBuilder::new(value).build();
                    output.insert(index, account);
                },
                TX_SEQUENCE => {
                    let value = tx_json.offer_sequence.to_string();

                    let amount = TxJsonAmountBuilder::new(value).build();
                    output.insert(index, amount);
                },

                _ => {
                    panic!("pppppppppppppppppppppppnic.................");
                }
            }

            index += 1;
        }
    }









}
