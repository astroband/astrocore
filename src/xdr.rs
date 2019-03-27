// GENERATED CODE
//
// Generated from src/xdr/Stellar-types.x by xdrgen.
//
// DO NOT EDIT

use serde_derive::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_xdr::opaque_data;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum CryptoKeyType {
    KEY_TYPE_ED25519 = 0,
    KEY_TYPE_PRE_AUTH_TX = 1,
    KEY_TYPE_HASH_X = 2,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct Curve25519Public {
    #[serde(with = "opaque_data::fixed_length")]
    pub key: [u8; 32],
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct Curve25519Secret {
    #[serde(with = "opaque_data::fixed_length")]
    pub key: [u8; 32],
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct Hash(#[serde(with = "opaque_data::fixed_length")] pub [u8; 32]);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct HmacSha256Key {
    #[serde(with = "opaque_data::fixed_length")]
    pub key: [u8; 32],
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct HmacSha256Mac {
    #[serde(with = "opaque_data::fixed_length")]
    pub mac: [u8; 32],
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum PublicKey {
    PUBLIC_KEY_TYPE_ED25519(uint256),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum PublicKeyType {
    PUBLIC_KEY_TYPE_ED25519 = 0,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct Signature(#[serde(with = "serde_bytes")] pub Vec<u8>);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct SignatureHint(#[serde(with = "opaque_data::fixed_length")] pub [u8; 4]);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum SignerKey {
    SIGNER_KEY_TYPE_ED25519(uint256),
    SIGNER_KEY_TYPE_PRE_AUTH_TX(uint256),
    SIGNER_KEY_TYPE_HASH_X(uint256),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum SignerKeyType {
    SIGNER_KEY_TYPE_ED25519 = 0,
    SIGNER_KEY_TYPE_PRE_AUTH_TX = 1,
    SIGNER_KEY_TYPE_HASH_X = 2,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct uint256(#[serde(with = "opaque_data::fixed_length")] pub [u8; 32]);

pub type NodeID = PublicKey;

pub type int32 = i32;

pub type int64 = i64;

pub type uint32 = u32;

pub type uint64 = u64;

// GENERATED CODE
//
// Generated from src/xdr/Stellar-transaction.x by xdrgen.
//
// DO NOT EDIT

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum AccountMergeResult {
    ACCOUNT_MERGE_SUCCESS(int64),
    default,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum AccountMergeResultCode {
    ACCOUNT_MERGE_SUCCESS = 0,
    ACCOUNT_MERGE_MALFORMED = -1,
    ACCOUNT_MERGE_NO_ACCOUNT = -2,
    ACCOUNT_MERGE_IMMUTABLE_SET = -3,
    ACCOUNT_MERGE_HAS_SUB_ENTRIES = -4,
    ACCOUNT_MERGE_SEQNUM_TOO_FAR = -5,
    ACCOUNT_MERGE_DEST_FULL = -6,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct AllowTrustOp {
    pub trustor: AccountID,
    pub authorize: bool,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum AllowTrustResult {
    ALLOW_TRUST_SUCCESS,
    default,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum AllowTrustResultCode {
    ALLOW_TRUST_SUCCESS = 0,
    ALLOW_TRUST_MALFORMED = -1,
    ALLOW_TRUST_NO_TRUST_LINE = -2,
    ALLOW_TRUST_TRUST_NOT_REQUIRED = -3,
    ALLOW_TRUST_CANT_REVOKE = -4,
    ALLOW_TRUST_SELF_NOT_ALLOWED = -5,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct BumpSequenceOp {
    pub bumpTo: SequenceNumber,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum BumpSequenceResult {
    BUMP_SEQUENCE_SUCCESS,
    default,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum BumpSequenceResultCode {
    BUMP_SEQUENCE_SUCCESS = 0,
    BUMP_SEQUENCE_BAD_SEQ = -1,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct ChangeTrustOp {
    pub line: Asset,
    pub limit: int64,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum ChangeTrustResult {
    CHANGE_TRUST_SUCCESS,
    default,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum ChangeTrustResultCode {
    CHANGE_TRUST_SUCCESS = 0,
    CHANGE_TRUST_MALFORMED = -1,
    CHANGE_TRUST_NO_ISSUER = -2,
    CHANGE_TRUST_INVALID_LIMIT = -3,
    CHANGE_TRUST_LOW_RESERVE = -4,
    CHANGE_TRUST_SELF_NOT_ALLOWED = -5,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct ClaimOfferAtom {
    pub sellerID: AccountID,
    pub offerID: uint64,
    pub assetSold: Asset,
    pub amountSold: int64,
    pub assetBought: Asset,
    pub amountBought: int64,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct CreateAccountOp {
    pub destination: AccountID,
    pub startingBalance: int64,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum CreateAccountResult {
    CREATE_ACCOUNT_SUCCESS,
    default,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum CreateAccountResultCode {
    CREATE_ACCOUNT_SUCCESS = 0,
    CREATE_ACCOUNT_MALFORMED = -1,
    CREATE_ACCOUNT_UNDERFUNDED = -2,
    CREATE_ACCOUNT_LOW_RESERVE = -3,
    CREATE_ACCOUNT_ALREADY_EXIST = -4,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct CreatePassiveOfferOp {
    pub selling: Asset,
    pub buying: Asset,
    pub amount: int64,
    pub price: Price,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct DecoratedSignature {
    pub hint: SignatureHint,
    pub signature: Signature,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct InflationPayout {
    pub destination: AccountID,
    pub amount: int64,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum InflationResult {
    INFLATION_SUCCESS(Vec<InflationPayout>),
    default,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum InflationResultCode {
    INFLATION_SUCCESS = 0,
    INFLATION_NOT_TIME = -1,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct ManageDataOp {
    pub dataName: string64,
    pub dataValue: Option<DataValue>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum ManageDataResult {
    MANAGE_DATA_SUCCESS,
    default,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum ManageDataResultCode {
    MANAGE_DATA_SUCCESS = 0,
    MANAGE_DATA_NOT_SUPPORTED_YET = -1,
    MANAGE_DATA_NAME_NOT_FOUND = -2,
    MANAGE_DATA_LOW_RESERVE = -3,
    MANAGE_DATA_INVALID_NAME = -4,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum ManageOfferEffect {
    MANAGE_OFFER_CREATED = 0,
    MANAGE_OFFER_UPDATED = 1,
    MANAGE_OFFER_DELETED = 2,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct ManageOfferOp {
    pub selling: Asset,
    pub buying: Asset,
    pub amount: int64,
    pub price: Price,
    pub offerID: uint64,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum ManageOfferResult {
    MANAGE_OFFER_SUCCESS(ManageOfferSuccessResult),
    default,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum ManageOfferResultCode {
    MANAGE_OFFER_SUCCESS = 0,
    MANAGE_OFFER_MALFORMED = -1,
    MANAGE_OFFER_SELL_NO_TRUST = -2,
    MANAGE_OFFER_BUY_NO_TRUST = -3,
    MANAGE_OFFER_SELL_NOT_AUTHORIZED = -4,
    MANAGE_OFFER_BUY_NOT_AUTHORIZED = -5,
    MANAGE_OFFER_LINE_FULL = -6,
    MANAGE_OFFER_UNDERFUNDED = -7,
    MANAGE_OFFER_CROSS_SELF = -8,
    MANAGE_OFFER_SELL_NO_ISSUER = -9,
    MANAGE_OFFER_BUY_NO_ISSUER = -10,
    MANAGE_OFFER_NOT_FOUND = -11,
    MANAGE_OFFER_LOW_RESERVE = -12,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct ManageOfferSuccessResult {
    pub offersClaimed: Vec<ClaimOfferAtom>,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum Memo {
    MEMO_NONE,
    MEMO_TEXT(String),
    MEMO_ID(uint64),
    MEMO_HASH(Hash),
    MEMO_RETURN(Hash),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum MemoType {
    MEMO_NONE = 0,
    MEMO_TEXT = 1,
    MEMO_ID = 2,
    MEMO_HASH = 3,
    MEMO_RETURN = 4,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum OperationResult {
    opINNER,
    default,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum OperationResultCode {
    opINNER = 0,
    opBAD_AUTH = -1,
    opNO_ACCOUNT = -2,
    opNOT_SUPPORTED = -3,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum OperationType {
    CREATE_ACCOUNT = 0,
    PAYMENT = 1,
    PATH_PAYMENT = 2,
    MANAGE_OFFER = 3,
    CREATE_PASSIVE_OFFER = 4,
    SET_OPTIONS = 5,
    CHANGE_TRUST = 6,
    ALLOW_TRUST = 7,
    ACCOUNT_MERGE = 8,
    INFLATION = 9,
    MANAGE_DATA = 10,
    BUMP_SEQUENCE = 11,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct PathPaymentOp {
    pub sendAsset: Asset,
    pub sendMax: int64,
    pub destination: AccountID,
    pub destAsset: Asset,
    pub destAmount: int64,
    pub path: Vec<Asset>,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum PathPaymentResult {
    PATH_PAYMENT_NO_ISSUER(Asset),
    default,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum PathPaymentResultCode {
    PATH_PAYMENT_SUCCESS = 0,
    PATH_PAYMENT_MALFORMED = -1,
    PATH_PAYMENT_UNDERFUNDED = -2,
    PATH_PAYMENT_SRC_NO_TRUST = -3,
    PATH_PAYMENT_SRC_NOT_AUTHORIZED = -4,
    PATH_PAYMENT_NO_DESTINATION = -5,
    PATH_PAYMENT_NO_TRUST = -6,
    PATH_PAYMENT_NOT_AUTHORIZED = -7,
    PATH_PAYMENT_LINE_FULL = -8,
    PATH_PAYMENT_NO_ISSUER = -9,
    PATH_PAYMENT_TOO_FEW_OFFERS = -10,
    PATH_PAYMENT_OFFER_CROSS_SELF = -11,
    PATH_PAYMENT_OVER_SENDMAX = -12,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct PaymentOp {
    pub destination: AccountID,
    pub asset: Asset,
    pub amount: int64,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum PaymentResult {
    PAYMENT_SUCCESS,
    default,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum PaymentResultCode {
    PAYMENT_SUCCESS = 0,
    PAYMENT_MALFORMED = -1,
    PAYMENT_UNDERFUNDED = -2,
    PAYMENT_SRC_NO_TRUST = -3,
    PAYMENT_SRC_NOT_AUTHORIZED = -4,
    PAYMENT_NO_DESTINATION = -5,
    PAYMENT_NO_TRUST = -6,
    PAYMENT_NOT_AUTHORIZED = -7,
    PAYMENT_LINE_FULL = -8,
    PAYMENT_NO_ISSUER = -9,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct SetOptionsOp {
    pub inflationDest: Option<AccountID>,
    pub clearFlags: Option<uint32>,
    pub setFlags: Option<uint32>,
    pub masterWeight: Option<uint32>,
    pub lowThreshold: Option<uint32>,
    pub medThreshold: Option<uint32>,
    pub highThreshold: Option<uint32>,
    pub homeDomain: Option<string32>,
    pub signer: Option<Signer>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum SetOptionsResult {
    SET_OPTIONS_SUCCESS,
    default,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum SetOptionsResultCode {
    SET_OPTIONS_SUCCESS = 0,
    SET_OPTIONS_LOW_RESERVE = -1,
    SET_OPTIONS_TOO_MANY_SIGNERS = -2,
    SET_OPTIONS_BAD_FLAGS = -3,
    SET_OPTIONS_INVALID_INFLATION = -4,
    SET_OPTIONS_CANT_CHANGE = -5,
    SET_OPTIONS_UNKNOWN_FLAG = -6,
    SET_OPTIONS_THRESHOLD_OUT_OF_RANGE = -7,
    SET_OPTIONS_BAD_SIGNER = -8,
    SET_OPTIONS_INVALID_HOME_DOMAIN = -9,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct SimplePaymentResult {
    pub destination: AccountID,
    pub asset: Asset,
    pub amount: int64,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct TimeBounds {
    pub minTime: TimePoint,
    pub maxTime: TimePoint,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Transaction {
    pub sourceAccount: AccountID,
    pub fee: uint32,
    pub seqNum: SequenceNumber,
    pub timeBounds: Option<TimeBounds>,
    pub memo: Memo,
    pub operations: Vec<Operation>,
    pub ext: TransactionExt,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct TransactionEnvelope {
    pub tx: Transaction,
    pub signatures: Vec<DecoratedSignature>,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct TransactionResult {
    pub feeCharged: int64,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum TransactionResultCode {
    txSUCCESS = 0,
    txFAILED = -1,
    txTOO_EARLY = -2,
    txTOO_LATE = -3,
    txMISSING_OPERATION = -4,
    txBAD_SEQ = -5,
    txBAD_AUTH = -6,
    txINSUFFICIENT_BALANCE = -7,
    txNO_ACCOUNT = -8,
    txINSUFFICIENT_FEE = -9,
    txBAD_AUTH_EXTRA = -10,
    txINTERNAL_ERROR = -11,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct TransactionSignaturePayload {
    pub networkId: Hash,
}

// GENERATED CODE
//
// Generated from src/xdr/Stellar-SCP.x by xdrgen.
//
// DO NOT EDIT

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct SCPBallot {
    pub counter: uint32,
    pub value: Value,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct SCPEnvelope {
    pub statement: SCPStatement,
    pub signature: Signature,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct SCPNomination {
    pub quorumSetHash: Hash,
    pub votes: Vec<Value>,
    pub accepted: Vec<Value>,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct SCPQuorumSet {
    pub threshold: uint32,
    pub validators: Vec<PublicKey>,
    pub innerSets: Vec<SCPQuorumSet>,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct SCPStatement {
    pub nodeID: NodeID,
    pub slotIndex: uint64,
    pub pledges: ScpStatementPledges,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct SCPStatementConfirm {
    pub ballot: SCPBallot,
    pub nPrepared: uint32,
    pub nCommit: uint32,
    pub nH: uint32,
    pub quorumSetHash: Hash,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct SCPStatementExternalize {
    pub commit: SCPBallot,
    pub nH: uint32,
    pub commitQuorumSetHash: Hash,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct SCPStatementPledges {
    pub commit: SCPBallot,
    pub nH: uint32,
    pub commitQuorumSetHash: Hash,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct SCPStatementPrepare {
    pub quorumSetHash: Hash,
    pub ballot: SCPBallot,
    pub prepared: Option<SCPBallot>,
    pub preparedPrime: Option<SCPBallot>,
    pub nC: uint32,
    pub nH: uint32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum SCPStatementType {
    SCP_ST_PREPARE = 0,
    SCP_ST_CONFIRM = 1,
    SCP_ST_EXTERNALIZE = 2,
    SCP_ST_NOMINATE = 3,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum ScpStatementPledges {
    SCP_ST_PREPARE(SCPStatementPrepare),
    SCP_ST_CONFIRM(SCPStatementConfirm),
    SCP_ST_EXTERNALIZE(SCPStatementExternalize),
    SCP_ST_NOMINATE(SCPNomination),
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct Value(#[serde(with = "serde_bytes")] pub Vec<u8>);

// GENERATED CODE
//
// Generated from src/xdr/Stellar-overlay.x by xdrgen.
//
// DO NOT EDIT

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Auth {
    pub unused: i32,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct AuthCert {
    pub pubkey: Curve25519Public,
    pub expiration: uint64,
    pub sig: Signature,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct AuthenticatedMessageV0 {
    pub sequence: uint64,
    pub message: StellarMessage,
    pub mac: HmacSha256Mac,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct DontHave {
    pub type_: MessageType,
    pub reqHash: uint256,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Error {
    pub code: ErrorCode,
    pub msg: String,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum ErrorCode {
    ERR_MISC = 0,
    ERR_DATA = 1,
    ERR_CONF = 2,
    ERR_AUTH = 3,
    ERR_LOAD = 4,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Hello {
    pub ledgerVersion: uint32,
    pub overlayVersion: uint32,
    pub overlayMinVersion: uint32,
    pub networkID: Hash,
    pub versionStr: String,
    pub listeningPort: i32,
    pub peerID: NodeID,
    pub cert: AuthCert,
    pub nonce: uint256,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum IPAddrType {
    IPv4 = 0,
    IPv6 = 1,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum MessageType {
    ERROR_MSG = 0,
    None,
    AUTH = 2,
    DONT_HAVE = 3,
    GET_PEERS = 4,
    PEERS = 5,
    GET_TX_SET = 6,
    TX_SET = 7,
    TRANSACTION = 8,
    GET_SCP_QUORUMSET = 9,
    SCP_QUORUMSET = 10,
    SCP_MESSAGE = 11,
    GET_SCP_STATE = 12,
    HELLO = 13,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct PeerAddress {
    pub ip: PeerAddressIP,
    pub port: uint32,
    pub numFailures: uint32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum PeerAddressIP {
    #[serde(with = "opaque_data::fixed_length")]
    IPv4([u8; 4]),
    #[serde(with = "opaque_data::fixed_length")]
    IPv6([u8; 16]),
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum StellarMessage {
    ERROR_MSG(Error),
    None,
    AUTH(Auth),
    DONT_HAVE(DontHave),
    GET_PEERS,
    PEERS(Vec<PeerAddress>),
    GET_TX_SET(uint256),
    TX_SET(TransactionSet),
    TRANSACTION(TransactionEnvelope),
    GET_SCP_QUORUMSET(uint256),
    SCP_QUORUMSET(SCPQuorumSet),
    SCP_MESSAGE(SCPEnvelope),
    GET_SCP_STATE(uint32),
    HELLO(Hello),
}

// GENERATED CODE
//
// Generated from src/xdr/Stellar-ledger.x by xdrgen.
//
// DO NOT EDIT

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum BucketEntry {
    LIVEENTRY(LedgerEntry),
    DEADENTRY(LedgerKey),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum BucketEntryType {
    LIVEENTRY = 0,
    DEADENTRY = 1,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum LedgerEntryChange {
    LEDGER_ENTRY_CREATED(LedgerEntry),
    LEDGER_ENTRY_UPDATED(LedgerEntry),
    LEDGER_ENTRY_REMOVED(LedgerKey),
    LEDGER_ENTRY_STATE(LedgerEntry),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum LedgerEntryChangeType {
    LEDGER_ENTRY_CREATED = 0,
    LEDGER_ENTRY_UPDATED = 1,
    LEDGER_ENTRY_REMOVED = 2,
    LEDGER_ENTRY_STATE = 3,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct LedgerEntryChanges(pub Vec<LedgerEntryChange>);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum LedgerEntryKeyType {
    LEDGER_ENTRY_KEY_TYPE_ACCOUNT = 0,
    LEDGER_ENTRY_KEY_TYPE_TRUSTLINE = 1,
    LEDGER_ENTRY_KEY_TYPE_OFFER = 2,
    LEDGER_ENTRY_KEY_TYPE_DATA = 3,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct LedgerHeader {
    pub ledgerVersion: uint32,
    pub previousLedgerHash: Hash,
    pub scpValue: StellarValue,
    pub txSetResultHash: Hash,
    pub bucketListHash: Hash,
    pub ledgerSeq: uint32,
    pub totalCoins: int64,
    pub feePool: int64,
    pub inflationSeq: uint32,
    pub idPool: uint64,
    pub baseFee: uint32,
    pub baseReserve: uint32,
    pub maxTxSetSize: uint32,
    pub skipList: [Hash; 4],
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct LedgerHeaderHistoryEntry {
    pub hash: Hash,
    pub header: LedgerHeader,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum LedgerKey {
    LEDGER_ENTRY_KEY_TYPE_ACCOUNT(LedgerKeyAccount),
    LEDGER_ENTRY_KEY_TYPE_TRUSTLINE(LedgerKeyTrustLine),
    LEDGER_ENTRY_KEY_TYPE_OFFER(LedgerKeyTrustOffer),
    LEDGER_ENTRY_KEY_TYPE_DATA(LedgerKeyTrustData),
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct LedgerKeyAccount {
    pub accountID: AccountID,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct LedgerKeyTrustData {
    pub accountID: AccountID,
    pub dataName: string64,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct LedgerKeyTrustLine {
    pub accountID: AccountID,
    pub asset: Asset,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct LedgerKeyTrustOffer {
    pub sellerID: AccountID,
    pub offerID: uint64,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct LedgerSCPMessages {
    pub ledgerSeq: uint32,
    pub messages: Vec<SCPEnvelope>,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum LedgerUpgrade {
    LEDGER_UPGRADE_VERSION(uint32),
    LEDGER_UPGRADE_BASE_FEE(uint32),
    LEDGER_UPGRADE_MAX_TX_SET_SIZE(uint32),
    LEDGER_UPGRADE_BASE_RESERVE(uint32),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum LedgerUpgradeType {
    LEDGER_UPGRADE_VERSION = 1,
    LEDGER_UPGRADE_BASE_FEE = 2,
    LEDGER_UPGRADE_MAX_TX_SET_SIZE = 3,
    LEDGER_UPGRADE_BASE_RESERVE = 4,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct OperationMeta {
    pub changes: LedgerEntryChanges,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct SCPHistoryEntryV0 {
    pub quorumSets: Vec<SCPQuorumSet>,
    pub ledgerMessages: LedgerSCPMessages,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct StellarValue {
    pub txSetHash: Hash,
    pub closeTime: TimePoint,
    pub upgrades: Vec<UpgradeType>,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct TransactionHistoryEntry {
    pub ledgerSeq: uint32,
    pub txSet: TransactionSet,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct TransactionHistoryResultEntry {
    pub ledgerSeq: uint32,
    pub txResultSet: TransactionResultSet,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct TransactionMetaV1 {
    pub txChanges: LedgerEntryChanges,
    pub operations: Vec<OperationMeta>,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct TransactionResultPair {
    pub transactionHash: Hash,
    pub result: TransactionResult,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct TransactionResultSet {
    pub results: Vec<TransactionResultPair>,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct TransactionSet {
    pub previousLedgerHash: Hash,
    pub txs: Vec<TransactionEnvelope>,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct UpgradeType(#[serde(with = "serde_bytes")] pub Vec<u8>);

// GENERATED CODE
//
// Generated from src/xdr/Stellar-ledger-entries.x by xdrgen.
//
// DO NOT EDIT

pub const MASK_ACCOUNT_FLAGS: i64 = 7i64;

pub const MASK_OFFERENTRY_FLAGS: i64 = 1i64;

pub const MASK_TRUSTLINE_FLAGS: i64 = 1i64;

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct AccountEntry {
    pub accountID: AccountID,
    pub balance: int64,
    pub seqNum: SequenceNumber,
    pub numSubEntries: uint32,
    pub inflationDest: Option<AccountID>,
    pub flags: uint32,
    pub homeDomain: string32,
    pub thresholds: Thresholds,
    pub signers: Vec<Signer>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum AccountFlags {
    AUTH_REQUIRED_FLAG = 1,
    AUTH_REVOCABLE_FLAG = 2,
    AUTH_IMMUTABLE_FLAG = 4,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum Asset {
    ASSET_TYPE_NATIVE,
    ASSET_TYPE_CREDIT_ALPHANUM4(AssetAlphaNum4),
    ASSET_TYPE_CREDIT_ALPHANUM12(AssetAlphaNum12),
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct AssetAlphaNum12 {
    #[serde(with = "opaque_data::fixed_length")]
    pub assetCode: [u8; 12],
    pub issuer: AccountID,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct AssetAlphaNum4 {
    #[serde(with = "opaque_data::fixed_length")]
    pub assetCode: [u8; 4],
    pub issuer: AccountID,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize_repr, Serialize_repr)]
#[repr(i32)]
pub enum AssetType {
    ASSET_TYPE_NATIVE = 0,
    ASSET_TYPE_CREDIT_ALPHANUM4 = 1,
    ASSET_TYPE_CREDIT_ALPHANUM12 = 2,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct DataEntry {
    pub accountID: AccountID,
    pub dataName: string64,
    pub dataValue: DataValue,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct DataValue(#[serde(with = "serde_bytes")] pub Vec<u8>);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize_repr, Serialize_repr)]
#[repr(i32)]
pub enum EnvelopeType {
    ENVELOPE_TYPE_SCP = 1,
    ENVELOPE_TYPE_TX = 2,
    ENVELOPE_TYPE_AUTH = 3,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct LedgerEntry {
    pub lastModifiedLedgerSeq: uint32,
    pub data: LedgerEntryData,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum LedgerEntryData {
    ACCOUNT(AccountEntry),
    TRUSTLINE(TrustLineEntry),
    OFFER(OfferEntry),
    DATA(DataEntry),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize_repr, Serialize_repr)]
#[repr(i32)]
pub enum LedgerEntryType {
    ACCOUNT = 0,
    TRUSTLINE = 1,
    OFFER = 2,
    DATA = 3,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct Liabilities {
    pub buying: int64,
    pub selling: int64,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct OfferEntry {
    pub sellerID: AccountID,
    pub offerID: uint64,
    pub selling: Asset,
    pub buying: Asset,
    pub amount: int64,
    pub price: Price,
    pub flags: uint32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum OfferEntryFlags {
    PASSIVE_FLAG = 1,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct Price {
    pub n: int32,
    pub d: int32,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Signer {
    pub key: SignerKey,
    pub weight: uint32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum ThresholdIndexes {
    THRESHOLD_MASTER_WEIGHT = 0,
    THRESHOLD_LOW = 1,
    THRESHOLD_MED = 2,
    THRESHOLD_HIGH = 3,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Thresholds(#[serde(with = "opaque_data::fixed_length")] pub [u8; 4]);

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct TrustLineEntry {
    pub accountID: AccountID,
    pub asset: Asset,
    pub balance: int64,
    pub limit: int64,
    pub flags: uint32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum TrustLineFlags {
    AUTHORIZED_FLAG = 1,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct string32(pub String);

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct string64(pub String);

pub type AccountID = PublicKey;

pub type SequenceNumber = int64;

pub type TimePoint = uint64;

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum AuthenticatedMessage {
    V0(AuthenticatedMessageV0),
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum TransactionExt {
    V0,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Operation {
    pub source: Option<PublicKey>,
    pub inner: OperationInner,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum OperationInner {
    CreateAccount(CreateAccountOperation),
    Payment(PaymentOperation),
    PathPayment(PathPaymentOperation),
    ManageOffer(ManageOfferOperation),
    CreatePassiveOffer(CreatePassiveOfferOperation),
    SetOptions,
    ChangeTrust,
    AllowTrust,
    AccountMerge,
    Inflation,
    ManageData(ManageDataOperation),
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct CreateAccountOperation {
    destination: PublicKey,
    balance: Stroops,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct PaymentOperation {
    destination: PublicKey,
    asset: Asset,
    amount: Stroops,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct PathPaymentOperation {
    send_asset: Asset,
    send_max: Stroops,
    destination: PublicKey,
    dest_asset: Asset,
    dest_amount: Stroops,
    path: Vec<Asset>,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct ManageOfferOperation {
    selling: Asset,
    buying: Asset,
    amount: Stroops,
    price: Price,
    offer_id: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct CreatePassiveOfferOperation {
    selling: Asset,
    buying: Asset,
    amount: Stroops,
    price: Price,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct ManageDataOperation {
    name: String,
    value: Option<Vec<u8>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Stroops(pub i64);
