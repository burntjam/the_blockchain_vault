use asn1_lib::*;
use crate::store_result_set::*;

pub trait StoreState: Send + Sync {
    
    // persist rdf entries
    fn persistAsnTripples(&self, tripples: &Vec<RDFNT>) -> Result<(), Box<dyn std::error::Error>>;
    fn persistAsnSubjects(&self, subjects: &Vec<RDFSubject>) -> Result<(), Box<dyn std::error::Error>>;

    // remove rdf entries
    fn removeAsnTripples(&self, tripples: &Vec<RDFNT>) -> Result<(), Box<dyn std::error::Error>>;
    fn removeAsnSubjects(&self, subjects: &Vec<RDFSubject>) -> Result<(), Box<dyn std::error::Error>>;

    // retrieve methods
    fn query(&self, query: &String) -> Result<RdfResultSet, Box<dyn std::error::Error>>;

    // transaction management
    fn commit(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn rollback(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    // store 
    fn increment(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn release(&mut self) -> Result<bool, Box<dyn std::error::Error>>;
}




