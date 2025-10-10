use std::str::FromStr;
use serde::Deserialize;
use std::fmt::Display;
use serde::de::Deserializer;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum StringOrNumber<T>
{
    String(String),
    Number(T),
}

pub fn deserialize_number<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr + Deserialize<'de>,
    T::Err: Display,
    D: Deserializer<'de>
{
    match StringOrNumber::<T>::deserialize(deserializer)?
    {
        StringOrNumber::String(s) => s.parse().map_err(|e| serde::de::Error::custom(e)),
        StringOrNumber::Number(n) => Ok(n)
    }
}