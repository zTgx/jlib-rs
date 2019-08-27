
use juniper::FieldResult;
use juniper::RootNode;

use serde::{Deserialize, Serialize};

use std::rc::Rc;
use std::cell::Cell;
use std::any::Any;

use crate::misc::config::*;
use crate::api::query::server_info::*;

#[derive(GraphQLEnum)]
enum Episode {
    NewHope,
    Empire,
    Jedi,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct Human {
    id: String,
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct NewHuman {
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

#[derive(GraphQLObject)]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ServerInfoResponse {
    pub build_version: String,
    pub complete_ledgers: String,
    pub hostid: String,
}


pub fn downcast_to_serverinfo(value: Rc<dyn Any>) -> ServerInfoResponse {
    let mut us = ServerInfoResponse::default();
    if let Ok(u) = value.downcast::<Cell<ServerInfoResponse>>() {
        us = u.take();
    }

    us
}


pub struct QueryRoot;

graphql_object!(QueryRoot: () |&self| {
    field human(&executor, id: String) -> FieldResult<Human> {
        Ok(Human{
            id: "1234".to_owned(),
            name: "Luke".to_owned(),
            appears_in: vec![Episode::NewHope],
            home_planet: "Mars".to_owned(),
        })
    }

    field server_info(&executor) -> FieldResult<ServerInfoResponse> {
        let s = Rc::new( Cell::new( ServerInfoResponse::default() ) );
        let config = Config::new(TEST1, true);
        ServerInfo::new().request_server_info(config.clone(), |x| match x {
            Ok(response) => {
                println!("build_version : {:?}", response.build_version);

                let ns = ServerInfoResponse {
                    build_version: response.build_version,
                    complete_ledgers: response.complete_ledgers,
                    hostid: response.hostid,
                };

                s.set(ns);

            }
            Err(_) => {
            }
        });

        Ok( downcast_to_serverinfo(s) )
    }
});

pub struct MutationRoot;

graphql_object!(MutationRoot: () |&self| {
    field createHuman(&executor, new_human: NewHuman) -> FieldResult<Human> {
        Ok(Human{
            id: "1234".to_owned(),
            name: new_human.name,
            appears_in: new_human.appears_in,
            home_planet: new_human.home_planet,
        })
    }
});

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
