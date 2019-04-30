use crate::ValUnc;
use serde::{
    de::{self, Deserialize, Deserializer, SeqAccess, Visitor},
    ser::{Serialize, Serializer},
};
use std::fmt;

impl Serialize for ValUnc {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (self.val, self.unc).serialize(s)
    }
}

impl<'de> Deserialize<'de> for ValUnc {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct ValUncVisitor;

        impl<'de> Visitor<'de> for ValUncVisitor {
            type Value = ValUnc;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("ValUnc")
            }

            fn visit_i64<E: de::Error>(self, v: i64) -> Result<Self::Value, E> {
                self.visit_f64(v as f64)
            }

            fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
                self.visit_f64(v as f64)
            }

            fn visit_f64<E: de::Error>(self, v: f64) -> Result<Self::Value, E> {
                Ok(v.into())
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<ValUnc, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let val = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let unc = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                Ok(ValUnc { val, unc })
            }
        }

        d.deserialize_any(ValUncVisitor)
    }
}
