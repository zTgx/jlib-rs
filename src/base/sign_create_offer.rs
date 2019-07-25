use std::rc::Rc;

use crate::message::transaction::offer_create::{OfferCreateTxJson};

use crate::base::signed_obj::{
    SignedTxJson, TxJsonBuilder,  TxJsonSigningPubKeyBuilder, 
    TxJsonFlagsBuilder, TxJsonFeeBuilder, TxJsonTransactionTypeBuilder, TxJsonAccountBuilder, TxJsonSequenceBuilder, 
    TxJsonTakerBuilder,
};

use crate::base::constants::{
    TX_FLAGS, TX_FEE, TX_ACCOUNT, TX_TRANSACTION_TYPE, TX_SEQUENCE, TX_SIGNING_PUB_KEY, TX_TAKERPAYS, TX_TAKERGETS,
    TXTakerType,
};

use crate::base::keypair::*;

use crate::base::sign_tx::{SignTx, PRE_FIELDS};

pub trait FormatSignTxJson {
    fn prepare(&mut self, sign_tx: &SignTx);
    fn format(&mut self, tx: &mut SignedTxJson);
}

pub struct SignTxCreateOffer <'a> {
    pub fields : Vec<&'a str>,
    pub keypair: &'a Keypair,
    pub tx_json: &'a OfferCreateTxJson,

    pub sequence: u32,
}

impl <'a> SignTxCreateOffer <'a> {
    pub fn with_params(keypair: &'a Keypair, tx_json: &'a OfferCreateTxJson, sequence: u32) -> Self {
        let mut pre = vec![];
        pre.extend_from_slice(&PRE_FIELDS);

        SignTxCreateOffer {
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

impl <'a> FormatSignTxJson for SignTxCreateOffer <'a> {
    fn prepare(&mut self, sign_tx: &SignTx) {
        sign_tx.update(&mut self.fields, TX_TAKERGETS);
        sign_tx.update(&mut self.fields, TX_TAKERPAYS);
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
                    let value = 8u16;//tx_json.transaction_type;
                    let transaction_type = TxJsonTransactionTypeBuilder::new(value).build();
                    output.insert(index, transaction_type);
                },
                TX_SIGNING_PUB_KEY => {
                    let value = String::from( self.keypair.property.public_key.as_str() );
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
                TX_TAKERPAYS => {
                    let value = &tx_json.taker_pays;
                    println!("tker pays amount: {:?}", &value);

                    let amount = TxJsonTakerBuilder::new(TXTakerType::Pays, &value).build();
                    output.insert(index, amount);
                },
                TX_TAKERGETS => {
                    let value = &tx_json.taker_gets;
                    println!("taker gets amount: {:?}", &value);

                    let amount = TxJsonTakerBuilder::new(TXTakerType::Gets, &value).build();
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
