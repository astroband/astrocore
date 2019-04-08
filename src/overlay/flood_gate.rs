use crate::overlay::message_abbr;
use crate::overlay::peer::PeerInterface;
use crate::xdr;
use log::info;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/**
 * FloodGate keeps track of which peers have sent us which broadcast messages,
 * in order to ensure that for each broadcast message M and for each peer P, we
 * either send M to P once (and only once), or receive M _from_ P (thereby
 * inhibit sending M to P at all).
 *
 * The broadcast message types are TRANSACTION and SCP_MESSAGE.
 *
 * All messages are marked with the ledger sequence number to which they
 * relate, and all flood-management information for a given ledger number
 * is purged from the FloodGate when the ledger closes.
 */

pub struct FloodGate {
    /// set of received messages
    pub flood_map: HashMap<String, FloodRecord>,
    /// shutdown flag
    pub m_shutting_down: bool,
}

pub struct FloodRecord {
    /// current ledger block
    pub m_ledger_seq: u32,
    /// received message
    pub m_message: xdr::StellarMessage,
    /// list of peers that sent us the message
    pub m_peers_told: Vec<String>,
}

impl FloodGate {
    pub fn new() -> Self {
        FloodGate {
            flood_map: HashMap::new(),
            m_shutting_down: false,
        }
    }

    // Floodgate will be cleared after every ledger close
    pub fn clear_below(&mut self, current_ledger: u32) {
        self.flood_map
            .retain(|_, record| record.m_ledger_seq + 10 > current_ledger);
    }

    // returns true if this is a new record
    pub fn add_record(
        &mut self,
        message: &xdr::StellarMessage,
        from_peer_addr: String,
        current_ledger: u32,
    ) -> bool {
        if self.m_shutting_down {
            return false;
        };

        let index = message_abbr(message);

        if let Some(record) = self.flood_map.get_mut(&index) {
            record.m_peers_told.push(from_peer_addr);
            return false;
        } else {
            self.flood_map.insert(
                index,
                FloodRecord::build(
                    current_ledger,
                    message.clone(),
                    vec![from_peer_addr.clone()],
                ),
            );
            return true;
        };
    }

    // send message to anyone you haven't gotten it from
    pub fn broadcast<T: PeerInterface>(&mut self, message: xdr::StellarMessage, force: bool, peers: &mut [T]) {
        if self.m_shutting_down {
            return;
        };

        let index = message_abbr(&message);
        info!("[Overlay] broadcast {}", index);

        let unix_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as u32;

        // no one has sent us this message
        if self.flood_map.get(&index).is_none() || force {
            self.add_record(&message, "self".to_string(), unix_time);
        };

        let previous_sent = &self.flood_map[&index].m_peers_told.clone();
        let record = self.flood_map.get_mut(&index).unwrap();
        for peer in peers {
            if peer.is_authenticated() && !previous_sent.contains(peer.address()) {
                peer.send_message(message.clone());
                record.m_peers_told.push(peer.address().clone());
            }
        }

        info!(
            "[Overlay] broadcast {} told {}",
            index,
            record.m_peers_told.len() - previous_sent.len()
        );
    }

    pub fn shutdown(&mut self) -> () {
        self.m_shutting_down = true;
        self.flood_map.clear();
    }
}

impl FloodRecord {
    pub fn build(
        m_ledger_seq: u32,
        m_message: xdr::StellarMessage,
        m_peers_told: Vec<String>,
    ) -> Self {
        FloodRecord {
            m_ledger_seq,
            m_message,
            m_peers_told,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::factories::flood_gate::build_flood_gate;

    mod clear_below {
        use super::*;

        #[test]
        fn lower_offset() {
            let mut flood_gate = build_flood_gate();

            let offset = 9;
            flood_gate.clear_below(200 + offset);

            assert_eq!(flood_gate.flood_map.len(), 3);
            assert!(flood_gate.flood_map.get("345").is_some());
            assert!(flood_gate.flood_map.get("678").is_some());
        }

        #[test]
        fn higher_offset() {
            let mut flood_gate = build_flood_gate();

            let offset = 10;
            flood_gate.clear_below(200 + offset);

            assert_eq!(flood_gate.flood_map.len(), 2);
            assert!(flood_gate.flood_map.get("678").is_some());
        }
    }

    mod add_record {
        use super::*;

        #[test]
        fn without_record() {
            let mut flood_gate = build_flood_gate();
            let before_add = flood_gate.flood_map.len();

            let message = xdr::StellarMessage::GetPeers;

            let result = flood_gate.add_record(&message, "192.168.5.5".to_string(), 500);

            assert!(result);
            assert_eq!(flood_gate.flood_map.len(), before_add + 1);
        }

        #[test]
        fn existed_record_with_new_peer() {
            let mut flood_gate = build_flood_gate();
            let before_add = flood_gate.flood_map.len();

            let message = xdr::StellarMessage::default();
            let index = message_abbr(&message);

            assert_eq!(
                flood_gate.flood_map[&index].m_peers_told,
                vec!["192.168.4.4".to_string()]
            );

            let result = flood_gate.add_record(&message, "192.168.99.99".to_string(), 500);

            assert_eq!(result, false);
            assert_eq!(flood_gate.flood_map.len(), before_add);
            assert_eq!(
                flood_gate.flood_map[&index].m_peers_told,
                vec!["192.168.4.4".to_string(), "192.168.99.99".to_string()]
            );
        }

        #[test]
        fn with_shutdown() {
            let mut flood_gate = build_flood_gate();
            flood_gate.shutdown();
            let result = flood_gate.add_record(
                &xdr::StellarMessage::GetPeers,
                "192.168.5.5".to_string(),
                500,
            );

            assert_eq!(result, false);
        }
    }

    mod broadcast {
        use super::*;
        use crate::factories::peer::{PeerMock};

        #[test]
        fn broadcast() {
            let mut flood_gate = build_flood_gate();
            flood_gate.flood_map.clear();

            let peer_mock1 = PeerMock { address: "0.0.0.0".to_string(), is_authenticated: true };
            let peer_mock2 = PeerMock { address: "0.0.0.1".to_string(), is_authenticated: true };
            let peer_mock3 = PeerMock { address: "0.0.0.2".to_string(), is_authenticated: true };

            let mut peers = vec![peer_mock1, peer_mock2, peer_mock3];

            let message = xdr::StellarMessage::default();
            let index = message_abbr(&message);

            flood_gate.broadcast(message, false, &mut peers);

            let record = flood_gate.flood_map.get(&index).expect("record should exist");

            let expect_m_peers_told = vec!["self".to_string(), "0.0.0.0".to_string(), "0.0.0.1".to_string(), "0.0.0.2".to_string()];
            assert_eq!(record.m_peers_told, expect_m_peers_told);
        }
    }

    #[test]
    fn shutdown() {
        let mut flood_gate = build_flood_gate();

        flood_gate.shutdown();

        assert_eq!(flood_gate.flood_map.len(), 0);
        assert!(flood_gate.m_shutting_down);
    }
}
