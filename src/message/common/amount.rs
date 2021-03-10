use std::marker::PhantomData;
use std::str::FromStr;
use serde::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer, SerializeStruct};

use serde::de::{self, Visitor, MapAccess};

extern crate void;
use void::Void;
use std::fmt;
use std::any::Any;

use crate::base::base_config::{CURRENCY};

#[derive(Deserialize, Debug, Default)]
pub struct Amount {
    #[serde(rename="value")]
    pub value: String,   //0.5

    #[serde(rename="currency")]
    pub currency: Option<String>,//'USD',

    #[serde(rename="issuer")]
    pub issuer: Option<String>,  //'jBciDE8Q3uJjf111VeiUNM775AMKHEbBLS',
}

impl Amount {
    pub fn new(currency: Option<String>, value: String, issuer: Option<String>) -> Self {
        Amount {
            value   : value,
            currency: currency,
            issuer  : issuer,
        }
    }

    pub fn is_string(&self) -> bool {
        let mut ret = false;
        if let Some(x) = &self.currency {
            if *x == CURRENCY.to_string() && self.issuer.is_none() {
                ret = true;
            }
        }

        ret
    }

    pub fn is_native(&self) -> bool {
        if let Some(x) = &self.currency {
            if *x == "SWT".to_string() {
                return true;
            }
        }

        false
    }

    pub fn mul_million(value: &String) -> String {
        let mut ret = 0f64;
        if let Ok(x) = value.parse::<f64>() {
            ret = x * 1_000_000f64;
        }

        (ret as u64).to_string()
    }

    // pub fn to_string(&self) -> Result<String, serde_json::error::Error> {
    //     let j = serde_json::to_string(&self)?;
    //     Ok(j)
    // }

    //TODO::
    // pub fn decorate(&mut self) -> Self {
    //     if self.is_string() {
    //         self.value = Amount::mul_million(&self.value);
    //     }

    //     Amount {
    //         value: String::from( self.value.as_str() ),
    //         currency: self.currency,
    //         issuer: self.issuer,
    //     }
    // }
}

impl Serialize for Amount {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Amount", 3)?;

        state.serialize_field("value", &Amount::mul_million(&self.value))?;

        if self.currency.is_some() {
            state.serialize_field("currency", &self.currency)?;
        }

        if self.issuer.is_some() {
            state.serialize_field("issuer", &self.issuer)?; 
        }

        state.end()
    }
}

impl FromStr for Amount {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(
            Amount {
                value: s.to_string(),
                currency: Some("SWT".to_string()),
                issuer: None,
            }
        )
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
