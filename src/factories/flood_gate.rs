use crate::factories::internal_xdr::build_envelope;
use crate::overlay::flood_gate::*;
use crate::overlay::message_abbr;

pub fn build_flood_gate() -> FloodGate {
    let mut flood_gate = FloodGate::new();

    let message = build_envelope();

    flood_gate.flood_map.insert(
        "123".to_string(),
        FloodRecord::build(100, message.clone(), vec!["192.168.1.1".to_string()]),
    );
    flood_gate.flood_map.insert(
        "345".to_string(),
        FloodRecord::build(200, message.clone(), vec!["192.168.2.2".to_string()]),
    );
    flood_gate.flood_map.insert(
        "678".to_string(),
        FloodRecord::build(300, message.clone(), vec!["192.168.3.3".to_string()]),
    );

    flood_gate.flood_map.insert(
        message_abbr(&message),
        FloodRecord::build(400, message.clone(), vec!["192.168.4.4".to_string()]),
    );

    flood_gate
}
