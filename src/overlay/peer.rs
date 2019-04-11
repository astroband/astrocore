use log::{debug, error, info};
use rand::Rng;
use sha2::Digest;
use std::hash::{Hash, Hasher};
use byteorder::{BigEndian, WriteBytesExt};
use std::io::{Cursor, Read, Write};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::crypto;
use crate::scp::local_node::LocalNode;
use crate::xdr;

use serde_xdr;

#[derive(Debug)]
pub struct Peer {
    /// Information about our node
    node_info: LocalNode,
    /// Socket for write/read with connected peer
    stream: std::net::TcpStream,
    /// Current message sequence position.
    send_message_sequence: xdr::Uint64,
    /// Signed certificate for a hour
    cached_auth_cert: xdr::AuthCert,
    // Authentication system keys. Our ECDH secret and public keys are randomized on startup
    // More info in: stellar-core/src/overlay/PeerAuth.h file
    /// Private authentication system key
    auth_secret_key: crypto::Curve25519Secret,
    /// Public authentication system key
    auth_public_key: crypto::Curve25519Public,
    /// Shared key with peer
    auth_shared_key: crypto::HmacSha256Key,
    /// Received MAC key from peer
    received_mac_key: crypto::HmacSha256Key,
    /// Sended MAC key to peer
    sended_mac_key: crypto::HmacSha256Key,
    /// Auth nonce
    nonce: [u8; 32],
    /// Signed Hello message
    hello: xdr::Hello,
    /// Peer remote address
    address: String,
    /// Received hello message from peer
    peer_info: xdr::Hello,
    /// authenticated peer flag
    is_authenticated: bool,
}

pub trait PeerInterface {
    fn start_authentication(&mut self) -> ();
    fn handle_hello(&mut self, received_hello: xdr::StellarMessage) -> ();
    fn set_remote_keys(
        &mut self,
        remote_pub_key: xdr::Curve25519Public,
        received_nonce: xdr::Uint256,
        we_called_remote: bool,
    ) -> ();
    fn new_auth_cert(
        node_info: &LocalNode,
        auth_public_key: &crypto::Curve25519Public,
    ) -> xdr::AuthCert;
    fn send_message(&mut self, message: xdr::StellarMessage);
    fn send_header(&mut self, message_length: u32);
    fn receive_message(
        &mut self,
    ) -> Result<xdr::AuthenticatedMessage, serde_xdr::CompatDeserializationError>;
    fn receive_header(&mut self) -> usize;
    fn increment_message_sequence(&mut self);
    fn set_authenticated(&mut self);
    fn is_authenticated(&self) -> bool;
    fn address(&self) -> &String;
}

impl Peer {
    /// Return peer instance with connection
    pub fn new(node_info: LocalNode, stream: std::net::TcpStream, address: String) -> Peer {
        let mut rng = rand::thread_rng();
        let nonce: [u8; 32] = rng.gen();

        let auth_secret_key = crypto::Curve25519Secret::random();
        let auth_public_key = crypto::Curve25519Public::derive_from_secret(&auth_secret_key);

        let mut public_key: [u8; 32] = Default::default();
        public_key.copy_from_slice(node_info.key_pair.public_key().buf());
        let peer_id = xdr::PublicKey::Ed25519(xdr::Uint256(public_key));

        let auth_cert = Peer::new_auth_cert(&node_info, &auth_public_key);

        let hello = xdr::Hello {
            ledger_version: 9000 as xdr::Uint32,
            overlay_version: 9000 as xdr::Uint32,
            overlay_min_version: 0 as xdr::Uint32,
            network_id: node_info.network_id,
            version_str: String::from("stellar-core-rust[alpha-0.0]"),
            listening_port: 11625,
            peer_id: peer_id,
            cert: auth_cert.clone(),
            nonce: xdr::Uint256(nonce),
        };

        Peer {
            node_info: node_info,
            stream: stream,
            send_message_sequence: 0 as xdr::Uint64,
            cached_auth_cert: auth_cert,
            auth_secret_key: auth_secret_key,
            auth_public_key: auth_public_key,
            auth_shared_key: crypto::HmacSha256Key::zero(),
            received_mac_key: crypto::HmacSha256Key::zero(),
            sended_mac_key: crypto::HmacSha256Key::zero(),
            nonce: nonce,
            hello: hello,
            address: address,
            peer_info: Default::default(),
            is_authenticated: false,
        }
    }
}

impl PeerInterface for Peer {
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
    fn start_authentication(&mut self) -> () {
        info!(
            "[Overlay] Started authentication proccess peer: {}",
            self.address
        );

        self.send_message(xdr::StellarMessage::Hello(self.hello.clone()));
        match self.receive_message() {
            Ok(xdr::AuthenticatedMessage::V0(hello)) => {
                self.handle_hello(hello.message);
            }
            _ => {
                error!(
                    "[Overlay] Received not hello message from peer {}. Authentication aborted",
                    self.address
                );
                return;
            }
        }

        self.send_message(xdr::StellarMessage::Auth(xdr::Auth { unused: 0 }));
        // last auth message from remote peer
        match self.receive_message() {
            Err(_) => {
                error!(
                    "[Overlay] Not received last auth message {}. Authentication aborted",
                    self.address
                );
                return;
            },
            _ => {},
        }

        self.set_authenticated();

        info!(
            "[Overlay] Authentication completed for peer {}",
            self.address
        );
    }

    fn handle_hello(&mut self, received_hello: xdr::StellarMessage) {
        match received_hello {
            xdr::StellarMessage::Hello(hello) => {
                self.set_remote_keys(hello.cert.pubkey, hello.nonce, true);
                self.peer_info = hello;
            }
            _ => error!("[Overlay] Received non hello message"),
        }
    }

    /// Set hmac keys with remote peer
    fn set_remote_keys(
        &mut self,
        remote_pub_key: xdr::Curve25519Public,
        received_nonce: xdr::Uint256,
        we_called_remote: bool,
    ) {
        let mut public_a: [u8; 32] = Default::default();
        let mut public_b: [u8; 32] = Default::default();

        if we_called_remote {
            public_a.copy_from_slice(&self.auth_public_key.0[..]);
            public_b.copy_from_slice(&remote_pub_key.key[..]);
        } else {
            public_a.copy_from_slice(&remote_pub_key.key[..]);
            public_b.copy_from_slice(&self.auth_public_key.0[..]);
        }

        let scalarmult =
            crypto::Curve25519Public::scalarmult(&self.auth_secret_key, &remote_pub_key.key);

        let mut buffer: Vec<u8> = Default::default();
        buffer.extend(&scalarmult[..]);
        buffer.extend(public_a.iter().cloned());
        buffer.extend(public_b.iter().cloned());

        self.auth_shared_key = crypto::HmacSha256Key::hkdf_extract(&buffer[..]);

        // Set up sendingMacKey
        // If weCalled then sending key is K_AB,
        // and A is local and B is remote.
        // If REMOTE_CALLED_US then sending key is K_BA,
        // and B is local and A is remote.

        let mut buffer: Vec<u8> = Default::default();
        if we_called_remote {
            buffer.push(0)
        } else {
            buffer.push(1)
        }
        buffer.extend(self.nonce.iter().cloned());
        buffer.extend(received_nonce.0.iter().cloned());

        self.sended_mac_key =
            crypto::HmacSha256Key::hkdf_expand(&self.auth_shared_key, &buffer[..]);

        // Set up receivingMacKey
        let mut buffer: Vec<u8> = Default::default();

        if we_called_remote {
            buffer.push(0)
        } else {
            buffer.push(1)
        }
        buffer.extend(received_nonce.0.iter().cloned());
        buffer.extend(self.nonce.iter().cloned());

        self.received_mac_key =
            crypto::HmacSha256Key::hkdf_expand(&self.auth_shared_key, &buffer[..]);
    }

    /// Make expired certicate for all connection with peers
    fn new_auth_cert(
        node_info: &LocalNode,
        auth_public_key: &crypto::Curve25519Public,
    ) -> xdr::AuthCert {
        let unix_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let expiration_limit: u64 = 3600; // 1 hour
        let expiration: xdr::Uint64 = expiration_limit + unix_time;

        let mut buffer = Vec::new();

        serde_xdr::to_writer(&mut buffer, &node_info.network_id).unwrap();
        serde_xdr::to_writer(&mut buffer, &xdr::EnvelopeType::EnvelopeTypeAuth).unwrap();
        serde_xdr::to_writer(&mut buffer, &expiration).unwrap();
        serde_xdr::to_writer(
            &mut buffer,
            &xdr::Curve25519Public {
                key: auth_public_key.0,
            },
        )
        .unwrap();

        let mut hasher = sha2::Sha256::new();
        hasher.input(buffer);
        let hash = hasher.result();
        let sign = &node_info.key_pair.sign(&hash);

        xdr::AuthCert {
            pubkey: xdr::Curve25519Public {
                key: auth_public_key.0,
            },
            expiration: expiration,
            sig: xdr::Signature(sign.to_vec()),
        }
    }

    // TODO: mutex required?
    /// Send XDR message to remote peer
    fn send_message(&mut self, message: xdr::StellarMessage) {
        let mut am0 = xdr::AuthenticatedMessageV0 {
            sequence: self.send_message_sequence,
            message: message,
            mac: xdr::HmacSha256Mac {
                mac: crypto::HmacSha256Mac::zero().0,
            },
        };

        match am0.message {
            xdr::StellarMessage::Hello(_) | xdr::StellarMessage::Error(_) => {}
            _ => {
                let mut packed_auth_message_v0 = Vec::new();
                serde_xdr::to_writer(&mut packed_auth_message_v0, &am0.sequence).unwrap();
                serde_xdr::to_writer(&mut packed_auth_message_v0, &am0.message).unwrap();
                let mac = crypto::HmacSha256Mac::authenticate(
                    &packed_auth_message_v0[..],
                    &self.sended_mac_key,
                );
                am0.mac = xdr::HmacSha256Mac { mac: mac.0 };
                self.increment_message_sequence();
            }
        };

        let am = xdr::AuthenticatedMessage::V0(am0);

        let packed_auth_message = serde_xdr::to_bytes(&am).unwrap();

        self.send_header(packed_auth_message.len() as u32);

        self.stream.write(&packed_auth_message[..]).unwrap();
    }

    /// Send legnth of of upcoming message fragment
    fn send_header(&mut self, message_length: u32) {
        // In RPC (see RFC5531 section 11), the high bit means this is the
        // last record fragment in a record.  If the high bit is clear, it
        // means another fragment follows.  We don't currently implement
        // continuation fragments, and instead always set the last-record
        // bit to produce a single-fragment record.

        let mut header = Vec::new();
        header
            .write_u32::<BigEndian>(message_length | 0x80000000)
            .unwrap();
        self.stream.write(&header[..]).unwrap();
    }

    // We always receive messages as single-fragment messages.
    /// Get legnth of incoming message fragment
    fn receive_header(&mut self) -> usize {
        let mut header: [u8; 4] = Default::default();
        if let Err(e) = self.stream.read_exact(&mut header) {
            error!("[Overlay] header reading error: {}", e);
            return 0;
        }

        let mut message_length: usize;
        message_length = header[0] as usize; // clear the XDR 'continuation' bit
        message_length &= 0x7f;
        message_length <<= 8;
        message_length |= header[1] as usize;
        message_length <<= 8;
        message_length |= header[2] as usize;
        message_length <<= 8;
        message_length |= header[3] as usize;

        debug!(
            "[Overlay] RECEIVE HEADER {:?} \nWITH LENGTH {:?}",
            header, message_length
        );

        return message_length;
    }

    fn receive_message(
        &mut self,
    ) -> Result<xdr::AuthenticatedMessage, serde_xdr::CompatDeserializationError> {
        let message_length = self.receive_header();

        let mut message_content = vec![0u8; message_length];
        debug!("[Overlay] Message len {:?}", message_content.len());

        self.stream.read_exact(&mut message_content).unwrap();
        debug!("[Overlay] Message content {:?}", message_content);

        let mut cursor = Cursor::new(message_content);

        let authenticated_message: Result<
            xdr::AuthenticatedMessage,
            serde_xdr::CompatDeserializationError,
        > = serde_xdr::from_reader(&mut cursor);

        return authenticated_message;
    }

    fn increment_message_sequence(&mut self) {
        self.send_message_sequence = self.send_message_sequence + 1;
    }

    fn set_authenticated(&mut self) {
        self.is_authenticated = true;
    }

    fn is_authenticated(&self) -> bool {
        self.is_authenticated
    }

    fn address(&self) -> &String {
        &self.address
    }
}

impl Hash for Peer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.address.hash(state);
    }
}

impl PartialEq for Peer {
    fn eq(&self, other: &Peer) -> bool {
        self.address == other.address
    }
}
impl Eq for Peer {}
