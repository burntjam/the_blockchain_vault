use crate::StoreState;
use crate::store_result_set::*;
use asn1_lib::*;
use std::sync::Arc;

pub struct MockStoreState;

impl MockStoreState {
    pub fn new() -> Arc<dyn StoreState> {
        Arc::new(MockStoreState{})
    }
}

impl StoreState for MockStoreState {
    // persist rdf entries
    fn persistAsnTripples(&self, tripples: &Vec<RDFNT>) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn persistAsnSubjects(&self, subjects: &Vec<RDFSubject>) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    // remove rdf entries
    fn removeAsnTripples(&self, tripples: &Vec<RDFNT>) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn removeAsnSubjects(&self, subjects: &Vec<RDFSubject>) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn query(&self, query: &String) -> Result<RdfResultSet, Box<dyn std::error::Error>> {
        Ok(RdfResultSet{column_headings:vec![],rows:vec![]})
    }

    // transaction management
    fn commit(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    fn rollback(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    // store 
    fn increment(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn release(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        Ok(false)
    }
}