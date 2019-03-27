use std::env;
use std::process::Command;

pub fn serialize(cmd_string: String) -> Vec<u8> {
    let ruby_xdr_path = env::current_dir().unwrap().join("ruby_xdr");

    Command::new("bundle")
        .current_dir(ruby_xdr_path)
        .arg("exec")
        .arg("bin/serialize")
        .arg(cmd_string)
        .output()
        .expect("failed to execute process")
        .stdout
}

pub fn build_hello() -> String {
    let cmd = r#"
    Stellar::Hello.new.tap do |h|
        h.ledger_version = 9000
        h.overlay_version = 9000
        h.overlay_min_version = 0
        h.network_id = pack [106, 109, 106, 120, 97, 107, 113, 118, 110, 121, 103, 107, 116, 102, 119, 99, 111, 112, 103, 102, 98, 100, 122, 115, 115, 100, 114, 116, 113, 114, 104, 118]
        h.version_str = "stellar-core-rust[alpha-0.0]"
        h.listening_port = 11625
        h.peer_id = {public_key}
        h.cert = {auth_cert}
        h.nonce = pack [18, 17, 21, 19, 25, 24, 26, 9, 2, 28, 7, 22, 11, 30, 23, 3, 14, 4, 0, 6, 16, 8, 29, 15, 20, 5, 10, 31, 27, 1, 13, 12]
    end
    "#;
    cmd.replace("{public_key}", &build_public_key())
        .replace("{auth_cert}", &build_auth_cert())
}

pub fn build_auth_cert() -> String {
    let cmd = r#"
    Stellar::AuthCert.new.
        tap do |c|
            c.pubkey = {pubkey}
        end.
        tap{ |c| c.expiration = 123 }.
        tap{ |c| c.sig = pack [1, 2, 3, 4, 5] }
    "#;
    cmd.replace("{pubkey}", &build_curve25519_public())
}

pub fn build_curve25519_public() -> String {
    let cmd = r#"
    Stellar::Curve25519Public.new.tap do |obj|
        obj.key = pack [115, 100, 105, 102, 100, 111, 99, 110, 109, 113, 98, 114, 118, 116, 102, 99, 104, 120, 101, 118, 98, 103, 99, 112, 105, 115, 110, 118, 97, 116, 100, 120]
    end
    "#;
    cmd.to_string()
}

pub fn build_public_key() -> String {
    let cmd = r#"
    Stellar::PublicKey.new(:public_key_type_ed25519, pack([114, 102, 121, 110, 104, 105, 115, 100, 99, 122, 112, 112, 119, 105, 108, 121, 122, 108, 102, 101, 107, 111, 102, 103, 109, 103, 106, 105, 98, 118, 110, 113]))
    "#;
    cmd.to_string()
}

pub fn build_operation_result_code() -> String {
    let cmd = r#"
    print Stellar::OperationResultCode.to_xdr(Stellar::OperationResultCode.op_no_account)
    return
    "#;
    cmd.to_string()
}

pub fn build_stellar_message_hello() -> String {
    let cmd = r#"
    Stellar::StellarMessage.new(:hello, {hello})
    "#;
    cmd.replace("{hello}", &build_hello())
}

pub fn build_authenticated_message_v0() -> String {
    let cmd = r#"
    Stellar::AuthenticatedMessage::V0.new.tap do |obj|
        obj.sequence = 4321
        obj.message = {stellar_message_hello}
        obj.mac = {hmac_sha_256_mac}
    end
    "#;
    cmd.replace("{stellar_message_hello}", &build_stellar_message_hello())
        .replace("{hmac_sha_256_mac}", &build_hmac_sha_256_mac())
}

pub fn build_hmac_sha_256_mac() -> String {
    let cmd = r#"
    Stellar::HmacSha256Mac.new.tap do |obj|
        obj.mac = pack [57, 57, 98, 57, 102, 51, 101, 50, 55, 49, 100, 49, 102, 51, 56, 49, 56, 54, 54, 101, 97, 49, 100, 97, 56, 101, 55, 54, 100, 52, 51, 48]
    end
    "#;
    cmd.to_string()
}

pub fn build_authenticated_message() -> String {
    let cmd = r#"
    Stellar::AuthenticatedMessage.new(0, {authenticated_message_v0})
    "#;
    cmd.replace(
        "{authenticated_message_v0}",
        &build_authenticated_message_v0(),
    )
}
