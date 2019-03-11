// GENERATED CODE
//
// Generated from src/xdr/Stellar-overlay.x by xdrgen.
//
// DO NOT EDIT

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Auth {
    pub unused: i32,
}

pub struct AuthCert {
    pub pubkey: Curve25519Public,
    pub expiration: uint64,
    pub sig: Signature,
}

pub struct AuthenticatedMessage {
    pub V: uint32,
    pub V0: AuthenticatedMessageV0,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AuthenticatedMessageType {
    V0 = 0isize,
}

pub struct AuthenticatedMessageV0 {
    pub sequence: uint64,
    pub message: StellarMessage,
    pub mac: HmacSha256Mac,
}

pub struct DontHave {
    pub type_: MessageType,
    pub reqHash: uint256,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Error {
    pub code: ErrorCode,
    pub msg: String,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ErrorCode {
    ERR_MISC = 0isize,
    ERR_DATA = 1isize,
    ERR_CONF = 2isize,
    ERR_AUTH = 3isize,
    ERR_LOAD = 4isize,
}

pub struct Hello {
    pub ledgerVersion: uint32,
    pub overlayVersion: uint32,
    pub overlayMinVersion: uint32,
    pub networkID: Hash,
    pub versionStr: String,
    pub listeningPort: i32,
    pub peerID: NodeID,
    pub cert: AuthCert,
    pub nonce: uint256,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum IPAddrType {
    IPv4 = 0isize,
    IPv6 = 1isize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MessageType {
    ERROR_MSG = 0isize,
    AUTH = 2isize,
    DONT_HAVE = 3isize,
    GET_PEERS = 4isize,
    PEERS = 5isize,
    GET_TX_SET = 6isize,
    TX_SET = 7isize,
    TRANSACTION = 8isize,
    GET_SCP_QUORUMSET = 9isize,
    SCP_QUORUMSET = 10isize,
    SCP_MESSAGE = 11isize,
    GET_SCP_STATE = 12isize,
    HELLO = 13isize,
}

pub struct PeerAddress {
    pub port: uint32,
    pub numFailures: uint32,
}

pub enum StellarMessage {
    ERROR_MSG(Error),
    HELLO(Hello),
    AUTH(Auth),
    DONT_HAVE(DontHave),
    GET_PEERS,
    PEERS(Vec<PeerAddress>),
    GET_TX_SET(uint256),
    TX_SET(TransactionSet),
    TRANSACTION(TransactionEnvelope),
    GET_SCP_QUORUMSET(uint256),
    SCP_QUORUMSET(SCPQuorumSet),
    SCP_MESSAGE(SCPEnvelope),
    GET_SCP_STATE(uint32),
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for Auth {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.unused.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for AuthCert {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.pubkey.pack(out)? + self.expiration.pack(out)? + self.sig.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for AuthenticatedMessage {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.V.pack(out)? + self.V0.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for AuthenticatedMessageType {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for AuthenticatedMessageV0 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.sequence.pack(out)? + self.message.pack(out)? + self.mac.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for DontHave {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.type_.pack(out)? + self.reqHash.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for Error {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.code.pack(out)?
            + xdr_codec::pack_string(&self.msg, Some(100i64 as usize), out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for ErrorCode {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for Hello {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.ledgerVersion.pack(out)?
            + self.overlayVersion.pack(out)?
            + self.overlayMinVersion.pack(out)?
            + self.networkID.pack(out)?
            + xdr_codec::pack_string(&self.versionStr, Some(100i64 as usize), out)?
            + self.listeningPort.pack(out)?
            + self.peerID.pack(out)?
            + self.cert.pack(out)?
            + self.nonce.pack(out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for IPAddrType {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for MessageType {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for PeerAddress {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.port.pack(out)? + self.numFailures.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for StellarMessage {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &StellarMessage::ERROR_MSG(ref val) => {
                (MessageType::ERROR_MSG as i32).pack(out)? + val.pack(out)?
            }
            &StellarMessage::HELLO(ref val) => {
                (MessageType::HELLO as i32).pack(out)? + val.pack(out)?
            }
            &StellarMessage::AUTH(ref val) => {
                (MessageType::AUTH as i32).pack(out)? + val.pack(out)?
            }
            &StellarMessage::DONT_HAVE(ref val) => {
                (MessageType::DONT_HAVE as i32).pack(out)? + val.pack(out)?
            }
            &StellarMessage::GET_PEERS => (MessageType::GET_PEERS as i32).pack(out)?,
            &StellarMessage::PEERS(ref val) => {
                (MessageType::PEERS as i32).pack(out)?
                    + xdr_codec::pack_flex(&val, Some(100i64 as usize), out)?
            }
            &StellarMessage::GET_TX_SET(ref val) => {
                (MessageType::GET_TX_SET as i32).pack(out)? + val.pack(out)?
            }
            &StellarMessage::TX_SET(ref val) => {
                (MessageType::TX_SET as i32).pack(out)? + val.pack(out)?
            }
            &StellarMessage::TRANSACTION(ref val) => {
                (MessageType::TRANSACTION as i32).pack(out)? + val.pack(out)?
            }
            &StellarMessage::GET_SCP_QUORUMSET(ref val) => {
                (MessageType::GET_SCP_QUORUMSET as i32).pack(out)? + val.pack(out)?
            }
            &StellarMessage::SCP_QUORUMSET(ref val) => {
                (MessageType::SCP_QUORUMSET as i32).pack(out)? + val.pack(out)?
            }
            &StellarMessage::SCP_MESSAGE(ref val) => {
                (MessageType::SCP_MESSAGE as i32).pack(out)? + val.pack(out)?
            }
            &StellarMessage::GET_SCP_STATE(ref val) => {
                (MessageType::GET_SCP_STATE as i32).pack(out)? + val.pack(out)?
            }
        })
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for Auth {
    fn unpack(input: &mut In) -> xdr_codec::Result<(Auth, usize)> {
        let mut sz = 0;
        Ok((
            Auth {
                unused: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for AuthCert {
    fn unpack(input: &mut In) -> xdr_codec::Result<(AuthCert, usize)> {
        let mut sz = 0;
        Ok((
            AuthCert {
                pubkey: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                expiration: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                sig: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for AuthenticatedMessage {
    fn unpack(input: &mut In) -> xdr_codec::Result<(AuthenticatedMessage, usize)> {
        let mut sz = 0;
        Ok((
            AuthenticatedMessage {
                V: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                V0: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for AuthenticatedMessageType {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(AuthenticatedMessageType, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == AuthenticatedMessageType::V0 as i32 => AuthenticatedMessageType::V0,
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for AuthenticatedMessageV0 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(AuthenticatedMessageV0, usize)> {
        let mut sz = 0;
        Ok((
            AuthenticatedMessageV0 {
                sequence: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                message: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                mac: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for DontHave {
    fn unpack(input: &mut In) -> xdr_codec::Result<(DontHave, usize)> {
        let mut sz = 0;
        Ok((
            DontHave {
                type_: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                reqHash: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for Error {
    fn unpack(input: &mut In) -> xdr_codec::Result<(Error, usize)> {
        let mut sz = 0;
        Ok((
            Error {
                code: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                msg: {
                    let (v, fsz) = xdr_codec::unpack_string(input, Some(100i64 as usize))?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for ErrorCode {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(ErrorCode, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == ErrorCode::ERR_MISC as i32 => ErrorCode::ERR_MISC,
                    x if x == ErrorCode::ERR_DATA as i32 => ErrorCode::ERR_DATA,
                    x if x == ErrorCode::ERR_CONF as i32 => ErrorCode::ERR_CONF,
                    x if x == ErrorCode::ERR_AUTH as i32 => ErrorCode::ERR_AUTH,
                    x if x == ErrorCode::ERR_LOAD as i32 => ErrorCode::ERR_LOAD,
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for Hello {
    fn unpack(input: &mut In) -> xdr_codec::Result<(Hello, usize)> {
        let mut sz = 0;
        Ok((
            Hello {
                ledgerVersion: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                overlayVersion: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                overlayMinVersion: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                networkID: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                versionStr: {
                    let (v, fsz) = xdr_codec::unpack_string(input, Some(100i64 as usize))?;
                    sz += fsz;
                    v
                },
                listeningPort: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                peerID: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                cert: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                nonce: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for IPAddrType {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(IPAddrType, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == IPAddrType::IPv4 as i32 => IPAddrType::IPv4,
                    x if x == IPAddrType::IPv6 as i32 => IPAddrType::IPv6,
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for MessageType {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(MessageType, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == MessageType::ERROR_MSG as i32 => MessageType::ERROR_MSG,
                    x if x == MessageType::AUTH as i32 => MessageType::AUTH,
                    x if x == MessageType::DONT_HAVE as i32 => MessageType::DONT_HAVE,
                    x if x == MessageType::GET_PEERS as i32 => MessageType::GET_PEERS,
                    x if x == MessageType::PEERS as i32 => MessageType::PEERS,
                    x if x == MessageType::GET_TX_SET as i32 => MessageType::GET_TX_SET,
                    x if x == MessageType::TX_SET as i32 => MessageType::TX_SET,
                    x if x == MessageType::TRANSACTION as i32 => MessageType::TRANSACTION,
                    x if x == MessageType::GET_SCP_QUORUMSET as i32 => {
                        MessageType::GET_SCP_QUORUMSET
                    }
                    x if x == MessageType::SCP_QUORUMSET as i32 => MessageType::SCP_QUORUMSET,
                    x if x == MessageType::SCP_MESSAGE as i32 => MessageType::SCP_MESSAGE,
                    x if x == MessageType::GET_SCP_STATE as i32 => MessageType::GET_SCP_STATE,
                    x if x == MessageType::HELLO as i32 => MessageType::HELLO,
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for PeerAddress {
    fn unpack(input: &mut In) -> xdr_codec::Result<(PeerAddress, usize)> {
        let mut sz = 0;
        Ok((
            PeerAddress {
                port: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                numFailures: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for StellarMessage {
    fn unpack(input: &mut In) -> xdr_codec::Result<(StellarMessage, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => StellarMessage::ERROR_MSG({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (13i32 as i32) => StellarMessage::HELLO({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (2i32 as i32) => StellarMessage::AUTH({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (3i32 as i32) => StellarMessage::DONT_HAVE({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (4i32 as i32) => StellarMessage::GET_PEERS,
                x if x == (5i32 as i32) => StellarMessage::PEERS({
                    let (v, fsz) = xdr_codec::unpack_flex(input, Some(100i64 as usize))?;
                    sz += fsz;
                    v
                }),
                x if x == (6i32 as i32) => StellarMessage::GET_TX_SET({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (7i32 as i32) => StellarMessage::TX_SET({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (8i32 as i32) => StellarMessage::TRANSACTION({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (9i32 as i32) => StellarMessage::GET_SCP_QUORUMSET({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (10i32 as i32) => StellarMessage::SCP_QUORUMSET({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (11i32 as i32) => StellarMessage::SCP_MESSAGE({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (12i32 as i32) => StellarMessage::GET_SCP_STATE({
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
