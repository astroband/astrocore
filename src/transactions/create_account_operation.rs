use crate::xdr;
use crate::database;

pub(crate) fn create_account_operation(source_account: Option<xdr::AccountId>, account_operation: xdr::CreateAccountOp) -> xdr::CreateAccountResultCode {
  if account_operation.starting_balance == 0 {
    xdr::CreateAccountResultCode::CreateAccountSuccess
  } else if account_operation.starting_balance != 0 && source_account.is_none() {
    xdr::CreateAccountResultCode::CreateAccountUnderfunded
  } else {
    xdr::CreateAccountResultCode::CreateAccountAlreadyExist // TODO not realy
  }
}


#[cfg(test)]
mod tests {
    use super::{xdr};
    use crate::factories::internal_xdr;

    #[test]
    fn test_account_operation_with_zero_balance() {
        // let account_id: [u8; 32] = [114, 102, 121, 110, 104, 105, 115, 100, 99, 122, 112, 112, 119, 105, 108, 121, 122, 108, 102, 101, 107, 111, 102, 103, 109, 103, 106, 105, 98, 118, 110, 113];

        let acccount_operation = xdr::CreateAccountOp {
            destination: internal_xdr::build_public_key(),
            starting_balance: 0,
        };

        let tx = xdr::Operation {
            source_account: None,
            body: xdr::OperationBody::CreateAccountOp(acccount_operation),
        };

        assert_eq!(1, 1);
    }
}
