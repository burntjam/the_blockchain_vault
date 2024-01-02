use rasn::*;

// Also valid
#[derive(AsnType,Clone)]
#[rasn(automatic_tags)]
struct Person2 {
    age: Option<String>,
    name: Option<String>,
}

#[derive(AsnType,Clone, Decode, Debug, PartialEq)]
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

#[derive(AsnType,Clone)]
#[rasn(automatic_tags)]
pub struct PrivateKey {
    version:         u32,
    // the encrypted private key value
    key:             rasn::types::OctetString,
}

#[derive(AsnType,Clone)]
#[rasn(automatic_tags)]
pub struct PublicKey {
    version:         u32,
    // the encrypted private key value
    key:             rasn::types::OctetString,
}


#[derive(AsnType,Clone)]
#[rasn(automatic_tags)]
pub struct Action {
    version:         u32,
    date:            rasn::types::UtcTime,
    contract:        Option<rasn::types::OctetString>,
    contractName:    Option<String>,
    parent:          rasn::types::OctetString,
    model:           rasn::types::Any,
}


#[derive(AsnType,Clone)]
#[rasn(automatic_tags)]
pub struct Transaction {
    version:                u32,
    date:                   rasn::types::UtcTime,
    value:                  u128,
    parent:                 rasn::types::OctetString,
    encrypted:              bool,
    // the source account for the transaction
    sourceAccount:          rasn::types::OctetString,
    // the target account for the transaction
    targetAccount:          rasn::types::OctetString,
    // the transaction signator and creator id are there
    // to track internal transactions for security and validation purposes
    transactionSignator:    rasn::types::OctetString,
    creatorId:              rasn::types::OctetString,
    actions:                Vec<Action>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq)]
#[rasn(choice)]
pub enum ChangeData {
    #[rasn(tag(1))]
    asn1Change(rasn::types::Any),
    #[rasn(tag(2))]
    binaryChange(rasn::types::OctetString)       
}


#[derive(AsnType,Clone)]
#[rasn(automatic_tags)]
pub struct ChangeSet {
    version:                u32,
    transactionHash:        rasn::types::OctetString,
    accountHash:            rasn::types::OctetString,
    status:                 Status,
    changes:                Vec<ChangeData>,
}

#[derive(AsnType,Clone)]
#[rasn(automatic_tags)]
pub struct SignedChangeSet {
    changeSet:              ChangeSet,
    changeSetHash:          rasn::types::OctetString,
    signature:              rasn::types::OctetString,
}

#[derive(AsnType,Clone)]
#[rasn(automatic_tags)]
pub struct SignedTransaction {
    version:                u32,
    transaction:            Transaction,
    transactionHash:        rasn::types::OctetString,
    signature:              rasn::types::OctetString,
}


#[derive(AsnType,Clone)]
#[rasn(automatic_tags)]
pub struct TransactionTrace {
    traceHash:              rasn::types::OctetString,
    signature:              rasn::types::OctetString,
    signatureHash:          rasn::types::OctetString,
}

#[derive(AsnType,Clone)]
#[rasn(automatic_tags)]
pub struct TransactionWrapper {
    version:             u32,
    // transaction header information
    sourceAccount:       rasn::types::OctetString,
    targetAccount:       rasn::types::OctetString,
    parent:              rasn::types::OctetString,
    feeAccount:          rasn::types::OctetString,
    transactionHash:     rasn::types::OctetString,
    signature:           rasn::types::OctetString,
    // transaction
    signedTransaction:   SignedTransaction,
    transactionTrace:    Vec<TransactionTrace>,
    // status and changeset
    currentStatus:       Status,
    changeSet:           Vec<SignedChangeSet>,
}


#[derive(AsnType,Clone)]
#[rasn(automatic_tags)]
pub struct EncryptedDataWrapper {
    version:             u32,
    transaction:         rasn::types::OctetString,
    // the hash of the transaction when encrypted
    // this is here to validate that the onion encryption is being
    // decrypted correctly through the various layers.
    hash:                Vec<rasn::types::OctetString>,
}


#[derive(AsnType,Clone)]
#[rasn(automatic_tags)]
pub struct TransactionMessage {
    version:              u32,
    // transaction header information
    transaction:          TransactionWrapper,
    availableTime:        u64,
    elapsedTime:          u64,
    sideTransactions:     Vec<TransactionMessage>,
    encryptedSideTransactions:  Vec<EncryptedDataWrapper>,
}

#[derive(AsnType,Clone)]
#[rasn(automatic_tags)]
pub struct SoftwareConsensus {
    version:         u32,
    date:            rasn::types::UtcTime,
    previousHash:    rasn::types::OctetString,
    account:         rasn::types::OctetString,
    seed:            rasn::types::OctetString,
    systemHashs:     Vec<rasn::types::OctetString>,
    merkelRoot:      rasn::types::OctetString,
    signature:       rasn::types::OctetString,
}


#[derive(AsnType,Clone)]
#[rasn(automatic_tags)]
pub struct Block {
    pub version:            u32,
    pub date:               rasn::types::UtcTime,
    pub parent:             rasn::types::OctetString,
    pub transactions:       Vec<TransactionWrapper>,
    pub acceptedCheck:      SoftwareConsensus,
    pub validateCheck:      SoftwareConsensus,
    pub merkelRoot:         rasn::types::OctetString
}

#[derive(AsnType,Clone)]
#[rasn(automatic_tags)]
pub struct SignedBlock {
    pub version:            u32,
    pub date:               rasn::types::UtcTime,
    pub parent:             rasn::types::OctetString,
    pub block:              Block,
    pub hash:               rasn::types::OctetString,
    pub signatures:         Vec<rasn::types::OctetString>,
}


// rdf
#[derive(AsnType,Clone, Decode, Debug, PartialEq)]
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


#[derive(AsnType,Clone)]
#[rasn(automatic_tags)]
pub struct RDFObject {
    pub value:              rasn::types::OctetString,
    pub _type:              rasn::types::OctetString,
    pub lang:               rasn::types::OctetString,
    pub dataType:           rasn::types::OctetString,
}


#[derive(AsnType,Clone)]
#[rasn(automatic_tags)]
pub struct RDFPredicate {
    pub predicate:          rasn::types::OctetString,
    pub rdfObjects:         Vec<RDFObject>,
}


#[derive(AsnType,Clone)]
#[rasn(automatic_tags)]
pub struct RDFSubject {
    pub subject:            rasn::types::OctetString,
    pub rdfPredicates:      Vec<RDFPredicate>
}


#[derive(AsnType,Clone)]
#[rasn(automatic_tags)]
pub struct RDFNT {
    pub version:            u32,
    pub ntSubject:          rasn::types::OctetString,
    pub ntPredicate:        rasn::types::OctetString,
    pub ntObject:           rasn::types::OctetString,
}


#[derive(AsnType,Clone)]
#[rasn(automatic_tags)]
pub struct RDFNtGroup {
    pub version:            u32,
    pub rdfNT:              Vec<RDFNT>,
}


#[derive(AsnType,Clone)]
#[rasn(automatic_tags)]
pub struct RDFModel {
    action:             RDFChange,
    rdfSubjects:        Vec<RDFSubject>,
    rdfNtGroups:        Vec<RDFNtGroup>,
}


// Election
#[derive(AsnType,Clone)]
#[rasn(automatic_tags)]
pub struct Election {
    version:             u32,
    date:                rasn::types::UtcTime,
    accountHash:         rasn::types::OctetString,
    acceptedCheck:       SoftwareConsensus,
    validateCheck:       SoftwareConsensus
}


#[derive(AsnType,Clone)]
#[rasn(automatic_tags)]
pub struct SignedElection {
    version:             u32,
    election:            Election,
    electionHash:        rasn::types::OctetString,
    signature:           rasn::types::OctetString,
}

#[derive(AsnType,Clone)]
#[rasn(automatic_tags)]
pub struct ElectNode {
    version:             u32,
    date:                rasn::types::UtcTime,
    accountHash:         rasn::types::OctetString,
    electedNode:         SignedElection,
    alternatives:        Vec<SignedElection>,
    acceptedCheck:       SoftwareConsensus,
    validateCheck:       SoftwareConsensus,
}

#[derive(AsnType,Clone)]
#[rasn(automatic_tags)]
pub struct SignedElectNode {
    version:              u32,
    electedNode:          ElectNode,
    electedHash:          rasn::types::OctetString,
    signature:            rasn::types::OctetString,
}