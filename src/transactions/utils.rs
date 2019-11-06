use std::result::Result;
use crate::database::{Account, BASE_RESERVE};

#[derive(Debug)]
pub enum Error {
    InsufficientBalance,
    MaxBalanceExceeded,
    TooMuchSellingLiabilities,
    TooMuchBuyingLiabilities
}

// `delta` can be negative for charging account
pub fn add_balance(account: &Account, delta: i64) -> Result<i64, Error> {
    if delta == 0 {
        return Ok(account.balance);
    }

    // check, whether we want to subtract too much
    if account.balance + delta < 0 {
        return Err(Error::InsufficientBalance);
    }

    if delta + account.balance > std::i64::MAX {
        return Err(Error::MaxBalanceExceeded);
    }

    let new_balance = account.balance + delta;

    if delta < 0 && new_balance - BASE_RESERVE < account.sellingliabilities {
        return Err(Error::TooMuchSellingLiabilities);
    }

    if delta > 0 && new_balance + account.buyingliabilities > std::i64::MAX {
        return Err(Error::TooMuchBuyingLiabilities);
    }

    Ok(new_balance)
}
