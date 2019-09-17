use super::{db_conn, schema::accounts};
use diesel::prelude::*;

// https://www.stellar.org/developers/guides/concepts/accounts.html

// TODO: We should take base reserve from ledger header
pub const BASE_RESERVE: i64 = 5000000;

#[derive(Identifiable, Queryable, Debug)]
#[primary_key(accountid)]
pub struct Account {
    /// The public key that was first used to create the account. You can replace the key used for signing the account’s transactions with a different public key, but the original account ID will always be used to identify the account.
    pub accountid: String,
    /// The number of lumens held by the account. The balance is denominated in 1/10,000,000th of a lumen, the smallest divisible unit of a lumen.
    pub balance: i64,
    /// The current transaction sequence number of the account. This number starts equal to the ledger number at which the account was created.
    pub seqnum: i64,
    /// Number of other entries the account owns. This number is used to calculate the account’s minimum balance.
    pub numsubentries: i32,
    /// (optional) Account designated to receive inflation. Every account with a balance of at least 100 XLM can vote to send inflation to a destination account.
    pub inflationdest: Option<String>,
    /// A domain name that can optionally be added to the account. Clients can look up a stellar.toml from this domain. This should be in the format of a fully qualified domain name such as example.com.
    /// The federation protocol can use the home domain to look up more details about a transaction’s memo or address details about an account. For more on federation, see the federation guide.
    pub homedomain: String,
    /// Operations have varying levels of access. This field specifies thresholds for low-, medium-, and high-access levels, as well as the weight of the master key. For more info, see multi-sig.
    pub thresholds: String,
    /// Currently there are three flags, used by issuers of assets.
    /// - Authorization required (0x1): Requires the issuing account to give other accounts permission before they can hold the issuing account’s credit.
    /// - Authorization revocable (0x2): Allows the issuing account to revoke its credit held by other accounts.
    /// - Authorization immutable (0x4): If this is set then none of the authorization flags can be set and the account can never be deleted.
    pub flags: i32,
    /// updated_at field
    pub lastmodified: i32,
    /// Starting in protocol version 10, each account also tracks its lumen liabilities. Buying liabilities equal the total amount of lumens offered to buy aggregated over all offers owned by this account, and selling liabilities equal the total amount of lumens offered to sell aggregated over all offers owned by this account. An account must always have balance sufficiently above the minimum reserve to satisfy its lumen selling liabilities, and a balance sufficiently below the maximum to accomodate its lumen buying liabilities
    pub buyingliabilities: i64,
    pub sellingliabilities: i64,
    /// Used for multi-sig. This field lists other public keys and their weights, which can be used to authorize transactions for this account.
    pub signers: Option<String>,
}

type Result<T> = std::result::Result<T, diesel::result::Error>;

impl Account {
    pub fn all() -> Result<Vec<Account>> {
        use self::accounts::dsl::*;

        accounts.load::<Account>(&*db_conn())
    }

    pub fn create(accountid: String) -> Result<usize> {
        let new_account = NewAccount::new(accountid);
        diesel::insert_into(accounts::table)
            .values(&new_account)
            .execute(&*db_conn())
    }

    pub fn get(g_accountid: &str) -> Result<Account> {
        use self::accounts::dsl::*;

        accounts
            .filter(accountid.eq(g_accountid))
            .first::<Account>(&*db_conn())
    }

    pub fn delete(g_accountid: &str) -> Result<usize> {
        use self::accounts::dsl::*;

        diesel::delete(accounts.filter(accountid.eq(g_accountid))).execute(&*db_conn())
    }

    // TODO: this should take base reserve from particular
    // ledger header into account
    // TODO: consider ledger version lower than 8, formulae was
    // different back then
    pub fn spendable_balance(&self) -> i64 {
        self.balance - ((2 + i64::from(self.numsubentries)) * BASE_RESERVE)
    }
}

#[derive(Insertable)]
#[table_name = "accounts"]
pub struct NewAccount {
    pub accountid: String,
    pub balance: i64,
    pub seqnum: i64,
    pub numsubentries: i32,
    pub inflationdest: Option<String>,
    pub homedomain: String,
    pub thresholds: String,
    pub flags: i32,
    pub lastmodified: i32,
    pub buyingliabilities: i64,
    pub sellingliabilities: i64,
    pub signers: Option<String>,
}

impl NewAccount {
    pub fn new(accountid: String) -> Self {
        NewAccount {
            accountid,
            balance: 0,
            seqnum: 0, // TODO: This number starts equal to the ledger number at which the account was created
            numsubentries: 0,
            inflationdest: None,
            homedomain: String::from("example.com"),
            thresholds: String::from(""),
            flags: 1,
            lastmodified: 0,
            buyingliabilities: 0,
            sellingliabilities: 0,
            signers: None,
        }
    }
}
