// GENERATED CODE
//
// Generated from src/xdr/Stellar-ledger.x by xdrgen.
//
// DO NOT EDIT

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BucketEntry {
    LIVEENTRY(LedgerEntry),
    DEADENTRY(LedgerKey),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BucketEntryType {
    LIVEENTRY = 0isize,
    DEADENTRY = 1isize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LedgerEntryChange {
    LEDGER_ENTRY_CREATED(LedgerEntry),
    LEDGER_ENTRY_UPDATED(LedgerEntry),
    LEDGER_ENTRY_REMOVED(LedgerKey),
    LEDGER_ENTRY_STATE(LedgerEntry),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum LedgerEntryChangeType {
    LEDGER_ENTRY_CREATED = 0isize,
    LEDGER_ENTRY_UPDATED = 1isize,
    LEDGER_ENTRY_REMOVED = 2isize,
    LEDGER_ENTRY_STATE = 3isize,
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct LedgerEntryChanges(pub Vec<LedgerEntryChange>);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum LedgerEntryKeyType {
    LEDGER_ENTRY_KEY_TYPE_ACCOUNT = 0isize,
    LEDGER_ENTRY_KEY_TYPE_TRUSTLINE = 1isize,
    LEDGER_ENTRY_KEY_TYPE_OFFER = 2isize,
    LEDGER_ENTRY_KEY_TYPE_DATA = 3isize,
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
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
    pub skipList: [Hash; 4i64 as usize],
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct LedgerHeaderHistoryEntry {
    pub hash: Hash,
    pub header: LedgerHeader,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LedgerKey {
    LEDGER_ENTRY_KEY_TYPE_ACCOUNT(LedgerKeyAccount),
    LEDGER_ENTRY_KEY_TYPE_TRUSTLINE(LedgerKeyTrustLine),
    LEDGER_ENTRY_KEY_TYPE_OFFER(LedgerKeyTrustOffer),
    LEDGER_ENTRY_KEY_TYPE_DATA(LedgerKeyTrustData),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LedgerKeyAccount {
    pub accountID: AccountID,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LedgerKeyTrustData {
    pub accountID: AccountID,
    pub dataName: string64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LedgerKeyTrustLine {
    pub accountID: AccountID,
    pub asset: Asset,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LedgerKeyTrustOffer {
    pub sellerID: AccountID,
    pub offerID: uint64,
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct LedgerSCPMessages {
    pub ledgerSeq: uint32,
    pub messages: Vec<SCPEnvelope>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LedgerUpgrade {
    LEDGER_UPGRADE_VERSION(uint32),
    LEDGER_UPGRADE_BASE_FEE(uint32),
    LEDGER_UPGRADE_MAX_TX_SET_SIZE(uint32),
    LEDGER_UPGRADE_BASE_RESERVE(uint32),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum LedgerUpgradeType {
    LEDGER_UPGRADE_VERSION = 1isize,
    LEDGER_UPGRADE_BASE_FEE = 2isize,
    LEDGER_UPGRADE_MAX_TX_SET_SIZE = 3isize,
    LEDGER_UPGRADE_BASE_RESERVE = 4isize,
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct OperationMeta {
    pub changes: LedgerEntryChanges,
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct SCPHistoryEntryV0 {
    pub quorumSets: Vec<SCPQuorumSet>,
    pub ledgerMessages: LedgerSCPMessages,
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct StellarValue {
    pub txSetHash: Hash,
    pub closeTime: TimePoint,
    pub upgrades: Vec<UpgradeType>,
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct TransactionHistoryEntry {
    pub ledgerSeq: uint32,
    pub txSet: TransactionSet,
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct TransactionHistoryResultEntry {
    pub ledgerSeq: uint32,
    pub txResultSet: TransactionResultSet,
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct TransactionMetaV1 {
    pub txChanges: LedgerEntryChanges,
    pub operations: Vec<OperationMeta>,
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct TransactionResultPair {
    pub transactionHash: Hash,
    pub result: TransactionResult,
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct TransactionResultSet {
    pub results: Vec<TransactionResultPair>,
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct TransactionSet {
    pub previousLedgerHash: Hash,
    pub txs: Vec<TransactionEnvelope>,
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct UpgradeType(pub Vec<u8>);

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for BucketEntry {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &BucketEntry::LIVEENTRY(ref val) => {
                (BucketEntryType::LIVEENTRY as i32).pack(out)? + val.pack(out)?
            }
            &BucketEntry::DEADENTRY(ref val) => {
                (BucketEntryType::DEADENTRY as i32).pack(out)? + val.pack(out)?
            }
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for BucketEntryType {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for LedgerEntryChange {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &LedgerEntryChange::LEDGER_ENTRY_CREATED(ref val) => {
                (LedgerEntryChangeType::LEDGER_ENTRY_CREATED as i32).pack(out)? + val.pack(out)?
            }
            &LedgerEntryChange::LEDGER_ENTRY_UPDATED(ref val) => {
                (LedgerEntryChangeType::LEDGER_ENTRY_UPDATED as i32).pack(out)? + val.pack(out)?
            }
            &LedgerEntryChange::LEDGER_ENTRY_REMOVED(ref val) => {
                (LedgerEntryChangeType::LEDGER_ENTRY_REMOVED as i32).pack(out)? + val.pack(out)?
            }
            &LedgerEntryChange::LEDGER_ENTRY_STATE(ref val) => {
                (LedgerEntryChangeType::LEDGER_ENTRY_STATE as i32).pack(out)? + val.pack(out)?
            }
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for LedgerEntryChangeType {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for LedgerEntryChanges {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(xdr_codec::pack_flex(&self.0, None, out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for LedgerEntryKeyType {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for LedgerHeader {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.ledgerVersion.pack(out)?
            + self.previousLedgerHash.pack(out)?
            + self.scpValue.pack(out)?
            + self.txSetResultHash.pack(out)?
            + self.bucketListHash.pack(out)?
            + self.ledgerSeq.pack(out)?
            + self.totalCoins.pack(out)?
            + self.feePool.pack(out)?
            + self.inflationSeq.pack(out)?
            + self.idPool.pack(out)?
            + self.baseFee.pack(out)?
            + self.baseReserve.pack(out)?
            + self.maxTxSetSize.pack(out)?
            + xdr_codec::pack_array(&self.skipList[..], self.skipList.len(), out, None)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for LedgerHeaderHistoryEntry {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.hash.pack(out)? + self.header.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for LedgerKey {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &LedgerKey::LEDGER_ENTRY_KEY_TYPE_ACCOUNT(ref val) => {
                (LedgerEntryKeyType::LEDGER_ENTRY_KEY_TYPE_ACCOUNT as i32).pack(out)?
                    + val.pack(out)?
            }
            &LedgerKey::LEDGER_ENTRY_KEY_TYPE_TRUSTLINE(ref val) => {
                (LedgerEntryKeyType::LEDGER_ENTRY_KEY_TYPE_TRUSTLINE as i32).pack(out)?
                    + val.pack(out)?
            }
            &LedgerKey::LEDGER_ENTRY_KEY_TYPE_OFFER(ref val) => {
                (LedgerEntryKeyType::LEDGER_ENTRY_KEY_TYPE_OFFER as i32).pack(out)?
                    + val.pack(out)?
            }
            &LedgerKey::LEDGER_ENTRY_KEY_TYPE_DATA(ref val) => {
                (LedgerEntryKeyType::LEDGER_ENTRY_KEY_TYPE_DATA as i32).pack(out)?
                    + val.pack(out)?
            }
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for LedgerKeyAccount {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.accountID.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for LedgerKeyTrustData {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.accountID.pack(out)? + self.dataName.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for LedgerKeyTrustLine {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.accountID.pack(out)? + self.asset.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for LedgerKeyTrustOffer {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.sellerID.pack(out)? + self.offerID.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for LedgerSCPMessages {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.ledgerSeq.pack(out)? + xdr_codec::pack_flex(&self.messages, None, out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for LedgerUpgrade {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &LedgerUpgrade::LEDGER_UPGRADE_VERSION(ref val) => {
                (LedgerUpgradeType::LEDGER_UPGRADE_VERSION as i32).pack(out)? + val.pack(out)?
            }
            &LedgerUpgrade::LEDGER_UPGRADE_BASE_FEE(ref val) => {
                (LedgerUpgradeType::LEDGER_UPGRADE_BASE_FEE as i32).pack(out)? + val.pack(out)?
            }
            &LedgerUpgrade::LEDGER_UPGRADE_MAX_TX_SET_SIZE(ref val) => {
                (LedgerUpgradeType::LEDGER_UPGRADE_MAX_TX_SET_SIZE as i32).pack(out)?
                    + val.pack(out)?
            }
            &LedgerUpgrade::LEDGER_UPGRADE_BASE_RESERVE(ref val) => {
                (LedgerUpgradeType::LEDGER_UPGRADE_BASE_RESERVE as i32).pack(out)?
                    + val.pack(out)?
            }
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for LedgerUpgradeType {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for OperationMeta {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.changes.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for SCPHistoryEntryV0 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(xdr_codec::pack_flex(&self.quorumSets, None, out)? + self.ledgerMessages.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for StellarValue {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.txSetHash.pack(out)?
            + self.closeTime.pack(out)?
            + xdr_codec::pack_flex(&self.upgrades, Some(6i64 as usize), out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for TransactionHistoryEntry {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.ledgerSeq.pack(out)? + self.txSet.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for TransactionHistoryResultEntry {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.ledgerSeq.pack(out)? + self.txResultSet.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for TransactionMetaV1 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.txChanges.pack(out)? + xdr_codec::pack_flex(&self.operations, None, out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for TransactionResultPair {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.transactionHash.pack(out)? + self.result.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for TransactionResultSet {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(xdr_codec::pack_flex(&self.results, None, out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for TransactionSet {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.previousLedgerHash.pack(out)? + xdr_codec::pack_flex(&self.txs, None, out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for UpgradeType {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(xdr_codec::pack_opaque_flex(
            &self.0,
            Some(128i64 as usize),
            out,
        )?)
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for BucketEntry {
    fn unpack(input: &mut In) -> xdr_codec::Result<(BucketEntry, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => BucketEntry::LIVEENTRY({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (1i32 as i32) => BucketEntry::DEADENTRY({
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

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for BucketEntryType {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(BucketEntryType, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == BucketEntryType::LIVEENTRY as i32 => BucketEntryType::LIVEENTRY,
                    x if x == BucketEntryType::DEADENTRY as i32 => BucketEntryType::DEADENTRY,
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for LedgerEntryChange {
    fn unpack(input: &mut In) -> xdr_codec::Result<(LedgerEntryChange, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => LedgerEntryChange::LEDGER_ENTRY_CREATED({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (1i32 as i32) => LedgerEntryChange::LEDGER_ENTRY_UPDATED({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (2i32 as i32) => LedgerEntryChange::LEDGER_ENTRY_REMOVED({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (3i32 as i32) => LedgerEntryChange::LEDGER_ENTRY_STATE({
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

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for LedgerEntryChangeType {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(LedgerEntryChangeType, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == LedgerEntryChangeType::LEDGER_ENTRY_CREATED as i32 => {
                        LedgerEntryChangeType::LEDGER_ENTRY_CREATED
                    }
                    x if x == LedgerEntryChangeType::LEDGER_ENTRY_UPDATED as i32 => {
                        LedgerEntryChangeType::LEDGER_ENTRY_UPDATED
                    }
                    x if x == LedgerEntryChangeType::LEDGER_ENTRY_REMOVED as i32 => {
                        LedgerEntryChangeType::LEDGER_ENTRY_REMOVED
                    }
                    x if x == LedgerEntryChangeType::LEDGER_ENTRY_STATE as i32 => {
                        LedgerEntryChangeType::LEDGER_ENTRY_STATE
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for LedgerEntryChanges {
    fn unpack(input: &mut In) -> xdr_codec::Result<(LedgerEntryChanges, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (v, usz) = xdr_codec::unpack_flex(input, None)?;
                sz = usz;
                LedgerEntryChanges(v)
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for LedgerEntryKeyType {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(LedgerEntryKeyType, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == LedgerEntryKeyType::LEDGER_ENTRY_KEY_TYPE_ACCOUNT as i32 => {
                        LedgerEntryKeyType::LEDGER_ENTRY_KEY_TYPE_ACCOUNT
                    }
                    x if x == LedgerEntryKeyType::LEDGER_ENTRY_KEY_TYPE_TRUSTLINE as i32 => {
                        LedgerEntryKeyType::LEDGER_ENTRY_KEY_TYPE_TRUSTLINE
                    }
                    x if x == LedgerEntryKeyType::LEDGER_ENTRY_KEY_TYPE_OFFER as i32 => {
                        LedgerEntryKeyType::LEDGER_ENTRY_KEY_TYPE_OFFER
                    }
                    x if x == LedgerEntryKeyType::LEDGER_ENTRY_KEY_TYPE_DATA as i32 => {
                        LedgerEntryKeyType::LEDGER_ENTRY_KEY_TYPE_DATA
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for LedgerHeader {
    fn unpack(input: &mut In) -> xdr_codec::Result<(LedgerHeader, usize)> {
        let mut sz = 0;
        Ok((
            LedgerHeader {
                ledgerVersion: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                previousLedgerHash: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                scpValue: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                txSetResultHash: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                bucketListHash: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                ledgerSeq: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                totalCoins: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                feePool: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                inflationSeq: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                idPool: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                baseFee: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                baseReserve: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                maxTxSetSize: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                skipList: {
                    let (v, fsz) = {
                        #[inline]
                        fn uninit_ptr_setter<T>(p: &mut T, v: T) {
                            unsafe { ::std::ptr::write(p, v) }
                        }
                        #[inline]
                        fn uninit_ptr_dropper<T>(p: &mut T) {
                            unsafe { ::std::ptr::drop_in_place(p) }
                        }
                        let mut buf: [Hash; 4i64 as usize] = unsafe { ::std::mem::uninitialized() };
                        let res =
                            ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(|| {
                                xdr_codec::unpack_array_with(
                                    input,
                                    &mut buf[..],
                                    4i64 as usize,
                                    uninit_ptr_setter,
                                    uninit_ptr_dropper,
                                    None,
                                )
                            }));
                        let sz = match res {
                            Ok(Ok(sz)) => sz,
                            Ok(Err(err)) => {
                                ::std::mem::forget(buf);
                                return Err(err);
                            }
                            Err(panic) => {
                                ::std::mem::forget(buf);
                                ::std::panic::resume_unwind(panic);
                            }
                        };
                        (buf, sz)
                    };
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for LedgerHeaderHistoryEntry {
    fn unpack(input: &mut In) -> xdr_codec::Result<(LedgerHeaderHistoryEntry, usize)> {
        let mut sz = 0;
        Ok((
            LedgerHeaderHistoryEntry {
                hash: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                header: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for LedgerKey {
    fn unpack(input: &mut In) -> xdr_codec::Result<(LedgerKey, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => LedgerKey::LEDGER_ENTRY_KEY_TYPE_ACCOUNT({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (1i32 as i32) => LedgerKey::LEDGER_ENTRY_KEY_TYPE_TRUSTLINE({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (2i32 as i32) => LedgerKey::LEDGER_ENTRY_KEY_TYPE_OFFER({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (3i32 as i32) => LedgerKey::LEDGER_ENTRY_KEY_TYPE_DATA({
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

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for LedgerKeyAccount {
    fn unpack(input: &mut In) -> xdr_codec::Result<(LedgerKeyAccount, usize)> {
        let mut sz = 0;
        Ok((
            LedgerKeyAccount {
                accountID: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for LedgerKeyTrustData {
    fn unpack(input: &mut In) -> xdr_codec::Result<(LedgerKeyTrustData, usize)> {
        let mut sz = 0;
        Ok((
            LedgerKeyTrustData {
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
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for LedgerKeyTrustLine {
    fn unpack(input: &mut In) -> xdr_codec::Result<(LedgerKeyTrustLine, usize)> {
        let mut sz = 0;
        Ok((
            LedgerKeyTrustLine {
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
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for LedgerKeyTrustOffer {
    fn unpack(input: &mut In) -> xdr_codec::Result<(LedgerKeyTrustOffer, usize)> {
        let mut sz = 0;
        Ok((
            LedgerKeyTrustOffer {
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
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for LedgerSCPMessages {
    fn unpack(input: &mut In) -> xdr_codec::Result<(LedgerSCPMessages, usize)> {
        let mut sz = 0;
        Ok((
            LedgerSCPMessages {
                ledgerSeq: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                messages: {
                    let (v, fsz) = xdr_codec::unpack_flex(input, None)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for LedgerUpgrade {
    fn unpack(input: &mut In) -> xdr_codec::Result<(LedgerUpgrade, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (1i32 as i32) => LedgerUpgrade::LEDGER_UPGRADE_VERSION({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (2i32 as i32) => LedgerUpgrade::LEDGER_UPGRADE_BASE_FEE({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (3i32 as i32) => LedgerUpgrade::LEDGER_UPGRADE_MAX_TX_SET_SIZE({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (4i32 as i32) => LedgerUpgrade::LEDGER_UPGRADE_BASE_RESERVE({
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

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for LedgerUpgradeType {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(LedgerUpgradeType, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == LedgerUpgradeType::LEDGER_UPGRADE_VERSION as i32 => {
                        LedgerUpgradeType::LEDGER_UPGRADE_VERSION
                    }
                    x if x == LedgerUpgradeType::LEDGER_UPGRADE_BASE_FEE as i32 => {
                        LedgerUpgradeType::LEDGER_UPGRADE_BASE_FEE
                    }
                    x if x == LedgerUpgradeType::LEDGER_UPGRADE_MAX_TX_SET_SIZE as i32 => {
                        LedgerUpgradeType::LEDGER_UPGRADE_MAX_TX_SET_SIZE
                    }
                    x if x == LedgerUpgradeType::LEDGER_UPGRADE_BASE_RESERVE as i32 => {
                        LedgerUpgradeType::LEDGER_UPGRADE_BASE_RESERVE
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for OperationMeta {
    fn unpack(input: &mut In) -> xdr_codec::Result<(OperationMeta, usize)> {
        let mut sz = 0;
        Ok((
            OperationMeta {
                changes: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for SCPHistoryEntryV0 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(SCPHistoryEntryV0, usize)> {
        let mut sz = 0;
        Ok((
            SCPHistoryEntryV0 {
                quorumSets: {
                    let (v, fsz) = xdr_codec::unpack_flex(input, None)?;
                    sz += fsz;
                    v
                },
                ledgerMessages: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for StellarValue {
    fn unpack(input: &mut In) -> xdr_codec::Result<(StellarValue, usize)> {
        let mut sz = 0;
        Ok((
            StellarValue {
                txSetHash: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                closeTime: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                upgrades: {
                    let (v, fsz) = xdr_codec::unpack_flex(input, Some(6i64 as usize))?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for TransactionHistoryEntry {
    fn unpack(input: &mut In) -> xdr_codec::Result<(TransactionHistoryEntry, usize)> {
        let mut sz = 0;
        Ok((
            TransactionHistoryEntry {
                ledgerSeq: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                txSet: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for TransactionHistoryResultEntry {
    fn unpack(input: &mut In) -> xdr_codec::Result<(TransactionHistoryResultEntry, usize)> {
        let mut sz = 0;
        Ok((
            TransactionHistoryResultEntry {
                ledgerSeq: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                txResultSet: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for TransactionMetaV1 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(TransactionMetaV1, usize)> {
        let mut sz = 0;
        Ok((
            TransactionMetaV1 {
                txChanges: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                operations: {
                    let (v, fsz) = xdr_codec::unpack_flex(input, None)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for TransactionResultPair {
    fn unpack(input: &mut In) -> xdr_codec::Result<(TransactionResultPair, usize)> {
        let mut sz = 0;
        Ok((
            TransactionResultPair {
                transactionHash: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                result: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for TransactionResultSet {
    fn unpack(input: &mut In) -> xdr_codec::Result<(TransactionResultSet, usize)> {
        let mut sz = 0;
        Ok((
            TransactionResultSet {
                results: {
                    let (v, fsz) = xdr_codec::unpack_flex(input, None)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for TransactionSet {
    fn unpack(input: &mut In) -> xdr_codec::Result<(TransactionSet, usize)> {
        let mut sz = 0;
        Ok((
            TransactionSet {
                previousLedgerHash: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                txs: {
                    let (v, fsz) = xdr_codec::unpack_flex(input, None)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for UpgradeType {
    fn unpack(input: &mut In) -> xdr_codec::Result<(UpgradeType, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (v, usz) = xdr_codec::unpack_opaque_flex(input, Some(128i64 as usize))?;
                sz = usz;
                UpgradeType(v)
            },
            sz,
        ))
    }
}
