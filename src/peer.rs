use sha2::{Digest};
use rand::{Rng};
use sodiumoxide::crypto::sign::ed25519;

use std::time::{SystemTime, UNIX_EPOCH};
use byteorder::{BigEndian, WriteBytesExt};

use crate::crypto;
use crate::xdr;
use crate::node_info::{NodeInfo};

pub struct Peer<'a, 'b> {
    /// Information about our node
    node_info: &'a NodeInfo,
    /// Socket for write/read with connected peer
    stream: &'b std::net::TcpStream,
    /// Current message sequence position.
    send_message_sequence: xdr::uint64,
    /// Signed certificate for a hour
    cached_auth_cert: xdr::AuthCert,
    // Authentication system keys. Our ECDH secret and public keys are randomized on startup
    // More info in: stellar-core/src/overlay/PeerAuth.h file
    /// Private authentication system key 
    auth_secret_key: crypto::Curve25519Secret,
    /// Public authentication system key
    auth_public_key: crypto::Curve25519Public,
    /// Shared key with peer
    auth_shared_key: Vec<u8>,
    /// Received MAC key from peer
    receiving_mac_key: Vec<u8>,
    /// Sended MAC key to peer
    sending_mac_key: Vec<u8>,
    /// Auth nonce
    nonce: [u8; 32],
    /// Signed Hello message
    hello: xdr::Hello,
    /// Peer remote address
    address: String,
}

impl<'a, 'b> Peer<'a, 'b> {
    /// Return peer instance with connection
    pub fn new(node_info: &'a NodeInfo, stream: &'b std::net::TcpStream, address: String) -> Peer<'a, 'b> {
        let mut rng = rand::thread_rng();
        let nonce: [u8; 32] = rng.gen();
    
        let auth_secret_key = crypto::Curve25519Secret::random();
        let auth_public_key = crypto::Curve25519Public::derive_from_secret(&auth_secret_key);

        let peer_id = xdr::PublicKey::PUBLIC_KEY_TYPE_ED25519(node_info.public_key);

        let auth_cert = Peer::get_auth_cert(&node_info, &auth_public_key);
        let cloned_auth_cert = auth_cert.clone();

        let hello = xdr::Hello {
            ledgerVersion: 9000 as xdr::uint32,
            overlayVersion: 9000 as xdr::uint32,
            overlayMinVersion: 0 as xdr::uint32,
            networkID: node_info.network_id,
            versionStr: String::from("stellar-core-rust[alpha-0.0]"),
            listeningPort: 11625,
            peerID: peer_id,
            cert: auth_cert,
            nonce: xdr::uint256{0: nonce},
        };

        dbg!("FAIL HERE >>>>");
        let mut buffer = Vec::new();
        xdr_codec::pack(&hello, &mut buffer).unwrap();

        Peer{
            node_info: &node_info,
            stream: &stream,
            send_message_sequence: 0 as xdr::uint64,
            cached_auth_cert: cloned_auth_cert,
            auth_secret_key: auth_secret_key,
            auth_public_key: auth_public_key,
            auth_shared_key: Default::default(),
            receiving_mac_key: Default::default(),
            sending_mac_key: Default::default(),
            nonce: nonce,
            hello: hello,
            address: address,
        }
    }

    // Connection process:
    // A wants to connect to B
    // A initiates a tcp connection to B
    // connection is established
    // A sends HELLO(CertA,NonceA) to B
    // B now has IP and listening port of A, sends HELLO(CertB,NonceB) back
    // A sends AUTH(signed([0],keyAB))
    // B verifies and either:
    //     sends AUTH(signed([0],keyBA)) back or
    //     disconnects, if it's full, optionally sending a list of other peers to try first
    // keyAB and keyBA are per-connection HMAC keys derived from non-interactive
    // ECDH on random curve25519 keys conveyed in CertA and CertB (certs signed by
    // Node Ed25519 keys) the result of which is then fed through HKDF with the
    // per-connection nonces. See PeerAuth.h.
    // If any verify step fails, the peer disconnects immediately.
    /// Start connection process to peer.
    /// More additional info: https://github.com/stellar/stellar-core/blob/ddef8bcacc5193bdd4daa07af404f1b6b1adaf39/src/overlay/OverlayManagerImpl.cpp#L28-L45
    pub fn start_authentication(&self) -> () {
        self.send_hello_message();

        // implement other stage of process
        
    }

    /// Make expired certicate for all connection with peers
    fn get_auth_cert(node_info: &NodeInfo, auth_public_key: &crypto::Curve25519Public) -> xdr::AuthCert {
        let unix_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        let expiration_limit: u64 = 3600; // 1 hour
        let expiration: xdr::uint64 = expiration_limit + unix_time;

        let mut buffer = Vec::new();

        xdr_codec::pack(&node_info.network_id, &mut buffer).unwrap();
        xdr_codec::pack(&xdr::EnvelopeType::ENVELOPE_TYPE_AUTH, &mut buffer).unwrap();
        xdr_codec::pack(&expiration, &mut buffer).unwrap();
        xdr_codec::pack(&xdr::Curve25519Public{key: auth_public_key.0}, &mut buffer).unwrap();

        let mut hasher = sha2::Sha256::new();
        hasher.input(buffer);
        let hash = hasher.result();
        let sign = ed25519::sign(&hash, &node_info.private_key);

        xdr::AuthCert{
            pubkey: xdr::Curve25519Public{key: auth_public_key.0},
            expiration: expiration,
            sig: xdr::Signature{0: sign},
        }
    }

    pub fn send_hello_message(&self) {
        let hello_message = xdr::StellarMessage::HELLO(self.hello.clone());

        let am0 = xdr::AuthenticatedMessageV0{
            sequence: 0 as xdr::uint64,
            message: hello_message,
            mac: xdr::HmacSha256Mac{mac: crypto::HmacSha256Mac::zero().0},
        };

        let mut buffer = Vec::new();
        xdr_codec::pack(&am0, &mut buffer).unwrap();
        self.send_header(buffer.len() as u32);
        // self.stream.write(buffer).unwrap();
    }

    fn send_header(&self, message_length: u32) {
        // In RPC (see RFC5531 section 11), the high bit means this is the
        // last record fragment in a record.  If the high bit is clear, it
        // means another fragment follows.  We don't currently implement
        // continuation fragments, and instead always set the last-record
        // bit to produce a single-fragment record.
       
        let mut header = Vec::new();
        header.write_u32::<BigEndian>(message_length | 0x80000000).unwrap();
        // self.stream.write(header).unwrap();
    }
}

// NOTE: should be implemented by xdrgen
impl Clone for xdr::AuthCert {
    fn clone(&self) -> xdr::AuthCert {
        xdr::AuthCert{
            pubkey: self.pubkey.clone(),
            expiration: self.expiration.clone(),
            sig: self.sig.clone(),
        }
    }
}

// NOTE: should be implemented by xdrgen
impl Clone for xdr::Hello {
    fn clone(&self) -> xdr::Hello {
        xdr::Hello {
            ledgerVersion: self.ledgerVersion.clone(),
            overlayVersion: self.overlayVersion.clone(),
            overlayMinVersion: self.overlayMinVersion.clone(),
            networkID: self.networkID.clone(),
            versionStr: self.versionStr.clone(),
            listeningPort: self.listeningPort.clone(),
            peerID: self.peerID.clone(),
            cert: self.cert.clone(),
            nonce: self.nonce.clone(),
        }
    }
}
