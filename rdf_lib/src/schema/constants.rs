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
    ACCOUNT_GROUP,
    CONTRACT,
    CONTRACT_VERSION
}

impl fmt::Display for RDFClasses {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let base_str = match self {
            RDFClasses::TRANSACTION => "Transaction",
            RDFClasses::TRANSACTION_LEG => "TransactionLeg",
            RDFClasses::DEBIT_TRANSACTION_LEG => "DebitLeg",
            RDFClasses::CREDIT_TRANSACTION_LEG => "CreditLeg",
            RDFClasses::BLOCK => "Block",
            RDFClasses::TANGLE => "Tangle",
            RDFClasses::ACCOUNT => "Account",
            RDFClasses::ACCOUNT_GROUP => "AccountGroup",
            RDFClasses::CONTRACT => "Contract",
            RDFClasses::CONTRACT_VERSION => "ContractVersion",
        };
        write!(f, "{}blockchain/classes#{}", SCHEMA_BASE, base_str)
    }
}

pub enum RDFMeta {
    ID,
    CREATED,
    MODIFIED,
}

impl fmt::Display for RDFMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let base_str = match self {
            RDFMeta::ID => "metaId",
            RDFMeta::CREATED => "created",
            RDFMeta::MODIFIED => "modified",
        };
        write!(f, "{}blockchain/meta#{}", SCHEMA_BASE, base_str)
    }
}

pub enum RDFAccount {
    ID,
    ACCOUNT,
}

impl fmt::Display for RDFAccount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let base_str = match self {
            RDFAccount::ID => "accountID",
            RDFAccount::ACCOUNT => "account",
        };
        write!(f, "{}blockchain/account#{}", SCHEMA_BASE, base_str)
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
            RDFTransaction::ID => "transactionID",
            RDFTransaction::PARENT_ID => "parentTransactionID",
            RDFTransaction::PARENT => "parentTransaction",
            RDFTransaction::DEBIT_ACCOUNT => "debitAccount",
            RDFTransaction::DEBIT_ACCOUNT_ID => "debitAccountId",
            RDFTransaction::CREDIT_ACCOUNT => "creditAccount",
            RDFTransaction::CREDIT_ACCOUNT_ID => "creditAccountId",
            RDFTransaction::VALUE => "transactionValue",
            RDFTransaction::DATA_BLOB => "dataBlob",
        };
        write!(f, "{}blockchain/transaction#{}", SCHEMA_BASE, base_str)
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
    DATA_BLOB,
}

impl fmt::Display for RDFBlock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let base_str = match self {
            RDFBlock::ID => "blockID",
            RDFBlock::PARENT_ID => "parentBlockID",
            RDFBlock::PARENT => "parentBlock",
            RDFBlock::MERKLE_TREE_ROOT => "merkleTreeRoot",
            RDFBlock::INCLUDES_TRANSACTIONS => "includesTransaction",
            RDFBlock::TANGLE => "blockTangle",
            RDFBlock::TANGLE_ID => "blockTangleId",
            RDFBlock::DATA_BLOB => "dataBlob",
        };
        write!(f, "{}blockchain/block#{}", SCHEMA_BASE, base_str)
    }
}

pub enum RDFTransactionLeg {
    LEG_STATE,
}

impl fmt::Display for RDFTransactionLeg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let base_str = match self {
            RDFTransactionLeg::LEG_STATE => "legState",
        };
        write!(f, "{}blockchain/leg#{}", SCHEMA_BASE, base_str)
    }
}

pub enum RDFTangle {
    ID,
    ACCOUNTS,
}

impl fmt::Display for RDFTangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let base_str = match self {
            RDFTangle::ID => "tangleID",
            RDFTangle::ACCOUNTS => "includesAccount",
        };
        write!(f, "{}blockchain/tangle#{}", SCHEMA_BASE, base_str)
    }
}


pub enum RDFContract {
    ID,
    ACCOUNT,
    ACCOUNT_ID,
    CONTRACT_ID,
    CONTRACT_NAME,    
}

impl fmt::Display for RDFContract {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let base_str = match self {
            RDFContract::ID => "contractID",
            RDFContract::ACCOUNT => "account",
            RDFContract::ACCOUNT_ID => "accountId",
            RDFContract::CONTRACT_ID => "contractId",
            RDFContract::CONTRACT_NAME => "contractName",
        };
        write!(f, "{}blockchain/countract#{}", SCHEMA_BASE, base_str)
    }
}

pub enum RDFContractVersion {
    ID,
    CONTRACT_ID,
    CONTRACT,
    VERSION,
    CODE,    
}

impl fmt::Display for RDFContractVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let base_str = match self {
            RDFContractVersion::ID => "contractVersionID",
            RDFContractVersion::CONTRACT_ID => "contratcId",
            RDFContractVersion::CONTRACT => "contract",
            RDFContractVersion::VERSION => "version",
            RDFContractVersion::CODE => "code",
        };
        write!(f, "{}blockchain/countractVersion#{}", SCHEMA_BASE, base_str)
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn test_classes() -> Result<(), Box<dyn Error>> {
        let url_1 =  RDFClasses::TRANSACTION.to_string();
        let url_2 =  String::from("http://example.com/blockchain/classes#Transaction");

        assert_eq!(url_1,url_2);
        Ok(())
    }

    #[test]
    fn test_meta_data() -> Result<(), Box<dyn Error>> {
        let url_1 =  RDFMeta::CREATED.to_string();
        let url_2 =  String::from("http://example.com/blockchain/meta#created");

        assert_eq!(url_1,url_2);
        Ok(())
    }

    #[test]
    fn test_account() -> Result<(), Box<dyn Error>> {
        let url_1 =  RDFAccount::ID.to_string();
        let url_2 =  String::from("http://example.com/blockchain/account#accountID");

        assert_eq!(url_1,url_2);
        Ok(())
    }

    #[test]
    fn test_transaction() -> Result<(), Box<dyn Error>> {
        let url_1 =  RDFTransaction::ID.to_string();
        let url_2 =  String::from("http://example.com/blockchain/transaction#transactionID");

        assert_eq!(url_1,url_2);
        Ok(())
    }

    #[test]
    fn test_block() -> Result<(), Box<dyn Error>> {
        let url_1 =  RDFBlock::ID.to_string();
        let url_2 =  String::from("http://example.com/blockchain/block#blockID");

        assert_eq!(url_1,url_2);
        Ok(())
    }

    #[test]
    fn test_transaction_leg() -> Result<(), Box<dyn Error>> {
        let url_1 =  RDFTransactionLeg::LEG_STATE.to_string();
        let url_2 =  String::from("http://example.com/blockchain/leg#legState");

        assert_eq!(url_1,url_2);
        Ok(())
    }

    #[test]
    fn test_tangle() -> Result<(), Box<dyn Error>> {
        let url_1 =  RDFTangle::ID.to_string();
        let url_2 =  String::from("http://example.com/blockchain/tangle#tangleID");

        assert_eq!(url_1,url_2);
        Ok(())
    }

    #[test]
    fn test_contract() -> Result<(), Box<dyn Error>> {
        let url_1 =  RDFContract::ID.to_string();
        let url_2 =  String::from("http://example.com/blockchain/countract#contractID");

        assert_eq!(url_1,url_2);
        Ok(())
    }


    #[test]
    fn test_contract_version() -> Result<(), Box<dyn Error>> {
        let url_1 =  RDFContractVersion::ID.to_string();
        let url_2 =  String::from("http://example.com/blockchain/countractVersion#contractVersionID");

        assert_eq!(url_1,url_2);
        Ok(())
    }
}