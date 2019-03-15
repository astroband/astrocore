// GENERATED CODE
//
// Generated from src/xdr/Stellar-types.x by xdrgen.
//
// DO NOT EDIT

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CryptoKeyType {
    KEY_TYPE_ED25519 = 0isize,
    KEY_TYPE_PRE_AUTH_TX = 1isize,
    KEY_TYPE_HASH_X = 2isize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
pub struct Curve25519Public {
    pub key: [u8; 32i64 as usize],
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
pub struct Curve25519Secret {
    pub key: [u8; 32i64 as usize],
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
pub struct Hash(pub [u8; 32i64 as usize]);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
pub struct HmacSha256Key {
    pub key: [u8; 32i64 as usize],
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
pub struct HmacSha256Mac {
    pub mac: [u8; 32i64 as usize],
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PublicKey {
    PUBLIC_KEY_TYPE_ED25519(uint256),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PublicKeyType {
    PUBLIC_KEY_TYPE_ED25519 = 0isize,
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct Signature(pub Vec<u8>);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
pub struct SignatureHint(pub [u8; 4i64 as usize]);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SignerKey {
    SIGNER_KEY_TYPE_ED25519(uint256),
    SIGNER_KEY_TYPE_PRE_AUTH_TX(uint256),
    SIGNER_KEY_TYPE_HASH_X(uint256),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SignerKeyType {
    SIGNER_KEY_TYPE_ED25519 = 0isize,
    SIGNER_KEY_TYPE_PRE_AUTH_TX = 1isize,
    SIGNER_KEY_TYPE_HASH_X = 2isize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
pub struct uint256(pub [u8; 32i64 as usize]);

pub type NodeID = PublicKey;

pub type int32 = i32;

pub type int64 = i64;

pub type uint32 = u32;

pub type uint64 = u64;

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for CryptoKeyType {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for Curve25519Public {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(xdr_codec::pack_opaque_array(&self.key[..], self.key.len(), out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for Curve25519Secret {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(xdr_codec::pack_opaque_array(&self.key[..], self.key.len(), out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for Hash {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(xdr_codec::pack_opaque_array(
            &self.0[..],
            self.0.len(),
            out,
        )?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for HmacSha256Key {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(xdr_codec::pack_opaque_array(&self.key[..], self.key.len(), out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for HmacSha256Mac {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(xdr_codec::pack_opaque_array(&self.mac[..], self.mac.len(), out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for PublicKey {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &PublicKey::PUBLIC_KEY_TYPE_ED25519(ref val) => {
                (PublicKeyType::PUBLIC_KEY_TYPE_ED25519 as i32).pack(out)? + val.pack(out)?
            }
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for PublicKeyType {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for Signature {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(xdr_codec::pack_opaque_flex(
            &self.0,
            Some(64i64 as usize),
            out,
        )?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for SignatureHint {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(xdr_codec::pack_opaque_array(
            &self.0[..],
            self.0.len(),
            out,
        )?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for SignerKey {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &SignerKey::SIGNER_KEY_TYPE_ED25519(ref val) => {
                (SignerKeyType::SIGNER_KEY_TYPE_ED25519 as i32).pack(out)? + val.pack(out)?
            }
            &SignerKey::SIGNER_KEY_TYPE_PRE_AUTH_TX(ref val) => {
                (SignerKeyType::SIGNER_KEY_TYPE_PRE_AUTH_TX as i32).pack(out)? + val.pack(out)?
            }
            &SignerKey::SIGNER_KEY_TYPE_HASH_X(ref val) => {
                (SignerKeyType::SIGNER_KEY_TYPE_HASH_X as i32).pack(out)? + val.pack(out)?
            }
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for SignerKeyType {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for uint256 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(xdr_codec::pack_opaque_array(
            &self.0[..],
            self.0.len(),
            out,
        )?)
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for CryptoKeyType {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(CryptoKeyType, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == CryptoKeyType::KEY_TYPE_ED25519 as i32 => {
                        CryptoKeyType::KEY_TYPE_ED25519
                    }
                    x if x == CryptoKeyType::KEY_TYPE_PRE_AUTH_TX as i32 => {
                        CryptoKeyType::KEY_TYPE_PRE_AUTH_TX
                    }
                    x if x == CryptoKeyType::KEY_TYPE_HASH_X as i32 => {
                        CryptoKeyType::KEY_TYPE_HASH_X
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for Curve25519Public {
    fn unpack(input: &mut In) -> xdr_codec::Result<(Curve25519Public, usize)> {
        let mut sz = 0;
        Ok((
            Curve25519Public {
                key: {
                    let (v, fsz) = {
                        let mut buf: [u8; 32i64 as usize] = unsafe { ::std::mem::uninitialized() };
                        let sz =
                            xdr_codec::unpack_opaque_array(input, &mut buf[..], 32i64 as usize)?;
                        (buf, sz)
                    };
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for Curve25519Secret {
    fn unpack(input: &mut In) -> xdr_codec::Result<(Curve25519Secret, usize)> {
        let mut sz = 0;
        Ok((
            Curve25519Secret {
                key: {
                    let (v, fsz) = {
                        let mut buf: [u8; 32i64 as usize] = unsafe { ::std::mem::uninitialized() };
                        let sz =
                            xdr_codec::unpack_opaque_array(input, &mut buf[..], 32i64 as usize)?;
                        (buf, sz)
                    };
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for Hash {
    fn unpack(input: &mut In) -> xdr_codec::Result<(Hash, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (v, usz) = {
                    let mut buf: [u8; 32i64 as usize] = unsafe { ::std::mem::uninitialized() };
                    let sz = xdr_codec::unpack_opaque_array(input, &mut buf[..], 32i64 as usize)?;
                    (buf, sz)
                };
                sz = usz;
                Hash(v)
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for HmacSha256Key {
    fn unpack(input: &mut In) -> xdr_codec::Result<(HmacSha256Key, usize)> {
        let mut sz = 0;
        Ok((
            HmacSha256Key {
                key: {
                    let (v, fsz) = {
                        let mut buf: [u8; 32i64 as usize] = unsafe { ::std::mem::uninitialized() };
                        let sz =
                            xdr_codec::unpack_opaque_array(input, &mut buf[..], 32i64 as usize)?;
                        (buf, sz)
                    };
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for HmacSha256Mac {
    fn unpack(input: &mut In) -> xdr_codec::Result<(HmacSha256Mac, usize)> {
        let mut sz = 0;
        Ok((
            HmacSha256Mac {
                mac: {
                    let (v, fsz) = {
                        let mut buf: [u8; 32i64 as usize] = unsafe { ::std::mem::uninitialized() };
                        let sz =
                            xdr_codec::unpack_opaque_array(input, &mut buf[..], 32i64 as usize)?;
                        (buf, sz)
                    };
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for PublicKey {
    fn unpack(input: &mut In) -> xdr_codec::Result<(PublicKey, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => PublicKey::PUBLIC_KEY_TYPE_ED25519({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                v => return Err(xdr_codec::Error::invalidcase(v as i32)),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for PublicKeyType {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(PublicKeyType, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == PublicKeyType::PUBLIC_KEY_TYPE_ED25519 as i32 => {
                        PublicKeyType::PUBLIC_KEY_TYPE_ED25519
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for Signature {
    fn unpack(input: &mut In) -> xdr_codec::Result<(Signature, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (v, usz) = xdr_codec::unpack_opaque_flex(input, Some(64i64 as usize))?;
                sz = usz;
                Signature(v)
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for SignatureHint {
    fn unpack(input: &mut In) -> xdr_codec::Result<(SignatureHint, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (v, usz) = {
                    let mut buf: [u8; 4i64 as usize] = unsafe { ::std::mem::uninitialized() };
                    let sz = xdr_codec::unpack_opaque_array(input, &mut buf[..], 4i64 as usize)?;
                    (buf, sz)
                };
                sz = usz;
                SignatureHint(v)
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for SignerKey {
    fn unpack(input: &mut In) -> xdr_codec::Result<(SignerKey, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => SignerKey::SIGNER_KEY_TYPE_ED25519({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (1i32 as i32) => SignerKey::SIGNER_KEY_TYPE_PRE_AUTH_TX({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (2i32 as i32) => SignerKey::SIGNER_KEY_TYPE_HASH_X({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                v => return Err(xdr_codec::Error::invalidcase(v as i32)),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for SignerKeyType {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(SignerKeyType, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == SignerKeyType::SIGNER_KEY_TYPE_ED25519 as i32 => {
                        SignerKeyType::SIGNER_KEY_TYPE_ED25519
                    }
                    x if x == SignerKeyType::SIGNER_KEY_TYPE_PRE_AUTH_TX as i32 => {
                        SignerKeyType::SIGNER_KEY_TYPE_PRE_AUTH_TX
                    }
                    x if x == SignerKeyType::SIGNER_KEY_TYPE_HASH_X as i32 => {
                        SignerKeyType::SIGNER_KEY_TYPE_HASH_X
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for uint256 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(uint256, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (v, usz) = {
                    let mut buf: [u8; 32i64 as usize] = unsafe { ::std::mem::uninitialized() };
                    let sz = xdr_codec::unpack_opaque_array(input, &mut buf[..], 32i64 as usize)?;
                    (buf, sz)
                };
                sz = usz;
                uint256(v)
            },
            sz,
        ))
    }
}
