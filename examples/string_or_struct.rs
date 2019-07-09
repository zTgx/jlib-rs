use std::fmt;
use std::marker::PhantomData;
use std::str::FromStr;

use serde::{Deserialize, Serialize, Deserializer};
use serde::de::{self, Visitor, MapAccess};

extern crate void;
use void::Void;

#[derive(Debug, Deserialize)]
pub struct A {
    #[serde(rename="Name")]
    pub name : String,
    
    #[serde(rename="TakerGets")]
    #[serde(deserialize_with = "string_or_struct")]
    pub taker_gets: Amount,
    
    #[serde(rename="TakerPays")]
    #[serde(deserialize_with = "string_or_struct")]
    pub taker_pays: Amount,
}

/*
        // "TakerPays": { "currency":"CNY",
        //             "issuer":"jHb9CJAWyB4jr91VRWn96DkukG4bwdtyTh",
        //             "value":"0.01"}
*/
fn main() {
    let build_string = r#"{ "Name": "张三", "TakerGets": {"value":"0.01"},
            "TakerPays": { "currency":"CNY",
                    "issuer":"jHb9CJAWyB4jr91VRWn96DkukG4bwdtyTh",
                    "value":"0.01"}
    }
    "#;
    let service: A = serde_json::from_str(build_string).unwrap();

    // context="./dir"
    // dockerfile=None
    // args={}
    println!("{:?}", service);

    // let build_struct = r#"
    //     {"amount": { "currency":"CNY",
    //                 "issuer":"jHb9CJAWyB4jr91VRWn96DkukG4bwdtyTh",
    //                 "value":"0.01"}
    //     }
    // "#;
    // let service: Service = serde_json::from_str(build_struct).unwrap();

    // // context="./dir"
    // // dockerfile=Some("Dockerfile-alternate")
    // // args={"buildno": "1"}
    // println!("{:?}", service);
}

// #[derive(Debug, Deserialize)]
// pub struct Service {
//     // The `string_or_struct` function delegates deserialization to a type's
//     // `FromStr` impl if given a string, and to the type's `Deserialize` impl if
//     // given a struct. The function is generic over the field type T (here T is
//     // `Build`) so it can be reused for any field that implements both `FromStr`
//     // and `Deserialize`.
//     #[serde(deserialize_with = "string_or_struct")]
//     #[serde(rename="TakerGets")]
//     // #[serde(rename="TakerPays")]
//     amount: Amount,
// }


#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Amount {

    #[serde(rename="value")]
    pub value: String,   //0.5

    #[serde(rename="currency")]
    pub currency: Option<String>,//'USD',

    #[serde(rename="issuer")]
    pub issuer: Option<String>,  //'jBciDE8Q3uJjf111VeiUNM775AMKHEbBLS',
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

fn string_or_struct<'de, T, D>(deserializer: D) -> Result<T, D::Error>
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