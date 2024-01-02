use serde::{Deserialize, Serialize};
use derive_new::new;

pub const DEBIT_TYPE: &str = "debit";
pub const CREDIT_TYPE: &str = "credit";

// Define your message types using structs
#[derive(Serialize, Deserialize, Debug, new, Clone)]
pub struct TransactionMessage{
    pub source_account_id: String,
    pub target_account_id: String,
    pub transaction_type: String,
    pub binary_transaction: Vec<u8>,
}

impl TransactionMessage {
    pub fn getAccountId(&self) -> String {
        if self.transaction_type.to_string() == DEBIT_TYPE {
            self.source_account_id.clone()
        } else {
            self.target_account_id.clone()
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_account_id_debit() {
        let transaction_message = TransactionMessage{source_account_id:"dr".to_string(),target_account_id:"cr".to_string(),
            transaction_type: DEBIT_TYPE.to_string().clone(), binary_transaction: vec![1,2,3,4,5,6]};
        let transaction_id  = transaction_message.getAccountId();
        assert_eq!("dr".to_string(),transaction_id)
    }

    #[test]
    fn test_get_account_id_credit() {
        let transaction_message = TransactionMessage{source_account_id:"dr".to_string(),target_account_id:"cr".to_string(),
            transaction_type: CREDIT_TYPE.to_string().clone(), binary_transaction: vec![1,2,3,4,5,6]};
        let transaction_id  = transaction_message.getAccountId();
        assert_eq!("cr".to_string(),transaction_id)
    }
}