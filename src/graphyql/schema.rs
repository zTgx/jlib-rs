
use juniper::FieldResult;
use juniper::RootNode;
use juniper::EmptyMutation;

use std::rc::Rc;
use std::cell::Cell;

use crate::misc::config::*;
use crate::api::query::server_info::*;

use crate::graphyql::model::*;
use crate::graphyql::util::{
    downcast_to_serverinfo,
};

pub struct QueryRoot;

graphql_object!(QueryRoot: () |&self| {
    field server_info(&executor) -> FieldResult<ServerInfoResponse> {
        let s = Rc::new( Cell::new( ServerInfoResponse::default() ) );
        //let mut done = Rc::new( Cell::new( false ));

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

                    //done.set(true);
                }
                Err(_) => {
                }
            });
        }

        //while ! downcast_to_bool(done.clone()) {}

        Ok( downcast_to_serverinfo(s) )
    }
});

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>>;
pub fn create_schema() -> Schema {
    Schema::new( QueryRoot {}, EmptyMutation::<()>::new())
}
