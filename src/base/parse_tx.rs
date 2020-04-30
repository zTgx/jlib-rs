#![allow(unused)]

use serde_json::json;
use cast_rs::hex;
use crate::base::base_config::{CURRENCY};
use serde_json::{Value};

static OFFSET_SECOND: i64 = 946684800;

pub fn parse_amount(x: &Value) -> Value {
    let mut result: Value = json!({"value": "0"});

    if x.is_string() { //SWT
        result["currency"] = json!("SWT");
        result["issuer"] = json!("");

        result["value"] = json!(x.as_str().unwrap().parse::<f32>().unwrap() / 1000000.0);
    } else {
        result["value"] = json!(x["value"].as_str().unwrap().parse::<f32>().unwrap());// x["value"].clone();
        result["currency"] = x["currency"].clone();
        result["issuer"] = x["issuer"].clone();
    }

    result
}

pub fn reverse_amount(x: &Value) -> Value {
    let mut result: Value = json!({"value": "0"});

    result["value"] = json!( "-".to_owned() + x["LimitAmount"]["value"].as_str().unwrap() );
    result["currency"] = x["LimitAmount"]["currency"].clone();
    result["issuer"] = x["Account"].clone();

    result
}

pub fn process_affect_node(an: &Value, result: &mut Value) {
    if ! an["CreatedNode"].is_null() {
        result.as_object_mut().unwrap()["diffType"] = json!("CreatedNode");
    }

    if ! an["ModifiedNode"].is_null() {
        result.as_object_mut().unwrap()["diffType"] = json!("ModifiedNode");
    }

    if ! an["DeletedNode"].is_null() {
        result.as_object_mut().unwrap()["diffType"] = json!("DeletedNode");
    }

    if result["diffType"].is_null() {
        return;
    }

    let an = an[result["diffType"].clone().as_str().unwrap()].clone();

    result["entryType"] = an["LedgerEntryType"].clone();
    result["ledgerIndex"] = an["LedgerIndex"].clone();

    result["fields"] = an["FinalFields"].clone();

    result["fieldsPrev"] = an["PreviousFields"].clone();
    result["fieldsNew"] = an["NewFields"].clone();
    result["fieldsFinal"] = an["FinalFields"].clone();
    result["PreviousTxnID"] = an["PreviousTxnID"].clone();
}

pub fn is_amount_zero(amount: &Value) -> bool {
    if amount.is_null() {
        return false;
    }

    return amount.as_f64().unwrap() < std::f64::EPSILON;
}

pub fn amount_subtract(amount1: &Value, amount2: &Value) -> Value {
    if amount1.is_null() { return amount2.clone(); }
    if amount2.is_null() { return amount1.clone(); }

    let mut v: Value = amount1.clone();
    v["currency"] = amount1["currency"].clone();
    v["issuer"] = amount1["issuer"].clone();
    v["value"] = json!( amount1["value"].as_f64().unwrap() - amount2["value"].as_f64().unwrap() );

    v
}

pub fn amount_ratio(amount1: &Value, amount2: &Value) -> Value {
    let v1 = amount1["value"].clone().as_f64().unwrap();
    let v2 = amount2["value"].clone().as_f64().unwrap();

    let result: Value = json!( v1/v2 );

    result
}

pub fn get_price(effect: &Value, funded: bool) -> Value {
    let mut g = effect.clone();
    if effect["got"].is_null() {
        g = effect["pays"].clone();
    } else {
        g = effect["got"].clone();
    }

    let mut p = effect.clone();
    if effect["paid"].is_null() {
        p = effect["gets"].clone();
    } else {
        p = effect["paid"].clone();
    }

    if ! funded {
        return amount_ratio(&g, &p);
    } else {
        return amount_ratio(&p, &g);
    }
}

pub fn txn_type(tx: &Value, account: &str) -> String {
    let tx_account = tx["Account"].as_str().unwrap();
    let tx_target = &tx["Target"];

    if tx_account == account ||
       ! tx_target.is_null() && tx_target.as_str().unwrap() == account ||
       ! tx["Destination"].is_null() && tx["Destination"].as_str().unwrap() == account ||
       ! tx["LimitAmount"].is_null() && tx["LimitAmount"]["issuer"].as_str().unwrap() == account {

        match tx["TransactionType"].as_str().unwrap() {
            "Payment" => {
                if tx["Account"].as_str().unwrap() == account {
                    if tx["Destination"].as_str().unwrap() == account {
                        return "convert".to_owned();
                    } else {
                        return "sent".to_owned();
                    }
                } else {
                    return "received".to_owned();
                }
            },

            "OfferCreate" => {
                return "offernew".to_owned();
            },

            "OfferCancel" => {
                return "offercancel".to_owned();
            },

            "TrustSet" => {
                if tx["Account"].as_str().unwrap() == account {
                    return "trusting".to_owned();
                }

                return "trusted".to_owned();
            },

            "RelationDel" | "AccountSet" | "SetRegularKey" | "RelationSet" | "SignSet" | "Operation" | "ConfigContract" | "AlethContract" | "Brokerage" => {
                return tx["TransactionType"].as_str().unwrap().to_ascii_lowercase();
            },

            _ => {
                return "unknown".to_owned();
            }
        }
    } else {
        return "offereffect".to_owned();
    }
}

pub fn type_result(x: &Value, result: &mut Value) {
    match result["type"].as_str().unwrap() {
        "sent" => {
            result["counterparty"] = x["Destination"].clone();
            result["amount"] = parse_amount(&x["Amount"]);
        },

        "received" => {
            result["counterparty"] = x["Account"].clone();
            result["amount"] = parse_amount(&x["Amount"]);
        },

        "trusted" => {
            result["counterparty"] = x["Account"].clone();
            result["amount"] = reverse_amount(&x["LimitAmount"]);
        },

        "trusting" => {
            result["counterparty"] = x["LimitAmount"]["issuer"].clone();
            result["amount"] = x["LimitAmount"].clone();
        },

        "convert" => {
            result["spent"] = parse_amount(&x["SendMax"]);
            result["amount"] = parse_amount(&x["Amount"]);
        },

        "offernew" => {
            if x["Flags"].as_u64().unwrap() & 0x00080000 as u64 != 0 {
                result["offertype"] = json!("sell");
            } else {
                result["offertype"] = json!("buy");
            }

            result["gets"] = parse_amount(&x["TakerGets"]);
            result["pays"] = parse_amount(&x["TakerPays"]);

            result["seq"] = x["Sequence"].clone();

            let v1 = result["pays"]["value"].clone().as_f64().unwrap();
            let v2 = result["gets"]["value"].clone().as_f64().unwrap();

            if result["offertype"] == json!("sell") {
                result["price"] = json!( v1/v2 );
            } else {
                result["price"] = json!( v2/ v1);
            }
        },

        "offercancel" => {
            result["offerseq"] = x["Sequence"].clone();
        },

        "relationset" => {
            if x["Account"] == x["Target"] {
                result["counterparty"] = x["Account"].clone();
                result["isactive"] = json!(false);
            } else {
                result["counterparty"] = x["Target"].clone();
                result["isactive"] = json!(true);
            }

            if let Some(tp) = x["RelationType"].as_i64() {
                if tp == 3 {
                    result["relationtype"] = json!("freeze");
                } else {
                    result["relationtype"] = json!("authorize");
                }
            }

            result["amount"] = parse_amount(&x["LimitAmount"]);
        },

        "relationdel" => {
            if x["Account"] == x["Target"] {
                result["counterparty"] = x["Account"].clone();
                result["isactive"] = json!(false);
            } else {
                result["counterparty"] = x["Target"].clone();
                result["isactive"] = json!(true);
            }

            if let Some(tp) = x["RelationType"].as_i64() {
                if tp == 3 {
                    result["relationtype"] = json!("unfreeze");
                } else {
                    result["relationtype"] = json!("unknown");
                }
            }

            result["amount"] = parse_amount(&x["LimitAmount"]);
        },

        "configcontract" => {
            if x["Method"].as_i64() == Some(0) {
                result["method"] = json!("deploy");
                result["payload"] = x["Payload"].clone();
            } else if x["Method"].as_i64() == Some(1) {
                result["method"] = json!("call");
                result["destination"] = x["Destination"].clone();
            }
        },

        "alethcontract" => {
            if x["Method"].as_u64() == Some(0u64) {
                result["method"] = json!("deploy");
                result["seq"] = x["Sequence"].clone();
                result["payload"] = x["Payload"].clone();
            } else if x["Method"].as_u64() == Some(1u64) {
                result["method"] = json!("call");
                result["seq"] = x["Sequence"].clone();
                result["destination"] = x["Destination"].clone();
                result["amount"] = x["Amount"].clone();

                let method = hex::decode(x["MethodSignature"].clone().as_str().unwrap()).unwrap();
                let method = String::from_utf8_lossy(&method);
                let v: Vec<_>  = method.match_indices("(").collect();
                let v1: Vec<_>  = method.match_indices(")").collect();
                let idx1 = v[0].0;
                let idx2 = v1[0].0;

                result["func"] = json!( method.get(0..idx1).unwrap() );

                let ret= method.get(idx1+1..idx2).unwrap();
                let v: Vec<&str> = ret.split(',').collect();
                if v.len() == 0 {
                    result["func_parms"] = json!([]);
                } else {
                    let mut idx = 0;
                    while idx < v.len() {
                        result["func_parms"][idx] = json!( v[idx] );

                        idx += 1;
                    }
                }
            }
        },

        "brokerage" => {
            result["feeAccount"] = x["FeeAccountID"].clone();

            result["mol"] = x["OfferFeeRateNum"].clone();
            result["den"] = x["OfferFeeRateDen"].clone();

            result["amount"] = parse_amount(&x["Amount"]);
            result["seq"] = x["Sequence"].clone();
        },

        _ => {
            // TODO parse other type
        }
    }
}

pub fn parse(tx: &str) -> Value {
    let mut result: Value = json!({});

    if let Ok(x) = serde_json::from_str(tx) as Result<Value, serde_json::error::Error> {
        //account
        let account = x["Account"].as_str().unwrap();

        //date
        if ! x["date"].is_null() {
            let timestamp = x["date"].as_i64().unwrap() + OFFSET_SECOND;
            result["date"] = json!(timestamp);
        } else if ! x["timestamp"].is_null() {
            let timestamp = x["timestamp"].as_i64().unwrap() + OFFSET_SECOND;
            result["date"] = json!(timestamp);
        }

        //hash
        if ! x["hash"].is_null() {
            result["hash"] = json!(x["hash"]);
        }

        //type
        result["type"] = json!(txn_type(&x, &account));

        //fee
        result["fee"] = json!(x["Fee"].as_str().unwrap().parse::<f64>().unwrap() / 1000000.0);

        //result
        if ! x["meta"].is_null() {
            result["result"] = json!(x["meta"]["TransactionResult"]);
        } else {
            result["result"] = json!("failed");
        }

        //type result
        type_result(&x, &mut result);

        //memos
        if x["Meta"].is_array() && x["Memos"].as_array().unwrap().len() > 0 {
            let mut m = 0;
            while m < x["Memos"].as_array().unwrap().len() {
                let memo_s = x["Memos"][m]["Memo"].clone();
                let mut memo = memo_s;
                result["memos"][m] = memo;

                m += 1;
            }
        }

        result["effects"] = json!([]);
        // no effect, return now
        if x["meta"].is_null() || x["meta"]["TransactionResult"] != json!("tesSUCCESS") {
            return result;
        }

        let mut cos: Vec<Value> = vec![];
        let mut gets_value = 0;
        let mut pays_value = 0;
        // let mut total_rate = 0;

        if ! result["gets"].is_null() {
            cos.push( result["gets"]["currency"].clone());
            cos.push( result["pays"]["currency"].clone());
        }

        result["balances"] = json!([]);
        result["balancesPrev"] = json!([]);

        let affected_nodes = x["meta"]["AffectedNodes"].clone();
        let mut n = 0;
        while n < affected_nodes.as_array().unwrap().len() {
            let node_src = affected_nodes[n].clone();
            let mut node = json!({"diffType": "node"});
            process_affect_node(&node_src, &mut node);
            let mut effect = json!({});

            if ! node.is_null() && node["entryType"] == json!("Offer") {

                let field_set = node["fields"].clone();
                let mut sell = 0;
                if ! node["fields"]["Flags"].is_null() {
                    sell = node["fields"]["Flags"].clone().as_u64().unwrap() & 0x00080000;
                }
                if node["fields"]["Account"] == account {
                    if node["diffType"] == json!("ModifiedNode") ||
                       ( node["diffType"] == json!("DeletedNode") &&
                       ! node["fieldsPrev"]["TakerGets"].is_null() &&
                       ! is_amount_zero(&parse_amount(&node["fieldsFinal"]["TakerGets"])) ) {

                        effect["effect"] = json!("offer_partially_funded");
                        effect["counterparty"]["account"] = x["Account"].clone();
                        effect["counterparty"]["seq"] = x["Sequence"].clone();
                        effect["counterparty"]["hash"] = x["hash"].clone();

                        if node["diffType"] != json!("DeletedNode") {
                            effect["remaining"] = json!( ! is_amount_zero(&parse_amount(&node["fields"]["TakerGets"])) );
                        } else {
                            effect["cancelled"] = json!(true);
                        }

                        effect["gets"] = parse_amount(&field_set["TakerGets"]);
                        effect["pays"] = parse_amount(&field_set["TakerPays"]);
                        effect["got"] = amount_subtract(&parse_amount(&node["fieldsPrev"]["TakerPays"]), &parse_amount(&node["fields"]["TakerPays"]));
                        effect["paid"] = amount_subtract(&parse_amount(&node["fieldsPrev"]["TakerGets"]), &parse_amount(&node["fields"]["TakerGets"]));

                        if sell != 0 {
                            effect["type"] = json!("sold");
                        } else {
                            effect["type"] = json!("bought");
                        }

                        if ! node["fields"]["OfferFeeRateNum"].is_null() {
                            effect["platform"] = node["fields"]["Platform"].clone();

                            let v1 = node["fields"]["OfferFeeRateNum"].clone().as_f64().unwrap();
                            let v2 = node["fields"]["OfferFeeRateDen"].clone().as_f64().unwrap();

                            effect["rate"] = json!( v1 / v2 );
                        }
                    } else {
                        if node["diffType"] == json!("CreatedNode") {
                            effect["effect"] = json!("offer_created");
                        } else {
                            if node["fieldsPrev"]["TakerPays"].is_null() {
                                effect["effect"] = json!("offer_cancelled");
                            } else {
                                effect["effect"] = json!("offer_funded");
                            }
                        }

                        // 2. offer_funded
                        if effect["effect"] == json!("offer_funded") {
                            // let field_set = node["fieldsPrev"].clone();

                            effect["counterparty"]["account"] = x["Account"].clone();
                            effect["counterparty"]["seq"] = x["Sequence"].clone();
                            effect["counterparty"]["hash"] = x["hash"].clone();

                            effect["got"] = amount_subtract(&parse_amount(&node["fieldsPrev"]["TakerPays"]), &parse_amount(&node["fields"]["TakerPays"]));
                            effect["paid"] = effect["got"].clone();
                            if sell != 0 {
                                effect["type"] = json!("sold");
                            } else {
                                effect["type"] = json!("bought");
                            }

                            if ! node["fields"]["OfferFeeRateNum"].is_null() {
                                effect["platform"] = node["fields"]["Platform"].clone();

                                let v1 = node["fields"]["OfferFeeRateNum"].clone().as_f64().unwrap();
                                let v2 = node["fields"]["OfferFeeRateDen"].clone().as_f64().unwrap();

                                effect["rate"] = json!( v1 / v2 );
                            }
                        }
                        // 3. offer_created
                        if effect["effect"] == json!("offer_created") {
                            effect["gets"] = parse_amount(&field_set["TakerGets"]);
                            effect["pays"] = parse_amount(&field_set["TakerPays"]);
                            if sell != 0 {
                                effect["type"] = json!("sell");
                            } else {
                                effect["type"] = json!("buy");
                            }

                            if ! field_set["OfferFeeRateNum"].is_null() {
                                effect["platform"] = field_set["Platform"].clone();

                                let v1 = node["fields"]["OfferFeeRateNum"].clone().as_f64().unwrap();
                                let v2 = node["fields"]["OfferFeeRateDen"].clone().as_f64().unwrap();

                                effect["rate"] = json!( v1 / v2 );
                            }
                        }
                        // 4. offer_cancelled
                        if effect["effect"] == json!("offer_cancelled") {
                            effect["hash"] = node["fields"]["PreviousTxnID"].clone();
                            // collect data for cancel transaction type
                            if result["type"] == json!("offercancel") {
                                result["gets"] = parse_amount(&field_set["TakerGets"]);
                                result["pays"] = parse_amount(&field_set["TakerPays"]);
                            }

                            effect["gets"] = parse_amount(&field_set["TakerGets"]);
                            effect["pays"] = parse_amount(&field_set["TakerPays"]);

                            if sell != 0 {
                                effect["type"] = json!("sell");
                            } else {
                                effect["type"] = json!("buy");
                            }

                            if !field_set["OfferFeeRateNum"].is_null() {
                                effect["platform"] = field_set["Platform"].clone();
                                let v1 = field_set["fields"]["OfferFeeRateNum"].clone().as_f64().unwrap();
                                let v2 = field_set["fields"]["OfferFeeRateDen"].clone().as_f64().unwrap();

                                effect["rate"] = json!( v1 / v2 );
                            }
                        }
                    }

                    effect["seq"] = node["fields"]["Sequence"].clone();
                }
                // 5. offer_bought
                else if x["Account"] == account && ! node["fieldsPrev"].is_null() {

                    effect["effect"] = json!("offer_bought");

                    effect["counterparty"]["account"] = node["Account"].clone();
                    effect["counterparty"]["seq"] = node["Sequence"].clone();
                    effect["counterparty"]["hash"] = node["hash"].clone();

                    effect["paid"] = amount_subtract(&parse_amount(&node["fieldsPrev"]["TakerPays"]), &parse_amount(&node["fields"]["TakerPays"]));
                    effect["got"] = amount_subtract(&parse_amount(&node["fieldsPrev"]["TakerGets"]), &parse_amount(&node["fields"]["TakerGets"]));

                    if result["offertype"] == json!("buy") && sell != 0 || result["offertype"] == json!("sell") && !(sell != 0 ) as bool {
                        if sell != 0 {
                            effect["type"] = json!("bought");
                        } else {
                            effect["type"] = json!("sold");
                        }

                    }else {
                        if sell != 0 {
                            effect["type"] = json!("sold");
                        } else {
                            effect["type"] = json!("bought");
                        }
                    }
                }
                // add price
                if ! effect["gets"].is_null() && ! effect["pays"].is_null() || ! effect["got"].is_null() && ! effect["paid"].is_null() {

                    let mut created = false;
                    if effect["effect"] == json!("offer_created") && effect["type"] == json!("buy") {
                        created = true;
                    }

                    let mut funded = false;
                    if effect["effect"] == json!("offer_funded") && effect["type"] == json!("bought") {
                        funded = true;
                    }

                    let mut cancelled = false;
                    if effect["effect"] == json!("offer_cancelled") && effect["type"] == json!("buy") {
                        cancelled = true;
                    }

                    let mut bought = false;
                    if effect["effect"] == json!("offer_bought") && effect["type"] == json!("bought") {
                        bought = true;
                    }

                    let mut partially_funded = false;
                    if effect["effect"] == json!("offer_partially_funded") && effect["type"] == json!("bought") {
                        partially_funded = true;
                    }

                    effect["price"] = get_price(&effect, created || funded || cancelled || bought ||  partially_funded );
                }
            }

            if result["type"] == json!("offereffect") && ! node.is_null() && node["entryType"] == json!("AccountRoot") {

                if node["fields"]["RegularKey"] == account {
                    effect["effect"] = json!("set_regular_key");
                    effect["type"] = json!("null");
                    effect["account"] = node["fields"]["Account"].clone();
                    effect["regularkey"] = json!(account);
                }
            }

            if ! node.is_null() && node["entryType"] == json!("Brokerage") {
                result["platform"] = node["fields"]["Platform"].clone();

                let v1 = node["fields"]["OfferFeeRateNum"].clone().as_f64().unwrap();
                let v2 = node["fields"]["OfferFeeRateDen"].clone().as_f64().unwrap();

                result["rate"] = json!( v1 / v2 );
            }

            if ! node.is_null() && node["entryType"] == json!("SkywellState") {//其他币种余额

                if node["fields"]["HighLimit"]["issuer"] == account || node["fields"]["LowLimit"]["issuer"] == account {
                    if let Some(cny) = node["fields"]["Balance"]["currency"].clone().as_str() {
                        let cny = json!( {cny: node["fields"]["Balance"]["value"].clone().as_str().unwrap().parse::<f64>().unwrap().abs() } );
                        result["balances"].as_array_mut().unwrap().push( cny.clone() );
                    }

                    if ! node["fieldsPrev"]["Balance"].is_null() {

                        if let Some(cnn) = node["fieldsPrev"]["Balance"]["currency"].clone().as_str() {
                            let v = json!( {cnn: node["fieldsPrev"]["Balance"]["value"].clone().as_str().unwrap().parse::<f64>().unwrap().abs() } );
                            result["balancesPrev"].as_array_mut().unwrap().push( v.clone() );
                        }

                    } else if ! node["fieldsNew"]["Balance"].is_null() {//新增币种

                        if let Some(cnn) = node["fields"]["Balance"]["currency"].clone().as_str() {
                            result["balancesPrev"].as_array_mut().unwrap().push( json!({cnn: 0}) );
                        }
                    } else {
                    }
                }
            }

            if !node.is_null() && node["entryType"] == json!("AccountRoot") {//基础币种余额
                  if node["fields"]["Account"] == account {

                      if let Ok(x) = node["fields"]["Balance"].clone().as_str().unwrap().parse::<f64>() {
                          let v = json!( { CURRENCY : x/1000000.0 } );
                          result["balances"].as_array_mut().unwrap().push( v );

                      } else {
                          let v = json!( { CURRENCY : 0 } );
                          result["balances"].as_array_mut().unwrap().push( v );
                      }

                      if ! node["fieldsPrev"]["Balance"].is_null() {
                          if let Ok(x) =  node["fieldsPrev"]["Balance"].clone().as_str().unwrap().parse::<f64>() {
                              let v = json!( {CURRENCY: x / 1000000.0} );
                              result["balancesPrev"].as_array_mut().unwrap().push( v );
                          } else {
                              let v = json!( {CURRENCY: 0.0} );
                              result["balancesPrev"].as_array_mut().unwrap().push( v );
                          }

                      } else if ! node["fieldsNew"]["Balance"].is_null() {

                          result["balancesPrev"].as_array_mut().unwrap().push( json!( {CURRENCY: 0 } ));

                      } else {
                      }
                  }
            }

            // add effect
            if ! effect.is_null() {
                if node["diffType"] == json!("DeletedNode") && effect["effect"] != json!("offer_bought") {
                    effect["deleted"] = json!(true);
                }

                if ! effect["type"].is_null() {
                    result["effects"].as_array_mut().unwrap().push(  effect.clone() );
                }
            }

            if result["type"] == json!("offernew") && ! effect["got"].clone().is_null() {
                    if result["gets"]["currency"] == effect["paid"]["currency"] {
                        gets_value = ( effect["paid"]["value"].clone().as_f64().unwrap() + gets_value as f64 ) as u64;
                    }

                    // let mut pays_value1 = 0;
                    if result["pays"]["currency"] == effect["got"]["currency"] {
                        gets_value = ( effect["got"]["value"].clone().as_f64().unwrap() + pays_value as f64 ) as u64;
                    }

                    if result["gets"]["currency"] != effect["paid"]["currency"] || result["pays"]["currency"] != effect["got"]["currency"] {
                        if ! cos.contains(&effect["got"]["currency"]) {
                            cos.push(effect["got"]["currency"].clone());
                        }

                        if ! cos.contains(&effect["paid"]["currency"]) {
                            cos.push(effect["paid"]["currency"].clone());
                        }
                    }
                }

                n += 1;
        } // end while

    } else {
        panic!("Error input.");
    }

    result
}
