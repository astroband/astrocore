CREATE TABLE ledgerheaders (
  ledgerhash      CHARACTER(64) PRIMARY KEY,
  prevhash        CHARACTER(64) NOT NULL,
  bucketlisthash  CHARACTER(64) NOT NULL,
  ledgerseq       INT UNIQUE CHECK (ledgerseq >= 0),
  closetime       BIGINT NOT NULL CHECK (closetime >= 0),
  data            TEXT NOT NULL
);
CREATE INDEX ledgersbyseq ON ledgerheaders ( ledgerseq );

CREATE TABLE accounts
(
  accountid       VARCHAR(56)  PRIMARY KEY,
  balance         BIGINT       NOT NULL CHECK (balance >= 0),
  seqnum          BIGINT       NOT NULL,
  numsubentries   INT          NOT NULL CHECK (numsubentries >= 0),
  inflationdest   VARCHAR(56),
  homedomain      VARCHAR(32)  NOT NULL,
  thresholds      TEXT         NOT NULL,
  flags           INT          NOT NULL,
  lastmodified    INT          NOT NULL
);
CREATE INDEX accountbalances ON accounts (balance) WHERE balance >= 1000000000;

CREATE TABLE signers
(
  accountid       VARCHAR(56) NOT NULL,
  publickey       VARCHAR(56) NOT NULL,
  weight          INT         NOT NULL,
  PRIMARY KEY (accountid, publickey)
);
CREATE INDEX signersaccount ON signers (accountid);

CREATE TABLE offers
(
  sellerid         VARCHAR(56)  NOT NULL,
  offerid          BIGINT       NOT NULL CHECK (offerid >= 0),
  sellingassettype INT          NOT NULL,
  sellingassetcode VARCHAR(12),
  sellingissuer    VARCHAR(56),
  buyingassettype  INT          NOT NULL,
  buyingassetcode  VARCHAR(12),
  buyingissuer     VARCHAR(56),
  amount           BIGINT           NOT NULL CHECK (amount >= 0),
  pricen           INT              NOT NULL,
  priced           INT              NOT NULL,
  price            DOUBLE PRECISION NOT NULL,
  flags            INT              NOT NULL,
  lastmodified     INT              NOT NULL,
  PRIMARY KEY      (offerid)
);
CREATE INDEX sellingissuerindex ON offers (sellingissuer);
CREATE INDEX buyingissuerindex ON offers (buyingissuer);
CREATE INDEX priceindex ON offers (price);

CREATE TABLE trustlines
(
  accountid    VARCHAR(56)     NOT NULL,
  assettype    INT             NOT NULL,
  issuer       VARCHAR(56)     NOT NULL,
  assetcode    VARCHAR(12)     NOT NULL,
  tlimit       BIGINT          NOT NULL CHECK (tlimit > 0),
  balance      BIGINT          NOT NULL CHECK (balance >= 0),
  flags        INT             NOT NULL,
  lastmodified INT             NOT NULL,
  PRIMARY KEY  (accountid, issuer, assetcode)
);

CREATE TABLE accountdata
(
  accountid    VARCHAR(56)  NOT NULL,
  dataname     VARCHAR(64)  NOT NULL,
  datavalue    VARCHAR(112) NOT NULL,
  lastmodified INT          NOT NULL,
  PRIMARY KEY  (accountid, dataname)
);

CREATE TABLE txhistory (
  txid        CHARACTER(64) NOT NULL,
  ledgerseq   INT NOT NULL CHECK (ledgerseq >= 0),
  txindex     INT NOT NULL,
  txbody      TEXT NOT NULL,
  txresult    TEXT NOT NULL,
  txmeta      TEXT NOT NULL,
  PRIMARY KEY (ledgerseq, txindex)
);
CREATE INDEX histbyseq ON txhistory (ledgerseq);

CREATE TABLE txfeehistory (
  txid        CHARACTER(64) NOT NULL,
  ledgerseq   INT NOT NULL CHECK (ledgerseq >= 0),
  txindex     INT NOT NULL,
  txchanges   TEXT NOT NULL,
  PRIMARY KEY (ledgerseq, txindex)
);
CREATE INDEX histfeebyseq ON txfeehistory (ledgerseq);

CREATE TABLE scphistory (
  nodeid      CHARACTER(56) NOT NULL,
  ledgerseq   INT NOT NULL CHECK (ledgerseq >= 0),
  envelope    TEXT NOT NULL
);
CREATE INDEX scpenvsbyseq ON scphistory(ledgerseq);

CREATE TABLE scpquorums (
  qsethash      CHARACTER(64) NOT NULL,
  lastledgerseq INT NOT NULL CHECK (lastledgerseq >= 0),
  qset          TEXT NOT NULL,
  PRIMARY KEY (qsethash)
);
CREATE INDEX scpquorumsbyseq ON scpquorums(lastledgerseq);

CREATE TABLE quoruminfo (
  nodeid      CHARACTER(56) NOT NULL,
  qsethash    CHARACTER(64) NOT NULL,
  PRIMARY KEY (nodeid)
);

CREATE TABLE storestate (
  statename   CHARACTER(32) PRIMARY KEY,
  state       TEXT
);

CREATE TABLE peers (
  ip            VARCHAR(15) NOT NULL,
  port          INT DEFAULT 0 CHECK (port > 0 AND port <= 65535) NOT NULL,
  nextattempt   TIMESTAMP NOT NULL,
  numfailures   INT DEFAULT 0 CHECK (numfailures >= 0) NOT NULL,
  flags         INT NOT NULL,
  PRIMARY KEY (ip, port)
);

CREATE TABLE upgradehistory (
  ledgerseq    INT NOT NULL CHECK (ledgerseq >= 0), 
  upgradeindex INT NOT NULL, 
  upgrade      TEXT NOT NULL, 
  changes      TEXT NOT NULL, 
  PRIMARY KEY (ledgerseq, upgradeindex)
);
CREATE INDEX upgradehistbyseq ON upgradehistory (ledgerseq);
