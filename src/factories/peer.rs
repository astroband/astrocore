        
use crate::overlay::peer::{PeerInterface};
use crate::xdr;
use crate::crypto;
use crate::scp::local_node::LocalNode;

pub struct PeerMock {
    pub address: String,
    pub is_authenticated: bool,
}


impl PeerInterface for PeerMock {
    fn start_authentication(&mut self) -> () {}

    fn handle_hello(&mut self, received_hello: xdr::StellarMessage) -> () {}

    fn set_remote_keys(
        &mut self,
        remote_pub_key: xdr::Curve25519Public,
        received_nonce: xdr::Uint256,
        we_called_remote: bool,
    ) {}

    fn new_auth_cert(
        node_info: &LocalNode,
        auth_public_key: &crypto::Curve25519Public,
    ) -> xdr::AuthCert {
        xdr::AuthCert::default()
    }

    fn send_message(&mut self, message: xdr::StellarMessage) {}

    fn send_header(&mut self, message_length: u32) {}

    fn receive_message(
        &mut self,
    ) -> Result<xdr::AuthenticatedMessage, serde_xdr::CompatDeserializationError> {
        Ok(xdr::AuthenticatedMessage::default())
    }

    fn receive_header(&mut self) -> usize { 128 }

    fn increment_message_sequence(&mut self) {}

    fn set_authenticated(&mut self) {}

    fn is_authenticated(&self) -> bool { true }

    fn address(&self) -> &String {
        &self.address
    }
}
