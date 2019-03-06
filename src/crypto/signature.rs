use super::error::{Error, Result};
use super::{PublicKey, SecretKey};
use sodiumoxide::crypto::sign::ed25519;

/// A signature.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signature {
    sig: ed25519::Signature,
}

impl Signature {
    /// Sign `data` using the `secret` key.
    pub fn sign(secret: &SecretKey, data: &[u8]) -> Signature {
        let sig = ed25519::sign_detached(data, &secret.inner());
        Signature { sig }
    }

    /// Return a `Signature` from bytes.
    pub fn from_slice(sb: &[u8]) -> Result<Signature> {
        let sig = ed25519::Signature::from_slice(sb).ok_or(Error::InvalidSignature)?;
        Ok(Signature { sig })
    }

    /// Length in bytes of the signature.
    pub fn len(&self) -> usize {
        self.sig.0.len()
    }

    /// Convert to `Vec<u8>`.
    pub fn to_vec(&self) -> Vec<u8> {
        self.sig.0.to_vec()
    }

    /// Inner buffer as slice.
    pub fn buf(&self) -> &[u8] {
        &self.sig.0
    }

    /// Verify the signature againt the `data` and the `public` key.
    /// Return `true` if the signature is valid, `false` otherwise.
    pub fn verify(&self, public: &PublicKey, data: &[u8]) -> bool {
        ed25519::verify_detached(&self.sig, data, &public.inner())
    }
}

/// Last 4 bytes of a public key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignatureHint(pub [u8; 4]);

impl SignatureHint {
    /// Create a `SignatureHint` with the last 4 bytes of the public key `pk`.
    pub fn from_public_key(pk: &PublicKey) -> SignatureHint {
        let mut hint: [u8; 4] = Default::default();
        let buf = pk.buf();
        let len = buf.len();
        hint.copy_from_slice(&buf[len - 4..len]);
        SignatureHint(hint)
    }

    /// Convert to `Vec<u8>`.
    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

/// A `Signature` together with the last 4 bytes of the public key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecoratedSignature {
    hint: SignatureHint,
    signature: Signature,
}

impl DecoratedSignature {
    /// Create a new `DecoratedSignature` with `hint` and `signature`.
    pub fn new(hint: SignatureHint, signature: Signature) -> DecoratedSignature {
        DecoratedSignature { hint, signature }
    }

    /// Return the decorated signature `hint`.
    pub fn hint(&self) -> &SignatureHint {
        &self.hint
    }

    /// Return the decorated signature `signature`.
    pub fn signature(&self) -> &Signature {
        &self.signature
    }
}
