use std::marker::PhantomData;
use std::str::FromStr;

use serde::{Deserialize, Serialize, Deserializer};
use serde::de::{self, Visitor, MapAccess};

extern crate void;
use void::Void;
use std::fmt; 

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
//End Amount
////////////////////////////////////////////////////////////////////////////////////////////////////////////
