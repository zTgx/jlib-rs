
use juniper::FieldResult;
use juniper::RootNode;
use juniper::EmptyMutation;

use std::rc::Rc;
use std::cell::Cell;

use crate::misc::config::*;
use crate::api::query::server_info::*;

use crate::graphyql::model::*;
use crate::graphyql::util::*;

use crate::WalletType;
use crate::generate_wallet;

use crate::api::query::spec_tx::*;
use crate::message::query::spec_tx::{RequestTxResponse};
use crate::api::query::ledger_closed::*;

pub struct QueryRoot;

graphql_object!(QueryRoot: () |&self| {
    field new_wallet(&executor) -> FieldResult<Wallet> {
        let wallet_raw = generate_wallet(WalletType::SECP256K1);

        let keypair = Keypair {
            private_key: wallet_raw.keypair.private_key,
            public_key: wallet_raw.keypair.public_key,
        };

        let wallet = Wallet {
            key_type: WType::SECP256K1,
            address : wallet_raw.address,
            secret  : wallet_raw.secret,
            keypair : keypair,
        };

        Ok( wallet )
    }

    field ledger_info(&executor, address: String) -> FieldResult<LedgerInfo> {
        let s = Rc::new( Cell::new( LedgerInfo::default() ) );

        let config = Config::new(TEST3, true);
        let _c = LedgerClosed::new().request_ledger_closed(config.clone(), |x| match x {
            Ok(response) => {
                let info = LedgerInfo {
                    ledger_hash: response.ledger_hash,
                    ledger_index: response.ledger_index.to_string(),
                };

                s.set( info );
            }

            Err(e) => {}
        });

        Ok( downcast_to_ledgerinfo(s) )
    }

    field tx_info(&executor, tx_hash: String) -> FieldResult<PaymentInfo> {
        let s = Rc::new( Cell::new( PaymentInfo::default() ) );

        let config = Config::new(TEST3, true);
        SpecTx::new().request_tx(config.clone(), tx_hash, |x| match x {
            Ok(response) => {
                let res: RequestTxResponse = response;

                let mut memos: Vec<String> = vec![];
                if let Some(m) = res.memos {
                    for i in m {
                        memos.push( i );
                    }
                }

                let amount = AmountG {
                    value: res.taker_pays.value,
                    currency: res.taker_pays.currency.unwrap(),
                    issuer: res.taker_pays.issuer.unwrap(),
                };
                let info = PaymentInfo {
                    hash: res.hash,
                    fee: res.fee,
                    date: res.date.to_string(),
                    memos: memos,
                    counterparty: res.account,
                    amount: amount,
                };

                s.set( info );
            },

            Err(e) => {
            }
        });

        Ok( downcast_to_paymentinfo(s) )
    }

    field server_info(&executor) -> FieldResult<ServerInfoResponse> {
        let s = Rc::new( Cell::new( ServerInfoResponse::default() ) );
        {
            let config = Config::new(TEST1, true);
            ServerInfo::new().request_server_info(config.clone(), |x| match x {
                Ok(response) => {
                    let last_close = LastClose {
                                    converge_time_s:response.last_close.converge_time_s.to_string(),
                                    proposers: response.last_close.proposers.to_string()
                                };
                    let valid_ledger=ValidatedLedger {
                        age: response.validated_ledger.age.to_string(),
                        base_fee_swt: response.validated_ledger.base_fee_swt,
                        fee_account_swt: response.validated_ledger.fee_account_swt,
                        hash: response.validated_ledger.hash,
                        issuerop_account: response.validated_ledger.issuerop_account,
                        manager_account: response.validated_ledger.manager_account,
                        reserve_base_swt: response.validated_ledger.reserve_base_swt.to_string(),
                        reserve_inc_swt: response.validated_ledger.reserve_inc_swt.to_string(),
                        seq: response.validated_ledger.seq.to_string(),
                    };

                    let info = ServerInfoResponse {
                        build_version: response.build_version,
                        complete_ledgers: response.complete_ledgers,
                        hostid: response.hostid,
                        io_latency_ms: response.io_latency_ms.to_string(),
                        last_close: last_close,

                        load_factor: response.load_factor.to_string(),
                        peers: response.peers.to_string(),
                        pubkey_node: response.pubkey_node,
                        server_state: response.server_state,
                        startup_time: response.startup_time,
                        validated_ledger: valid_ledger,
                        validation_quorum: response.validation_quorum.to_string(),
                    };

                    s.set(info);
                }
                Err(_) => {
                }
            });
        }

        Ok( downcast_to_serverinfo(s) )
    }
});

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>>;
pub fn create_schema() -> Schema {
    Schema::new( QueryRoot {}, EmptyMutation::<()>::new())
}
