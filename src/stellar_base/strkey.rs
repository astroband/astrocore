use crate::xdr::{AccountId};
use base32;
use byteorder::{ByteOrder, LittleEndian};
use crc16::{State, XMODEM};

const ACCOUNT_ID_VERSION_BYTE: u8 = 6 << 3; // G
const SECRET_SEED_VERSION_BYTE: u8 = 18 << 3; // S
const PRE_AUTH_TX_VERSION_BYTE: u8 = 19 << 3; // T
const SHA256_HASH_VERSION_BYTE: u8 = 23 << 3; // X

static ALPHABET: base32::Alphabet = base32::Alphabet::RFC4648 { padding: false };

#[derive(Debug)]
pub enum Error {
    /// Error that can occur when parsing a key.
    InvalidStrKey,
    /// Invalid version byte in key.
    InvalidStrKeyVersionByte,
    /// Invalid checksum in key.
    InvalidStrKeyChecksum
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn encode_ed25519_public_key(key: AccountId) -> Result<String> {
    match key {
        AccountId::Ed25519(opaque) => encode_account_id(&opaque.0)
    }
}

pub fn encode_account_id(data: &[u8]) -> Result<String> {
    encode_check(ACCOUNT_ID_VERSION_BYTE, data)
}

pub fn decode_account_id(data: &str) -> Result<Vec<u8>> {
    decode_check(ACCOUNT_ID_VERSION_BYTE, data)
}

pub fn encode_secret_seed(data: &[u8]) -> Result<String> {
    encode_check(SECRET_SEED_VERSION_BYTE, data)
}
pub fn decode_secret_seed(data: &str) -> Result<Vec<u8>> {
    decode_check(SECRET_SEED_VERSION_BYTE, data)
}

pub fn encode_pre_auth_tx(data: &[u8]) -> Result<String> {
    encode_check(PRE_AUTH_TX_VERSION_BYTE, data)
}
pub fn decode_pre_auth_tx(data: &str) -> Result<Vec<u8>> {
    decode_check(PRE_AUTH_TX_VERSION_BYTE, data)
}

pub fn encode_sha256_hash(data: &[u8]) -> Result<String> {
    encode_check(SHA256_HASH_VERSION_BYTE, data)
}
pub fn decode_sha256_hash(data: &str) -> Result<Vec<u8>> {
    decode_check(SHA256_HASH_VERSION_BYTE, data)
}

fn encode_check(version: u8, indata: &[u8]) -> Result<String> {
    let mut data = Vec::with_capacity(35);
    data.push(version);
    data.extend_from_slice(&indata);
    let checksum = calculate_checksum(&data);
    let data_end = data.len();
    data.resize(data_end + 2, 0);
    LittleEndian::write_u16(&mut data[data_end..], checksum);
    Ok(base32::encode(ALPHABET, &data))
}

fn decode_check(expected_version: u8, data: &str) -> Result<Vec<u8>> {
    let decoded = base32::decode(ALPHABET, &data).ok_or(Error::InvalidStrKey)?;
    let decoded_len = decoded.len();
    let version_byte = decoded[0];
    if version_byte != expected_version {
        return Err(Error::InvalidStrKeyVersionByte);
    }

    let payload = &decoded[..decoded_len - 2];
    let data = &payload[1..];
    let checksum_bytes = &decoded[decoded_len - 2..];
    let checksum = calculate_checksum(payload);

    if verify_checksum(checksum, checksum_bytes) {
        let key = data.to_vec();
        Ok(key)
    } else {
        Err(Error::InvalidStrKeyChecksum)
    }
}

fn calculate_checksum(payload: &[u8]) -> u16 {
    State::<XMODEM>::calculate(payload)
}

fn verify_checksum(checksum: u16, bytes: &[u8]) -> bool {
    let expected = LittleEndian::read_u16(bytes);
    expected == checksum
}

#[cfg(test)]
mod tests {
    use super::{decode_account_id, encode_account_id};
    use super::{decode_secret_seed, encode_secret_seed};

    #[test]
    fn test_encode_decode_secret_seed() {
        let seed = "SDJHRQF4GCMIIKAAAQ6IHY42X73FQFLHUULAPSKKD4DFDM7UXWWCRHBE";
        let secret = decode_secret_seed(&seed).unwrap();
        let encoded = encode_secret_seed(&secret).unwrap();
        assert_eq!(seed, &encoded);
    }

    #[test]
    fn test_encode_decode_account_id() {
        let addr = "GCZHXL5HXQX5ABDM26LHYRCQZ5OJFHLOPLZX47WEBP3V2PF5AVFK2A5D";
        let accountid = decode_account_id(&addr).unwrap();
        let encoded = encode_account_id(&accountid).unwrap();
        assert_eq!(addr, &encoded);
    }

    #[test]
    fn test_invalid_version() {
        let addr = "GCZHXL5HXQX5ABDM26LHYRCQZ5OJFHLOPLZX47WEBP3V2PF5AVFK2A5D";
        let result = decode_secret_seed(&addr);
        assert!(result.is_err());
    }
}
