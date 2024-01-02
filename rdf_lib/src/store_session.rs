use asn1_lib::*;
use crate::store_result_set::*;

pub trait StoreSession: Send + Sync {
    
    // persist rdf entries
    fn persistAsnTripples(&self, tripples: &Vec<RDFNT>) -> Result<(), Box<dyn std::error::Error>>;
    fn persistAsnSubjects(&self, subjects: &Vec<RDFSubject>) -> Result<(), Box<dyn std::error::Error>>;

    // remove rdf entries
    fn removeAsnTripples(&self, tripples: &Vec<RDFNT>) -> Result<(), Box<dyn std::error::Error>>;
    fn removeAsnSubjects(&self, subjects: &Vec<RDFSubject>) -> Result<(), Box<dyn std::error::Error>>;

    // perform the query
    fn query(&self, query: &String) -> Result<RdfResultSet, Box<dyn std::error::Error>>;

    // transaction management
    fn commit(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn rollback(&self) -> Result<(), Box<dyn std::error::Error>>;

}


