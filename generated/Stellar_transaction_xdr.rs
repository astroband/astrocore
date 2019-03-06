// GENERATED CODE
//
// Generated from src/xdr/Stellar-transaction.x by xdrgen.
//
// DO NOT EDIT

pub enum AccountMergeResult {
    ACCOUNT_MERGE_SUCCESS(int64),
    default,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AccountMergeResultCode {
    ACCOUNT_MERGE_SUCCESS = 0isize,
    ACCOUNT_MERGE_MALFORMED = -1isize,
    ACCOUNT_MERGE_NO_ACCOUNT = -2isize,
    ACCOUNT_MERGE_IMMUTABLE_SET = -3isize,
    ACCOUNT_MERGE_HAS_SUB_ENTRIES = -4isize,
    ACCOUNT_MERGE_SEQNUM_TOO_FAR = -5isize,
    ACCOUNT_MERGE_DEST_FULL = -6isize,
}

pub struct AllowTrustOp {
    pub trustor: AccountID,
    pub authorize: bool,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AllowTrustResult {
    ALLOW_TRUST_SUCCESS,
    default,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AllowTrustResultCode {
    ALLOW_TRUST_SUCCESS = 0isize,
    ALLOW_TRUST_MALFORMED = -1isize,
    ALLOW_TRUST_NO_TRUST_LINE = -2isize,
    ALLOW_TRUST_TRUST_NOT_REQUIRED = -3isize,
    ALLOW_TRUST_CANT_REVOKE = -4isize,
    ALLOW_TRUST_SELF_NOT_ALLOWED = -5isize,
}

pub struct BumpSequenceOp {
    pub bumpTo: SequenceNumber,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BumpSequenceResult {
    BUMP_SEQUENCE_SUCCESS,
    default,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BumpSequenceResultCode {
    BUMP_SEQUENCE_SUCCESS = 0isize,
    BUMP_SEQUENCE_BAD_SEQ = -1isize,
}

pub struct ChangeTrustOp {
    pub line: Asset,
    pub limit: int64,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ChangeTrustResult {
    CHANGE_TRUST_SUCCESS,
    default,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ChangeTrustResultCode {
    CHANGE_TRUST_SUCCESS = 0isize,
    CHANGE_TRUST_MALFORMED = -1isize,
    CHANGE_TRUST_NO_ISSUER = -2isize,
    CHANGE_TRUST_INVALID_LIMIT = -3isize,
    CHANGE_TRUST_LOW_RESERVE = -4isize,
    CHANGE_TRUST_SELF_NOT_ALLOWED = -5isize,
}

pub struct ClaimOfferAtom {
    pub sellerID: AccountID,
    pub offerID: uint64,
    pub assetSold: Asset,
    pub amountSold: int64,
    pub assetBought: Asset,
    pub amountBought: int64,
}

pub struct CreateAccountOp {
    pub destination: AccountID,
    pub startingBalance: int64,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateAccountResult {
    CREATE_ACCOUNT_SUCCESS,
    default,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateAccountResultCode {
    CREATE_ACCOUNT_SUCCESS = 0isize,
    CREATE_ACCOUNT_MALFORMED = -1isize,
    CREATE_ACCOUNT_UNDERFUNDED = -2isize,
    CREATE_ACCOUNT_LOW_RESERVE = -3isize,
    CREATE_ACCOUNT_ALREADY_EXIST = -4isize,
}

pub struct CreatePassiveOfferOp {
    pub selling: Asset,
    pub buying: Asset,
    pub amount: int64,
    pub price: Price,
}

pub struct DecoratedSignature {
    pub hint: SignatureHint,
    pub signature: Signature,
}

pub struct InflationPayout {
    pub destination: AccountID,
    pub amount: int64,
}

pub enum InflationResult {
    INFLATION_SUCCESS(Vec<InflationPayout>),
    default,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum InflationResultCode {
    INFLATION_SUCCESS = 0isize,
    INFLATION_NOT_TIME = -1isize,
}

pub struct ManageDataOp {
    pub dataName: string64,
    pub dataValue: Option<Box<DataValue>>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ManageDataResult {
    MANAGE_DATA_SUCCESS,
    default,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ManageDataResultCode {
    MANAGE_DATA_SUCCESS = 0isize,
    MANAGE_DATA_NOT_SUPPORTED_YET = -1isize,
    MANAGE_DATA_NAME_NOT_FOUND = -2isize,
    MANAGE_DATA_LOW_RESERVE = -3isize,
    MANAGE_DATA_INVALID_NAME = -4isize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ManageOfferEffect {
    MANAGE_OFFER_CREATED = 0isize,
    MANAGE_OFFER_UPDATED = 1isize,
    MANAGE_OFFER_DELETED = 2isize,
}

pub struct ManageOfferOp {
    pub selling: Asset,
    pub buying: Asset,
    pub amount: int64,
    pub price: Price,
    pub offerID: uint64,
}

pub enum ManageOfferResult {
    MANAGE_OFFER_SUCCESS(ManageOfferSuccessResult),
    default,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ManageOfferResultCode {
    MANAGE_OFFER_SUCCESS = 0isize,
    MANAGE_OFFER_MALFORMED = -1isize,
    MANAGE_OFFER_SELL_NO_TRUST = -2isize,
    MANAGE_OFFER_BUY_NO_TRUST = -3isize,
    MANAGE_OFFER_SELL_NOT_AUTHORIZED = -4isize,
    MANAGE_OFFER_BUY_NOT_AUTHORIZED = -5isize,
    MANAGE_OFFER_LINE_FULL = -6isize,
    MANAGE_OFFER_UNDERFUNDED = -7isize,
    MANAGE_OFFER_CROSS_SELF = -8isize,
    MANAGE_OFFER_SELL_NO_ISSUER = -9isize,
    MANAGE_OFFER_BUY_NO_ISSUER = -10isize,
    MANAGE_OFFER_NOT_FOUND = -11isize,
    MANAGE_OFFER_LOW_RESERVE = -12isize,
}

pub struct ManageOfferSuccessResult {
    pub offersClaimed: Vec<ClaimOfferAtom>,
}

pub enum Memo {
    MEMO_NONE,
    MEMO_TEXT(String),
    MEMO_ID(uint64),
    MEMO_HASH(Hash),
    MEMO_RETURN(Hash),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MemoType {
    MEMO_NONE = 0isize,
    MEMO_TEXT = 1isize,
    MEMO_ID = 2isize,
    MEMO_HASH = 3isize,
    MEMO_RETURN = 4isize,
}

pub struct Operation {
    pub sourceAccount: Option<Box<AccountID>>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum OperationResult {
    opINNER,
    default,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum OperationResultCode {
    opINNER = 0isize,
    opBAD_AUTH = -1isize,
    opNO_ACCOUNT = -2isize,
    opNOT_SUPPORTED = -3isize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum OperationType {
    CREATE_ACCOUNT = 0isize,
    PAYMENT = 1isize,
    PATH_PAYMENT = 2isize,
    MANAGE_OFFER = 3isize,
    CREATE_PASSIVE_OFFER = 4isize,
    SET_OPTIONS = 5isize,
    CHANGE_TRUST = 6isize,
    ALLOW_TRUST = 7isize,
    ACCOUNT_MERGE = 8isize,
    INFLATION = 9isize,
    MANAGE_DATA = 10isize,
    BUMP_SEQUENCE = 11isize,
}

pub struct PathPaymentOp {
    pub sendAsset: Asset,
    pub sendMax: int64,
    pub destination: AccountID,
    pub destAsset: Asset,
    pub destAmount: int64,
    pub path: Vec<Asset>,
}

pub enum PathPaymentResult {
    PATH_PAYMENT_NO_ISSUER(Asset),
    default,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PathPaymentResultCode {
    PATH_PAYMENT_SUCCESS = 0isize,
    PATH_PAYMENT_MALFORMED = -1isize,
    PATH_PAYMENT_UNDERFUNDED = -2isize,
    PATH_PAYMENT_SRC_NO_TRUST = -3isize,
    PATH_PAYMENT_SRC_NOT_AUTHORIZED = -4isize,
    PATH_PAYMENT_NO_DESTINATION = -5isize,
    PATH_PAYMENT_NO_TRUST = -6isize,
    PATH_PAYMENT_NOT_AUTHORIZED = -7isize,
    PATH_PAYMENT_LINE_FULL = -8isize,
    PATH_PAYMENT_NO_ISSUER = -9isize,
    PATH_PAYMENT_TOO_FEW_OFFERS = -10isize,
    PATH_PAYMENT_OFFER_CROSS_SELF = -11isize,
    PATH_PAYMENT_OVER_SENDMAX = -12isize,
}

pub struct PaymentOp {
    pub destination: AccountID,
    pub asset: Asset,
    pub amount: int64,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PaymentResult {
    PAYMENT_SUCCESS,
    default,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PaymentResultCode {
    PAYMENT_SUCCESS = 0isize,
    PAYMENT_MALFORMED = -1isize,
    PAYMENT_UNDERFUNDED = -2isize,
    PAYMENT_SRC_NO_TRUST = -3isize,
    PAYMENT_SRC_NOT_AUTHORIZED = -4isize,
    PAYMENT_NO_DESTINATION = -5isize,
    PAYMENT_NO_TRUST = -6isize,
    PAYMENT_NOT_AUTHORIZED = -7isize,
    PAYMENT_LINE_FULL = -8isize,
    PAYMENT_NO_ISSUER = -9isize,
}

pub struct SetOptionsOp {
    pub inflationDest: Option<Box<AccountID>>,
    pub clearFlags: Option<Box<uint32>>,
    pub setFlags: Option<Box<uint32>>,
    pub masterWeight: Option<Box<uint32>>,
    pub lowThreshold: Option<Box<uint32>>,
    pub medThreshold: Option<Box<uint32>>,
    pub highThreshold: Option<Box<uint32>>,
    pub homeDomain: Option<Box<string32>>,
    pub signer: Option<Box<Signer>>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SetOptionsResult {
    SET_OPTIONS_SUCCESS,
    default,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SetOptionsResultCode {
    SET_OPTIONS_SUCCESS = 0isize,
    SET_OPTIONS_LOW_RESERVE = -1isize,
    SET_OPTIONS_TOO_MANY_SIGNERS = -2isize,
    SET_OPTIONS_BAD_FLAGS = -3isize,
    SET_OPTIONS_INVALID_INFLATION = -4isize,
    SET_OPTIONS_CANT_CHANGE = -5isize,
    SET_OPTIONS_UNKNOWN_FLAG = -6isize,
    SET_OPTIONS_THRESHOLD_OUT_OF_RANGE = -7isize,
    SET_OPTIONS_BAD_SIGNER = -8isize,
    SET_OPTIONS_INVALID_HOME_DOMAIN = -9isize,
}

pub struct SimplePaymentResult {
    pub destination: AccountID,
    pub asset: Asset,
    pub amount: int64,
}

pub struct TimeBounds {
    pub minTime: TimePoint,
    pub maxTime: TimePoint,
}

pub struct Transaction {
    pub sourceAccount: AccountID,
    pub fee: uint32,
    pub seqNum: SequenceNumber,
    pub timeBounds: Option<Box<TimeBounds>>,
    pub memo: Memo,
    pub operations: Vec<Operation>,
}

pub struct TransactionEnvelope {
    pub tx: Transaction,
    pub signatures: Vec<DecoratedSignature>,
}

pub struct TransactionResult {
    pub feeCharged: int64,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TransactionResultCode {
    txSUCCESS = 0isize,
    txFAILED = -1isize,
    txTOO_EARLY = -2isize,
    txTOO_LATE = -3isize,
    txMISSING_OPERATION = -4isize,
    txBAD_SEQ = -5isize,
    txBAD_AUTH = -6isize,
    txINSUFFICIENT_BALANCE = -7isize,
    txNO_ACCOUNT = -8isize,
    txINSUFFICIENT_FEE = -9isize,
    txBAD_AUTH_EXTRA = -10isize,
    txINTERNAL_ERROR = -11isize,
}

pub struct TransactionSignaturePayload {
    pub networkId: Hash,
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for AccountMergeResult {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &AccountMergeResult::ACCOUNT_MERGE_SUCCESS(ref val) => {
                (AccountMergeResultCode::ACCOUNT_MERGE_SUCCESS as i32).pack(out)? + val.pack(out)?
            }
            &AccountMergeResult::default => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for AccountMergeResultCode {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for AllowTrustOp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.trustor.pack(out)? + self.authorize.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for AllowTrustResult {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &AllowTrustResult::ALLOW_TRUST_SUCCESS => {
                (AllowTrustResultCode::ALLOW_TRUST_SUCCESS as i32).pack(out)?
            }
            &AllowTrustResult::default => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for AllowTrustResultCode {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for BumpSequenceOp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.bumpTo.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for BumpSequenceResult {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &BumpSequenceResult::BUMP_SEQUENCE_SUCCESS => {
                (BumpSequenceResultCode::BUMP_SEQUENCE_SUCCESS as i32).pack(out)?
            }
            &BumpSequenceResult::default => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for BumpSequenceResultCode {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for ChangeTrustOp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.line.pack(out)? + self.limit.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for ChangeTrustResult {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &ChangeTrustResult::CHANGE_TRUST_SUCCESS => {
                (ChangeTrustResultCode::CHANGE_TRUST_SUCCESS as i32).pack(out)?
            }
            &ChangeTrustResult::default => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for ChangeTrustResultCode {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for ClaimOfferAtom {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.sellerID.pack(out)?
            + self.offerID.pack(out)?
            + self.assetSold.pack(out)?
            + self.amountSold.pack(out)?
            + self.assetBought.pack(out)?
            + self.amountBought.pack(out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for CreateAccountOp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.destination.pack(out)? + self.startingBalance.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for CreateAccountResult {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &CreateAccountResult::CREATE_ACCOUNT_SUCCESS => {
                (CreateAccountResultCode::CREATE_ACCOUNT_SUCCESS as i32).pack(out)?
            }
            &CreateAccountResult::default => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for CreateAccountResultCode {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for CreatePassiveOfferOp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.selling.pack(out)?
            + self.buying.pack(out)?
            + self.amount.pack(out)?
            + self.price.pack(out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for DecoratedSignature {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.hint.pack(out)? + self.signature.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for InflationPayout {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.destination.pack(out)? + self.amount.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for InflationResult {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &InflationResult::INFLATION_SUCCESS(ref val) => {
                (InflationResultCode::INFLATION_SUCCESS as i32).pack(out)?
                    + xdr_codec::pack_flex(&val, None, out)?
            }
            &InflationResult::default => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for InflationResultCode {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for ManageDataOp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.dataName.pack(out)? + self.dataValue.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for ManageDataResult {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &ManageDataResult::MANAGE_DATA_SUCCESS => {
                (ManageDataResultCode::MANAGE_DATA_SUCCESS as i32).pack(out)?
            }
            &ManageDataResult::default => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for ManageDataResultCode {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for ManageOfferEffect {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for ManageOfferOp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.selling.pack(out)?
            + self.buying.pack(out)?
            + self.amount.pack(out)?
            + self.price.pack(out)?
            + self.offerID.pack(out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for ManageOfferResult {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &ManageOfferResult::MANAGE_OFFER_SUCCESS(ref val) => {
                (ManageOfferResultCode::MANAGE_OFFER_SUCCESS as i32).pack(out)? + val.pack(out)?
            }
            &ManageOfferResult::default => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for ManageOfferResultCode {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for ManageOfferSuccessResult {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(xdr_codec::pack_flex(&self.offersClaimed, None, out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for Memo {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &Memo::MEMO_NONE => (MemoType::MEMO_NONE as i32).pack(out)?,
            &Memo::MEMO_TEXT(ref val) => {
                (MemoType::MEMO_TEXT as i32).pack(out)?
                    + xdr_codec::pack_string(&val, Some(28i64 as usize), out)?
            }
            &Memo::MEMO_ID(ref val) => (MemoType::MEMO_ID as i32).pack(out)? + val.pack(out)?,
            &Memo::MEMO_HASH(ref val) => (MemoType::MEMO_HASH as i32).pack(out)? + val.pack(out)?,
            &Memo::MEMO_RETURN(ref val) => {
                (MemoType::MEMO_RETURN as i32).pack(out)? + val.pack(out)?
            }
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for MemoType {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for Operation {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.sourceAccount.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for OperationResult {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &OperationResult::opINNER => (OperationResultCode::opINNER as i32).pack(out)?,
            &OperationResult::default => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for OperationResultCode {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for OperationType {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for PathPaymentOp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.sendAsset.pack(out)?
            + self.sendMax.pack(out)?
            + self.destination.pack(out)?
            + self.destAsset.pack(out)?
            + self.destAmount.pack(out)?
            + xdr_codec::pack_flex(&self.path, Some(5i64 as usize), out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for PathPaymentResult {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &PathPaymentResult::PATH_PAYMENT_NO_ISSUER(ref val) => {
                (PathPaymentResultCode::PATH_PAYMENT_NO_ISSUER as i32).pack(out)? + val.pack(out)?
            }
            &PathPaymentResult::default => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for PathPaymentResultCode {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for PaymentOp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.destination.pack(out)? + self.asset.pack(out)? + self.amount.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for PaymentResult {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &PaymentResult::PAYMENT_SUCCESS => {
                (PaymentResultCode::PAYMENT_SUCCESS as i32).pack(out)?
            }
            &PaymentResult::default => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for PaymentResultCode {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for SetOptionsOp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.inflationDest.pack(out)?
            + self.clearFlags.pack(out)?
            + self.setFlags.pack(out)?
            + self.masterWeight.pack(out)?
            + self.lowThreshold.pack(out)?
            + self.medThreshold.pack(out)?
            + self.highThreshold.pack(out)?
            + self.homeDomain.pack(out)?
            + self.signer.pack(out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for SetOptionsResult {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &SetOptionsResult::SET_OPTIONS_SUCCESS => {
                (SetOptionsResultCode::SET_OPTIONS_SUCCESS as i32).pack(out)?
            }
            &SetOptionsResult::default => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for SetOptionsResultCode {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for SimplePaymentResult {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.destination.pack(out)? + self.asset.pack(out)? + self.amount.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for TimeBounds {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.minTime.pack(out)? + self.maxTime.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for Transaction {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.sourceAccount.pack(out)?
            + self.fee.pack(out)?
            + self.seqNum.pack(out)?
            + self.timeBounds.pack(out)?
            + self.memo.pack(out)?
            + xdr_codec::pack_flex(&self.operations, Some(100i64 as usize), out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for TransactionEnvelope {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.tx.pack(out)?
            + xdr_codec::pack_flex(&self.signatures, Some(20i64 as usize), out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for TransactionResult {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.feeCharged.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for TransactionResultCode {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for TransactionSignaturePayload {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.networkId.pack(out)? + 0)
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for AccountMergeResult {
    fn unpack(input: &mut In) -> xdr_codec::Result<(AccountMergeResult, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => AccountMergeResult::ACCOUNT_MERGE_SUCCESS({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                _ => AccountMergeResult::default,
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for AccountMergeResultCode {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(AccountMergeResultCode, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == AccountMergeResultCode::ACCOUNT_MERGE_SUCCESS as i32 => {
                        AccountMergeResultCode::ACCOUNT_MERGE_SUCCESS
                    }
                    x if x == AccountMergeResultCode::ACCOUNT_MERGE_MALFORMED as i32 => {
                        AccountMergeResultCode::ACCOUNT_MERGE_MALFORMED
                    }
                    x if x == AccountMergeResultCode::ACCOUNT_MERGE_NO_ACCOUNT as i32 => {
                        AccountMergeResultCode::ACCOUNT_MERGE_NO_ACCOUNT
                    }
                    x if x == AccountMergeResultCode::ACCOUNT_MERGE_IMMUTABLE_SET as i32 => {
                        AccountMergeResultCode::ACCOUNT_MERGE_IMMUTABLE_SET
                    }
                    x if x == AccountMergeResultCode::ACCOUNT_MERGE_HAS_SUB_ENTRIES as i32 => {
                        AccountMergeResultCode::ACCOUNT_MERGE_HAS_SUB_ENTRIES
                    }
                    x if x == AccountMergeResultCode::ACCOUNT_MERGE_SEQNUM_TOO_FAR as i32 => {
                        AccountMergeResultCode::ACCOUNT_MERGE_SEQNUM_TOO_FAR
                    }
                    x if x == AccountMergeResultCode::ACCOUNT_MERGE_DEST_FULL as i32 => {
                        AccountMergeResultCode::ACCOUNT_MERGE_DEST_FULL
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for AllowTrustOp {
    fn unpack(input: &mut In) -> xdr_codec::Result<(AllowTrustOp, usize)> {
        let mut sz = 0;
        Ok((
            AllowTrustOp {
                trustor: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                authorize: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for AllowTrustResult {
    fn unpack(input: &mut In) -> xdr_codec::Result<(AllowTrustResult, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => AllowTrustResult::ALLOW_TRUST_SUCCESS,
                _ => AllowTrustResult::default,
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for AllowTrustResultCode {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(AllowTrustResultCode, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == AllowTrustResultCode::ALLOW_TRUST_SUCCESS as i32 => {
                        AllowTrustResultCode::ALLOW_TRUST_SUCCESS
                    }
                    x if x == AllowTrustResultCode::ALLOW_TRUST_MALFORMED as i32 => {
                        AllowTrustResultCode::ALLOW_TRUST_MALFORMED
                    }
                    x if x == AllowTrustResultCode::ALLOW_TRUST_NO_TRUST_LINE as i32 => {
                        AllowTrustResultCode::ALLOW_TRUST_NO_TRUST_LINE
                    }
                    x if x == AllowTrustResultCode::ALLOW_TRUST_TRUST_NOT_REQUIRED as i32 => {
                        AllowTrustResultCode::ALLOW_TRUST_TRUST_NOT_REQUIRED
                    }
                    x if x == AllowTrustResultCode::ALLOW_TRUST_CANT_REVOKE as i32 => {
                        AllowTrustResultCode::ALLOW_TRUST_CANT_REVOKE
                    }
                    x if x == AllowTrustResultCode::ALLOW_TRUST_SELF_NOT_ALLOWED as i32 => {
                        AllowTrustResultCode::ALLOW_TRUST_SELF_NOT_ALLOWED
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for BumpSequenceOp {
    fn unpack(input: &mut In) -> xdr_codec::Result<(BumpSequenceOp, usize)> {
        let mut sz = 0;
        Ok((
            BumpSequenceOp {
                bumpTo: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for BumpSequenceResult {
    fn unpack(input: &mut In) -> xdr_codec::Result<(BumpSequenceResult, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => BumpSequenceResult::BUMP_SEQUENCE_SUCCESS,
                _ => BumpSequenceResult::default,
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for BumpSequenceResultCode {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(BumpSequenceResultCode, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == BumpSequenceResultCode::BUMP_SEQUENCE_SUCCESS as i32 => {
                        BumpSequenceResultCode::BUMP_SEQUENCE_SUCCESS
                    }
                    x if x == BumpSequenceResultCode::BUMP_SEQUENCE_BAD_SEQ as i32 => {
                        BumpSequenceResultCode::BUMP_SEQUENCE_BAD_SEQ
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for ChangeTrustOp {
    fn unpack(input: &mut In) -> xdr_codec::Result<(ChangeTrustOp, usize)> {
        let mut sz = 0;
        Ok((
            ChangeTrustOp {
                line: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                limit: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for ChangeTrustResult {
    fn unpack(input: &mut In) -> xdr_codec::Result<(ChangeTrustResult, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => ChangeTrustResult::CHANGE_TRUST_SUCCESS,
                _ => ChangeTrustResult::default,
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for ChangeTrustResultCode {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(ChangeTrustResultCode, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == ChangeTrustResultCode::CHANGE_TRUST_SUCCESS as i32 => {
                        ChangeTrustResultCode::CHANGE_TRUST_SUCCESS
                    }
                    x if x == ChangeTrustResultCode::CHANGE_TRUST_MALFORMED as i32 => {
                        ChangeTrustResultCode::CHANGE_TRUST_MALFORMED
                    }
                    x if x == ChangeTrustResultCode::CHANGE_TRUST_NO_ISSUER as i32 => {
                        ChangeTrustResultCode::CHANGE_TRUST_NO_ISSUER
                    }
                    x if x == ChangeTrustResultCode::CHANGE_TRUST_INVALID_LIMIT as i32 => {
                        ChangeTrustResultCode::CHANGE_TRUST_INVALID_LIMIT
                    }
                    x if x == ChangeTrustResultCode::CHANGE_TRUST_LOW_RESERVE as i32 => {
                        ChangeTrustResultCode::CHANGE_TRUST_LOW_RESERVE
                    }
                    x if x == ChangeTrustResultCode::CHANGE_TRUST_SELF_NOT_ALLOWED as i32 => {
                        ChangeTrustResultCode::CHANGE_TRUST_SELF_NOT_ALLOWED
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for ClaimOfferAtom {
    fn unpack(input: &mut In) -> xdr_codec::Result<(ClaimOfferAtom, usize)> {
        let mut sz = 0;
        Ok((
            ClaimOfferAtom {
                sellerID: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                offerID: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                assetSold: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                amountSold: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                assetBought: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                amountBought: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for CreateAccountOp {
    fn unpack(input: &mut In) -> xdr_codec::Result<(CreateAccountOp, usize)> {
        let mut sz = 0;
        Ok((
            CreateAccountOp {
                destination: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                startingBalance: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for CreateAccountResult {
    fn unpack(input: &mut In) -> xdr_codec::Result<(CreateAccountResult, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => CreateAccountResult::CREATE_ACCOUNT_SUCCESS,
                _ => CreateAccountResult::default,
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for CreateAccountResultCode {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(CreateAccountResultCode, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == CreateAccountResultCode::CREATE_ACCOUNT_SUCCESS as i32 => {
                        CreateAccountResultCode::CREATE_ACCOUNT_SUCCESS
                    }
                    x if x == CreateAccountResultCode::CREATE_ACCOUNT_MALFORMED as i32 => {
                        CreateAccountResultCode::CREATE_ACCOUNT_MALFORMED
                    }
                    x if x == CreateAccountResultCode::CREATE_ACCOUNT_UNDERFUNDED as i32 => {
                        CreateAccountResultCode::CREATE_ACCOUNT_UNDERFUNDED
                    }
                    x if x == CreateAccountResultCode::CREATE_ACCOUNT_LOW_RESERVE as i32 => {
                        CreateAccountResultCode::CREATE_ACCOUNT_LOW_RESERVE
                    }
                    x if x == CreateAccountResultCode::CREATE_ACCOUNT_ALREADY_EXIST as i32 => {
                        CreateAccountResultCode::CREATE_ACCOUNT_ALREADY_EXIST
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for CreatePassiveOfferOp {
    fn unpack(input: &mut In) -> xdr_codec::Result<(CreatePassiveOfferOp, usize)> {
        let mut sz = 0;
        Ok((
            CreatePassiveOfferOp {
                selling: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                buying: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                amount: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                price: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for DecoratedSignature {
    fn unpack(input: &mut In) -> xdr_codec::Result<(DecoratedSignature, usize)> {
        let mut sz = 0;
        Ok((
            DecoratedSignature {
                hint: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                signature: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for InflationPayout {
    fn unpack(input: &mut In) -> xdr_codec::Result<(InflationPayout, usize)> {
        let mut sz = 0;
        Ok((
            InflationPayout {
                destination: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                amount: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for InflationResult {
    fn unpack(input: &mut In) -> xdr_codec::Result<(InflationResult, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => InflationResult::INFLATION_SUCCESS({
                    let (v, fsz) = xdr_codec::unpack_flex(input, None)?;
                    sz += fsz;
                    v
                }),
                _ => InflationResult::default,
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for InflationResultCode {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(InflationResultCode, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == InflationResultCode::INFLATION_SUCCESS as i32 => {
                        InflationResultCode::INFLATION_SUCCESS
                    }
                    x if x == InflationResultCode::INFLATION_NOT_TIME as i32 => {
                        InflationResultCode::INFLATION_NOT_TIME
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for ManageDataOp {
    fn unpack(input: &mut In) -> xdr_codec::Result<(ManageDataOp, usize)> {
        let mut sz = 0;
        Ok((
            ManageDataOp {
                dataName: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                dataValue: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for ManageDataResult {
    fn unpack(input: &mut In) -> xdr_codec::Result<(ManageDataResult, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => ManageDataResult::MANAGE_DATA_SUCCESS,
                _ => ManageDataResult::default,
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for ManageDataResultCode {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(ManageDataResultCode, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == ManageDataResultCode::MANAGE_DATA_SUCCESS as i32 => {
                        ManageDataResultCode::MANAGE_DATA_SUCCESS
                    }
                    x if x == ManageDataResultCode::MANAGE_DATA_NOT_SUPPORTED_YET as i32 => {
                        ManageDataResultCode::MANAGE_DATA_NOT_SUPPORTED_YET
                    }
                    x if x == ManageDataResultCode::MANAGE_DATA_NAME_NOT_FOUND as i32 => {
                        ManageDataResultCode::MANAGE_DATA_NAME_NOT_FOUND
                    }
                    x if x == ManageDataResultCode::MANAGE_DATA_LOW_RESERVE as i32 => {
                        ManageDataResultCode::MANAGE_DATA_LOW_RESERVE
                    }
                    x if x == ManageDataResultCode::MANAGE_DATA_INVALID_NAME as i32 => {
                        ManageDataResultCode::MANAGE_DATA_INVALID_NAME
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for ManageOfferEffect {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(ManageOfferEffect, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == ManageOfferEffect::MANAGE_OFFER_CREATED as i32 => {
                        ManageOfferEffect::MANAGE_OFFER_CREATED
                    }
                    x if x == ManageOfferEffect::MANAGE_OFFER_UPDATED as i32 => {
                        ManageOfferEffect::MANAGE_OFFER_UPDATED
                    }
                    x if x == ManageOfferEffect::MANAGE_OFFER_DELETED as i32 => {
                        ManageOfferEffect::MANAGE_OFFER_DELETED
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for ManageOfferOp {
    fn unpack(input: &mut In) -> xdr_codec::Result<(ManageOfferOp, usize)> {
        let mut sz = 0;
        Ok((
            ManageOfferOp {
                selling: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                buying: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                amount: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                price: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                offerID: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for ManageOfferResult {
    fn unpack(input: &mut In) -> xdr_codec::Result<(ManageOfferResult, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => ManageOfferResult::MANAGE_OFFER_SUCCESS({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                _ => ManageOfferResult::default,
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for ManageOfferResultCode {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(ManageOfferResultCode, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == ManageOfferResultCode::MANAGE_OFFER_SUCCESS as i32 => {
                        ManageOfferResultCode::MANAGE_OFFER_SUCCESS
                    }
                    x if x == ManageOfferResultCode::MANAGE_OFFER_MALFORMED as i32 => {
                        ManageOfferResultCode::MANAGE_OFFER_MALFORMED
                    }
                    x if x == ManageOfferResultCode::MANAGE_OFFER_SELL_NO_TRUST as i32 => {
                        ManageOfferResultCode::MANAGE_OFFER_SELL_NO_TRUST
                    }
                    x if x == ManageOfferResultCode::MANAGE_OFFER_BUY_NO_TRUST as i32 => {
                        ManageOfferResultCode::MANAGE_OFFER_BUY_NO_TRUST
                    }
                    x if x == ManageOfferResultCode::MANAGE_OFFER_SELL_NOT_AUTHORIZED as i32 => {
                        ManageOfferResultCode::MANAGE_OFFER_SELL_NOT_AUTHORIZED
                    }
                    x if x == ManageOfferResultCode::MANAGE_OFFER_BUY_NOT_AUTHORIZED as i32 => {
                        ManageOfferResultCode::MANAGE_OFFER_BUY_NOT_AUTHORIZED
                    }
                    x if x == ManageOfferResultCode::MANAGE_OFFER_LINE_FULL as i32 => {
                        ManageOfferResultCode::MANAGE_OFFER_LINE_FULL
                    }
                    x if x == ManageOfferResultCode::MANAGE_OFFER_UNDERFUNDED as i32 => {
                        ManageOfferResultCode::MANAGE_OFFER_UNDERFUNDED
                    }
                    x if x == ManageOfferResultCode::MANAGE_OFFER_CROSS_SELF as i32 => {
                        ManageOfferResultCode::MANAGE_OFFER_CROSS_SELF
                    }
                    x if x == ManageOfferResultCode::MANAGE_OFFER_SELL_NO_ISSUER as i32 => {
                        ManageOfferResultCode::MANAGE_OFFER_SELL_NO_ISSUER
                    }
                    x if x == ManageOfferResultCode::MANAGE_OFFER_BUY_NO_ISSUER as i32 => {
                        ManageOfferResultCode::MANAGE_OFFER_BUY_NO_ISSUER
                    }
                    x if x == ManageOfferResultCode::MANAGE_OFFER_NOT_FOUND as i32 => {
                        ManageOfferResultCode::MANAGE_OFFER_NOT_FOUND
                    }
                    x if x == ManageOfferResultCode::MANAGE_OFFER_LOW_RESERVE as i32 => {
                        ManageOfferResultCode::MANAGE_OFFER_LOW_RESERVE
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for ManageOfferSuccessResult {
    fn unpack(input: &mut In) -> xdr_codec::Result<(ManageOfferSuccessResult, usize)> {
        let mut sz = 0;
        Ok((
            ManageOfferSuccessResult {
                offersClaimed: {
                    let (v, fsz) = xdr_codec::unpack_flex(input, None)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for Memo {
    fn unpack(input: &mut In) -> xdr_codec::Result<(Memo, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => Memo::MEMO_NONE,
                x if x == (1i32 as i32) => Memo::MEMO_TEXT({
                    let (v, fsz) = xdr_codec::unpack_string(input, Some(28i64 as usize))?;
                    sz += fsz;
                    v
                }),
                x if x == (2i32 as i32) => Memo::MEMO_ID({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (3i32 as i32) => Memo::MEMO_HASH({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (4i32 as i32) => Memo::MEMO_RETURN({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                v => return Err(xdr_codec::Error::invalidcase(v as i32)),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for MemoType {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(MemoType, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == MemoType::MEMO_NONE as i32 => MemoType::MEMO_NONE,
                    x if x == MemoType::MEMO_TEXT as i32 => MemoType::MEMO_TEXT,
                    x if x == MemoType::MEMO_ID as i32 => MemoType::MEMO_ID,
                    x if x == MemoType::MEMO_HASH as i32 => MemoType::MEMO_HASH,
                    x if x == MemoType::MEMO_RETURN as i32 => MemoType::MEMO_RETURN,
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for Operation {
    fn unpack(input: &mut In) -> xdr_codec::Result<(Operation, usize)> {
        let mut sz = 0;
        Ok((
            Operation {
                sourceAccount: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for OperationResult {
    fn unpack(input: &mut In) -> xdr_codec::Result<(OperationResult, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => OperationResult::opINNER,
                _ => OperationResult::default,
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for OperationResultCode {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(OperationResultCode, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == OperationResultCode::opINNER as i32 => OperationResultCode::opINNER,
                    x if x == OperationResultCode::opBAD_AUTH as i32 => {
                        OperationResultCode::opBAD_AUTH
                    }
                    x if x == OperationResultCode::opNO_ACCOUNT as i32 => {
                        OperationResultCode::opNO_ACCOUNT
                    }
                    x if x == OperationResultCode::opNOT_SUPPORTED as i32 => {
                        OperationResultCode::opNOT_SUPPORTED
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for OperationType {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(OperationType, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == OperationType::CREATE_ACCOUNT as i32 => OperationType::CREATE_ACCOUNT,
                    x if x == OperationType::PAYMENT as i32 => OperationType::PAYMENT,
                    x if x == OperationType::PATH_PAYMENT as i32 => OperationType::PATH_PAYMENT,
                    x if x == OperationType::MANAGE_OFFER as i32 => OperationType::MANAGE_OFFER,
                    x if x == OperationType::CREATE_PASSIVE_OFFER as i32 => {
                        OperationType::CREATE_PASSIVE_OFFER
                    }
                    x if x == OperationType::SET_OPTIONS as i32 => OperationType::SET_OPTIONS,
                    x if x == OperationType::CHANGE_TRUST as i32 => OperationType::CHANGE_TRUST,
                    x if x == OperationType::ALLOW_TRUST as i32 => OperationType::ALLOW_TRUST,
                    x if x == OperationType::ACCOUNT_MERGE as i32 => OperationType::ACCOUNT_MERGE,
                    x if x == OperationType::INFLATION as i32 => OperationType::INFLATION,
                    x if x == OperationType::MANAGE_DATA as i32 => OperationType::MANAGE_DATA,
                    x if x == OperationType::BUMP_SEQUENCE as i32 => OperationType::BUMP_SEQUENCE,
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for PathPaymentOp {
    fn unpack(input: &mut In) -> xdr_codec::Result<(PathPaymentOp, usize)> {
        let mut sz = 0;
        Ok((
            PathPaymentOp {
                sendAsset: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                sendMax: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                destination: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                destAsset: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                destAmount: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                path: {
                    let (v, fsz) = xdr_codec::unpack_flex(input, Some(5i64 as usize))?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for PathPaymentResult {
    fn unpack(input: &mut In) -> xdr_codec::Result<(PathPaymentResult, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (-9i32 as i32) => PathPaymentResult::PATH_PAYMENT_NO_ISSUER({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                _ => PathPaymentResult::default,
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for PathPaymentResultCode {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(PathPaymentResultCode, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == PathPaymentResultCode::PATH_PAYMENT_SUCCESS as i32 => {
                        PathPaymentResultCode::PATH_PAYMENT_SUCCESS
                    }
                    x if x == PathPaymentResultCode::PATH_PAYMENT_MALFORMED as i32 => {
                        PathPaymentResultCode::PATH_PAYMENT_MALFORMED
                    }
                    x if x == PathPaymentResultCode::PATH_PAYMENT_UNDERFUNDED as i32 => {
                        PathPaymentResultCode::PATH_PAYMENT_UNDERFUNDED
                    }
                    x if x == PathPaymentResultCode::PATH_PAYMENT_SRC_NO_TRUST as i32 => {
                        PathPaymentResultCode::PATH_PAYMENT_SRC_NO_TRUST
                    }
                    x if x == PathPaymentResultCode::PATH_PAYMENT_SRC_NOT_AUTHORIZED as i32 => {
                        PathPaymentResultCode::PATH_PAYMENT_SRC_NOT_AUTHORIZED
                    }
                    x if x == PathPaymentResultCode::PATH_PAYMENT_NO_DESTINATION as i32 => {
                        PathPaymentResultCode::PATH_PAYMENT_NO_DESTINATION
                    }
                    x if x == PathPaymentResultCode::PATH_PAYMENT_NO_TRUST as i32 => {
                        PathPaymentResultCode::PATH_PAYMENT_NO_TRUST
                    }
                    x if x == PathPaymentResultCode::PATH_PAYMENT_NOT_AUTHORIZED as i32 => {
                        PathPaymentResultCode::PATH_PAYMENT_NOT_AUTHORIZED
                    }
                    x if x == PathPaymentResultCode::PATH_PAYMENT_LINE_FULL as i32 => {
                        PathPaymentResultCode::PATH_PAYMENT_LINE_FULL
                    }
                    x if x == PathPaymentResultCode::PATH_PAYMENT_NO_ISSUER as i32 => {
                        PathPaymentResultCode::PATH_PAYMENT_NO_ISSUER
                    }
                    x if x == PathPaymentResultCode::PATH_PAYMENT_TOO_FEW_OFFERS as i32 => {
                        PathPaymentResultCode::PATH_PAYMENT_TOO_FEW_OFFERS
                    }
                    x if x == PathPaymentResultCode::PATH_PAYMENT_OFFER_CROSS_SELF as i32 => {
                        PathPaymentResultCode::PATH_PAYMENT_OFFER_CROSS_SELF
                    }
                    x if x == PathPaymentResultCode::PATH_PAYMENT_OVER_SENDMAX as i32 => {
                        PathPaymentResultCode::PATH_PAYMENT_OVER_SENDMAX
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for PaymentOp {
    fn unpack(input: &mut In) -> xdr_codec::Result<(PaymentOp, usize)> {
        let mut sz = 0;
        Ok((
            PaymentOp {
                destination: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                asset: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                amount: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for PaymentResult {
    fn unpack(input: &mut In) -> xdr_codec::Result<(PaymentResult, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => PaymentResult::PAYMENT_SUCCESS,
                _ => PaymentResult::default,
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for PaymentResultCode {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(PaymentResultCode, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == PaymentResultCode::PAYMENT_SUCCESS as i32 => {
                        PaymentResultCode::PAYMENT_SUCCESS
                    }
                    x if x == PaymentResultCode::PAYMENT_MALFORMED as i32 => {
                        PaymentResultCode::PAYMENT_MALFORMED
                    }
                    x if x == PaymentResultCode::PAYMENT_UNDERFUNDED as i32 => {
                        PaymentResultCode::PAYMENT_UNDERFUNDED
                    }
                    x if x == PaymentResultCode::PAYMENT_SRC_NO_TRUST as i32 => {
                        PaymentResultCode::PAYMENT_SRC_NO_TRUST
                    }
                    x if x == PaymentResultCode::PAYMENT_SRC_NOT_AUTHORIZED as i32 => {
                        PaymentResultCode::PAYMENT_SRC_NOT_AUTHORIZED
                    }
                    x if x == PaymentResultCode::PAYMENT_NO_DESTINATION as i32 => {
                        PaymentResultCode::PAYMENT_NO_DESTINATION
                    }
                    x if x == PaymentResultCode::PAYMENT_NO_TRUST as i32 => {
                        PaymentResultCode::PAYMENT_NO_TRUST
                    }
                    x if x == PaymentResultCode::PAYMENT_NOT_AUTHORIZED as i32 => {
                        PaymentResultCode::PAYMENT_NOT_AUTHORIZED
                    }
                    x if x == PaymentResultCode::PAYMENT_LINE_FULL as i32 => {
                        PaymentResultCode::PAYMENT_LINE_FULL
                    }
                    x if x == PaymentResultCode::PAYMENT_NO_ISSUER as i32 => {
                        PaymentResultCode::PAYMENT_NO_ISSUER
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for SetOptionsOp {
    fn unpack(input: &mut In) -> xdr_codec::Result<(SetOptionsOp, usize)> {
        let mut sz = 0;
        Ok((
            SetOptionsOp {
                inflationDest: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                clearFlags: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                setFlags: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                masterWeight: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                lowThreshold: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                medThreshold: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                highThreshold: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                homeDomain: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                signer: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for SetOptionsResult {
    fn unpack(input: &mut In) -> xdr_codec::Result<(SetOptionsResult, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => SetOptionsResult::SET_OPTIONS_SUCCESS,
                _ => SetOptionsResult::default,
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for SetOptionsResultCode {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(SetOptionsResultCode, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == SetOptionsResultCode::SET_OPTIONS_SUCCESS as i32 => {
                        SetOptionsResultCode::SET_OPTIONS_SUCCESS
                    }
                    x if x == SetOptionsResultCode::SET_OPTIONS_LOW_RESERVE as i32 => {
                        SetOptionsResultCode::SET_OPTIONS_LOW_RESERVE
                    }
                    x if x == SetOptionsResultCode::SET_OPTIONS_TOO_MANY_SIGNERS as i32 => {
                        SetOptionsResultCode::SET_OPTIONS_TOO_MANY_SIGNERS
                    }
                    x if x == SetOptionsResultCode::SET_OPTIONS_BAD_FLAGS as i32 => {
                        SetOptionsResultCode::SET_OPTIONS_BAD_FLAGS
                    }
                    x if x == SetOptionsResultCode::SET_OPTIONS_INVALID_INFLATION as i32 => {
                        SetOptionsResultCode::SET_OPTIONS_INVALID_INFLATION
                    }
                    x if x == SetOptionsResultCode::SET_OPTIONS_CANT_CHANGE as i32 => {
                        SetOptionsResultCode::SET_OPTIONS_CANT_CHANGE
                    }
                    x if x == SetOptionsResultCode::SET_OPTIONS_UNKNOWN_FLAG as i32 => {
                        SetOptionsResultCode::SET_OPTIONS_UNKNOWN_FLAG
                    }
                    x if x == SetOptionsResultCode::SET_OPTIONS_THRESHOLD_OUT_OF_RANGE as i32 => {
                        SetOptionsResultCode::SET_OPTIONS_THRESHOLD_OUT_OF_RANGE
                    }
                    x if x == SetOptionsResultCode::SET_OPTIONS_BAD_SIGNER as i32 => {
                        SetOptionsResultCode::SET_OPTIONS_BAD_SIGNER
                    }
                    x if x == SetOptionsResultCode::SET_OPTIONS_INVALID_HOME_DOMAIN as i32 => {
                        SetOptionsResultCode::SET_OPTIONS_INVALID_HOME_DOMAIN
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for SimplePaymentResult {
    fn unpack(input: &mut In) -> xdr_codec::Result<(SimplePaymentResult, usize)> {
        let mut sz = 0;
        Ok((
            SimplePaymentResult {
                destination: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                asset: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                amount: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for TimeBounds {
    fn unpack(input: &mut In) -> xdr_codec::Result<(TimeBounds, usize)> {
        let mut sz = 0;
        Ok((
            TimeBounds {
                minTime: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                maxTime: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for Transaction {
    fn unpack(input: &mut In) -> xdr_codec::Result<(Transaction, usize)> {
        let mut sz = 0;
        Ok((
            Transaction {
                sourceAccount: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                fee: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                seqNum: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                timeBounds: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                memo: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                operations: {
                    let (v, fsz) = xdr_codec::unpack_flex(input, Some(100i64 as usize))?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for TransactionEnvelope {
    fn unpack(input: &mut In) -> xdr_codec::Result<(TransactionEnvelope, usize)> {
        let mut sz = 0;
        Ok((
            TransactionEnvelope {
                tx: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                signatures: {
                    let (v, fsz) = xdr_codec::unpack_flex(input, Some(20i64 as usize))?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for TransactionResult {
    fn unpack(input: &mut In) -> xdr_codec::Result<(TransactionResult, usize)> {
        let mut sz = 0;
        Ok((
            TransactionResult {
                feeCharged: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for TransactionResultCode {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(TransactionResultCode, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == TransactionResultCode::txSUCCESS as i32 => {
                        TransactionResultCode::txSUCCESS
                    }
                    x if x == TransactionResultCode::txFAILED as i32 => {
                        TransactionResultCode::txFAILED
                    }
                    x if x == TransactionResultCode::txTOO_EARLY as i32 => {
                        TransactionResultCode::txTOO_EARLY
                    }
                    x if x == TransactionResultCode::txTOO_LATE as i32 => {
                        TransactionResultCode::txTOO_LATE
                    }
                    x if x == TransactionResultCode::txMISSING_OPERATION as i32 => {
                        TransactionResultCode::txMISSING_OPERATION
                    }
                    x if x == TransactionResultCode::txBAD_SEQ as i32 => {
                        TransactionResultCode::txBAD_SEQ
                    }
                    x if x == TransactionResultCode::txBAD_AUTH as i32 => {
                        TransactionResultCode::txBAD_AUTH
                    }
                    x if x == TransactionResultCode::txINSUFFICIENT_BALANCE as i32 => {
                        TransactionResultCode::txINSUFFICIENT_BALANCE
                    }
                    x if x == TransactionResultCode::txNO_ACCOUNT as i32 => {
                        TransactionResultCode::txNO_ACCOUNT
                    }
                    x if x == TransactionResultCode::txINSUFFICIENT_FEE as i32 => {
                        TransactionResultCode::txINSUFFICIENT_FEE
                    }
                    x if x == TransactionResultCode::txBAD_AUTH_EXTRA as i32 => {
                        TransactionResultCode::txBAD_AUTH_EXTRA
                    }
                    x if x == TransactionResultCode::txINTERNAL_ERROR as i32 => {
                        TransactionResultCode::txINTERNAL_ERROR
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for TransactionSignaturePayload {
    fn unpack(input: &mut In) -> xdr_codec::Result<(TransactionSignaturePayload, usize)> {
        let mut sz = 0;
        Ok((
            TransactionSignaturePayload {
                networkId: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}
