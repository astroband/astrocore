// GENERATED CODE
//
// Generated from src/xdr/Stellar-ledger-entries.x by xdrgen.
//
// DO NOT EDIT

pub const MASK_ACCOUNT_FLAGS: i64 = 7i64;

pub const MASK_OFFERENTRY_FLAGS: i64 = 1i64;

pub const MASK_TRUSTLINE_FLAGS: i64 = 1i64;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountEntry {
    pub accountID: AccountID,
    pub balance: int64,
    pub seqNum: SequenceNumber,
    pub numSubEntries: uint32,
    pub inflationDest: Option<Box<AccountID>>,
    pub flags: uint32,
    pub homeDomain: string32,
    pub thresholds: Thresholds,
    pub signers: Vec<Signer>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AccountFlags {
    AUTH_REQUIRED_FLAG = 1isize,
    AUTH_REVOCABLE_FLAG = 2isize,
    AUTH_IMMUTABLE_FLAG = 4isize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Asset {
    ASSET_TYPE_NATIVE,
    ASSET_TYPE_CREDIT_ALPHANUM4(AssetAlphaNum4),
    ASSET_TYPE_CREDIT_ALPHANUM12(AssetAlphaNum12),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssetAlphaNum12 {
    pub assetCode: [u8; 12i64 as usize],
    pub issuer: AccountID,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssetAlphaNum4 {
    pub assetCode: [u8; 4i64 as usize],
    pub issuer: AccountID,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AssetType {
    ASSET_TYPE_NATIVE = 0isize,
    ASSET_TYPE_CREDIT_ALPHANUM4 = 1isize,
    ASSET_TYPE_CREDIT_ALPHANUM12 = 2isize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataEntry {
    pub accountID: AccountID,
    pub dataName: string64,
    pub dataValue: DataValue,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataValue(pub Vec<u8>);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum EnvelopeType {
    ENVELOPE_TYPE_SCP = 1isize,
    ENVELOPE_TYPE_TX = 2isize,
    ENVELOPE_TYPE_AUTH = 3isize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LedgerEntry {
    pub lastModifiedLedgerSeq: uint32,
    pub data: LedgerEntryData,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LedgerEntryData {
    ACCOUNT(AccountEntry),
    TRUSTLINE(TrustLineEntry),
    OFFER(OfferEntry),
    DATA(DataEntry),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum LedgerEntryType {
    ACCOUNT = 0isize,
    TRUSTLINE = 1isize,
    OFFER = 2isize,
    DATA = 3isize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Liabilities {
    pub buying: int64,
    pub selling: int64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OfferEntry {
    pub sellerID: AccountID,
    pub offerID: uint64,
    pub selling: Asset,
    pub buying: Asset,
    pub amount: int64,
    pub price: Price,
    pub flags: uint32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum OfferEntryFlags {
    PASSIVE_FLAG = 1isize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Price {
    pub n: int32,
    pub d: int32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Signer {
    pub key: SignerKey,
    pub weight: uint32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ThresholdIndexes {
    THRESHOLD_MASTER_WEIGHT = 0isize,
    THRESHOLD_LOW = 1isize,
    THRESHOLD_MED = 2isize,
    THRESHOLD_HIGH = 3isize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Thresholds(pub [u8; 4i64 as usize]);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrustLineEntry {
    pub accountID: AccountID,
    pub asset: Asset,
    pub balance: int64,
    pub limit: int64,
    pub flags: uint32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TrustLineFlags {
    AUTHORIZED_FLAG = 1isize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct string32(pub String);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct string64(pub String);

pub type AccountID = PublicKey;

pub type SequenceNumber = int64;

pub type TimePoint = uint64;

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for AccountEntry {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.accountID.pack(out)?
            + self.balance.pack(out)?
            + self.seqNum.pack(out)?
            + self.numSubEntries.pack(out)?
            + self.inflationDest.pack(out)?
            + self.flags.pack(out)?
            + self.homeDomain.pack(out)?
            + self.thresholds.pack(out)?
            + xdr_codec::pack_flex(&self.signers, Some(20i64 as usize), out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for AccountFlags {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for Asset {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &Asset::ASSET_TYPE_NATIVE => (AssetType::ASSET_TYPE_NATIVE as i32).pack(out)?,
            &Asset::ASSET_TYPE_CREDIT_ALPHANUM4(ref val) => {
                (AssetType::ASSET_TYPE_CREDIT_ALPHANUM4 as i32).pack(out)? + val.pack(out)?
            }
            &Asset::ASSET_TYPE_CREDIT_ALPHANUM12(ref val) => {
                (AssetType::ASSET_TYPE_CREDIT_ALPHANUM12 as i32).pack(out)? + val.pack(out)?
            }
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for AssetAlphaNum12 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(
            xdr_codec::pack_opaque_array(&self.assetCode[..], self.assetCode.len(), out)?
                + self.issuer.pack(out)?
                + 0,
        )
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for AssetAlphaNum4 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(
            xdr_codec::pack_opaque_array(&self.assetCode[..], self.assetCode.len(), out)?
                + self.issuer.pack(out)?
                + 0,
        )
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for AssetType {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for DataEntry {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.accountID.pack(out)? + self.dataName.pack(out)? + self.dataValue.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for DataValue {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(xdr_codec::pack_opaque_flex(
            &self.0,
            Some(64i64 as usize),
            out,
        )?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for EnvelopeType {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for LedgerEntry {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.lastModifiedLedgerSeq.pack(out)? + self.data.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for LedgerEntryData {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &LedgerEntryData::ACCOUNT(ref val) => {
                (LedgerEntryType::ACCOUNT as i32).pack(out)? + val.pack(out)?
            }
            &LedgerEntryData::TRUSTLINE(ref val) => {
                (LedgerEntryType::TRUSTLINE as i32).pack(out)? + val.pack(out)?
            }
            &LedgerEntryData::OFFER(ref val) => {
                (LedgerEntryType::OFFER as i32).pack(out)? + val.pack(out)?
            }
            &LedgerEntryData::DATA(ref val) => {
                (LedgerEntryType::DATA as i32).pack(out)? + val.pack(out)?
            }
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for LedgerEntryType {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for Liabilities {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.buying.pack(out)? + self.selling.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for OfferEntry {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.sellerID.pack(out)?
            + self.offerID.pack(out)?
            + self.selling.pack(out)?
            + self.buying.pack(out)?
            + self.amount.pack(out)?
            + self.price.pack(out)?
            + self.flags.pack(out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for OfferEntryFlags {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for Price {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.n.pack(out)? + self.d.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for Signer {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.key.pack(out)? + self.weight.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for ThresholdIndexes {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for Thresholds {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(xdr_codec::pack_opaque_array(
            &self.0[..],
            self.0.len(),
            out,
        )?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for TrustLineEntry {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.accountID.pack(out)?
            + self.asset.pack(out)?
            + self.balance.pack(out)?
            + self.limit.pack(out)?
            + self.flags.pack(out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for TrustLineFlags {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for string32 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(xdr_codec::pack_string(&self.0, Some(32i64 as usize), out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for string64 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(xdr_codec::pack_string(&self.0, Some(64i64 as usize), out)?)
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for AccountEntry {
    fn unpack(input: &mut In) -> xdr_codec::Result<(AccountEntry, usize)> {
        let mut sz = 0;
        Ok((
            AccountEntry {
                accountID: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                balance: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                seqNum: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                numSubEntries: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                inflationDest: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                flags: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                homeDomain: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                thresholds: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                signers: {
                    let (v, fsz) = xdr_codec::unpack_flex(input, Some(20i64 as usize))?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for AccountFlags {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(AccountFlags, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == AccountFlags::AUTH_REQUIRED_FLAG as i32 => {
                        AccountFlags::AUTH_REQUIRED_FLAG
                    }
                    x if x == AccountFlags::AUTH_REVOCABLE_FLAG as i32 => {
                        AccountFlags::AUTH_REVOCABLE_FLAG
                    }
                    x if x == AccountFlags::AUTH_IMMUTABLE_FLAG as i32 => {
                        AccountFlags::AUTH_IMMUTABLE_FLAG
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for Asset {
    fn unpack(input: &mut In) -> xdr_codec::Result<(Asset, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => Asset::ASSET_TYPE_NATIVE,
                x if x == (1i32 as i32) => Asset::ASSET_TYPE_CREDIT_ALPHANUM4({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (2i32 as i32) => Asset::ASSET_TYPE_CREDIT_ALPHANUM12({
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

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for AssetAlphaNum12 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(AssetAlphaNum12, usize)> {
        let mut sz = 0;
        Ok((
            AssetAlphaNum12 {
                assetCode: {
                    let (v, fsz) = {
                        let mut buf: [u8; 12i64 as usize] = unsafe { ::std::mem::uninitialized() };
                        let sz =
                            xdr_codec::unpack_opaque_array(input, &mut buf[..], 12i64 as usize)?;
                        (buf, sz)
                    };
                    sz += fsz;
                    v
                },
                issuer: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for AssetAlphaNum4 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(AssetAlphaNum4, usize)> {
        let mut sz = 0;
        Ok((
            AssetAlphaNum4 {
                assetCode: {
                    let (v, fsz) = {
                        let mut buf: [u8; 4i64 as usize] = unsafe { ::std::mem::uninitialized() };
                        let sz =
                            xdr_codec::unpack_opaque_array(input, &mut buf[..], 4i64 as usize)?;
                        (buf, sz)
                    };
                    sz += fsz;
                    v
                },
                issuer: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for AssetType {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(AssetType, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == AssetType::ASSET_TYPE_NATIVE as i32 => AssetType::ASSET_TYPE_NATIVE,
                    x if x == AssetType::ASSET_TYPE_CREDIT_ALPHANUM4 as i32 => {
                        AssetType::ASSET_TYPE_CREDIT_ALPHANUM4
                    }
                    x if x == AssetType::ASSET_TYPE_CREDIT_ALPHANUM12 as i32 => {
                        AssetType::ASSET_TYPE_CREDIT_ALPHANUM12
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for DataEntry {
    fn unpack(input: &mut In) -> xdr_codec::Result<(DataEntry, usize)> {
        let mut sz = 0;
        Ok((
            DataEntry {
                accountID: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
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

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for DataValue {
    fn unpack(input: &mut In) -> xdr_codec::Result<(DataValue, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (v, usz) = xdr_codec::unpack_opaque_flex(input, Some(64i64 as usize))?;
                sz = usz;
                DataValue(v)
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for EnvelopeType {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(EnvelopeType, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == EnvelopeType::ENVELOPE_TYPE_SCP as i32 => {
                        EnvelopeType::ENVELOPE_TYPE_SCP
                    }
                    x if x == EnvelopeType::ENVELOPE_TYPE_TX as i32 => {
                        EnvelopeType::ENVELOPE_TYPE_TX
                    }
                    x if x == EnvelopeType::ENVELOPE_TYPE_AUTH as i32 => {
                        EnvelopeType::ENVELOPE_TYPE_AUTH
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for LedgerEntry {
    fn unpack(input: &mut In) -> xdr_codec::Result<(LedgerEntry, usize)> {
        let mut sz = 0;
        Ok((
            LedgerEntry {
                lastModifiedLedgerSeq: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                data: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for LedgerEntryData {
    fn unpack(input: &mut In) -> xdr_codec::Result<(LedgerEntryData, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => LedgerEntryData::ACCOUNT({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (1i32 as i32) => LedgerEntryData::TRUSTLINE({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (2i32 as i32) => LedgerEntryData::OFFER({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (3i32 as i32) => LedgerEntryData::DATA({
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

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for LedgerEntryType {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(LedgerEntryType, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == LedgerEntryType::ACCOUNT as i32 => LedgerEntryType::ACCOUNT,
                    x if x == LedgerEntryType::TRUSTLINE as i32 => LedgerEntryType::TRUSTLINE,
                    x if x == LedgerEntryType::OFFER as i32 => LedgerEntryType::OFFER,
                    x if x == LedgerEntryType::DATA as i32 => LedgerEntryType::DATA,
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for Liabilities {
    fn unpack(input: &mut In) -> xdr_codec::Result<(Liabilities, usize)> {
        let mut sz = 0;
        Ok((
            Liabilities {
                buying: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                selling: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for OfferEntry {
    fn unpack(input: &mut In) -> xdr_codec::Result<(OfferEntry, usize)> {
        let mut sz = 0;
        Ok((
            OfferEntry {
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
                flags: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for OfferEntryFlags {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(OfferEntryFlags, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == OfferEntryFlags::PASSIVE_FLAG as i32 => OfferEntryFlags::PASSIVE_FLAG,
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for Price {
    fn unpack(input: &mut In) -> xdr_codec::Result<(Price, usize)> {
        let mut sz = 0;
        Ok((
            Price {
                n: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                d: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for Signer {
    fn unpack(input: &mut In) -> xdr_codec::Result<(Signer, usize)> {
        let mut sz = 0;
        Ok((
            Signer {
                key: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                weight: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for ThresholdIndexes {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(ThresholdIndexes, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == ThresholdIndexes::THRESHOLD_MASTER_WEIGHT as i32 => {
                        ThresholdIndexes::THRESHOLD_MASTER_WEIGHT
                    }
                    x if x == ThresholdIndexes::THRESHOLD_LOW as i32 => {
                        ThresholdIndexes::THRESHOLD_LOW
                    }
                    x if x == ThresholdIndexes::THRESHOLD_MED as i32 => {
                        ThresholdIndexes::THRESHOLD_MED
                    }
                    x if x == ThresholdIndexes::THRESHOLD_HIGH as i32 => {
                        ThresholdIndexes::THRESHOLD_HIGH
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for Thresholds {
    fn unpack(input: &mut In) -> xdr_codec::Result<(Thresholds, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (v, usz) = {
                    let mut buf: [u8; 4i64 as usize] = unsafe { ::std::mem::uninitialized() };
                    let sz = xdr_codec::unpack_opaque_array(input, &mut buf[..], 4i64 as usize)?;
                    (buf, sz)
                };
                sz = usz;
                Thresholds(v)
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for TrustLineEntry {
    fn unpack(input: &mut In) -> xdr_codec::Result<(TrustLineEntry, usize)> {
        let mut sz = 0;
        Ok((
            TrustLineEntry {
                accountID: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                asset: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                balance: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                limit: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                flags: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for TrustLineFlags {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(TrustLineFlags, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == TrustLineFlags::AUTHORIZED_FLAG as i32 => {
                        TrustLineFlags::AUTHORIZED_FLAG
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for string32 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(string32, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (v, usz) = xdr_codec::unpack_string(input, Some(32i64 as usize))?;
                sz = usz;
                string32(v)
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for string64 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(string64, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (v, usz) = xdr_codec::unpack_string(input, Some(64i64 as usize))?;
                sz = usz;
                string64(v)
            },
            sz,
        ))
    }
}
