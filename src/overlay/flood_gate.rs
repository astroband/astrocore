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

struct FloodGate {
    pub flood_map: HashMap<xdr::Uint256, FloodRecord>,
    
    // medida::Counter& mFloodMapSize;
    // medida::Meter& mSendFromBroadcast;
    pub m_shutting_down: bool;
};

impl FloodGate {
    fn new() -> Self {};

    // Floodgate will be cleared after every ledger close
    pub fn clear_below(ledger_edge: u32) -> {};

    // returns true if this is a new record
    pub fn add_record(msg: xdr::StellarMessage, from_peer_addr: String) -> bool {};

    pub fn broadcast(msg: xdr::StellarMessage, force: bool) {};

    // returns the list of peers that sent us the item with hash `h`
    // std::set<Peer::pointer> getPeersKnows(Hash const& h);

    // pub fn shutdown() -> {};
}

struct FloodRecord {
    pub m_ledger_seq: u32;
    pub m_message: xdr::StellarMessage;
    // std::set<std::string> mPeersTold;
};
