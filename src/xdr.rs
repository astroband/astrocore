/*
  Package stellar is generated from:

  - Stellar-types.x
  - Stellar-transaction.x
  - Stellar-overlay.x
  - Stellar-ledger.x
  - Stellar-ledger-entries.x
  - Stellar-SCP.x

 DO NOT EDIT or your changes may be overwritten
*/
#![allow(dead_code)]

extern crate serde;
extern crate serde_bytes;
extern crate serde_xdr;

use serde_derive::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_xdr::opaque_data;
use serde_enum::{DeserializeEnum, SerializeEnum};

/* ==== Namespace: stellar ==== */
/*
   Hash is an XDR Typedef defined as:

     typedef opaque Hash[32];
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct Hash(#[serde(with = "opaque_data::fixed_length")] pub [u8; 32]);

/*
   Uint256 is an XDR Typedef defined as:

     typedef opaque uint256[32];
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct Uint256(#[serde(with = "opaque_data::fixed_length")] pub [u8; 32]);

/*
   Uint32 is an XDR Typedef defined as:

     typedef unsigned int uint32;
*/
pub type Uint32 = u32;

/*
   Int32 is an XDR Typedef defined as:

     typedef int int32;
*/
pub type Int32 = i32;

/*
   Uint64 is an XDR Typedef defined as:

     typedef unsigned hyper uint64;
*/
pub type Uint64 = u64;

/*
   Int64 is an XDR Typedef defined as:

     typedef hyper int64;
*/
pub type Int64 = i64;

/*
   CryptoKeyType is an XDR Enum defined as:

     enum CryptoKeyType
     {
         KEY_TYPE_ED25519 = 0,
         KEY_TYPE_PRE_AUTH_TX = 1,
         KEY_TYPE_HASH_X = 2
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum CryptoKeyType {
    KeyTypeEd25519 = 0,
    KeyTypePreAuthTx = 1,
    KeyTypeHashX = 2,
}

/*
   PublicKeyType is an XDR Enum defined as:

     enum PublicKeyType
     {
         PUBLIC_KEY_TYPE_ED25519 = KEY_TYPE_ED25519
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum PublicKeyType {
    PublicKeyTypeEd25519 = 0,
}

impl Default for PublicKeyType {
    fn default() -> Self {
        PublicKeyType::PublicKeyTypeEd25519
    }
}

/*
   SignerKeyType is an XDR Enum defined as:

     enum SignerKeyType
     {
         SIGNER_KEY_TYPE_ED25519 = KEY_TYPE_ED25519,
         SIGNER_KEY_TYPE_PRE_AUTH_TX = KEY_TYPE_PRE_AUTH_TX,
         SIGNER_KEY_TYPE_HASH_X = KEY_TYPE_HASH_X
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum SignerKeyType {
    SignerKeyTypeEd25519 = 0,
    SignerKeyTypePreAuthTx = 1,
    SignerKeyTypeHashX = 2,
}

/*
   PublicKey is an XDR Union defined as:

     union PublicKey switch (PublicKeyType type)
     {
     case PUBLIC_KEY_TYPE_ED25519:
         uint256 ed25519;
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum PublicKey {
    Ed25519(Uint256),
}

impl Default for PublicKey {
    fn default() -> Self {
        PublicKey::Ed25519(Uint256::default())
    }
}

/*
SignerKey is an XDR Union defined as:

  union SignerKey switch (SignerKeyType type)
  {
  case SIGNER_KEY_TYPE_ED25519:
      uint256 ed25519;
  case SIGNER_KEY_TYPE_PRE_AUTH_TX:
      /* SHA-256 Hash of TransactionSignaturePayload structure */
uint256 preAuthTx;
case SIGNER_KEY_TYPE_HASH_X:
/* Hash of random 256 bit preimage X */
uint256 hashX;
};
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum SignerKey {
    Ed25519(Uint256),
    PreAuthTx(Uint256),
    HashX(Uint256),
}

impl Default for SignerKey {
    fn default() -> Self {
        SignerKey::Ed25519(Uint256::default())
    }
}

/*
   Signature is an XDR Typedef defined as:

     typedef opaque Signature<64>;
*/
#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct Signature(#[serde(with = "serde_bytes")] pub Vec<u8>);

/*
   SignatureHint is an XDR Typedef defined as:

     typedef opaque SignatureHint[4];
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct SignatureHint(#[serde(with = "opaque_data::fixed_length")] pub [u8; 4]);

/*
   NodeId is an XDR Typedef defined as:

     typedef PublicKey NodeID;
*/
pub type NodeId = PublicKey;

/*
   Curve25519Secret is an XDR Struct defined as:

     struct Curve25519Secret
     {
             opaque key[32];
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct Curve25519Secret {
    #[serde(with = "opaque_data::fixed_length")]
    pub key: [u8; 32],
}

/*
   Curve25519Public is an XDR Struct defined as:

     struct Curve25519Public
     {
             opaque key[32];
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct Curve25519Public {
    #[serde(with = "opaque_data::fixed_length")]
    pub key: [u8; 32],
}

/*
   HmacSha256Key is an XDR Struct defined as:

     struct HmacSha256Key
     {
             opaque key[32];
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct HmacSha256Key {
    #[serde(with = "opaque_data::fixed_length")]
    pub key: [u8; 32],
}

/*
   HmacSha256Mac is an XDR Struct defined as:

     struct HmacSha256Mac
     {
             opaque mac[32];
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct HmacSha256Mac {
    #[serde(with = "opaque_data::fixed_length")]
    pub mac: [u8; 32],
}

/* ==== Namespace: stellar ==== */
/*
   DecoratedSignature is an XDR Struct defined as:

     struct DecoratedSignature
     {
         SignatureHint hint;  // last 4 bytes of the public key, used as a hint
         Signature signature; // actual signature
     };
*/
#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct DecoratedSignature {
    pub hint: SignatureHint,
    pub signature: Signature,
}

/*
   OperationType is an XDR Enum defined as:

     enum OperationType
     {
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
         BUMP_SEQUENCE = 11
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum OperationType {
    CreateAccount = 0,
    Payment = 1,
    PathPayment = 2,
    ManageOffer = 3,
    CreatePassiveOffer = 4,
    SetOptions = 5,
    ChangeTrust = 6,
    AllowTrust = 7,
    AccountMerge = 8,
    Inflation = 9,
    ManageData = 10,
    BumpSequence = 11,
}

/*
   CreateAccountOp is an XDR Struct defined as:

     struct CreateAccountOp
     {
         AccountID destination; // account to create
         int64 startingBalance; // amount they end up with
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct CreateAccountOp {
    pub destination: AccountId,
    pub starting_balance: Int64,
}

/*
   PaymentOp is an XDR Struct defined as:

     struct PaymentOp
     {
         AccountID destination; // recipient of the payment
         Asset asset;           // what they end up with
         int64 amount;          // amount they end up with
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct PaymentOp {
    pub destination: AccountId,
    pub asset: Asset,
    pub amount: Int64,
}

/*
   PathPaymentOp is an XDR Struct defined as:

     struct PathPaymentOp
     {
         Asset sendAsset; // asset we pay with
         int64 sendMax;   // the maximum amount of sendAsset to
                          // send (excluding fees).
                          // The operation will fail if can't be met

         AccountID destination; // recipient of the payment
         Asset destAsset;       // what they end up with
         int64 destAmount;      // amount they end up with

         Asset path<5>; // additional hops it must go through to get there
     };
*/
#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct PathPaymentOp {
    pub send_asset: Asset,
    pub send_max: Int64,
    pub destination: AccountId,
    pub dest_asset: Asset,
    pub dest_amount: Int64,
    pub path: Vec<Asset>,
}

/*
   ManageOfferOp is an XDR Struct defined as:

     struct ManageOfferOp
     {
         Asset selling;
         Asset buying;
         int64 amount; // amount being sold. if set to 0, delete the offer
         Price price;  // price of thing being sold in terms of what you are buying

         // 0=create a new offer, otherwise edit an existing offer
         uint64 offerID;
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct ManageOfferOp {
    pub selling: Asset,
    pub buying: Asset,
    pub amount: Int64,
    pub price: Price,
    pub offer_id: Uint64,
}

/*
   CreatePassiveOfferOp is an XDR Struct defined as:

     struct CreatePassiveOfferOp
     {
         Asset selling; // A
         Asset buying;  // B
         int64 amount;  // amount taker gets. if set to 0, delete the offer
         Price price;   // cost of A in terms of B
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct CreatePassiveOfferOp {
    pub selling: Asset,
    pub buying: Asset,
    pub amount: Int64,
    pub price: Price,
}

/*
   SetOptionsOp is an XDR Struct defined as:

     struct SetOptionsOp
     {
         AccountID* inflationDest; // sets the inflation destination

         uint32* clearFlags; // which flags to clear
         uint32* setFlags;   // which flags to set

         // account threshold manipulation
         uint32* masterWeight; // weight of the master account
         uint32* lowThreshold;
         uint32* medThreshold;
         uint32* highThreshold;

         string32* homeDomain; // sets the home domain

         // Add, update or remove a signer for the account
         // signer is deleted if the weight is 0
         Signer* signer;
     };
*/
#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct SetOptionsOp {
    pub inflation_dest: Option<AccountId>,
    pub clear_flags: Option<Uint32>,
    pub set_flags: Option<Uint32>,
    pub master_weight: Option<Uint32>,
    pub low_threshold: Option<Uint32>,
    pub med_threshold: Option<Uint32>,
    pub high_threshold: Option<Uint32>,
    pub home_domain: Option<String32>,
    pub signer: Option<Signer>,
}

/*
   ChangeTrustOp is an XDR Struct defined as:

     struct ChangeTrustOp
     {
         Asset line;

         // if limit is set to 0, deletes the trust line
         int64 limit;
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct ChangeTrustOp {
    pub line: Asset,
    pub limit: Int64,
}

/*
   AllowTrustOpAsset is an XDR NestedUnion defined as:

     union switch (AssetType type)
         {
         // ASSET_TYPE_NATIVE is not allowed
         case ASSET_TYPE_CREDIT_ALPHANUM4:
             opaque assetCode4[4];

         case ASSET_TYPE_CREDIT_ALPHANUM12:
             opaque assetCode12[12];

             // add other asset types here in the future
         }
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum AllowTrustOpAsset {
    #[serde(with = "opaque_data::fixed_length")]
    AssetCode4([u8; 4]),
    #[serde(with = "opaque_data::fixed_length")]
    AssetCode12([u8; 12]),
}

impl Default for AllowTrustOpAsset {
    fn default() -> Self {
        AllowTrustOpAsset::AssetCode4([0, 0, 0, 0])
    }
}

/*
   AllowTrustOp is an XDR Struct defined as:

     struct AllowTrustOp
     {
         AccountID trustor;
         union switch (AssetType type)
         {
         // ASSET_TYPE_NATIVE is not allowed
         case ASSET_TYPE_CREDIT_ALPHANUM4:
             opaque assetCode4[4];

         case ASSET_TYPE_CREDIT_ALPHANUM12:
             opaque assetCode12[12];

             // add other asset types here in the future
         }
         asset;

         bool authorize;
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct AllowTrustOp {
    pub trustor: AccountId,
    pub asset: AllowTrustOpAsset,
    pub authorize: bool,
}

/*
   ManageDataOp is an XDR Struct defined as:

     struct ManageDataOp
     {
         string64 dataName;
         DataValue* dataValue; // set to null to clear
     };
*/
#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct ManageDataOp {
    pub data_name: String64,
    pub data_value: Option<DataValue>,
}

/*
   BumpSequenceOp is an XDR Struct defined as:

     struct BumpSequenceOp
     {
         SequenceNumber bumpTo;
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct BumpSequenceOp {
    pub bump_to: SequenceNumber,
}

/*
   OperationBody is an XDR NestedUnion defined as:

     union switch (OperationType type)
         {
         case CREATE_ACCOUNT:
             CreateAccountOp createAccountOp;
         case PAYMENT:
             PaymentOp paymentOp;
         case PATH_PAYMENT:
             PathPaymentOp pathPaymentOp;
         case MANAGE_OFFER:
             ManageOfferOp manageOfferOp;
         case CREATE_PASSIVE_OFFER:
             CreatePassiveOfferOp createPassiveOfferOp;
         case SET_OPTIONS:
             SetOptionsOp setOptionsOp;
         case CHANGE_TRUST:
             ChangeTrustOp changeTrustOp;
         case ALLOW_TRUST:
             AllowTrustOp allowTrustOp;
         case ACCOUNT_MERGE:
             AccountID destination;
         case INFLATION:
             void;
         case MANAGE_DATA:
             ManageDataOp manageDataOp;
         case BUMP_SEQUENCE:
             BumpSequenceOp bumpSequenceOp;
         }
*/
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum OperationBody {
    CreateAccountOp(CreateAccountOp),
    PaymentOp(PaymentOp),
    PathPaymentOp(PathPaymentOp),
    ManageOfferOp(ManageOfferOp),
    CreatePassiveOfferOp(CreatePassiveOfferOp),
    SetOptionsOp(SetOptionsOp),
    ChangeTrustOp(ChangeTrustOp),
    AllowTrustOp(AllowTrustOp),
    Destination(AccountId),
    Void,
    ManageDataOp(ManageDataOp),
    BumpSequenceOp(BumpSequenceOp),
}

impl Default for OperationBody {
    fn default() -> Self {
        OperationBody::Void
    }
}

/*
   Operation is an XDR Struct defined as:

     struct Operation
     {
         // sourceAccount is the account used to run the operation
         // if not set, the runtime defaults to "sourceAccount" specified at
         // the transaction level
         AccountID* sourceAccount;

         union switch (OperationType type)
         {
         case CREATE_ACCOUNT:
             CreateAccountOp createAccountOp;
         case PAYMENT:
             PaymentOp paymentOp;
         case PATH_PAYMENT:
             PathPaymentOp pathPaymentOp;
         case MANAGE_OFFER:
             ManageOfferOp manageOfferOp;
         case CREATE_PASSIVE_OFFER:
             CreatePassiveOfferOp createPassiveOfferOp;
         case SET_OPTIONS:
             SetOptionsOp setOptionsOp;
         case CHANGE_TRUST:
             ChangeTrustOp changeTrustOp;
         case ALLOW_TRUST:
             AllowTrustOp allowTrustOp;
         case ACCOUNT_MERGE:
             AccountID destination;
         case INFLATION:
             void;
         case MANAGE_DATA:
             ManageDataOp manageDataOp;
         case BUMP_SEQUENCE:
             BumpSequenceOp bumpSequenceOp;
         }
         body;
     };
*/
#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct Operation {
    pub source_account: Option<AccountId>,
    pub body: OperationBody,
}

/*
   MemoType is an XDR Enum defined as:

     enum MemoType
     {
         MEMO_NONE = 0,
         MEMO_TEXT = 1,
         MEMO_ID = 2,
         MEMO_HASH = 3,
         MEMO_RETURN = 4
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum MemoType {
    MemoNone = 0,
    MemoText = 1,
    MemoId = 2,
    MemoHash = 3,
    MemoReturn = 4,
}

/*
   Memo is an XDR Union defined as:

     union Memo switch (MemoType type)
     {
     case MEMO_NONE:
         void;
     case MEMO_TEXT:
         string text<28>;
     case MEMO_ID:
         uint64 id;
     case MEMO_HASH:
         Hash hash; // the hash of what to pull from the content server
     case MEMO_RETURN:
         Hash retHash; // the hash of the tx you are rejecting
     };
*/
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum Memo {
    Void,
    Text(String),
    Id(Uint64),
    Hash(Hash),
    RetHash(Hash),
}

impl Default for Memo {
    fn default() -> Self {
        Memo::Void
    }
}

/*
   TimeBounds is an XDR Struct defined as:

     struct TimeBounds
     {
         TimePoint minTime;
         TimePoint maxTime; // 0 here means no maxTime
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct TimeBounds {
    pub min_time: TimePoint,
    pub max_time: TimePoint,
}

/*
   TransactionExt is an XDR NestedUnion defined as:

     union switch (int v)
         {
         case 0:
             void;
         }
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum TransactionExt {
    Void,
}

impl Default for TransactionExt {
    fn default() -> Self {
        TransactionExt::Void
    }
}

/*
   Transaction is an XDR Struct defined as:

     struct Transaction
     {
         // account used to run the transaction
         AccountID sourceAccount;

         // the fee the sourceAccount will pay
         uint32 fee;

         // sequence number to consume in the account
         SequenceNumber seqNum;

         // validity range (inclusive) for the last ledger close time
         TimeBounds* timeBounds;

         Memo memo;

         Operation operations<100>;

         // reserved for future use
         union switch (int v)
         {
         case 0:
             void;
         }
         ext;
     };
*/
#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct Transaction {
    pub source_account: AccountId,
    pub fee: Uint32,
    pub seq_num: SequenceNumber,
    pub time_bounds: Option<TimeBounds>,
    pub memo: Memo,
    pub operations: Vec<Operation>,
    pub ext: TransactionExt,
}

/*
TransactionSignaturePayloadTaggedTransaction is an XDR NestedUnion defined as:

  union switch (EnvelopeType type)
      {
      case ENVELOPE_TYPE_TX:
          Transaction tx;
          /* All other values of type are invalid */
}
*/
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum TransactionSignaturePayloadTaggedTransaction {
    Tx(Transaction),
}

impl Default for TransactionSignaturePayloadTaggedTransaction {
    fn default() -> Self {
        TransactionSignaturePayloadTaggedTransaction::Tx(Transaction::default())
    }
}

/*
TransactionSignaturePayload is an XDR Struct defined as:

  struct TransactionSignaturePayload
  {
      Hash networkId;
      union switch (EnvelopeType type)
      {
      case ENVELOPE_TYPE_TX:
          Transaction tx;
          /* All other values of type are invalid */
}
taggedTransaction;
};
*/
#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct TransactionSignaturePayload {
    pub network_id: Hash,
    pub tagged_transaction: TransactionSignaturePayloadTaggedTransaction,
}

/*
TransactionEnvelope is an XDR Struct defined as:

  struct TransactionEnvelope
  {
      Transaction tx;
      /* Each decorated signature is a signature over the SHA256 hash of
       * a TransactionSignaturePayload */
DecoratedSignature signatures<20>;
};
*/
#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct TransactionEnvelope {
    pub tx: Transaction,
    pub signatures: Vec<DecoratedSignature>,
}

/*
   ClaimOfferAtom is an XDR Struct defined as:

     struct ClaimOfferAtom
     {
         // emitted to identify the offer
         AccountID sellerID; // Account that owns the offer
         uint64 offerID;

         // amount and asset taken from the owner
         Asset assetSold;
         int64 amountSold;

         // amount and asset sent to the owner
         Asset assetBought;
         int64 amountBought;
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct ClaimOfferAtom {
    pub seller_id: AccountId,
    pub offer_id: Uint64,
    pub asset_sold: Asset,
    pub amount_sold: Int64,
    pub asset_bought: Asset,
    pub amount_bought: Int64,
}

/*
   CreateAccountResultCode is an XDR Enum defined as:

     enum CreateAccountResultCode
     {
         // codes considered as "success" for the operation
         CREATE_ACCOUNT_SUCCESS = 0, // account was created

         // codes considered as "failure" for the operation
         CREATE_ACCOUNT_MALFORMED = -1,   // invalid destination
         CREATE_ACCOUNT_UNDERFUNDED = -2, // not enough funds in source account
         CREATE_ACCOUNT_LOW_RESERVE =
             -3, // would create an account below the min reserve
         CREATE_ACCOUNT_ALREADY_EXIST = -4 // account already exists
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum CreateAccountResultCode {
    CreateAccountSuccess = 0,
    CreateAccountMalformed = -1,
    CreateAccountUnderfunded = -2,
    CreateAccountLowReserve = -3,
    CreateAccountAlreadyExist = -4,
}

/*
   CreateAccountResult is an XDR Union defined as:

     union CreateAccountResult switch (CreateAccountResultCode code)
     {
     case CREATE_ACCOUNT_SUCCESS:
         void;
     default:
         void;
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum CreateAccountResult {
    Void,
    Void1,
}

impl Default for CreateAccountResult {
    fn default() -> Self {
        CreateAccountResult::Void
    }
}

/*
   PaymentResultCode is an XDR Enum defined as:

     enum PaymentResultCode
     {
         // codes considered as "success" for the operation
         PAYMENT_SUCCESS = 0, // payment successfuly completed

         // codes considered as "failure" for the operation
         PAYMENT_MALFORMED = -1,          // bad input
         PAYMENT_UNDERFUNDED = -2,        // not enough funds in source account
         PAYMENT_SRC_NO_TRUST = -3,       // no trust line on source account
         PAYMENT_SRC_NOT_AUTHORIZED = -4, // source not authorized to transfer
         PAYMENT_NO_DESTINATION = -5,     // destination account does not exist
         PAYMENT_NO_TRUST = -6,       // destination missing a trust line for asset
         PAYMENT_NOT_AUTHORIZED = -7, // destination not authorized to hold asset
         PAYMENT_LINE_FULL = -8,      // destination would go above their limit
         PAYMENT_NO_ISSUER = -9       // missing issuer on asset
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum PaymentResultCode {
    PaymentSuccess = 0,
    PaymentMalformed = -1,
    PaymentUnderfunded = -2,
    PaymentSrcNoTrust = -3,
    PaymentSrcNotAuthorized = -4,
    PaymentNoDestination = -5,
    PaymentNoTrust = -6,
    PaymentNotAuthorized = -7,
    PaymentLineFull = -8,
    PaymentNoIssuer = -9,
}

/*
   PaymentResult is an XDR Union defined as:

     union PaymentResult switch (PaymentResultCode code)
     {
     case PAYMENT_SUCCESS:
         void;
     default:
         void;
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum PaymentResult {
    Void,
    Void1,
}

/*
   PathPaymentResultCode is an XDR Enum defined as:

     enum PathPaymentResultCode
     {
         // codes considered as "success" for the operation
         PATH_PAYMENT_SUCCESS = 0, // success

         // codes considered as "failure" for the operation
         PATH_PAYMENT_MALFORMED = -1,          // bad input
         PATH_PAYMENT_UNDERFUNDED = -2,        // not enough funds in source account
         PATH_PAYMENT_SRC_NO_TRUST = -3,       // no trust line on source account
         PATH_PAYMENT_SRC_NOT_AUTHORIZED = -4, // source not authorized to transfer
         PATH_PAYMENT_NO_DESTINATION = -5,     // destination account does not exist
         PATH_PAYMENT_NO_TRUST = -6,           // dest missing a trust line for asset
         PATH_PAYMENT_NOT_AUTHORIZED = -7,     // dest not authorized to hold asset
         PATH_PAYMENT_LINE_FULL = -8,          // dest would go above their limit
         PATH_PAYMENT_NO_ISSUER = -9,          // missing issuer on one asset
         PATH_PAYMENT_TOO_FEW_OFFERS = -10,    // not enough offers to satisfy path
         PATH_PAYMENT_OFFER_CROSS_SELF = -11,  // would cross one of its own offers
         PATH_PAYMENT_OVER_SENDMAX = -12       // could not satisfy sendmax
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum PathPaymentResultCode {
    PathPaymentSuccess = 0,
    PathPaymentMalformed = -1,
    PathPaymentUnderfunded = -2,
    PathPaymentSrcNoTrust = -3,
    PathPaymentSrcNotAuthorized = -4,
    PathPaymentNoDestination = -5,
    PathPaymentNoTrust = -6,
    PathPaymentNotAuthorized = -7,
    PathPaymentLineFull = -8,
    PathPaymentNoIssuer = -9,
    PathPaymentTooFewOffers = -10,
    PathPaymentOfferCrossSelf = -11,
    PathPaymentOverSendmax = -12,
}

/*
   SimplePaymentResult is an XDR Struct defined as:

     struct SimplePaymentResult
     {
         AccountID destination;
         Asset asset;
         int64 amount;
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct SimplePaymentResult {
    pub destination: AccountId,
    pub asset: Asset,
    pub amount: Int64,
}

/*
   PathPaymentResultSuccess is an XDR NestedStruct defined as:

     struct
         {
             ClaimOfferAtom offers<>;
             SimplePaymentResult last;
         }
*/
#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct PathPaymentResultSuccess {
    pub offers: Vec<ClaimOfferAtom>,
    pub last: SimplePaymentResult,
}

/*
   PathPaymentResult is an XDR Union defined as:

     union PathPaymentResult switch (PathPaymentResultCode code)
     {
     case PATH_PAYMENT_SUCCESS:
         struct
         {
             ClaimOfferAtom offers<>;
             SimplePaymentResult last;
         } success;
     case PATH_PAYMENT_NO_ISSUER:
         Asset noIssuer; // the asset that caused the error
     default:
         void;
     };
*/
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum PathPaymentResult {
    Success(PathPaymentResultSuccess),
    NoIssuer(Asset),
    Void,
}

/*
   ManageOfferResultCode is an XDR Enum defined as:

     enum ManageOfferResultCode
     {
         // codes considered as "success" for the operation
         MANAGE_OFFER_SUCCESS = 0,

         // codes considered as "failure" for the operation
         MANAGE_OFFER_MALFORMED = -1,     // generated offer would be invalid
         MANAGE_OFFER_SELL_NO_TRUST = -2, // no trust line for what we're selling
         MANAGE_OFFER_BUY_NO_TRUST = -3,  // no trust line for what we're buying
         MANAGE_OFFER_SELL_NOT_AUTHORIZED = -4, // not authorized to sell
         MANAGE_OFFER_BUY_NOT_AUTHORIZED = -5,  // not authorized to buy
         MANAGE_OFFER_LINE_FULL = -6,      // can't receive more of what it's buying
         MANAGE_OFFER_UNDERFUNDED = -7,    // doesn't hold what it's trying to sell
         MANAGE_OFFER_CROSS_SELF = -8,     // would cross an offer from the same user
         MANAGE_OFFER_SELL_NO_ISSUER = -9, // no issuer for what we're selling
         MANAGE_OFFER_BUY_NO_ISSUER = -10, // no issuer for what we're buying

         // update errors
         MANAGE_OFFER_NOT_FOUND = -11, // offerID does not match an existing offer

         MANAGE_OFFER_LOW_RESERVE = -12 // not enough funds to create a new Offer
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum ManageOfferResultCode {
    ManageOfferSuccess = 0,
    ManageOfferMalformed = -1,
    ManageOfferSellNoTrust = -2,
    ManageOfferBuyNoTrust = -3,
    ManageOfferSellNotAuthorized = -4,
    ManageOfferBuyNotAuthorized = -5,
    ManageOfferLineFull = -6,
    ManageOfferUnderfunded = -7,
    ManageOfferCrossSelf = -8,
    ManageOfferSellNoIssuer = -9,
    ManageOfferBuyNoIssuer = -10,
    ManageOfferNotFound = -11,
    ManageOfferLowReserve = -12,
}

/*
   ManageOfferEffect is an XDR Enum defined as:

     enum ManageOfferEffect
     {
         MANAGE_OFFER_CREATED = 0,
         MANAGE_OFFER_UPDATED = 1,
         MANAGE_OFFER_DELETED = 2
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum ManageOfferEffect {
    ManageOfferCreated = 0,
    ManageOfferUpdated = 1,
    ManageOfferDeleted = 2,
}

/*
   ManageOfferSuccessResultOffer is an XDR NestedUnion defined as:

     union switch (ManageOfferEffect effect)
         {
         case MANAGE_OFFER_CREATED:
         case MANAGE_OFFER_UPDATED:
             OfferEntry offer;
         default:
             void;
         }
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum ManageOfferSuccessResultOffer {
    Offer(OfferEntry),
    Void,
}

impl Default for ManageOfferSuccessResultOffer {
    fn default() -> Self {
        ManageOfferSuccessResultOffer::Void
    }
}

/*
   ManageOfferSuccessResult is an XDR Struct defined as:

     struct ManageOfferSuccessResult
     {
         // offers that got claimed while creating this offer
         ClaimOfferAtom offersClaimed<>;

         union switch (ManageOfferEffect effect)
         {
         case MANAGE_OFFER_CREATED:
         case MANAGE_OFFER_UPDATED:
             OfferEntry offer;
         default:
             void;
         }
         offer;
     };
*/
#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct ManageOfferSuccessResult {
    pub offers_claimed: Vec<ClaimOfferAtom>,
    pub offer: ManageOfferSuccessResultOffer,
}

/*
   ManageOfferResult is an XDR Union defined as:

     union ManageOfferResult switch (ManageOfferResultCode code)
     {
     case MANAGE_OFFER_SUCCESS:
         ManageOfferSuccessResult success;
     default:
         void;
     };
*/
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum ManageOfferResult {
    Success(ManageOfferSuccessResult),
    Void,
}

/*
   SetOptionsResultCode is an XDR Enum defined as:

     enum SetOptionsResultCode
     {
         // codes considered as "success" for the operation
         SET_OPTIONS_SUCCESS = 0,
         // codes considered as "failure" for the operation
         SET_OPTIONS_LOW_RESERVE = -1,      // not enough funds to add a signer
         SET_OPTIONS_TOO_MANY_SIGNERS = -2, // max number of signers already reached
         SET_OPTIONS_BAD_FLAGS = -3,        // invalid combination of clear/set flags
         SET_OPTIONS_INVALID_INFLATION = -4,      // inflation account does not exist
         SET_OPTIONS_CANT_CHANGE = -5,            // can no longer change this option
         SET_OPTIONS_UNKNOWN_FLAG = -6,           // can't set an unknown flag
         SET_OPTIONS_THRESHOLD_OUT_OF_RANGE = -7, // bad value for weight/threshold
         SET_OPTIONS_BAD_SIGNER = -8,             // signer cannot be masterkey
         SET_OPTIONS_INVALID_HOME_DOMAIN = -9     // malformed home domain
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum SetOptionsResultCode {
    SetOptionsSuccess = 0,
    SetOptionsLowReserve = -1,
    SetOptionsTooManySigners = -2,
    SetOptionsBadFlags = -3,
    SetOptionsInvalidInflation = -4,
    SetOptionsCantChange = -5,
    SetOptionsUnknownFlag = -6,
    SetOptionsThresholdOutOfRange = -7,
    SetOptionsBadSigner = -8,
    SetOptionsInvalidHomeDomain = -9,
}

/*
   SetOptionsResult is an XDR Union defined as:

     union SetOptionsResult switch (SetOptionsResultCode code)
     {
     case SET_OPTIONS_SUCCESS:
         void;
     default:
         void;
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum SetOptionsResult {
    Void,
    Void1,
}

/*
   ChangeTrustResultCode is an XDR Enum defined as:

     enum ChangeTrustResultCode
     {
         // codes considered as "success" for the operation
         CHANGE_TRUST_SUCCESS = 0,
         // codes considered as "failure" for the operation
         CHANGE_TRUST_MALFORMED = -1,     // bad input
         CHANGE_TRUST_NO_ISSUER = -2,     // could not find issuer
         CHANGE_TRUST_INVALID_LIMIT = -3, // cannot drop limit below balance
                                          // cannot create with a limit of 0
         CHANGE_TRUST_LOW_RESERVE =
             -4, // not enough funds to create a new trust line,
         CHANGE_TRUST_SELF_NOT_ALLOWED = -5 // trusting self is not allowed
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum ChangeTrustResultCode {
    ChangeTrustSuccess = 0,
    ChangeTrustMalformed = -1,
    ChangeTrustNoIssuer = -2,
    ChangeTrustInvalidLimit = -3,
    ChangeTrustLowReserve = -4,
    ChangeTrustSelfNotAllowed = -5,
}

/*
   ChangeTrustResult is an XDR Union defined as:

     union ChangeTrustResult switch (ChangeTrustResultCode code)
     {
     case CHANGE_TRUST_SUCCESS:
         void;
     default:
         void;
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum ChangeTrustResult {
    Void,
    Void1,
}

/*
   AllowTrustResultCode is an XDR Enum defined as:

     enum AllowTrustResultCode
     {
         // codes considered as "success" for the operation
         ALLOW_TRUST_SUCCESS = 0,
         // codes considered as "failure" for the operation
         ALLOW_TRUST_MALFORMED = -1,     // asset is not ASSET_TYPE_ALPHANUM
         ALLOW_TRUST_NO_TRUST_LINE = -2, // trustor does not have a trustline
                                         // source account does not require trust
         ALLOW_TRUST_TRUST_NOT_REQUIRED = -3,
         ALLOW_TRUST_CANT_REVOKE = -4,     // source account can't revoke trust,
         ALLOW_TRUST_SELF_NOT_ALLOWED = -5 // trusting self is not allowed
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum AllowTrustResultCode {
    AllowTrustSuccess = 0,
    AllowTrustMalformed = -1,
    AllowTrustNoTrustLine = -2,
    AllowTrustTrustNotRequired = -3,
    AllowTrustCantRevoke = -4,
    AllowTrustSelfNotAllowed = -5,
}

/*
   AllowTrustResult is an XDR Union defined as:

     union AllowTrustResult switch (AllowTrustResultCode code)
     {
     case ALLOW_TRUST_SUCCESS:
         void;
     default:
         void;
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum AllowTrustResult {
    Void,
    Void1,
}

/*
   AccountMergeResultCode is an XDR Enum defined as:

     enum AccountMergeResultCode
     {
         // codes considered as "success" for the operation
         ACCOUNT_MERGE_SUCCESS = 0,
         // codes considered as "failure" for the operation
         ACCOUNT_MERGE_MALFORMED = -1,       // can't merge onto itself
         ACCOUNT_MERGE_NO_ACCOUNT = -2,      // destination does not exist
         ACCOUNT_MERGE_IMMUTABLE_SET = -3,   // source account has AUTH_IMMUTABLE set
         ACCOUNT_MERGE_HAS_SUB_ENTRIES = -4, // account has trust lines/offers
         ACCOUNT_MERGE_SEQNUM_TOO_FAR = -5,  // sequence number is over max allowed
         ACCOUNT_MERGE_DEST_FULL = -6        // can't add source balance to
                                             // destination balance
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum AccountMergeResultCode {
    AccountMergeSuccess = 0,
    AccountMergeMalformed = -1,
    AccountMergeNoAccount = -2,
    AccountMergeImmutableSet = -3,
    AccountMergeHasSubEntries = -4,
    AccountMergeSeqnumTooFar = -5,
    AccountMergeDestFull = -6,
}

/*
   AccountMergeResult is an XDR Union defined as:

     union AccountMergeResult switch (AccountMergeResultCode code)
     {
     case ACCOUNT_MERGE_SUCCESS:
         int64 sourceAccountBalance; // how much got transfered from source account
     default:
         void;
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum AccountMergeResult {
    SourceAccountBalance(Int64),
    Void,
}

impl Default for AccountMergeResult {
    fn default() -> Self {
        AccountMergeResult::Void
    }
}

/*
   InflationResultCode is an XDR Enum defined as:

     enum InflationResultCode
     {
         // codes considered as "success" for the operation
         INFLATION_SUCCESS = 0,
         // codes considered as "failure" for the operation
         INFLATION_NOT_TIME = -1
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum InflationResultCode {
    InflationSuccess = 0,
    InflationNotTime = -1,
}

/*
   InflationPayout is an XDR Struct defined as:

     struct InflationPayout // or use PaymentResultAtom to limit types?
     {
         AccountID destination;
         int64 amount;
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct InflationPayout {
    pub destination: AccountId,
    pub amount: Int64,
}

/*
   InflationResult is an XDR Union defined as:

     union InflationResult switch (InflationResultCode code)
     {
     case INFLATION_SUCCESS:
         InflationPayout payouts<>;
     default:
         void;
     };
*/
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum InflationResult {
    Payouts(Vec<InflationPayout>),
    Void,
}

impl Default for InflationResult {
    fn default() -> Self {
        InflationResult::Void
    }
}

/*
   ManageDataResultCode is an XDR Enum defined as:

     enum ManageDataResultCode
     {
         // codes considered as "success" for the operation
         MANAGE_DATA_SUCCESS = 0,
         // codes considered as "failure" for the operation
         MANAGE_DATA_NOT_SUPPORTED_YET =
             -1, // The network hasn't moved to this protocol change yet
         MANAGE_DATA_NAME_NOT_FOUND =
             -2, // Trying to remove a Data Entry that isn't there
         MANAGE_DATA_LOW_RESERVE = -3, // not enough funds to create a new Data Entry
         MANAGE_DATA_INVALID_NAME = -4 // Name not a valid string
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum ManageDataResultCode {
    ManageDataSuccess = 0,
    ManageDataNotSupportedYet = -1,
    ManageDataNameNotFound = -2,
    ManageDataLowReserve = -3,
    ManageDataInvalidName = -4,
}

impl Default for ManageDataResultCode {
    fn default() -> Self {
        ManageDataResultCode::ManageDataSuccess
    }
}

/*
   ManageDataResult is an XDR Union defined as:

     union ManageDataResult switch (ManageDataResultCode code)
     {
     case MANAGE_DATA_SUCCESS:
         void;
     default:
         void;
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum ManageDataResult {
    Void,
    Void1,
}

impl Default for ManageDataResult {
    fn default() -> Self {
        ManageDataResult::Void
    }
}

/*
   BumpSequenceResultCode is an XDR Enum defined as:

     enum BumpSequenceResultCode
     {
         // codes considered as "success" for the operation
         BUMP_SEQUENCE_SUCCESS = 0,
         // codes considered as "failure" for the operation
         BUMP_SEQUENCE_BAD_SEQ = -1 // `bumpTo` is not within bounds
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum BumpSequenceResultCode {
    BumpSequenceSuccess = 0,
    BumpSequenceBadSeq = -1,
}

impl Default for BumpSequenceResultCode {
    fn default() -> Self {
        BumpSequenceResultCode::BumpSequenceSuccess
    }
}

/*
   BumpSequenceResult is an XDR Union defined as:

     union BumpSequenceResult switch (BumpSequenceResultCode code)
     {
     case BUMP_SEQUENCE_SUCCESS:
         void;
     default:
         void;
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum BumpSequenceResult {
    Void,
    Void1,
}

/*
   OperationResultCode is an XDR Enum defined as:

     enum OperationResultCode
     {
         opINNER = 0, // inner object result is valid

         opBAD_AUTH = -1,     // too few valid signatures / wrong network
         opNO_ACCOUNT = -2,   // source account was not found
         opNOT_SUPPORTED = -3 // operation not supported at this time
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum OperationResultCode {
    OpInner = 0,
    OpBadAuth = -1,
    OpNoAccount = -2,
    OpNotSupported = -3,
}

impl Default for OperationResultCode {
    fn default() -> Self {
        OperationResultCode::OpInner
    }
}

/*
   OperationResultTr is an XDR NestedUnion defined as:

     union switch (OperationType type)
         {
         case CREATE_ACCOUNT:
             CreateAccountResult createAccountResult;
         case PAYMENT:
             PaymentResult paymentResult;
         case PATH_PAYMENT:
             PathPaymentResult pathPaymentResult;
         case MANAGE_OFFER:
             ManageOfferResult manageOfferResult;
         case CREATE_PASSIVE_OFFER:
             ManageOfferResult createPassiveOfferResult;
         case SET_OPTIONS:
             SetOptionsResult setOptionsResult;
         case CHANGE_TRUST:
             ChangeTrustResult changeTrustResult;
         case ALLOW_TRUST:
             AllowTrustResult allowTrustResult;
         case ACCOUNT_MERGE:
             AccountMergeResult accountMergeResult;
         case INFLATION:
             InflationResult inflationResult;
         case MANAGE_DATA:
             ManageDataResult manageDataResult;
         case BUMP_SEQUENCE:
             BumpSequenceResult bumpSeqResult;
         }
*/
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum OperationResultTr {
    CreateAccountResult(CreateAccountResult),
    PaymentResult(PaymentResult),
    PathPaymentResult(PathPaymentResult),
    ManageOfferResult(ManageOfferResult),
    CreatePassiveOfferResult(ManageOfferResult),
    SetOptionsResult(SetOptionsResult),
    ChangeTrustResult(ChangeTrustResult),
    AllowTrustResult(AllowTrustResult),
    AccountMergeResult(AccountMergeResult),
    InflationResult(InflationResult),
    ManageDataResult(ManageDataResult),
    BumpSeqResult(BumpSequenceResult),
}

impl Default for OperationResultTr {
    fn default() -> Self {
        OperationResultTr::CreateAccountResult(CreateAccountResult::default())
    }
}

/*
   OperationResult is an XDR Union defined as:

     union OperationResult switch (OperationResultCode code)
     {
     case opINNER:
         union switch (OperationType type)
         {
         case CREATE_ACCOUNT:
             CreateAccountResult createAccountResult;
         case PAYMENT:
             PaymentResult paymentResult;
         case PATH_PAYMENT:
             PathPaymentResult pathPaymentResult;
         case MANAGE_OFFER:
             ManageOfferResult manageOfferResult;
         case CREATE_PASSIVE_OFFER:
             ManageOfferResult createPassiveOfferResult;
         case SET_OPTIONS:
             SetOptionsResult setOptionsResult;
         case CHANGE_TRUST:
             ChangeTrustResult changeTrustResult;
         case ALLOW_TRUST:
             AllowTrustResult allowTrustResult;
         case ACCOUNT_MERGE:
             AccountMergeResult accountMergeResult;
         case INFLATION:
             InflationResult inflationResult;
         case MANAGE_DATA:
             ManageDataResult manageDataResult;
         case BUMP_SEQUENCE:
             BumpSequenceResult bumpSeqResult;
         }
         tr;
     default:
         void;
     };
*/
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum OperationResult {
    Tr(OperationResultTr),
    Void,
}

impl Default for OperationResult {
    fn default() -> Self {
        OperationResult::Void
    }
}

/*
   TransactionResultCode is an XDR Enum defined as:

     enum TransactionResultCode
     {
         txSUCCESS = 0, // all operations succeeded

         txFAILED = -1, // one of the operations failed (none were applied)

         txTOO_EARLY = -2,         // ledger closeTime before minTime
         txTOO_LATE = -3,          // ledger closeTime after maxTime
         txMISSING_OPERATION = -4, // no operation was specified
         txBAD_SEQ = -5,           // sequence number does not match source account

         txBAD_AUTH = -6,             // too few valid signatures / wrong network
         txINSUFFICIENT_BALANCE = -7, // fee would bring account below reserve
         txNO_ACCOUNT = -8,           // source account not found
         txINSUFFICIENT_FEE = -9,     // fee is too small
         txBAD_AUTH_EXTRA = -10,      // unused signatures attached to transaction
         txINTERNAL_ERROR = -11       // an unknown error occured
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum TransactionResultCode {
    TxSuccess = 0,
    TxFailed = -1,
    TxTooEarly = -2,
    TxTooLate = -3,
    TxMissingOperation = -4,
    TxBadSeq = -5,
    TxBadAuth = -6,
    TxInsufficientBalance = -7,
    TxNoAccount = -8,
    TxInsufficientFee = -9,
    TxBadAuthExtra = -10,
    TxInternalError = -11,
}

impl Default for TransactionResultCode {
    fn default() -> Self {
        TransactionResultCode::TxSuccess
    }
}

/*
   TransactionResultResult is an XDR NestedUnion defined as:

     union switch (TransactionResultCode code)
         {
         case txSUCCESS:
         case txFAILED:
             OperationResult results<>;
         default:
             void;
         }
*/
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum TransactionResultResult {
    Results(Vec<OperationResult>),
    Void,
}

impl Default for TransactionResultResult {
    fn default() -> Self {
        TransactionResultResult::Void
    }
}

/*
   TransactionResultExt is an XDR NestedUnion defined as:

     union switch (int v)
         {
         case 0:
             void;
         }
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum TransactionResultExt {
    Void,
}

impl Default for TransactionResultExt {
    fn default() -> Self {
        TransactionResultExt::Void
    }
}

/*
   TransactionResult is an XDR Struct defined as:

     struct TransactionResult
     {
         int64 feeCharged; // actual fee charged for the transaction

         union switch (TransactionResultCode code)
         {
         case txSUCCESS:
         case txFAILED:
             OperationResult results<>;
         default:
             void;
         }
         result;

         // reserved for future use
         union switch (int v)
         {
         case 0:
             void;
         }
         ext;
     };
*/
#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct TransactionResult {
    pub fee_charged: Int64,
    pub result: TransactionResultResult,
    pub ext: TransactionResultExt,
}

/* ==== Namespace: stellar ==== */
/*
   ErrorCode is an XDR Enum defined as:

     enum ErrorCode
     {
         ERR_MISC = 0, // Unspecific error
         ERR_DATA = 1, // Malformed data
         ERR_CONF = 2, // Misconfiguration error
         ERR_AUTH = 3, // Authentication failure
         ERR_LOAD = 4  // System overloaded
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum ErrorCode {
    ErrMisc = 0,
    ErrData = 1,
    ErrConf = 2,
    ErrAuth = 3,
    ErrLoad = 4,
}

impl Default for ErrorCode {
    fn default() -> Self {
        ErrorCode::ErrMisc
    }
}

/*
   Error is an XDR Struct defined as:

     struct Error
     {
         ErrorCode code;
         string msg<100>;
     };
*/
#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct Error {
    pub code: ErrorCode,
    pub msg: String,
}

/*
   AuthCert is an XDR Struct defined as:

     struct AuthCert
     {
         Curve25519Public pubkey;
         uint64 expiration;
         Signature sig;
     };
*/
#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct AuthCert {
    pub pubkey: Curve25519Public,
    pub expiration: Uint64,
    pub sig: Signature,
}

/*
   Hello is an XDR Struct defined as:

     struct Hello
     {
         uint32 ledgerVersion;
         uint32 overlayVersion;
         uint32 overlayMinVersion;
         Hash networkID;
         string versionStr<100>;
         int listeningPort;
         NodeID peerID;
         AuthCert cert;
         uint256 nonce;
     };
*/
#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct Hello {
    pub ledger_version: Uint32,
    pub overlay_version: Uint32,
    pub overlay_min_version: Uint32,
    pub network_id: Hash,
    pub version_str: String,
    pub listening_port: i32,
    pub peer_id: NodeId,
    pub cert: AuthCert,
    pub nonce: Uint256,
}

/*
   Auth is an XDR Struct defined as:

     struct Auth
     {
         // Empty message, just to confirm
         // establishment of MAC keys.
         int unused;
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct Auth {
    pub unused: i32,
}

/*
   IpAddrType is an XDR Enum defined as:

     enum IPAddrType
     {
         IPv4 = 0,
         IPv6 = 1
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum IpAddrType {
    IPv4 = 0,
    IPv6 = 1,
}

impl Default for IpAddrType {
    fn default() -> Self {
        IpAddrType::IPv4
    }
}

/*
   PeerAddressIp is an XDR NestedUnion defined as:

     union switch (IPAddrType type)
         {
         case IPv4:
             opaque ipv4[4];
         case IPv6:
             opaque ipv6[16];
         }
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum PeerAddressIp {
    #[serde(with = "opaque_data::fixed_length")]
    Ipv4([u8; 4]),
    #[serde(with = "opaque_data::fixed_length")]
    Ipv6([u8; 16]),
}

impl Default for PeerAddressIp {
    fn default() -> Self {
        PeerAddressIp::Ipv4([0, 0, 0, 0])
    }
}

/*
   PeerAddress is an XDR Struct defined as:

     struct PeerAddress
     {
         union switch (IPAddrType type)
         {
         case IPv4:
             opaque ipv4[4];
         case IPv6:
             opaque ipv6[16];
         }
         ip;
         uint32 port;
         uint32 numFailures;
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct PeerAddress {
    pub ip: PeerAddressIp,
    pub port: Uint32,
    pub num_failures: Uint32,
}

/*
   MessageType is an XDR Enum defined as:

     enum MessageType
     {
         ERROR_MSG = 0,
         AUTH = 2,
         DONT_HAVE = 3,

         GET_PEERS = 4, // gets a list of peers this guy knows about
         PEERS = 5,

         GET_TX_SET = 6, // gets a particular txset by hash
         TX_SET = 7,

         TRANSACTION = 8, // pass on a tx you have heard about

         // SCP
         GET_SCP_QUORUMSET = 9,
         SCP_QUORUMSET = 10,
         SCP_MESSAGE = 11,
         GET_SCP_STATE = 12,

         // new messages
         HELLO = 13
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum MessageType {
    ErrorMsg = 0,
    Auth = 2,
    DontHave = 3,
    GetPeers = 4,
    Peers = 5,
    GetTxSet = 6,
    TxSet = 7,
    Transaction = 8,
    GetScpQuorumset = 9,
    ScpQuorumset = 10,
    ScpMessage = 11,
    GetScpState = 12,
    Hello = 13,
}

impl Default for MessageType {
    fn default() -> Self {
        MessageType::ErrorMsg
    }
}

/*
   DontHave is an XDR Struct defined as:

     struct DontHave
     {
         MessageType type;
         uint256 reqHash;
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct DontHave {
    pub type_: MessageType,
    pub req_hash: Uint256,
}

/*
   StellarMessage is an XDR Union defined as:

     union StellarMessage switch (MessageType type)
     {
     case ERROR_MSG:
         Error error;
     case HELLO:
         Hello hello;
     case AUTH:
         Auth auth;
     case DONT_HAVE:
         DontHave dontHave;
     case GET_PEERS:
         void;
     case PEERS:
         PeerAddress peers<100>;

     case GET_TX_SET:
         uint256 txSetHash;
     case TX_SET:
         TransactionSet txSet;

     case TRANSACTION:
         TransactionEnvelope transaction;

     // SCP
     case GET_SCP_QUORUMSET:
         uint256 qSetHash;
     case SCP_QUORUMSET:
         SCPQuorumSet qSet;
     case SCP_MESSAGE:
         SCPEnvelope envelope;
     case GET_SCP_STATE:
         uint32 getSCPLedgerSeq; // ledger seq requested ; if 0, requests the latest
     };
*/
#[derive(Clone, Debug, Eq, PartialEq, DeserializeEnum, SerializeEnum)]
pub enum StellarMessage {
    Error(Error),
    #[serde_enum(variant_id = 2)]
    Auth(Auth),
    DontHave(DontHave),
    GetPeers,
    Peers(Vec<PeerAddress>),
    TxSetHash(Uint256),
    TxSet(TransactionSet),
    Transaction(TransactionEnvelope),
    QSetHash(Uint256),
    QSet(ScpQuorumSet),
    Envelope(ScpEnvelope),
    GetScpLedgerSeq(Uint32),
    Hello(Hello),
}

impl Default for StellarMessage {
    fn default() -> Self {
        StellarMessage::GetPeers
    }
}

/*
   AuthenticatedMessageV0 is an XDR NestedStruct defined as:

     struct
     {
        uint64 sequence;
        StellarMessage message;
        HmacSha256Mac mac;
         }
*/
#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct AuthenticatedMessageV0 {
    pub sequence: Uint64,
    pub message: StellarMessage,
    pub mac: HmacSha256Mac,
}

/*
   AuthenticatedMessage is an XDR Union defined as:

     union AuthenticatedMessage switch (uint32 v)
     {
     case 0:
         struct
     {
        uint64 sequence;
        StellarMessage message;
        HmacSha256Mac mac;
         } v0;
     };
*/
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum AuthenticatedMessage {
    V0(AuthenticatedMessageV0),
}

impl Default for AuthenticatedMessage {
    fn default() -> Self {
        AuthenticatedMessage::V0(AuthenticatedMessageV0::default())
    }
}

/* ==== Namespace: stellar ==== */
/*
   UpgradeType is an XDR Typedef defined as:

     typedef opaque UpgradeType<128>;
*/
#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct UpgradeType(#[serde(with = "serde_bytes")] pub Vec<u8>);

/*
   StellarValueExt is an XDR NestedUnion defined as:

     union switch (int v)
         {
         case 0:
             void;
         }
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum StellarValueExt {
    Void,
}

impl Default for StellarValueExt {
    fn default() -> Self {
        StellarValueExt::Void
    }
}

/*
   StellarValue is an XDR Struct defined as:

     struct StellarValue
     {
         Hash txSetHash;   // transaction set to apply to previous ledger
         TimePoint closeTime; // network close time

         // upgrades to apply to the previous ledger (usually empty)
         // this is a vector of encoded 'LedgerUpgrade' so that nodes can drop
         // unknown steps during consensus if needed.
         // see notes below on 'LedgerUpgrade' for more detail
         // max size is dictated by number of upgrade types (+ room for future)
         UpgradeType upgrades<6>;

         // reserved for future use
         union switch (int v)
         {
         case 0:
             void;
         }
         ext;
     };
*/
#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct StellarValue {
    pub tx_set_hash: Hash,
    pub close_time: TimePoint,
    pub upgrades: Vec<UpgradeType>,
    pub ext: StellarValueExt,
}

/*
   LedgerHeaderExt is an XDR NestedUnion defined as:

     union switch (int v)
         {
         case 0:
             void;
         }
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum LedgerHeaderExt {
    Void,
}

impl Default for LedgerHeaderExt {
    fn default() -> Self {
        LedgerHeaderExt::Void
    }
}

/*
   LedgerHeader is an XDR Struct defined as:

     struct LedgerHeader
     {
         uint32 ledgerVersion;    // the protocol version of the ledger
         Hash previousLedgerHash; // hash of the previous ledger header
         StellarValue scpValue;   // what consensus agreed to
         Hash txSetResultHash;    // the TransactionResultSet that led to this ledger
         Hash bucketListHash;     // hash of the ledger state

         uint32 ledgerSeq; // sequence number of this ledger

         int64 totalCoins; // total number of stroops in existence.
                           // 10,000,000 stroops in 1 XLM

         int64 feePool;       // fees burned since last inflation run
         uint32 inflationSeq; // inflation sequence number

         uint64 idPool; // last used global ID, used for generating objects

         uint32 baseFee;     // base fee per operation in stroops
         uint32 baseReserve; // account base reserve in stroops

         uint32 maxTxSetSize; // maximum size a transaction set can be

         Hash skipList[4]; // hashes of ledgers in the past. allows you to jump back
                           // in time without walking the chain back ledger by ledger
                           // each slot contains the oldest ledger that is mod of
                           // either 50  5000  50000 or 500000 depending on index
                           // skipList[0] mod(50), skipList[1] mod(5000), etc

         // reserved for future use
         union switch (int v)
         {
         case 0:
             void;
         }
         ext;
     };
*/
#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct LedgerHeader {
    pub ledger_version: Uint32,
    pub previous_ledger_hash: Hash,
    pub scp_value: StellarValue,
    pub tx_set_result_hash: Hash,
    pub bucket_list_hash: Hash,
    pub ledger_seq: Uint32,
    pub total_coins: Int64,
    pub fee_pool: Int64,
    pub inflation_seq: Uint32,
    pub id_pool: Uint64,
    pub base_fee: Uint32,
    pub base_reserve: Uint32,
    pub max_tx_set_size: Uint32,
    pub skip_list: [Hash; 4],
    pub ext: LedgerHeaderExt,
}

/*
   LedgerUpgradeType is an XDR Enum defined as:

     enum LedgerUpgradeType
     {
         LEDGER_UPGRADE_VERSION = 1,
         LEDGER_UPGRADE_BASE_FEE = 2,
         LEDGER_UPGRADE_MAX_TX_SET_SIZE = 3,
         LEDGER_UPGRADE_BASE_RESERVE = 4
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum LedgerUpgradeType {
    LedgerUpgradeVersion = 1,
    LedgerUpgradeBaseFee = 2,
    LedgerUpgradeMaxTxSetSize = 3,
    LedgerUpgradeBaseReserve = 4,
}

/*
   LedgerUpgrade is an XDR Union defined as:

     union LedgerUpgrade switch (LedgerUpgradeType type)
     {
     case LEDGER_UPGRADE_VERSION:
         uint32 newLedgerVersion; // update ledgerVersion
     case LEDGER_UPGRADE_BASE_FEE:
         uint32 newBaseFee; // update baseFee
     case LEDGER_UPGRADE_MAX_TX_SET_SIZE:
         uint32 newMaxTxSetSize; // update maxTxSetSize
     case LEDGER_UPGRADE_BASE_RESERVE:
         uint32 newBaseReserve; // update baseReserve
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum LedgerUpgrade {
    NewLedgerVersion(Uint32),
    NewBaseFee(Uint32),
    NewMaxTxSetSize(Uint32),
    NewBaseReserve(Uint32),
}

/*
   LedgerKeyAccount is an XDR NestedStruct defined as:

     struct
         {
             AccountID accountID;
         }
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct LedgerKeyAccount {
    pub account_id: AccountId,
}

/*
   LedgerKeyTrustLine is an XDR NestedStruct defined as:

     struct
         {
             AccountID accountID;
             Asset asset;
         }
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct LedgerKeyTrustLine {
    pub account_id: AccountId,
    pub asset: Asset,
}

/*
   LedgerKeyOffer is an XDR NestedStruct defined as:

     struct
         {
             AccountID sellerID;
             uint64 offerID;
         }
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct LedgerKeyOffer {
    pub seller_id: AccountId,
    pub offer_id: Uint64,
}

/*
   LedgerKeyData is an XDR NestedStruct defined as:

     struct
         {
             AccountID accountID;
             string64 dataName;
         }
*/
#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct LedgerKeyData {
    pub account_id: AccountId,
    pub data_name: String64,
}

/*
   LedgerKey is an XDR Union defined as:

     union LedgerKey switch (LedgerEntryType type)
     {
     case ACCOUNT:
         struct
         {
             AccountID accountID;
         } account;

     case TRUSTLINE:
         struct
         {
             AccountID accountID;
             Asset asset;
         } trustLine;

     case OFFER:
         struct
         {
             AccountID sellerID;
             uint64 offerID;
         } offer;

     case DATA:
         struct
         {
             AccountID accountID;
             string64 dataName;
         } data;
     };
*/
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum LedgerKey {
    Account(LedgerKeyAccount),
    TrustLine(LedgerKeyTrustLine),
    Offer(LedgerKeyOffer),
    Data(LedgerKeyData),
}

impl Default for LedgerKey {
    fn default() -> Self {
        LedgerKey::Account(LedgerKeyAccount::default())
    }
}

/*
   BucketEntryType is an XDR Enum defined as:

     enum BucketEntryType
     {
         LIVEENTRY = 0,
         DEADENTRY = 1
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum BucketEntryType {
    Liveentry = 0,
    Deadentry = 1,
}

impl Default for BucketEntryType {
    fn default() -> Self {
        BucketEntryType::Liveentry
    }
}

/*
   BucketEntry is an XDR Union defined as:

     union BucketEntry switch (BucketEntryType type)
     {
     case LIVEENTRY:
         LedgerEntry liveEntry;

     case DEADENTRY:
         LedgerKey deadEntry;
     };
*/
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum BucketEntry {
    LiveEntry(LedgerEntry),
    DeadEntry(LedgerKey),
}

impl Default for BucketEntry {
    fn default() -> Self {
        BucketEntry::LiveEntry(LedgerEntry::default())
    }
}

/*
   TransactionSet is an XDR Struct defined as:

     struct TransactionSet
     {
         Hash previousLedgerHash;
         TransactionEnvelope txs<>;
     };
*/
#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct TransactionSet {
    pub previous_ledger_hash: Hash,
    pub txs: Vec<TransactionEnvelope>,
}

/*
   TransactionResultPair is an XDR Struct defined as:

     struct TransactionResultPair
     {
         Hash transactionHash;
         TransactionResult result; // result for the transaction
     };
*/
#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct TransactionResultPair {
    pub transaction_hash: Hash,
    pub result: TransactionResult,
}

/*
   TransactionResultSet is an XDR Struct defined as:

     struct TransactionResultSet
     {
         TransactionResultPair results<>;
     };
*/
#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct TransactionResultSet {
    pub results: Vec<TransactionResultPair>,
}

/*
   TransactionHistoryEntryExt is an XDR NestedUnion defined as:

     union switch (int v)
         {
         case 0:
             void;
         }
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum TransactionHistoryEntryExt {
    Void,
}

impl Default for TransactionHistoryEntryExt {
    fn default() -> Self {
        TransactionHistoryEntryExt::Void
    }
}

/*
   TransactionHistoryEntry is an XDR Struct defined as:

     struct TransactionHistoryEntry
     {
         uint32 ledgerSeq;
         TransactionSet txSet;

         // reserved for future use
         union switch (int v)
         {
         case 0:
             void;
         }
         ext;
     };
*/
#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct TransactionHistoryEntry {
    pub ledger_seq: Uint32,
    pub tx_set: TransactionSet,
    pub ext: TransactionHistoryEntryExt,
}

/*
   TransactionHistoryResultEntryExt is an XDR NestedUnion defined as:

     union switch (int v)
         {
         case 0:
             void;
         }
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum TransactionHistoryResultEntryExt {
    Void,
}

impl Default for TransactionHistoryResultEntryExt {
    fn default() -> Self {
        TransactionHistoryResultEntryExt::Void
    }
}

/*
   TransactionHistoryResultEntry is an XDR Struct defined as:

     struct TransactionHistoryResultEntry
     {
         uint32 ledgerSeq;
         TransactionResultSet txResultSet;

         // reserved for future use
         union switch (int v)
         {
         case 0:
             void;
         }
         ext;
     };
*/
#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct TransactionHistoryResultEntry {
    pub ledger_seq: Uint32,
    pub tx_result_set: TransactionResultSet,
    pub ext: TransactionHistoryResultEntryExt,
}

/*
   LedgerHeaderHistoryEntryExt is an XDR NestedUnion defined as:

     union switch (int v)
         {
         case 0:
             void;
         }
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum LedgerHeaderHistoryEntryExt {
    Void,
}

impl Default for LedgerHeaderHistoryEntryExt {
    fn default() -> Self {
        LedgerHeaderHistoryEntryExt::Void
    }
}

/*
   LedgerHeaderHistoryEntry is an XDR Struct defined as:

     struct LedgerHeaderHistoryEntry
     {
         Hash hash;
         LedgerHeader header;

         // reserved for future use
         union switch (int v)
         {
         case 0:
             void;
         }
         ext;
     };
*/
#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct LedgerHeaderHistoryEntry {
    pub hash: Hash,
    pub header: LedgerHeader,
    pub ext: LedgerHeaderHistoryEntryExt,
}

/*
   LedgerScpMessages is an XDR Struct defined as:

     struct LedgerSCPMessages
     {
         uint32 ledgerSeq;
         SCPEnvelope messages<>;
     };
*/
#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct LedgerScpMessages {
    pub ledger_seq: Uint32,
    pub messages: Vec<ScpEnvelope>,
}

/*
   ScpHistoryEntryV0 is an XDR Struct defined as:

     struct SCPHistoryEntryV0
     {
         SCPQuorumSet quorumSets<>; // additional quorum sets used by ledgerMessages
         LedgerSCPMessages ledgerMessages;
     };
*/
#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct ScpHistoryEntryV0 {
    pub quorum_sets: Vec<ScpQuorumSet>,
    pub ledger_messages: LedgerScpMessages,
}

/*
   ScpHistoryEntry is an XDR Union defined as:

     union SCPHistoryEntry switch (int v)
     {
     case 0:
         SCPHistoryEntryV0 v0;
     };
*/
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum ScpHistoryEntry {
    V0(ScpHistoryEntryV0),
}

impl Default for ScpHistoryEntry {
    fn default() -> Self {
        ScpHistoryEntry::V0(ScpHistoryEntryV0::default())
    }
}

/*
   LedgerEntryChangeType is an XDR Enum defined as:

     enum LedgerEntryChangeType
     {
         LEDGER_ENTRY_CREATED = 0, // entry was added to the ledger
         LEDGER_ENTRY_UPDATED = 1, // entry was modified in the ledger
         LEDGER_ENTRY_REMOVED = 2, // entry was removed from the ledger
         LEDGER_ENTRY_STATE = 3    // value of the entry
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum LedgerEntryChangeType {
    LedgerEntryCreated = 0,
    LedgerEntryUpdated = 1,
    LedgerEntryRemoved = 2,
    LedgerEntryState = 3,
}

/*
   LedgerEntryChange is an XDR Union defined as:

     union LedgerEntryChange switch (LedgerEntryChangeType type)
     {
     case LEDGER_ENTRY_CREATED:
         LedgerEntry created;
     case LEDGER_ENTRY_UPDATED:
         LedgerEntry updated;
     case LEDGER_ENTRY_REMOVED:
         LedgerKey removed;
     case LEDGER_ENTRY_STATE:
         LedgerEntry state;
     };
*/
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum LedgerEntryChange {
    Created(LedgerEntry),
    Updated(LedgerEntry),
    Removed(LedgerKey),
    State(LedgerEntry),
}

impl Default for LedgerEntryChange {
    fn default() -> Self {
        LedgerEntryChange::Created(LedgerEntry::default())
    }
}

/*
   LedgerEntryChanges is an XDR Typedef defined as:

     typedef LedgerEntryChange LedgerEntryChanges<>;
*/
#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct LedgerEntryChanges(pub Vec<LedgerEntryChange>);

/*
   OperationMeta is an XDR Struct defined as:

     struct OperationMeta
     {
         LedgerEntryChanges changes;
     };
*/
#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct OperationMeta {
    pub changes: LedgerEntryChanges,
}

/*
   TransactionMetaV1 is an XDR Struct defined as:

     struct TransactionMetaV1
     {
         LedgerEntryChanges txChanges; // tx level changes if any
         OperationMeta operations<>; // meta for each operation
     };
*/
#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct TransactionMetaV1 {
    pub tx_changes: LedgerEntryChanges,
    pub operations: Vec<OperationMeta>,
}

/*
   TransactionMeta is an XDR Union defined as:

     union TransactionMeta switch (int v)
     {
     case 0:
         OperationMeta operations<>;
     case 1:
         TransactionMetaV1 v1;
     };
*/
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum TransactionMeta {
    Operations(Vec<OperationMeta>),
    V1(TransactionMetaV1),
}

impl Default for TransactionMeta {
    fn default() -> Self {
        TransactionMeta::V1(TransactionMetaV1::default())
    }
}

/* ==== Namespace: stellar ==== */
/*
   AccountId is an XDR Typedef defined as:

     typedef PublicKey AccountID;
*/
pub type AccountId = PublicKey;

/*
   Thresholds is an XDR Typedef defined as:

     typedef opaque Thresholds[4];
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct Thresholds(#[serde(with = "opaque_data::fixed_length")] pub [u8; 4]);

/*
   String32 is an XDR Typedef defined as:

     typedef string string32<32>;
*/
pub type String32 = String;

/*
   String64 is an XDR Typedef defined as:

     typedef string string64<64>;
*/
pub type String64 = String;

/*
   SequenceNumber is an XDR Typedef defined as:

     typedef int64 SequenceNumber;
*/
pub type SequenceNumber = Int64;

/*
   TimePoint is an XDR Typedef defined as:

     typedef uint64 TimePoint;
*/
pub type TimePoint = Uint64;

/*
   DataValue is an XDR Typedef defined as:

     typedef opaque DataValue<64>;
*/
#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct DataValue(#[serde(with = "serde_bytes")] pub Vec<u8>);

/*
   AssetType is an XDR Enum defined as:

     enum AssetType
     {
         ASSET_TYPE_NATIVE = 0,
         ASSET_TYPE_CREDIT_ALPHANUM4 = 1,
         ASSET_TYPE_CREDIT_ALPHANUM12 = 2
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum AssetType {
    AssetTypeNative = 0,
    AssetTypeCreditAlphanum4 = 1,
    AssetTypeCreditAlphanum12 = 2,
}

/*
   AssetAlphaNum4 is an XDR NestedStruct defined as:

     struct
         {
             opaque assetCode[4]; // 1 to 4 characters
             AccountID issuer;
         }
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct AssetAlphaNum4 {
    #[serde(with = "opaque_data::fixed_length")]
    pub asset_code: [u8; 4],
    pub issuer: AccountId,
}

/*
   AssetAlphaNum12 is an XDR NestedStruct defined as:

     struct
         {
             opaque assetCode[12]; // 5 to 12 characters
             AccountID issuer;
         }
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct AssetAlphaNum12 {
    #[serde(with = "opaque_data::fixed_length")]
    pub asset_code: [u8; 12],
    pub issuer: AccountId,
}

/*
   Asset is an XDR Union defined as:

     union Asset switch (AssetType type)
     {
     case ASSET_TYPE_NATIVE: // Not credit
         void;

     case ASSET_TYPE_CREDIT_ALPHANUM4:
         struct
         {
             opaque assetCode[4]; // 1 to 4 characters
             AccountID issuer;
         } alphaNum4;

     case ASSET_TYPE_CREDIT_ALPHANUM12:
         struct
         {
             opaque assetCode[12]; // 5 to 12 characters
             AccountID issuer;
         } alphaNum12;

         // add other asset types here in the future
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum Asset {
    Void,
    AlphaNum4(AssetAlphaNum4),
    AlphaNum12(AssetAlphaNum12),
}

impl Default for Asset {
    fn default() -> Self {
        Asset::Void
    }
}

/*
   Price is an XDR Struct defined as:

     struct Price
     {
         int32 n; // numerator
         int32 d; // denominator
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct Price {
    pub n: Int32,
    pub d: Int32,
}

/*
   Liabilities is an XDR Struct defined as:

     struct Liabilities
     {
         int64 buying;
         int64 selling;
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct Liabilities {
    pub buying: Int64,
    pub selling: Int64,
}

/*
   ThresholdIndexes is an XDR Enum defined as:

     enum ThresholdIndexes
     {
         THRESHOLD_MASTER_WEIGHT = 0,
         THRESHOLD_LOW = 1,
         THRESHOLD_MED = 2,
         THRESHOLD_HIGH = 3
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum ThresholdIndexes {
    ThresholdMasterWeight = 0,
    ThresholdLow = 1,
    ThresholdMed = 2,
    ThresholdHigh = 3,
}

impl Default for ThresholdIndexes {
    fn default() -> Self {
        ThresholdIndexes::ThresholdMasterWeight
    }
}

/*
   LedgerEntryType is an XDR Enum defined as:

     enum LedgerEntryType
     {
         ACCOUNT = 0,
         TRUSTLINE = 1,
         OFFER = 2,
         DATA = 3
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum LedgerEntryType {
    Account = 0,
    Trustline = 1,
    Offer = 2,
    Data = 3,
}

impl Default for LedgerEntryType {
    fn default() -> Self {
        LedgerEntryType::Account
    }
}

/*
   Signer is an XDR Struct defined as:

     struct Signer
     {
         SignerKey key;
         uint32 weight; // really only need 1byte
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct Signer {
    pub key: SignerKey,
    pub weight: Uint32,
}

/*
   AccountFlags is an XDR Enum defined as:

     enum AccountFlags
     { // masks for each flag

         // Flags set on issuer accounts
         // TrustLines are created with authorized set to "false" requiring
         // the issuer to set it for each TrustLine
         AUTH_REQUIRED_FLAG = 0x1,
         // If set, the authorized flag in TrustLines can be cleared
         // otherwise, authorization cannot be revoked
         AUTH_REVOCABLE_FLAG = 0x2,
         // Once set, causes all AUTH_* flags to be read-only
         AUTH_IMMUTABLE_FLAG = 0x4
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum AccountFlags {
    AuthRequiredFlag = 1,
    AuthRevocableFlag = 2,
    AuthImmutableFlag = 4,
}

impl Default for AccountFlags {
    fn default() -> Self {
        AccountFlags::AuthRequiredFlag
    }
}

/*
   MaskAccountFlags is an XDR Const defined as:

     const MASK_ACCOUNT_FLAGS = 0x7;
*/
const MASK_ACCOUNT_FLAGS: u64 = 0x7;

/*
   AccountEntryV1Ext is an XDR NestedUnion defined as:

     union switch (int v)
                 {
                 case 0:
                     void;
                 }
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum AccountEntryV1Ext {
    Void,
}

impl Default for AccountEntryV1Ext {
    fn default() -> Self {
        AccountEntryV1Ext::Void
    }
}

/*
   AccountEntryV1 is an XDR NestedStruct defined as:

     struct
             {
                 Liabilities liabilities;

                 union switch (int v)
                 {
                 case 0:
                     void;
                 }
                 ext;
             }
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct AccountEntryV1 {
    pub liabilities: Liabilities,
    pub ext: AccountEntryV1Ext,
}

/*
   AccountEntryExt is an XDR NestedUnion defined as:

     union switch (int v)
         {
         case 0:
             void;
         case 1:
             struct
             {
                 Liabilities liabilities;

                 union switch (int v)
                 {
                 case 0:
                     void;
                 }
                 ext;
             } v1;
         }
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum AccountEntryExt {
    Void,
    V1(AccountEntryV1),
}

impl Default for AccountEntryExt {
    fn default() -> Self {
        AccountEntryExt::Void
    }
}

/*
   AccountEntry is an XDR Struct defined as:

     struct AccountEntry
     {
         AccountID accountID;      // master public key for this account
         int64 balance;            // in stroops
         SequenceNumber seqNum;    // last sequence number used for this account
         uint32 numSubEntries;     // number of sub-entries this account has
                                   // drives the reserve
         AccountID* inflationDest; // Account to vote for during inflation
         uint32 flags;             // see AccountFlags

         string32 homeDomain; // can be used for reverse federation and memo lookup

         // fields used for signatures
         // thresholds stores unsigned bytes: [weight of master|low|medium|high]
         Thresholds thresholds;

         Signer signers<20>; // possible signers for this account

         // reserved for future use
         union switch (int v)
         {
         case 0:
             void;
         case 1:
             struct
             {
                 Liabilities liabilities;

                 union switch (int v)
                 {
                 case 0:
                     void;
                 }
                 ext;
             } v1;
         }
         ext;
     };
*/
#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct AccountEntry {
    pub account_id: AccountId,
    pub balance: Int64,
    pub seq_num: SequenceNumber,
    pub num_sub_entries: Uint32,
    pub inflation_dest: Option<AccountId>,
    pub flags: Uint32,
    pub home_domain: String32,
    pub thresholds: Thresholds,
    pub signers: Vec<Signer>,
    pub ext: AccountEntryExt,
}

/*
   TrustLineFlags is an XDR Enum defined as:

     enum TrustLineFlags
     {
         // issuer has authorized account to perform transactions with its credit
         AUTHORIZED_FLAG = 1
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum TrustLineFlags {
    AuthorizedFlag = 1,
}

impl Default for TrustLineFlags {
    fn default() -> Self {
        TrustLineFlags::AuthorizedFlag
    }
}

/*
   MaskTrustlineFlags is an XDR Const defined as:

     const MASK_TRUSTLINE_FLAGS = 1;
*/
const MASK_TRUSTLINE_FLAGS: u64 = 1;

/*
   TrustLineEntryV1Ext is an XDR NestedUnion defined as:

     union switch (int v)
                 {
                 case 0:
                     void;
                 }
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum TrustLineEntryV1Ext {
    Void,
}

impl Default for TrustLineEntryV1Ext {
    fn default() -> Self {
        TrustLineEntryV1Ext::Void
    }
}

/*
   TrustLineEntryV1 is an XDR NestedStruct defined as:

     struct
             {
                 Liabilities liabilities;

                 union switch (int v)
                 {
                 case 0:
                     void;
                 }
                 ext;
             }
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct TrustLineEntryV1 {
    pub liabilities: Liabilities,
    pub ext: TrustLineEntryV1Ext,
}

/*
   TrustLineEntryExt is an XDR NestedUnion defined as:

     union switch (int v)
         {
         case 0:
             void;
         case 1:
             struct
             {
                 Liabilities liabilities;

                 union switch (int v)
                 {
                 case 0:
                     void;
                 }
                 ext;
             } v1;
         }
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum TrustLineEntryExt {
    Void,
    V1(TrustLineEntryV1),
}

impl Default for TrustLineEntryExt {
    fn default() -> Self {
        TrustLineEntryExt::Void
    }
}

/*
   TrustLineEntry is an XDR Struct defined as:

     struct TrustLineEntry
     {
         AccountID accountID; // account this trustline belongs to
         Asset asset;         // type of asset (with issuer)
         int64 balance;       // how much of this asset the user has.
                              // Asset defines the unit for this;

         int64 limit;  // balance cannot be above this
         uint32 flags; // see TrustLineFlags

         // reserved for future use
         union switch (int v)
         {
         case 0:
             void;
         case 1:
             struct
             {
                 Liabilities liabilities;

                 union switch (int v)
                 {
                 case 0:
                     void;
                 }
                 ext;
             } v1;
         }
         ext;
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct TrustLineEntry {
    pub account_id: AccountId,
    pub asset: Asset,
    pub balance: Int64,
    pub limit: Int64,
    pub flags: Uint32,
    pub ext: TrustLineEntryExt,
}

/*
   OfferEntryFlags is an XDR Enum defined as:

     enum OfferEntryFlags
     {
         // issuer has authorized account to perform transactions with its credit
         PASSIVE_FLAG = 1
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum OfferEntryFlags {
    PassiveFlag = 1,
}

impl Default for OfferEntryFlags {
    fn default() -> Self {
        OfferEntryFlags::PassiveFlag
    }
}

/*
   MaskOfferentryFlags is an XDR Const defined as:

     const MASK_OFFERENTRY_FLAGS = 1;
*/
const MASK_OFFERENTRY_FLAGS: u64 = 1;

/*
   OfferEntryExt is an XDR NestedUnion defined as:

     union switch (int v)
         {
         case 0:
             void;
         }
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum OfferEntryExt {
    Void,
}

impl Default for OfferEntryExt {
    fn default() -> Self {
        OfferEntryExt::Void
    }
}

/*
OfferEntry is an XDR Struct defined as:

  struct OfferEntry
  {
      AccountID sellerID;
      uint64 offerID;
      Asset selling; // A
      Asset buying;  // B
      int64 amount;  // amount of A

      /* price for this offer:
          price of A in terms of B
          price=AmountB/AmountA=priceNumerator/priceDenominator
          price is after fees
      */
Price price;
uint32 flags; // see OfferEntryFlags

// reserved for future use
union switch (int v)
{
case 0:
void;
}
ext;
};
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct OfferEntry {
    pub seller_id: AccountId,
    pub offer_id: Uint64,
    pub selling: Asset,
    pub buying: Asset,
    pub amount: Int64,
    pub price: Price,
    pub flags: Uint32,
    pub ext: OfferEntryExt,
}

/*
   DataEntryExt is an XDR NestedUnion defined as:

     union switch (int v)
         {
         case 0:
             void;
         }
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum DataEntryExt {
    Void,
}

impl Default for DataEntryExt {
    fn default() -> Self {
        DataEntryExt::Void
    }
}

/*
   DataEntry is an XDR Struct defined as:

     struct DataEntry
     {
         AccountID accountID; // account this data belongs to
         string64 dataName;
         DataValue dataValue;

         // reserved for future use
         union switch (int v)
         {
         case 0:
             void;
         }
         ext;
     };
*/
#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct DataEntry {
    pub account_id: AccountId,
    pub data_name: String64,
    pub data_value: DataValue,
    pub ext: DataEntryExt,
}

/*
   LedgerEntryData is an XDR NestedUnion defined as:

     union switch (LedgerEntryType type)
         {
         case ACCOUNT:
             AccountEntry account;
         case TRUSTLINE:
             TrustLineEntry trustLine;
         case OFFER:
             OfferEntry offer;
         case DATA:
             DataEntry data;
         }
*/
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum LedgerEntryData {
    Account(AccountEntry),
    TrustLine(TrustLineEntry),
    Offer(OfferEntry),
    Data(DataEntry),
}

impl Default for LedgerEntryData {
    fn default() -> Self {
        LedgerEntryData::Account(AccountEntry::default())
    }
}

/*
   LedgerEntryExt is an XDR NestedUnion defined as:

     union switch (int v)
         {
         case 0:
             void;
         }
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum LedgerEntryExt {
    Void,
}

impl Default for LedgerEntryExt {
    fn default() -> Self {
        LedgerEntryExt::Void
    }
}

/*
   LedgerEntry is an XDR Struct defined as:

     struct LedgerEntry
     {
         uint32 lastModifiedLedgerSeq; // ledger the LedgerEntry was last changed

         union switch (LedgerEntryType type)
         {
         case ACCOUNT:
             AccountEntry account;
         case TRUSTLINE:
             TrustLineEntry trustLine;
         case OFFER:
             OfferEntry offer;
         case DATA:
             DataEntry data;
         }
         data;

         // reserved for future use
         union switch (int v)
         {
         case 0:
             void;
         }
         ext;
     };
*/
#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct LedgerEntry {
    pub last_modified_ledger_seq: Uint32,
    pub data: LedgerEntryData,
    pub ext: LedgerEntryExt,
}

/*
   EnvelopeType is an XDR Enum defined as:

     enum EnvelopeType
     {
         ENVELOPE_TYPE_SCP = 1,
         ENVELOPE_TYPE_TX = 2,
         ENVELOPE_TYPE_AUTH = 3
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum EnvelopeType {
    EnvelopeTypeScp = 1,
    EnvelopeTypeTx = 2,
    EnvelopeTypeAuth = 3,
}

impl Default for EnvelopeType {
    fn default() -> Self {
        EnvelopeType::EnvelopeTypeScp
    }
}

/* ==== Namespace: stellar ==== */
/*
   Value is an XDR Typedef defined as:

     typedef opaque Value<>;
*/
#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct Value(#[serde(with = "serde_bytes")] pub Vec<u8>);

/*
   ScpBallot is an XDR Struct defined as:

     struct SCPBallot
     {
         uint32 counter; // n
         Value value;    // x
     };
*/
#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct ScpBallot {
    pub counter: Uint32,
    pub value: Value,
}

/*
   ScpStatementType is an XDR Enum defined as:

     enum SCPStatementType
     {
         SCP_ST_PREPARE = 0,
         SCP_ST_CONFIRM = 1,
         SCP_ST_EXTERNALIZE = 2,
         SCP_ST_NOMINATE = 3
     };
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum ScpStatementType {
    ScpStPrepare = 0,
    ScpStConfirm = 1,
    ScpStExternalize = 2,
    ScpStNominate = 3,
}

impl Default for ScpStatementType {
    fn default() -> Self {
        ScpStatementType::ScpStPrepare
    }
}

/*
   ScpNomination is an XDR Struct defined as:

     struct SCPNomination
     {
         Hash quorumSetHash; // D
         Value votes<>;      // X
         Value accepted<>;   // Y
     };
*/
#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct ScpNomination {
    pub quorum_set_hash: Hash,
    pub votes: Vec<Value>,
    pub accepted: Vec<Value>,
}

/*
   ScpStatementPrepare is an XDR NestedStruct defined as:

     struct
             {
                 Hash quorumSetHash;       // D
                 SCPBallot ballot;         // b
                 SCPBallot* prepared;      // p
                 SCPBallot* preparedPrime; // p'
                 uint32 nC;                // c.n
                 uint32 nH;                // h.n
             }
*/
#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct ScpStatementPrepare {
    pub quorum_set_hash: Hash,
    pub ballot: ScpBallot,
    pub prepared: Option<ScpBallot>,
    pub prepared_prime: Option<ScpBallot>,
    pub n_c: Uint32,
    pub n_h: Uint32,
}

/*
   ScpStatementConfirm is an XDR NestedStruct defined as:

     struct
             {
                 SCPBallot ballot;   // b
                 uint32 nPrepared;   // p.n
                 uint32 nCommit;     // c.n
                 uint32 nH;          // h.n
                 Hash quorumSetHash; // D
             }
*/
#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct ScpStatementConfirm {
    pub ballot: ScpBallot,
    pub n_prepared: Uint32,
    pub n_commit: Uint32,
    pub n_h: Uint32,
    pub quorum_set_hash: Hash,
}

/*
   ScpStatementExternalize is an XDR NestedStruct defined as:

     struct
             {
                 SCPBallot commit;         // c
                 uint32 nH;                // h.n
                 Hash commitQuorumSetHash; // D used before EXTERNALIZE
             }
*/
#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct ScpStatementExternalize {
    pub commit: ScpBallot,
    pub n_h: Uint32,
    pub commit_quorum_set_hash: Hash,
}

/*
   ScpStatementPledges is an XDR NestedUnion defined as:

     union switch (SCPStatementType type)
         {
         case SCP_ST_PREPARE:
             struct
             {
                 Hash quorumSetHash;       // D
                 SCPBallot ballot;         // b
                 SCPBallot* prepared;      // p
                 SCPBallot* preparedPrime; // p'
                 uint32 nC;                // c.n
                 uint32 nH;                // h.n
             } prepare;
         case SCP_ST_CONFIRM:
             struct
             {
                 SCPBallot ballot;   // b
                 uint32 nPrepared;   // p.n
                 uint32 nCommit;     // c.n
                 uint32 nH;          // h.n
                 Hash quorumSetHash; // D
             } confirm;
         case SCP_ST_EXTERNALIZE:
             struct
             {
                 SCPBallot commit;         // c
                 uint32 nH;                // h.n
                 Hash commitQuorumSetHash; // D used before EXTERNALIZE
             } externalize;
         case SCP_ST_NOMINATE:
             SCPNomination nominate;
         }
*/
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum ScpStatementPledges {
    Prepare(ScpStatementPrepare),
    Confirm(ScpStatementConfirm),
    Externalize(ScpStatementExternalize),
    Nominate(ScpNomination),
}

impl Default for ScpStatementPledges {
    fn default() -> Self {
        ScpStatementPledges::Prepare(ScpStatementPrepare::default())
    }
}

/*
   ScpStatement is an XDR Struct defined as:

     struct SCPStatement
     {
         NodeID nodeID;    // v
         uint64 slotIndex; // i

         union switch (SCPStatementType type)
         {
         case SCP_ST_PREPARE:
             struct
             {
                 Hash quorumSetHash;       // D
                 SCPBallot ballot;         // b
                 SCPBallot* prepared;      // p
                 SCPBallot* preparedPrime; // p'
                 uint32 nC;                // c.n
                 uint32 nH;                // h.n
             } prepare;
         case SCP_ST_CONFIRM:
             struct
             {
                 SCPBallot ballot;   // b
                 uint32 nPrepared;   // p.n
                 uint32 nCommit;     // c.n
                 uint32 nH;          // h.n
                 Hash quorumSetHash; // D
             } confirm;
         case SCP_ST_EXTERNALIZE:
             struct
             {
                 SCPBallot commit;         // c
                 uint32 nH;                // h.n
                 Hash commitQuorumSetHash; // D used before EXTERNALIZE
             } externalize;
         case SCP_ST_NOMINATE:
             SCPNomination nominate;
         }
         pledges;
     };
*/
#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct ScpStatement {
    pub node_id: NodeId,
    pub slot_index: Uint64,
    pub pledges: ScpStatementPledges,
}

/*
   ScpEnvelope is an XDR Struct defined as:

     struct SCPEnvelope
     {
         SCPStatement statement;
         Signature signature;
     };
*/
#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct ScpEnvelope {
    pub statement: ScpStatement,
    pub signature: Signature,
}

/*
   ScpQuorumSet is an XDR Struct defined as:

     struct SCPQuorumSet
     {
         uint32 threshold;
         PublicKey validators<>;
         SCPQuorumSet innerSets<>;
     };
*/
#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
pub struct ScpQuorumSet {
    pub threshold: Uint32,
    pub validators: Vec<PublicKey>,
    pub inner_sets: Vec<ScpQuorumSet>,
}

/* Bottom matter */

impl From<AuthenticatedMessage> for StellarMessage {
    fn from(item: AuthenticatedMessage) -> Self {
        let AuthenticatedMessage::V0(msg) = item;
        msg.message
    }
}
