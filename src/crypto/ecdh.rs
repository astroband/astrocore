use sodiumoxide::crypto::scalarmult::curve25519;

/// EC secret key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Curve25519Secret(pub [u8; 32]);

impl Curve25519Secret {
    /// Create a random secret key.
    pub fn random() -> Curve25519Secret {
        let seed = super::random_bytes(32);
        let mut data = [0; 32];
        data.copy_from_slice(&seed[..32]);
        Curve25519Secret(data)
    }
}

/// EC public key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Curve25519Public(pub [u8; 32]);

impl Curve25519Public {
    /// Create a public key, derived from the secret key.
    pub fn derive_from_secret(secret: &Curve25519Secret) -> Curve25519Public {
        let scalar = curve25519::Scalar(secret.0);
        let group_element = curve25519::scalarmult_base(&scalar);
        Curve25519Public(group_element.0)
    }
}
