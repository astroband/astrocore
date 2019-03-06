// GENERATED CODE
//
// Generated from src/xdr/Stellar-SCP.x by xdrgen.
//
// DO NOT EDIT

pub struct SCPBallot {
    pub counter: uint32,
    pub value: Value,
}

pub struct SCPEnvelope {
    pub statement: SCPStatement,
    pub signature: Signature,
}

pub struct SCPNomination {
    pub quorumSetHash: Hash,
    pub votes: Vec<Value>,
    pub accepted: Vec<Value>,
}

pub struct SCPQuorumSet {
    pub threshold: uint32,
    pub validators: Vec<PublicKey>,
    pub innerSets: Vec<SCPQuorumSet>,
}

pub struct SCPStatement {
    pub nodeID: NodeID,
    pub slotIndex: uint64,
    pub pledges: ScpStatementPledges,
}

pub struct SCPStatementConfirm {
    pub ballot: SCPBallot,
    pub nPrepared: uint32,
    pub nCommit: uint32,
    pub nH: uint32,
    pub quorumSetHash: Hash,
}

pub struct SCPStatementExternalize {
    pub commit: SCPBallot,
    pub nH: uint32,
    pub commitQuorumSetHash: Hash,
}

pub struct SCPStatementPledges {
    pub commit: SCPBallot,
    pub nH: uint32,
    pub commitQuorumSetHash: Hash,
}

pub struct SCPStatementPrepare {
    pub quorumSetHash: Hash,
    pub ballot: SCPBallot,
    pub prepared: Option<Box<SCPBallot>>,
    pub preparedPrime: Option<Box<SCPBallot>>,
    pub nC: uint32,
    pub nH: uint32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SCPStatementType {
    SCP_ST_PREPARE = 0isize,
    SCP_ST_CONFIRM = 1isize,
    SCP_ST_EXTERNALIZE = 2isize,
    SCP_ST_NOMINATE = 3isize,
}

pub enum ScpStatementPledges {
    SCP_ST_PREPARE(SCPStatementPrepare),
    SCP_ST_CONFIRM(SCPStatementConfirm),
    SCP_ST_EXTERNALIZE(SCPStatementExternalize),
    SCP_ST_NOMINATE(SCPNomination),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Value(pub Vec<u8>);

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for SCPBallot {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.counter.pack(out)? + self.value.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for SCPEnvelope {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.statement.pack(out)? + self.signature.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for SCPNomination {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.quorumSetHash.pack(out)?
            + xdr_codec::pack_flex(&self.votes, None, out)?
            + xdr_codec::pack_flex(&self.accepted, None, out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for SCPQuorumSet {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.threshold.pack(out)?
            + xdr_codec::pack_flex(&self.validators, None, out)?
            + xdr_codec::pack_flex(&self.innerSets, None, out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for SCPStatement {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.nodeID.pack(out)? + self.slotIndex.pack(out)? + self.pledges.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for SCPStatementConfirm {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.ballot.pack(out)?
            + self.nPrepared.pack(out)?
            + self.nCommit.pack(out)?
            + self.nH.pack(out)?
            + self.quorumSetHash.pack(out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for SCPStatementExternalize {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.commit.pack(out)? + self.nH.pack(out)? + self.commitQuorumSetHash.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for SCPStatementPledges {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.commit.pack(out)? + self.nH.pack(out)? + self.commitQuorumSetHash.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for SCPStatementPrepare {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.quorumSetHash.pack(out)?
            + self.ballot.pack(out)?
            + self.prepared.pack(out)?
            + self.preparedPrime.pack(out)?
            + self.nC.pack(out)?
            + self.nH.pack(out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for SCPStatementType {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for ScpStatementPledges {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &ScpStatementPledges::SCP_ST_PREPARE(ref val) => {
                (SCPStatementType::SCP_ST_PREPARE as i32).pack(out)? + val.pack(out)?
            }
            &ScpStatementPledges::SCP_ST_CONFIRM(ref val) => {
                (SCPStatementType::SCP_ST_CONFIRM as i32).pack(out)? + val.pack(out)?
            }
            &ScpStatementPledges::SCP_ST_EXTERNALIZE(ref val) => {
                (SCPStatementType::SCP_ST_EXTERNALIZE as i32).pack(out)? + val.pack(out)?
            }
            &ScpStatementPledges::SCP_ST_NOMINATE(ref val) => {
                (SCPStatementType::SCP_ST_NOMINATE as i32).pack(out)? + val.pack(out)?
            }
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for Value {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(xdr_codec::pack_opaque_flex(&self.0, None, out)?)
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for SCPBallot {
    fn unpack(input: &mut In) -> xdr_codec::Result<(SCPBallot, usize)> {
        let mut sz = 0;
        Ok((
            SCPBallot {
                counter: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                value: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for SCPEnvelope {
    fn unpack(input: &mut In) -> xdr_codec::Result<(SCPEnvelope, usize)> {
        let mut sz = 0;
        Ok((
            SCPEnvelope {
                statement: {
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

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for SCPNomination {
    fn unpack(input: &mut In) -> xdr_codec::Result<(SCPNomination, usize)> {
        let mut sz = 0;
        Ok((
            SCPNomination {
                quorumSetHash: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                votes: {
                    let (v, fsz) = xdr_codec::unpack_flex(input, None)?;
                    sz += fsz;
                    v
                },
                accepted: {
                    let (v, fsz) = xdr_codec::unpack_flex(input, None)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for SCPQuorumSet {
    fn unpack(input: &mut In) -> xdr_codec::Result<(SCPQuorumSet, usize)> {
        let mut sz = 0;
        Ok((
            SCPQuorumSet {
                threshold: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                validators: {
                    let (v, fsz) = xdr_codec::unpack_flex(input, None)?;
                    sz += fsz;
                    v
                },
                innerSets: {
                    let (v, fsz) = xdr_codec::unpack_flex(input, None)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for SCPStatement {
    fn unpack(input: &mut In) -> xdr_codec::Result<(SCPStatement, usize)> {
        let mut sz = 0;
        Ok((
            SCPStatement {
                nodeID: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                slotIndex: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                pledges: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for SCPStatementConfirm {
    fn unpack(input: &mut In) -> xdr_codec::Result<(SCPStatementConfirm, usize)> {
        let mut sz = 0;
        Ok((
            SCPStatementConfirm {
                ballot: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                nPrepared: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                nCommit: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                nH: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                quorumSetHash: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for SCPStatementExternalize {
    fn unpack(input: &mut In) -> xdr_codec::Result<(SCPStatementExternalize, usize)> {
        let mut sz = 0;
        Ok((
            SCPStatementExternalize {
                commit: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                nH: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                commitQuorumSetHash: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for SCPStatementPledges {
    fn unpack(input: &mut In) -> xdr_codec::Result<(SCPStatementPledges, usize)> {
        let mut sz = 0;
        Ok((
            SCPStatementPledges {
                commit: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                nH: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                commitQuorumSetHash: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for SCPStatementPrepare {
    fn unpack(input: &mut In) -> xdr_codec::Result<(SCPStatementPrepare, usize)> {
        let mut sz = 0;
        Ok((
            SCPStatementPrepare {
                quorumSetHash: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                ballot: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                prepared: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                preparedPrime: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                nC: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                nH: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for SCPStatementType {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(SCPStatementType, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == SCPStatementType::SCP_ST_PREPARE as i32 => {
                        SCPStatementType::SCP_ST_PREPARE
                    }
                    x if x == SCPStatementType::SCP_ST_CONFIRM as i32 => {
                        SCPStatementType::SCP_ST_CONFIRM
                    }
                    x if x == SCPStatementType::SCP_ST_EXTERNALIZE as i32 => {
                        SCPStatementType::SCP_ST_EXTERNALIZE
                    }
                    x if x == SCPStatementType::SCP_ST_NOMINATE as i32 => {
                        SCPStatementType::SCP_ST_NOMINATE
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for ScpStatementPledges {
    fn unpack(input: &mut In) -> xdr_codec::Result<(ScpStatementPledges, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => ScpStatementPledges::SCP_ST_PREPARE({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (1i32 as i32) => ScpStatementPledges::SCP_ST_CONFIRM({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (2i32 as i32) => ScpStatementPledges::SCP_ST_EXTERNALIZE({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (3i32 as i32) => ScpStatementPledges::SCP_ST_NOMINATE({
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

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for Value {
    fn unpack(input: &mut In) -> xdr_codec::Result<(Value, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (v, usz) = xdr_codec::unpack_opaque_flex(input, None)?;
                sz = usz;
                Value(v)
            },
            sz,
        ))
    }
}
