// pub(crate) use crate::config::CONFIG;
// pub(crate) use crate::crypto;
// pub(crate) use crate::network::Network;
use crate::xdr;

mod create_account_operation;
mod utils;

use create_account_operation::create_account_operation;
// pub(crate) use lazy_static::lazy_static;

//  pub(crate) use crate::database::models::account;

pub fn do_apply(operation: xdr::Operation) {
    match operation.body {
        xdr::OperationBody::CreateAccountOp(acccount_operation) => {
            create_account_operation(operation.source_account, acccount_operation);
        },
        _ => {}
    }
}
