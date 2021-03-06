#![allow(unused_variables)]

use crate::overlay::peer::{MessageReceiveError, PeerInterface};
use crate::scp::local_node::LocalNode;
use crate::xdr;
use x25519_dalek::PublicKey;

pub struct PeerMock {
    pub address: String,
    pub is_authenticated: bool,
}

impl PeerInterface for PeerMock {
    fn start_authentication(&mut self, we_called_remote: bool) {}

    fn handle_hello(&mut self, received_hello: xdr::StellarMessage, we_called_remote: bool) {}

    fn set_remote_keys(
        &mut self,
        remote_pub_key: xdr::Curve25519Public,
        received_nonce: xdr::Uint256,
        we_called_remote: bool,
    ) {
    }

    fn new_auth_cert(node_info: &LocalNode, auth_public_key: &PublicKey) -> xdr::AuthCert {
        xdr::AuthCert::default()
    }

    fn send_message(&mut self, message: xdr::StellarMessage) {}

    fn send_header(&mut self, message_length: u32) {}

    fn receive_message(&mut self) -> Result<xdr::AuthenticatedMessage, MessageReceiveError> {
        Ok(xdr::AuthenticatedMessage::default())
    }

    fn receive_header(&mut self) -> usize {
        128
    }

    fn increment_message_sequence(&mut self) {}

    fn set_authenticated(&mut self) {}

    fn is_authenticated(&self) -> bool {
        true
    }

    fn address(&self) -> &String {
        &self.address
    }
}
