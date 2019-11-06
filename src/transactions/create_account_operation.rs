use crate::xdr;
use crate::database::{db_conn, Account, BASE_RESERVE};
use crate::stellar_base::strkey;
use super::utils;
// TODO: I believe, everything should work without using `prelude`. But it doesn't
use diesel::prelude::*;
use diesel::result::Error::NotFound;

pub(crate) fn create_account_operation(source_account: Option<xdr::AccountId>, account_operation: xdr::CreateAccountOp) -> xdr::CreateAccountResultCode {
    let dest_account_id = strkey::encode_ed25519_public_key(account_operation.destination).unwrap();

    if account_operation.starting_balance < BASE_RESERVE {
        xdr::CreateAccountResultCode::CreateAccountLowReserve
    } else if Account::get(&dest_account_id).is_ok() {
        xdr::CreateAccountResultCode::CreateAccountAlreadyExist
    } else {
        match source_account {
            Some(public_key) => {
                let source_account_id = strkey::encode_ed25519_public_key(public_key).unwrap();

                match Account::get(&source_account_id) {
                  Ok(account) => {
                      if account.spendable_balance() < account_operation.starting_balance {
                          xdr::CreateAccountResultCode::CreateAccountUnderfunded
                      } else {
                          // Actually apply operation
                          use crate::schema::accounts::dsl::balance;

                          let new_balance = utils::add_balance(&account, account_operation.starting_balance).unwrap();
                          diesel::update(&account)
                              .set(balance.eq(new_balance))
                              .execute(&*db_conn())
                              .unwrap();

                          xdr::CreateAccountResultCode::CreateAccountSuccess
                      }
                  },
                  Err(e) => {
                      match e {
                          NotFound => {
                              xdr::CreateAccountResultCode::CreateAccountMalformed
                          },
                          _ => {
                              // TODO: What do we do in this case?
                              xdr::CreateAccountResultCode::CreateAccountMalformed
                          }
                      }
                  }
                }
            }
            None => {
                xdr::CreateAccountResultCode::CreateAccountUnderfunded
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::{xdr};
    use crate::factories::internal_xdr;

    #[test]
    fn test_account_operation_with_zero_balance() {
        let source_account = internal_xdr::random_public_key();

        let operation = xdr::CreateAccountOp {
            destination: internal_xdr::random_public_key(),
            starting_balance: 0,
        };

        let result = super::create_account_operation(Some(source_account), operation);

        assert_eq!(result, xdr::CreateAccountResultCode::CreateAccountLowReserve);
    }
}
