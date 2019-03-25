// Copyright 2015 Stellar Development Foundation and contributors. Licensed
// under the Apache License, Version 2.0. See the COPYING file at the root
// of this distribution or at http://www.apache.org/licenses/LICENSE-2.0

//%#include "xdr/Stellar-types.h"

//namespace stellar
//{

typedef opaque Value<>;

struct SCPBallot
{
    uint32 counter; // n
    Value value;    // x
};

enum SCPStatementType
{
    SCP_ST_PREPARE = 0,
    SCP_ST_CONFIRM = 1,
    SCP_ST_EXTERNALIZE = 2,
    SCP_ST_NOMINATE = 3
};

struct SCPNomination
{
    Hash quorumSetHash; // D
    Value votes<>;      // X
    Value accepted<>;   // Y
};

struct SCPStatementPrepare // compability struct
{
    Hash quorumSetHash;       // D
    SCPBallot ballot;         // b
    SCPBallot* prepared;      // p
    SCPBallot* preparedPrime; // p'
    uint32 nC;                // c.n
    uint32 nH;                // h.n
};

struct SCPStatementConfirm // compability struct
{
    SCPBallot ballot;   // b
    uint32 nPrepared;   // p.n
    uint32 nCommit;     // c.n
    uint32 nH;          // h.n
    Hash quorumSetHash; // D
};

struct SCPStatementExternalize // compability struct
{
    SCPBallot commit;         // c
    uint32 nH;                // h.n
    Hash commitQuorumSetHash; // D used before EXTERNALIZE
};

struct SCPStatementPledges // compability struct
{
    SCPBallot commit;         // c
    uint32 nH;                // h.n
    Hash commitQuorumSetHash; // D used before EXTERNALIZE
};

struct ScpStatementPledges // compability struct
{
  SCPStatementType type;
  SCPStatementPrepare prepare;
  SCPStatementConfirm confirm; 
  SCPStatementExternalize externalize;
  SCPNomination mominate;
};

struct SCPStatement // compability struct
{
    NodeID nodeID;    // v
    uint64 slotIndex; // i
    ScpStatementPledges pledges;
};

union ScpStatementPledges switch (SCPStatementType type)
{
    case SCP_ST_PREPARE:
        SCPStatementPrepare prepare;
    case SCP_ST_CONFIRM:
        SCPStatementConfirm confirm;
    case SCP_ST_EXTERNALIZE:
        SCPStatementExternalize externalize;
    case SCP_ST_NOMINATE:
        SCPNomination nominate;
};

struct SCPEnvelope
{
    SCPStatement statement;
    Signature signature;
};

// supports things like: A,B,C,(D,E,F),(G,H,(I,J,K,L))
// only allows 2 levels of nesting
struct SCPQuorumSet
{
    uint32 threshold;
    PublicKey validators<>;
    SCPQuorumSet innerSets<>;
};
//}
