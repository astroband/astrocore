use base64;
use bigdecimal;
use serde_xdr;
use std::convert::From;
use std::result;
use std::str;

/// The Errors that can occur.
#[derive(Debug)]
pub enum Error {
    /// Error that can occur when parsing a key.
    InvalidStrKey,
    /// Invalid version byte in key.
    InvalidStrKeyVersionByte,
    /// Invalid checksum in key.
    InvalidStrKeyChecksum,
    /// Invalid keypair seed.
    InvalidSeed,
    /// Invalid Asset code.
    InvalidAssetCode,
    /// Invalid signature.
    InvalidSignature,
    /// Invalid memo text: too long.
    InvalidMemoText,
    /// Error that can occur when parsing amounts from stroops.
    InvalidStroopsAmount,
    /// Error that can occur when converting an amount with more than 7 digits.
    InvalidAmountScale,
    /// Invalid network id: too long.
    InvalidNetworkId,
    /// Invalid public key.
    InvalidPublicKey,
    /// Error that can occur when interpreting a sequence of `u8` as utf-8.
    Utf8Error(str::Utf8Error),
    /// Error that can occour when decoding base64 encoded data.
    DecodeError(base64::DecodeError),
    /// Error that can occur when parsing amounts.
    ParseAmountError(bigdecimal::ParseBigDecimalError),
    /// Error that can occur when serializing to XDR.
    SerializationError(serde_xdr::CompatSerializationError),
    /// Error that can occur when deserializing from XDR.
    DeserializationError(serde_xdr::CompatDeserializationError),
}

/// A `Result` alias where `Error` is a `shuttle_core::Error`.
pub type Result<T> = result::Result<T, Error>;

impl From<str::Utf8Error> for Error {
    fn from(err: str::Utf8Error) -> Self {
        Error::Utf8Error(err)
    }
}

impl From<base64::DecodeError> for Error {
    fn from(err: base64::DecodeError) -> Self {
        Error::DecodeError(err)
    }
}

impl From<bigdecimal::ParseBigDecimalError> for Error {
    fn from(err: bigdecimal::ParseBigDecimalError) -> Self {
        Error::ParseAmountError(err)
    }
}

impl From<serde_xdr::CompatDeserializationError> for Error {
    fn from(err: serde_xdr::CompatDeserializationError) -> Self {
        Error::DeserializationError(err)
    }
}

impl From<serde_xdr::CompatSerializationError> for Error {
    fn from(err: serde_xdr::CompatSerializationError) -> Self {
        Error::SerializationError(err)
    }
}
