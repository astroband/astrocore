use sodiumoxide::crypto::auth::hmacsha256;

/// HMAC key
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HmacSha256Key(pub [u8; 32]);

impl HmacSha256Key {
    /// Create a key filled with zero.
    fn zero() -> HmacSha256Key {
        HmacSha256Key([0; 32])
    }

    /// `hkdf-extract(data) == HMAC(<zero>, data)`
    pub fn hkdf_extract(data: &[u8]) -> HmacSha256Key {
        let zero = HmacSha256Key::zero();
        let mac = HmacSha256Mac::authenticate(&data, &zero);
        HmacSha256Key(mac.0)
    }

    /// `hkdf-expand(key, data) == HMAC(key, data|0x1)`
    pub fn hkdf_expand(key: &HmacSha256Key, data: &[u8]) -> HmacSha256Key {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&data);
        bytes.push(1);
        let mac = HmacSha256Mac::authenticate(&bytes, &key);
        HmacSha256Key(mac.0)
    }
}

/// HMAC mac.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HmacSha256Mac(pub [u8; 32]);

impl HmacSha256Mac {
    /// Create a mac filled with zero.
    pub fn zero() -> HmacSha256Mac {
        HmacSha256Mac([0; 32])
    }

    /// Authenticate the message with the key.
    pub fn authenticate(data: &[u8], key: &HmacSha256Key) -> HmacSha256Mac {
        let hkey = hmacsha256::Key(key.0);
        let tag = hmacsha256::authenticate(&data, &hkey);
        HmacSha256Mac(tag.0)
    }
}
