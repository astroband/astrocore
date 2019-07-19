table! {
    accountdata (accountid, dataname) {
        accountid -> Text,
        dataname -> Text,
        datavalue -> Text,
        lastmodified -> Integer,
    }
}

table! {
    accounts (accountid) {
        accountid -> Nullable<Text>,
        balance -> BigInt,
        seqnum -> BigInt,
        numsubentries -> Integer,
        inflationdest -> Nullable<Text>,
        homedomain -> Text,
        thresholds -> Text,
        flags -> Integer,
        lastmodified -> Integer,
        buyingliabilities -> Nullable<BigInt>,
        sellingliabilities -> Nullable<BigInt>,
        signers -> Nullable<Text>,
    }
}

table! {
    ban (nodeid) {
        nodeid -> Text,
    }
}

table! {
    ledgerheaders (ledgerhash) {
        ledgerhash -> Nullable<Text>,
        prevhash -> Text,
        bucketlisthash -> Text,
        ledgerseq -> Nullable<Integer>,
        closetime -> BigInt,
        data -> Text,
    }
}

table! {
    offers (offerid) {
        sellerid -> Text,
        offerid -> BigInt,
        sellingasset -> Text,
        buyingasset -> Text,
        amount -> BigInt,
        pricen -> Integer,
        priced -> Integer,
        price -> Double,
        flags -> Integer,
        lastmodified -> Integer,
    }
}

table! {
    peers (ip, port) {
        ip -> Text,
        port -> Integer,
        nextattempt -> Timestamp,
        numfailures -> Integer,
        #[sql_name = "type"]
        type_ -> Integer,
    }
}

table! {
    publishqueue (ledger) {
        ledger -> Nullable<Integer>,
        state -> Nullable<Text>,
    }
}

table! {
    pubsub (resid) {
        resid -> Nullable<Text>,
        lastread -> Nullable<Integer>,
    }
}

table! {
    quoruminfo (nodeid) {
        nodeid -> Text,
        qsethash -> Text,
    }
}

table! {
    scphistory (nodeid) {
        nodeid -> Text,
        ledgerseq -> Integer,
        envelope -> Text,
    }
}

table! {
    scpquorums (qsethash) {
        qsethash -> Text,
        lastledgerseq -> Integer,
        qset -> Text,
    }
}

table! {
    storestate (statename) {
        statename -> Nullable<Text>,
        state -> Nullable<Text>,
    }
}

table! {
    trustlines (accountid, issuer, assetcode) {
        accountid -> Text,
        assettype -> Integer,
        issuer -> Text,
        assetcode -> Text,
        tlimit -> BigInt,
        balance -> BigInt,
        flags -> Integer,
        lastmodified -> Integer,
        buyingliabilities -> Nullable<BigInt>,
        sellingliabilities -> Nullable<BigInt>,
    }
}

table! {
    txfeehistory (ledgerseq, txindex) {
        txid -> Text,
        ledgerseq -> Integer,
        txindex -> Integer,
        txchanges -> Text,
    }
}

table! {
    txhistory (ledgerseq, txindex) {
        txid -> Text,
        ledgerseq -> Integer,
        txindex -> Integer,
        txbody -> Text,
        txresult -> Text,
        txmeta -> Text,
    }
}

table! {
    upgradehistory (ledgerseq, upgradeindex) {
        ledgerseq -> Integer,
        upgradeindex -> Integer,
        upgrade -> Text,
        changes -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    accountdata,
    accounts,
    ban,
    ledgerheaders,
    offers,
    peers,
    publishqueue,
    pubsub,
    quoruminfo,
    scphistory,
    scpquorums,
    storestate,
    trustlines,
    txfeehistory,
    txhistory,
    upgradehistory,
);
