extern crate astrocore;
use astrocore::xdr;

extern crate serde_xdr;
use serde::ser::Serialize;
use serde_xdr::to_bytes;

pub fn serialize<T: Serialize>(xdr_struct: T) -> Vec<u8> {
    to_bytes(&xdr_struct).unwrap()
}

pub fn build_hello() -> xdr::Hello {
    let nonce: [u8; 32] = [
        18, 17, 21, 19, 25, 24, 26, 9, 2, 28, 7, 22, 11, 30, 23, 3, 14, 4, 0, 6, 16, 8, 29, 15, 20,
        5, 10, 31, 27, 1, 13, 12,
    ];
    let network_id = xdr::Hash([
        106, 109, 106, 120, 97, 107, 113, 118, 110, 121, 103, 107, 116, 102, 119, 99, 111, 112,
        103, 102, 98, 100, 122, 115, 115, 100, 114, 116, 113, 114, 104, 118,
    ]);

    xdr::Hello {
        ledgerVersion: 9000 as xdr::uint32,
        overlayVersion: 9000 as xdr::uint32,
        overlayMinVersion: 0 as xdr::uint32,
        networkID: network_id,
        versionStr: "stellar-core-rust[alpha-0.0]".to_string(),
        listeningPort: 11625,
        peerID: build_public_key(),
        cert: build_auth_cert(),
        nonce: xdr::uint256 { 0: nonce },
    }
}

pub fn build_auth_cert() -> xdr::AuthCert {
    xdr::AuthCert {
        pubkey: build_curve25519_public(),
        expiration: 123,
        sig: xdr::Signature {
            0: [1, 2, 3, 4, 5].to_vec(),
        },
    }
}

pub fn build_curve25519_public() -> xdr::Curve25519Public {
    xdr::Curve25519Public {
        key: [
            115, 100, 105, 102, 100, 111, 99, 110, 109, 113, 98, 114, 118, 116, 102, 99, 104, 120,
            101, 118, 98, 103, 99, 112, 105, 115, 110, 118, 97, 116, 100, 120,
        ],
    }
}

pub fn build_public_key() -> xdr::PublicKey {
    xdr::PublicKey::PUBLIC_KEY_TYPE_ED25519(xdr::uint256 {
        0: [
            114, 102, 121, 110, 104, 105, 115, 100, 99, 122, 112, 112, 119, 105, 108, 121, 122,
            108, 102, 101, 107, 111, 102, 103, 109, 103, 106, 105, 98, 118, 110, 113,
        ],
    })
}

pub fn build_operation_result_code() -> xdr::OperationResultCode {
    xdr::OperationResultCode::opNO_ACCOUNT
}

pub fn build_stellar_message_hello() -> xdr::StellarMessage {
    xdr::StellarMessage::HELLO(build_hello())
}

pub fn build_hmac_sha_256_mac() -> xdr::HmacSha256Mac {
    xdr::HmacSha256Mac {
        mac: [
            57, 57, 98, 57, 102, 51, 101, 50, 55, 49, 100, 49, 102, 51, 56, 49, 56, 54, 54, 101,
            97, 49, 100, 97, 56, 101, 55, 54, 100, 52, 51, 48,
        ],
    }
}

pub fn build_authenticated_message_v0() -> xdr::AuthenticatedMessageV0 {
    xdr::AuthenticatedMessageV0 {
        sequence: 4321 as xdr::uint64,
        message: build_stellar_message_hello(),
        mac: build_hmac_sha_256_mac(),
    }
}

pub fn build_authenticated_message() -> xdr::AuthenticatedMessage {
    xdr::AuthenticatedMessage::V0(build_authenticated_message_v0())
}
