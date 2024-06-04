use rasn::*;
use chrono::prelude::*;
use chrono::DateTime;

// Also valid
#[derive(AsnType,Clone)]
#[rasn(automatic_tags)]
struct Person2 {
    pub age: Option<String>,
    pub name: Option<String>,
}

#[derive(AsnType,Clone, Decode, Debug, Encode, Copy,  PartialEq)]
#[repr(u8)] // underlying integer type, if necessary for your use case
#[rasn(enumerated)]
pub enum Status {
    #[rasn(tag = "0")]
    init = 0,
    #[rasn(tag = "1")]
    debit =  1,
    #[rasn(tag = "2")]
    processing =  2,
    #[rasn(tag = "3")]
    waiting = 3,
    #[rasn(tag = "4")]
    credit = 4,
    #[rasn(tag = "5")]
    complete = 5,
    #[rasn(tag = "6")]
    nested  = 6,

}

#[derive(AsnType,Clone, Decode, Debug, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct PrivateKey {
    pub version:         u32,
    // the encrypted private key value
    pub key:             rasn::types::OctetString,
}

#[derive(AsnType,Clone, Decode, Debug, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct PublicKey {
    pub version:         u32,
    // the encrypted private key value
    pub key:             rasn::types::OctetString,
}


#[derive(AsnType,Clone, Decode, Debug, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct Action {
    pub version:         u32,
    pub date:            rasn::types::UtcTime,
    pub contract:        Option<rasn::types::OctetString>,
    pub contractName:    Option<rasn::types::OctetString>,
    pub parent:          rasn::types::OctetString,
    pub model:           rasn::types::Any,
}


#[derive(AsnType,Clone, Decode, Debug, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct Transaction {
    pub version:                u32,
    pub date:                   rasn::types::UtcTime,
    pub value:                  u128,
    pub parent:                 rasn::types::OctetString,
    pub encrypted:              bool,
    // the source account for the transaction
    pub sourceAccount:          rasn::types::OctetString,
    // the target account for the transaction
    pub targetAccount:          rasn::types::OctetString,
    // the transaction signator and creator id are there
    // to track internal transactions for security and validation purposes
    pub transactionSignator:    rasn::types::OctetString,
    pub creatorId:              rasn::types::OctetString,
    pub actions:                rasn::types::SequenceOf<Action>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq)]
#[rasn(choice)]
pub enum ChangeData {
    #[rasn(tag(1))]
    asn1Change(rasn::types::Any),
    #[rasn(tag(2))]
    binaryChange(rasn::types::OctetString)       
}


#[derive(AsnType,Clone, Decode, Debug, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct ChangeSet {
    pub version:                u32,
    pub transactionHash:        rasn::types::OctetString,
    pub accountHash:            rasn::types::OctetString,
    pub status:                 Status,
    pub changes:                rasn::types::SequenceOf<ChangeData>,
}

#[derive(AsnType,Clone, Decode, Debug, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct SignedChangeSet {
    pub changeSet:              ChangeSet,
    pub changeSetHash:          rasn::types::OctetString,
    pub signature:              rasn::types::OctetString,
}

#[derive(AsnType,Clone, Decode, Debug, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct SignedTransaction {
    pub version:                u32,
    pub transaction:            Transaction,
    pub transactionHash:        rasn::types::OctetString,
    pub signature:              rasn::types::OctetString,
}


#[derive(AsnType,Clone, Decode, Debug, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct TransactionTrace {
    pub traceHash:              rasn::types::OctetString,
    pub signature:              rasn::types::OctetString,
    pub signatureHash:          rasn::types::OctetString,
}

#[derive(AsnType,Clone, Decode, Debug, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct TransactionWrapper {
    pub version:             u32,
    // transaction header information
    pub sourceAccount:       rasn::types::OctetString,
    pub targetAccount:       rasn::types::OctetString,
    pub parent:              rasn::types::OctetString,
    pub feeAccount:          rasn::types::OctetString,
    pub transactionHash:     rasn::types::OctetString,
    pub signature:           rasn::types::OctetString,
    // transaction
    pub signedTransaction:   SignedTransaction,
    pub transactionTrace:    rasn::types::SequenceOf<TransactionTrace>,
    // status and changeset
    pub currentStatus:       Status,
    pub changeSet:           rasn::types::SequenceOf<SignedChangeSet>,
}


#[derive(AsnType,Clone, Decode, Debug, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct EncryptedDataWrapper {
    pub version:             u32,
    pub transaction:         rasn::types::OctetString,
    // the hash of the transaction when encrypted
    // this is here to validate that the onion encryption is being
    // decrypted correctly through the various layers.
    pub hash:                rasn::types::SequenceOf<rasn::types::OctetString>,
}


#[derive(AsnType,Clone, Decode, Debug, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct TransactionMessage {
    pub version:              u32,
    // transaction header information
    pub transaction:          TransactionWrapper,
    pub availableTime:        u64,
    pub elapsedTime:          u64,
    pub sideTransactions:     rasn::types::SequenceOf<TransactionMessage>,
    pub encryptedSideTransactions:  rasn::types::SequenceOf<EncryptedDataWrapper>,
}

#[derive(AsnType,Clone, Decode, Debug, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct SoftwareConsensus {
    pub version:         u32,
    pub date:            rasn::types::UtcTime,
    pub previousHash:    rasn::types::OctetString,
    pub account:         rasn::types::OctetString,
    pub seed:            rasn::types::OctetString,
    pub systemHashs:     rasn::types::SequenceOf<rasn::types::OctetString>,
    pub merkelRoot:      rasn::types::OctetString,
    pub signature:       rasn::types::OctetString,
}

impl SoftwareConsensus {
    pub fn blank_software_consensus() -> SoftwareConsensus {
        SoftwareConsensus {
            version: 1,
            date: Utc::now(),
            previousHash: rasn::types::OctetString::from(""),
            account: rasn::types::OctetString::from(""),
            seed: rasn::types::OctetString::from(""),
            systemHashs: Vec::new(),
            merkelRoot: rasn::types::OctetString::from(""),
            signature: rasn::types::OctetString::from(""),
        }
    }
}


#[derive(AsnType,Clone, Decode, Debug, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct Block {
    pub version:            u32,
    pub date:               rasn::types::UtcTime,
    pub parent:             rasn::types::OctetString,
    pub transactions:       rasn::types::SequenceOf<TransactionWrapper>,
    pub acceptedCheck:      SoftwareConsensus,
    pub validateCheck:      SoftwareConsensus,
    pub merkelRoot:         rasn::types::OctetString
}

impl Block {
    pub fn blank_block() -> Block {
        Block {
            version: 1,
            date: Utc::now(),
            parent: rasn::types::OctetString::from(""),
            transactions: Vec::new(),
            acceptedCheck: SoftwareConsensus::blank_software_consensus(),
            validateCheck: SoftwareConsensus::blank_software_consensus(),
            merkelRoot: rasn::types::OctetString::from(""),
        }
    }
}

#[derive(AsnType,Clone, Decode, Debug, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct SignedBlock {
    pub version:            u32,
    pub date:               rasn::types::UtcTime,
    pub parent:             rasn::types::OctetString,
    pub tangle_hash:        rasn::types::OctetString,
    pub block:              Block,
    pub hash:               rasn::types::OctetString,
    pub signatures:         rasn::types::SequenceOf<rasn::types::OctetString>,
}

impl SignedBlock {
    pub fn new(block: &Block) -> SignedBlock {
        SignedBlock {
            version: 1,
            date: Utc::now(),
            parent: rasn::types::OctetString::from(""),
            tangle_hash: rasn::types::OctetString::from(""),
            block: block.clone(),
            hash: rasn::types::OctetString::from(""),
            signatures: Vec::new(),
        }
    }
}


// rdf
#[derive(AsnType,Clone, Decode, Debug, Encode, Copy, PartialEq)]
#[repr(u8)] // underlying integer type, if necessary for your use case
#[rasn(enumerated)]
pub enum RDFChange {
    #[rasn(tag = "0")]
    persist = 0,
    #[rasn(tag = "1")]
    remove = 1,
    #[rasn(tag = "2")]
    none = 2
}


#[derive(AsnType,Clone, Decode, Debug, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct RDFObject {
    pub value:              rasn::types::OctetString,
    pub _type:              rasn::types::OctetString,
    pub lang:               rasn::types::OctetString,
    pub dataType:           rasn::types::OctetString,
}


#[derive(AsnType,Clone, Decode, Debug, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct RDFPredicate {
    pub predicate:          rasn::types::OctetString,
    pub rdfObjects:         rasn::types::SequenceOf<RDFObject>,
}


#[derive(AsnType,Clone, Decode, Debug, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct RDFSubject {
    pub subject:            rasn::types::OctetString,
    pub rdfPredicates:      rasn::types::SequenceOf<RDFPredicate>
}


#[derive(AsnType,Clone, Decode, Debug, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct RDFNT {
    pub version:            u32,
    pub ntSubject:          rasn::types::OctetString,
    pub ntPredicate:        rasn::types::OctetString,
    pub ntObject:           rasn::types::OctetString,
}

impl RDFNT {
    pub fn new(ntSubject: &String, ntPredicate: &String, ntObject: &String) -> RDFNT {
        RDFNT {
            version: 1,
            ntSubject: rasn::types::OctetString::from(ntSubject.clone()),
            ntPredicate: rasn::types::OctetString::from(ntPredicate.clone()),
            ntObject: rasn::types::OctetString::from(ntObject.clone()),
        }
    }
}


#[derive(AsnType,Clone, Decode, Debug, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct RDFNtGroup {
    pub version:            u32,
    pub rdfNT:              rasn::types::SequenceOf<RDFNT>,
}


#[derive(AsnType,Clone, Decode, Debug, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct RDFModel {
    pub action:             RDFChange,
    pub rdfSubjects:        rasn::types::SequenceOf<RDFSubject>,
    pub rdfNtGroups:        rasn::types::SequenceOf<RDFNtGroup>,
}


// Election
#[derive(AsnType,Clone, Decode, Debug, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct Election {
    pub version:             u32,
    pub date:                rasn::types::UtcTime,
    pub accountHash:         rasn::types::OctetString,
    pub acceptedCheck:       SoftwareConsensus,
    pub validateCheck:       SoftwareConsensus
}


#[derive(AsnType,Clone, Decode, Debug, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct SignedElection {
    pub version:             u32,
    pub election:            Election,
    pub electionHash:        rasn::types::OctetString,
    pub signature:           rasn::types::OctetString,
}

#[derive(AsnType,Clone, Decode, Debug, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct ElectNode {
    pub version:             u32,
    pub date:                rasn::types::UtcTime,
    pub accountHash:         rasn::types::OctetString,
    pub electedNode:         SignedElection,
    pub alternatives:        rasn::types::SequenceOf<SignedElection>,
    pub acceptedCheck:       SoftwareConsensus,
    pub validateCheck:       SoftwareConsensus,
}

#[derive(AsnType,Clone, Decode, Debug, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct SignedElectNode {
    pub version:              u32,
    pub electedNode:          ElectNode,
    pub electedHash:          rasn::types::OctetString,
    pub signature:            rasn::types::OctetString,
}