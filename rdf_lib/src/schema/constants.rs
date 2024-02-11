use std::fmt;

const SCHEMA_BASE: &str = "http://example.com/";

pub enum RDFClasses {
    TRANSACTION,
    TRANSACTION_LEG,
    DEBIT_TRANSACTION_LEG,
    CREDIT_TRANSACTION_LEG,
    BLOCK,
    TANGLE,
    ACCOUNT,
}

impl fmt::Display for RDFClasses {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let base_str = match self {
            RDFClasses::TRANSACTION => "blockchain#Transaction",
            RDFClasses::TRANSACTION_LEG => "blockchain#TransactionLeg",
            RDFClasses::DEBIT_TRANSACTION_LEG => "blockchain#DebitLeg",
            RDFClasses::CREDIT_TRANSACTION_LEG => "blockchain#CreditLeg",
            RDFClasses::BLOCK => "blockchain#Block",
            RDFClasses::TANGLE => "blockchain#Tangle",
            RDFClasses::ACCOUNT => "blockchain#ACCOUNT",
        };
        write!(f, "{}{}", SCHEMA_BASE, base_str)
    }
}

pub enum RDFMeta {
    CREATED,
    MODIFIED,
}

impl fmt::Display for RDFMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let base_str = match self {
            RDFMeta::CREATED => "blockchain#created",
            RDFMeta::MODIFIED => "blockchain#modified",
        };
        write!(f, "{}{}", SCHEMA_BASE, base_str)
    }
}

pub enum RDFAccount {
    ID,
}

impl fmt::Display for RDFAccount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let base_str = match self {
            RDFAccount::ID => "blockchain#accountID",
            // ... other match arms
        };
        write!(f, "{}{}", SCHEMA_BASE, base_str)
    }
}

pub enum RDFTransaction {
    ID,
    PARENT_ID,
    PARENT,
    DEBIT_ACCOUNT,
    DEBIT_ACCOUNT_ID,
    CREDIT_ACCOUNT,
    CREDIT_ACCOUNT_ID,
    VALUE,
    DATA_BLOB,
}

impl fmt::Display for RDFTransaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let base_str = match self {
            RDFTransaction::ID => "blockchain#transactionID",
            RDFTransaction::PARENT_ID => "blockchain#parentTransactionID",
            RDFTransaction::PARENT => "blockchain#parentTransaction",
            RDFTransaction::DEBIT_ACCOUNT => "blockchain#debitAccount",
            RDFTransaction::DEBIT_ACCOUNT_ID => "blockchain#debitAccountId",
            RDFTransaction::CREDIT_ACCOUNT => "blockchain#creditAccount",
            RDFTransaction::CREDIT_ACCOUNT_ID => "blockchain#creditAccountId",
            RDFTransaction::VALUE => "blockchain#transactionValue",
            RDFTransaction::DATA_BLOB => "blockchain#dataBlob",
        };
        write!(f, "{}{}", SCHEMA_BASE, base_str)
    }
}

pub enum RDFBlock {
    ID,
    PARENT_ID,
    PARENT,
    MERKLE_TREE_ROOT,
    INCLUDES_TRANSACTIONS,
    TANGLE,
    TANGLE_ID,
}

impl fmt::Display for RDFBlock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let base_str = match self {
            RDFBlock::ID => "blockchain#blockID",
            RDFBlock::PARENT_ID => "blockchain#parentBlockID",
            RDFBlock::PARENT => "blockchain#parentBlock",
            RDFBlock::MERKLE_TREE_ROOT => "blockchain#merkleTreeRoot",
            RDFBlock::INCLUDES_TRANSACTIONS => "blockchain#includesTransaction",
            RDFBlock::TANGLE => "blockchain#blockTangle",
            RDFBlock::TANGLE_ID => "blockchain#blockTangleId",
        };
        write!(f, "{}{}", SCHEMA_BASE, base_str)
    }
}

pub enum RDFTransactionLeg {
    LEG_STATE,
}

impl fmt::Display for RDFTransactionLeg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let base_str = match self {
            RDFTransactionLeg::LEG_STATE => "blockchain#legState",
            // ... other match arms
        };
        write!(f, "{}{}", SCHEMA_BASE, base_str)
    }
}

pub enum RDFTangle {
    ID,
    ACCOUNTS,
}

impl fmt::Display for RDFTangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let base_str = match self {
            RDFTangle::ID => "blockchain#tangleID",
            RDFTangle::ACCOUNTS => "blockchain#includesAccount",
            // ... other match arms
        };
        write!(f, "{}{}", SCHEMA_BASE, base_str)
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn test_classes() -> Result<(), Box<dyn Error>> {
        let url_1 =  RDFClasses::TRANSACTION.to_string();
        let url_2 =  String::from("http://example.com/blockchain#Transaction");

        assert_eq!(url_1,url_2);
        Ok(())
    }

    #[test]
    fn test_meta_data() -> Result<(), Box<dyn Error>> {
        let url_1 =  RDFMeta::CREATED.to_string();
        let url_2 =  String::from("http://example.com/blockchain#created");

        assert_eq!(url_1,url_2);
        Ok(())
    }

    #[test]
    fn test_account() -> Result<(), Box<dyn Error>> {
        let url_1 =  RDFAccount::ID.to_string();
        let url_2 =  String::from("http://example.com/blockchain#accountID");

        assert_eq!(url_1,url_2);
        Ok(())
    }

    #[test]
    fn test_transaction() -> Result<(), Box<dyn Error>> {
        let url_1 =  RDFTransaction::ID.to_string();
        let url_2 =  String::from("http://example.com/blockchain#transactionID");

        assert_eq!(url_1,url_2);
        Ok(())
    }

    #[test]
    fn test_block() -> Result<(), Box<dyn Error>> {
        let url_1 =  RDFBlock::ID.to_string();
        let url_2 =  String::from("http://example.com/blockchain#blockID");

        assert_eq!(url_1,url_2);
        Ok(())
    }

    #[test]
    fn test_tangle() -> Result<(), Box<dyn Error>> {
        let url_1 =  RDFTangle::ID.to_string();
        let url_2 =  String::from("http://example.com/blockchain#tangleID");

        assert_eq!(url_1,url_2);
        Ok(())
    }

    #[test]
    fn test_transaction_leg() -> Result<(), Box<dyn Error>> {
        let url_1 =  RDFTransactionLeg::LEG_STATE.to_string();
        let url_2 =  String::from("http://example.com/blockchain#legState");

        assert_eq!(url_1,url_2);
        Ok(())
    }
}