use crate::store_result_set::*;
use std::sync::{Mutex,Arc};
use std::ops::Drop;
use asn1_lib::*;
use crate::{StoreState, StoreSession};


pub struct MockStoreSession {
}

impl MockStoreSession {
    pub fn new() -> Result<Box<Mutex<dyn StoreSession>>, Box<dyn std::error::Error>> {
        Ok(Box::new(Mutex::new(
            MockStoreSession{})))
    }
}

impl StoreSession for MockStoreSession {
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

    // perform the query
    fn query(&self, query: &String) -> Result<RdfResultSet, Box<dyn std::error::Error>> {
        Ok(RdfResultSet{column_headings:vec![],rows:vec![]})
    }

    // transaction management
    fn commit(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    fn rollback(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}


