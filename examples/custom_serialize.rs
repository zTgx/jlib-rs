
use serde::ser::{Serializer, SerializeStruct};
use std::fmt; //fmt METHOD
use std::marker::PhantomData;
use std::str::FromStr;

use serde::{Deserialize, Serialize, Deserializer};
use serde::de::{self, Visitor, MapAccess};

extern crate void;
use void::Void;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Amount {

    #[serde(rename="value")]
    pub value: String,   //0.5

    #[serde(rename="currency")]
    pub currency: Option<String>,//'USD',

    #[serde(rename="issuer")]
    pub issuer: Option<String>,  //'jBciDE8Q3uJjf111VeiUNM775AMKHEbBLS',
}

impl Amount {
    pub fn new(currency: String, value: String, issuer: String) -> Self {
        Amount {
            value: value,
            currency: Some(currency),
            issuer: Some(issuer),
        }
    }

    pub fn is_string(&self) -> bool {
        let mut ret = false;
        if let Some(x) = &self.currency {
            if let Some(y) = &self.issuer {
                if *x == "SWT".to_string() && y.len() == 0 {
                    ret = true;
                }
            }
        }
        
        ret
    }
}
impl FromStr for Amount {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Amount {
            value: s.to_string(),
            currency: None,
            issuer: None,
        })
    }
}
pub fn string_or_struct<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + FromStr<Err = Void>,
    D: Deserializer<'de>,
{
    struct StringOrStruct<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for StringOrStruct<T>
    where
        T: Deserialize<'de> + FromStr<Err = Void>,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string")
        }

        fn visit_str<E>(self, value: &str) -> Result<T, E>
        where
            E: de::Error,
        {
            Ok(FromStr::from_str(value).unwrap())
        }

        fn visit_map<M>(self, map: M) -> Result<T, M::Error>
        where
            M: MapAccess<'de>,
        {
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
        }
    }

    deserializer.deserialize_any(StringOrStruct(PhantomData))
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OfferCreateTxJson {
    #[serde(rename="Flags")]
    pub flags: i32, 

    #[serde(rename="Fee")]
    pub fee: u64,

    #[serde(rename="TransactionType")]
    pub transaction_type: String,

    #[serde(rename="Account")]
    pub account: String,

    #[serde(rename="TakerPays")]
    #[serde(deserialize_with = "string_or_struct")]
    taker_pays: Amount, 

    #[serde(rename="TakerGets")]
    #[serde(deserialize_with = "string_or_struct")]
    taker_gets: Amount,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OfferCreateTx {
    #[serde(rename="id")]
    id: u64, 

    #[serde(rename="tx_json")]
    pub tx_json: OfferCreateTxJson,
}
impl OfferCreateTxJson {
    pub fn new(account: String, taker_gets: Amount,  taker_pays: Amount) -> Self {
        OfferCreateTxJson {

            flags: 524288, ///////////////Hard code
            fee: 10000, 
            transaction_type: "OfferCreate".to_string(),
            account: account,
            taker_pays: taker_pays,
            taker_gets: taker_gets,
        }
    }
}

impl Serialize for OfferCreateTxJson {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("OfferCreateTxJson", 6)?;
        
        state.serialize_field("Flags", &self.flags)?;
        state.serialize_field("Fee", &self.fee)?;
        state.serialize_field("TransactionType", &self.transaction_type)?;
        state.serialize_field("Account", &self.account)?;
        if self.taker_gets.is_string () {
            state.serialize_field("TakerGets", &self.taker_gets.value)?;
        } else {
            state.serialize_field("TakerGets", &self.taker_gets)?;
        }

        if self.taker_pays.is_string () {
            state.serialize_field("TakerPays", &self.taker_pays.value)?;
        } else {
            state.serialize_field("TakerPays", &self.taker_pays)?;
        }

        state.end()
    }
}

fn main() {

    let js = OfferCreateTxJson::new(
                    "xxxxx".to_string(),
                    Amount::new("SWT".to_string(), "3".to_string(), "".to_string()),
                    Amount::new("4".to_string(), "currency".to_string(), "issuer".to_string()));
                    
    let xx = Box::new( OfferCreateTx{id: 5, tx_json: js} );
        println!("js : {:?}", serde_json::to_string(&xx));
}
