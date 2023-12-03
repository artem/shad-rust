use crate::data::WalletId;

use anyhow::{Context, Result};
use chrono::{DateTime, LocalResult, TimeZone, Utc};
use rsa::{PublicKeyEncoding, RSAPrivateKey, RSAPublicKey};
use serde::{
    de::{self, Deserializer},
    ser::{self, Serializer},
    Deserialize,
};

////////////////////////////////////////////////////////////////////////////////

fn decode_pkcs8_plaintext(raw: &str) -> Result<Vec<u8>> {
    let der_encoded =
        raw.lines()
            .filter(|line| !line.starts_with('-'))
            .fold(String::new(), |mut data, line| {
                data.push_str(line);
                data
            });
    base64::decode(der_encoded).context("failed to decode base64")
}

pub fn parse_pkcs8_public(raw: &str) -> Result<RSAPublicKey> {
    let der_bytes = decode_pkcs8_plaintext(raw)?;
    RSAPublicKey::from_pkcs8(&der_bytes).context("failed to decode pkcs8 bytes")
}

pub fn parse_pkcs8_private(raw: &str) -> Result<RSAPrivateKey> {
    let der_bytes = decode_pkcs8_plaintext(raw)?;
    RSAPrivateKey::from_pkcs8(&der_bytes).context("failed to decode pkcs8 bytes")
}

////////////////////////////////////////////////////////////////////////////////

pub fn serialize_base64<T, S>(array: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: AsRef<[u8]>,
    S: Serializer,
{
    serializer.serialize_str(&base64::encode(array.as_ref()))
}

pub fn deserialize_base64<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let string = String::deserialize(deserializer)?;
    base64::decode(string).map_err(|err| de::Error::custom(format!("invalid base64: {}", err)))
}

pub fn deserialize_base64_fixed<'de, D, const SIZE: usize>(
    deserializer: D,
) -> Result<[u8; SIZE], D::Error>
where
    D: Deserializer<'de>,
{
    let bytes = deserialize_base64(deserializer)?;
    if bytes.len() != SIZE {
        return Err(de::Error::custom(format!(
            "invalid length: expected {}, got {}",
            SIZE,
            bytes.len()
        )));
    }

    let mut array = [0u8; SIZE];
    for (i, byte) in bytes.into_iter().enumerate() {
        array[i] = byte;
    }
    Ok(array)
}

////////////////////////////////////////////////////////////////////////////////

pub fn serialize_wallet_id<S>(wallet: &WalletId, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let bytes = wallet
        .public_key
        .to_pkcs8()
        .map_err(|err| ser::Error::custom(format!("failed to encode key as PKCS8: {}", err)))?;
    serialize_base64(&bytes, serializer)
}

pub fn deserialize_wallet_id<'de, D>(deserializer: D) -> Result<WalletId, D::Error>
where
    D: Deserializer<'de>,
{
    let bytes = deserialize_base64(deserializer)?;
    RSAPublicKey::from_pkcs8(&bytes)
        .map(|public_key| WalletId { public_key })
        .map_err(|err| de::Error::custom(format!("invalid PKCS8: {}", err)))
}

////////////////////////////////////////////////////////////////////////////////

pub fn serialize_utc<S>(key: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_i64(key.timestamp())
}

pub fn deserialize_utc<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let ts = i64::deserialize(deserializer)?;
    match Utc.timestamp_opt(ts, 0) {
        LocalResult::Single(dt) => Ok(dt),
        LocalResult::None => Err(de::Error::custom("invalid timestamp")),
        LocalResult::Ambiguous(_, _) => Err(de::Error::custom("ambiguous timestamp")),
    }
}
