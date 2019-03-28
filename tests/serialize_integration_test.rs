mod factories;
use factories::{external_xdr, internal_xdr, prepare};

#[test]
fn hello() {
    let (subject, expect) = prepare(internal_xdr::build_hello(), external_xdr::build_hello());

    assert_eq!(subject, expect);
}

#[test]
fn auth_cert() {
    let (subject, expect) = prepare(
        internal_xdr::build_auth_cert(),
        external_xdr::build_auth_cert(),
    );

    assert_eq!(subject, expect);
}

#[test]
fn curve25519_public() {
    let (subject, expect) = prepare(
        internal_xdr::build_curve25519_public(),
        external_xdr::build_curve25519_public(),
    );

    assert_eq!(subject, expect);
}

#[test]
fn public_key() {
    let (subject, expect) = prepare(
        internal_xdr::build_public_key(),
        external_xdr::build_public_key(),
    );

    assert_eq!(subject, expect);
}

#[test]
fn operation_result_code() {
    let (subject, expect) = prepare(
        internal_xdr::build_operation_result_code(),
        external_xdr::build_operation_result_code(),
    );

    assert_eq!(subject, expect);
}

#[test]
fn stellar_message_hello() {
    let (subject, expect) = prepare(
        internal_xdr::build_stellar_message_hello(),
        external_xdr::build_stellar_message_hello(),
    );

    assert_eq!(subject, expect);
}

#[test]
fn hmac_sha_256_mac() {
    let (subject, expect) = prepare(
        internal_xdr::build_hmac_sha_256_mac(),
        external_xdr::build_hmac_sha_256_mac(),
    );

    assert_eq!(subject, expect);
}

#[test]
fn authenticated_message_v0() {
    let (subject, expect) = prepare(
        internal_xdr::build_authenticated_message_v0(),
        external_xdr::build_authenticated_message_v0(),
    );

    assert_eq!(subject, expect);
}

#[test]
fn authenticated_message() {
    let (subject, expect) = prepare(
        internal_xdr::build_authenticated_message(),
        external_xdr::build_authenticated_message(),
    );

    assert_eq!(subject, expect);
}
