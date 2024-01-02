use crate::store_result_set::*;
use std::sync::{Mutex,Arc};
use std::ops::Drop;
use asn1_lib::*;
use crate::{StoreState, StoreSession};


pub struct OxigraphStoreSession {
    store_state: Arc<Mutex<dyn StoreState>>,
}

impl OxigraphStoreSession {
    pub fn new(store_state: Arc<Mutex<dyn StoreState>>) -> Result<Box<Mutex<dyn StoreSession>>, Box<dyn std::error::Error>> {
        store_state.lock().unwrap().increment()?;
        Ok(Box::new(Mutex::new(
            OxigraphStoreSession{
                store_state})))
    }
}

impl StoreSession for OxigraphStoreSession {

    // persist rdf entries
    fn persistAsnTripples(&self, tripples: &Vec<RDFNT>) -> Result<(), Box<dyn std::error::Error>> {
        Ok(self.store_state.lock().unwrap().persistAsnTripples(tripples)?)
    }

    fn persistAsnSubjects(&self, subjects: &Vec<RDFSubject>) -> Result<(), Box<dyn std::error::Error>> {
        Ok(self.store_state.lock().unwrap().persistAsnSubjects(subjects)?)
    }

    // remove rdf entries
    fn removeAsnTripples(&self, tripples: &Vec<RDFNT>) -> Result<(), Box<dyn std::error::Error>> {
        Ok(self.store_state.lock().unwrap().removeAsnTripples(tripples)?)
    }

    fn removeAsnSubjects(&self, subjects: &Vec<RDFSubject>) -> Result<(), Box<dyn std::error::Error>> {
        Ok(self.store_state.lock().unwrap().removeAsnSubjects(subjects)?)
    }

    // perform the query
    fn query(&self, query: &String) -> Result<RdfResultSet, Box<dyn std::error::Error>> {
        let rdf_result_set = self.store_state.lock().unwrap().query(query)?;
        Ok(rdf_result_set)
    }

    // transaction management
    fn commit(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(self.store_state.lock().unwrap().commit()?)
    }
    fn rollback(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(self.store_state.lock().unwrap().rollback()?)
    }
    

}

impl Drop for OxigraphStoreSession {
    fn drop(&mut self) {
        self.store_state.lock().unwrap().release();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxigraph::store::Store;
    use std::error::Error;
    use rasn::types::OctetString;
    use crate::{OxigraphStoreState};

    #[test]
    fn test_session_state() -> Result<(), Box<dyn Error>> {
        let store = Arc::new(Mutex::new(Store::new()?));

        let store_state = OxigraphStoreState::new(store.clone())?;

        let session = OxigraphStoreSession::new(store_state.clone())?;

        let session_state = session.lock().unwrap();

        Ok(())
    }


    #[test]
    fn test_session_subject_persist() -> Result<(), Box<dyn Error>> {
        let store = Arc::new(Mutex::new(Store::new()?));

        let store_state = OxigraphStoreState::new(store.clone())?;

        {
            let session = OxigraphStoreSession::new(store_state.clone())?;

            let session_state = session.lock().unwrap();
        
            // add subjects
            let mut subjects: Vec<RDFSubject> = Vec::new();
            let mut predicates: Vec<RDFPredicate> = Vec::new();
            let mut objects: Vec<RDFObject> = Vec::new();
            objects.push(RDFObject{
                value:OctetString::from("test".as_bytes().to_vec()),
                _type:OctetString::from("xsd:string".as_bytes().to_vec()),
                lang:OctetString::from("test".as_bytes().to_vec()),
                dataType:OctetString::from("xsd:string".as_bytes().to_vec()),
            });
            predicates.push(RDFPredicate{predicate:OctetString::from("test".as_bytes().to_vec()),rdfObjects:objects});
            subjects.push( RDFSubject{subject:OctetString::from("test".as_bytes().to_vec()),rdfPredicates:predicates.clone()});
            subjects.push( RDFSubject{subject:OctetString::from("test2".as_bytes().to_vec()),rdfPredicates:predicates.clone()});
            subjects.push( RDFSubject{subject:OctetString::from("test3".as_bytes().to_vec()),rdfPredicates:predicates.clone()});

            session_state.persistAsnSubjects(&subjects);

            // commit
            session_state.commit();
        }

        {
            let session = OxigraphStoreSession::new(store_state.clone())?;

            let session_state = session.lock().unwrap();
        
            let query_str = "SELECT ?s ?p ?o WHERE { ?s ?p ?o }";

            let result = session_state.query(&query_str.clone().to_string())?;

            assert_eq!(3,result.rows.len());
            assert_eq!(3,result.column_headings.len());
        }

        Ok(())
    }

    #[test]
    fn test_session_subject_remove() -> Result<(), Box<dyn Error>> {
        let store = Arc::new(Mutex::new(Store::new()?));

        let store_state = OxigraphStoreState::new(store.clone())?;

        {
            let session = OxigraphStoreSession::new(store_state.clone())?;

            let session_state = session.lock().unwrap();
        
            // add subjects
            let mut subjects: Vec<RDFSubject> = Vec::new();
            let mut predicates: Vec<RDFPredicate> = Vec::new();
            let mut objects: Vec<RDFObject> = Vec::new();
            objects.push(RDFObject{
                value:OctetString::from("test".as_bytes().to_vec()),
                _type:OctetString::from("xsd:string".as_bytes().to_vec()),
                lang:OctetString::from("test".as_bytes().to_vec()),
                dataType:OctetString::from("xsd:string".as_bytes().to_vec()),
            });
            predicates.push(RDFPredicate{predicate:OctetString::from("test".as_bytes().to_vec()),rdfObjects:objects});
            subjects.push( RDFSubject{subject:OctetString::from("test".as_bytes().to_vec()),rdfPredicates:predicates.clone()});
            subjects.push( RDFSubject{subject:OctetString::from("test2".as_bytes().to_vec()),rdfPredicates:predicates.clone()});
            subjects.push( RDFSubject{subject:OctetString::from("test3".as_bytes().to_vec()),rdfPredicates:predicates.clone()});

            session_state.persistAsnSubjects(&subjects);

            // commit
            session_state.commit();
        }

        {
            let session = OxigraphStoreSession::new(store_state.clone())?;

            let session_state = session.lock().unwrap();
        
            let query_str = "SELECT ?s ?p ?o WHERE { ?s ?p ?o }";

            let result = session_state.query(&query_str.clone().to_string())?;

            assert_eq!(3,result.rows.len());
            assert_eq!(3,result.column_headings.len());
        }

        let remove_store_state = OxigraphStoreState::new(store.clone())?;

        {
            let session = OxigraphStoreSession::new(remove_store_state.clone())?;

            let session_state = session.lock().unwrap();
        
            // add subjects
            let mut subjects: Vec<RDFSubject> = Vec::new();
            let mut predicates: Vec<RDFPredicate> = Vec::new();
            let mut objects: Vec<RDFObject> = Vec::new();
            objects.push(RDFObject{
                value:OctetString::from("test".as_bytes().to_vec()),
                _type:OctetString::from("xsd:string".as_bytes().to_vec()),
                lang:OctetString::from("test".as_bytes().to_vec()),
                dataType:OctetString::from("xsd:string".as_bytes().to_vec()),
            });
            predicates.push(RDFPredicate{predicate:OctetString::from("test".as_bytes().to_vec()),rdfObjects:objects});
            subjects.push( RDFSubject{subject:OctetString::from("test".as_bytes().to_vec()),rdfPredicates:predicates.clone()});
            subjects.push( RDFSubject{subject:OctetString::from("test2".as_bytes().to_vec()),rdfPredicates:predicates.clone()});
            subjects.push( RDFSubject{subject:OctetString::from("test3".as_bytes().to_vec()),rdfPredicates:predicates.clone()});

            session_state.removeAsnSubjects(&subjects);

            // commit
            session_state.commit();
        }

        {
            let session = OxigraphStoreSession::new(remove_store_state.clone())?;

            let session_state = session.lock().unwrap();
        
            let query_str = "SELECT ?s ?p ?o WHERE { ?s ?p ?o }";

            let result = session_state.query(&query_str.clone().to_string())?;

            assert_eq!(0,result.rows.len());
            assert_eq!(0,result.column_headings.len());
        }

        Ok(())
    }


    #[test]
    fn test_session_triple_persist() -> Result<(), Box<dyn Error>> {
        let store = Arc::new(Mutex::new(Store::new()?));

        let store_state = OxigraphStoreState::new(store.clone())?;

        {
            let session = OxigraphStoreSession::new(store_state.clone())?;

            let session_state = session.lock().unwrap();
        
            // add subjects
            let mut triples: Vec<RDFNT> = Vec::new();
            triples.push(RDFNT { version: 1, 
                ntSubject:OctetString::from("test".as_bytes().to_vec()), 
                ntPredicate:OctetString::from("test".as_bytes().to_vec()), 
                ntObject:OctetString::from("test".as_bytes().to_vec()),});
            triples.push(RDFNT { version: 1, 
                ntSubject:OctetString::from("test2".as_bytes().to_vec()), 
                ntPredicate:OctetString::from("test".as_bytes().to_vec()), 
                ntObject:OctetString::from("test".as_bytes().to_vec()),});
            triples.push(RDFNT { version: 1, 
                ntSubject:OctetString::from("test3".as_bytes().to_vec()), 
                ntPredicate:OctetString::from("test".as_bytes().to_vec()), 
                ntObject:OctetString::from("test".as_bytes().to_vec()),});

            session_state.persistAsnTripples(&triples);

            // commit
            session_state.commit();
        }

        {
            let session = OxigraphStoreSession::new(store_state.clone())?;

            let session_state = session.lock().unwrap();
        
            let query_str = "SELECT ?s ?p ?o WHERE { ?s ?p ?o }";

            let result = session_state.query(&query_str.clone().to_string())?;

            assert_eq!(3,result.rows.len());
            assert_eq!(3,result.column_headings.len());
        }

        Ok(())
    }


    #[test]
    fn test_session_triple_remove() -> Result<(), Box<dyn Error>> {
        let store = Arc::new(Mutex::new(Store::new()?));

        let store_state = OxigraphStoreState::new(store.clone())?;

        {
            let session = OxigraphStoreSession::new(store_state.clone())?;

            let session_state = session.lock().unwrap();
        
            // add subjects
            let mut triples: Vec<RDFNT> = Vec::new();
            triples.push(RDFNT { version: 1, 
                ntSubject:OctetString::from("test".as_bytes().to_vec()), 
                ntPredicate:OctetString::from("test".as_bytes().to_vec()), 
                ntObject:OctetString::from("test".as_bytes().to_vec()),});
            triples.push(RDFNT { version: 1, 
                ntSubject:OctetString::from("test2".as_bytes().to_vec()), 
                ntPredicate:OctetString::from("test".as_bytes().to_vec()), 
                ntObject:OctetString::from("test".as_bytes().to_vec()),});
            triples.push(RDFNT { version: 1, 
                ntSubject:OctetString::from("test3".as_bytes().to_vec()), 
                ntPredicate:OctetString::from("test".as_bytes().to_vec()), 
                ntObject:OctetString::from("test".as_bytes().to_vec()),});

            session_state.persistAsnTripples(&triples);

            // commit
            session_state.commit();
        }

        {
            let session = OxigraphStoreSession::new(store_state.clone())?;

            let session_state = session.lock().unwrap();
        
            let query_str = "SELECT ?s ?p ?o WHERE { ?s ?p ?o }";

            let result = session_state.query(&query_str.clone().to_string())?;

            assert_eq!(3,result.rows.len());
            assert_eq!(3,result.column_headings.len());
        }


        let remove_store_state = OxigraphStoreState::new(store.clone())?;

        {
            let session = OxigraphStoreSession::new(remove_store_state.clone())?;

            let session_state = session.lock().unwrap();
        
            // add subjects
            let mut triples: Vec<RDFNT> = Vec::new();
            triples.push(RDFNT { version: 1, 
                ntSubject:OctetString::from("test".as_bytes().to_vec()), 
                ntPredicate:OctetString::from("test".as_bytes().to_vec()), 
                ntObject:OctetString::from("test".as_bytes().to_vec()),});
            triples.push(RDFNT { version: 1, 
                ntSubject:OctetString::from("test2".as_bytes().to_vec()), 
                ntPredicate:OctetString::from("test".as_bytes().to_vec()), 
                ntObject:OctetString::from("test".as_bytes().to_vec()),});
            triples.push(RDFNT { version: 1, 
                ntSubject:OctetString::from("test3".as_bytes().to_vec()), 
                ntPredicate:OctetString::from("test".as_bytes().to_vec()), 
                ntObject:OctetString::from("test".as_bytes().to_vec()),});

            session_state.removeAsnTripples(&triples);

            // commit
            session_state.commit();
        }

        {
            let session = OxigraphStoreSession::new(store_state.clone())?;

            let session_state = session.lock().unwrap();
        
            let query_str = "SELECT ?s ?p ?o WHERE { ?s ?p ?o }";

            let result = session_state.query(&query_str.clone().to_string())?;

            assert_eq!(0,result.rows.len());
            assert_eq!(0,result.column_headings.len());
        }

        Ok(())
    }

}