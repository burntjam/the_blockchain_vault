use crate::StoreState;
use crate::store_result_set::*;
use asn1_lib::*;
use std::cell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Error;
use std::sync::{Mutex,Arc};
use oxigraph::store::{StorageError,Store};
use oxigraph::model::*;
use oxigraph::sparql::QueryResults;
use crate::oxigraph_store_result_set::*;
use crate::oxigraph_error::*;


pub struct OxigraphStoreState {
    ref_count: usize,
    active: bool,
    store: Arc<Mutex<Store>>,   
    change_store: Arc<Mutex<Store>>,
    remove_store: Arc<Mutex<Store>>,
}

impl OxigraphStoreState {
    pub fn new(store: Arc<Mutex<Store>>) -> Result<Arc<Mutex<dyn StoreState>>, Box<dyn std::error::Error>> {
        let change_store = Arc::new(Mutex::new(Store::new()?));
        let remove_store = Arc::new(Mutex::new(Store::new()?));
        Ok(Arc::new(Mutex::new(OxigraphStoreState{ref_count:1,active:true.clone(),store,change_store,remove_store})))
    }
}

impl StoreState for OxigraphStoreState {
    // persist rdf entries
    fn persistAsnTripples(&self, tripples: &Vec<RDFNT>) -> Result<(), Box<dyn std::error::Error>> {
        let mut change_store = self.change_store.lock().unwrap();
        change_store.transaction(|mut transaction|{
            for tripple in tripples {
                let subject_str = String::from_utf8(tripple.ntSubject.to_vec())
                    .map_err(|e| StorageError::Other(Box::new(e)))?;
                let predicate_str = String::from_utf8(tripple.ntPredicate.to_vec())
                    .map_err(|e| StorageError::Other(Box::new(e)))?;
                let object_str = String::from_utf8(tripple.ntObject.to_vec())
                    .map_err(|e| StorageError::Other(Box::new(e)))?;

                let quad = QuadRef::new(
                    NamedNodeRef::new_unchecked(subject_str.as_str()),
                    NamedNodeRef::new_unchecked(predicate_str.as_str()),
                    NamedNodeRef::new_unchecked(object_str.as_str()),
                    GraphNameRef::DefaultGraph,
                );

                transaction.insert(quad)?;
            }
            Result::<_, StorageError>::Ok(())
        })?;
        Ok(())
    }

    fn persistAsnSubjects(&self, subjects: &Vec<RDFSubject>) -> Result<(), Box<dyn std::error::Error>> {
        let mut change_store = self.change_store.lock().unwrap();
        change_store.transaction(|mut transaction|{
            for subject in subjects {
                for predicate in &subject.rdfPredicates {
                    for object in &predicate.rdfObjects {
                        let subject_str = String::from_utf8(subject.subject.to_vec())
                            .map_err(|e| StorageError::Other(Box::new(e)))?;
                        let predicate_str = String::from_utf8(predicate.predicate.to_vec())
                            .map_err(|e| StorageError::Other(Box::new(e)))?;
                        let object_str = String::from_utf8(object.value.to_vec())
                            .map_err(|e| StorageError::Other(Box::new(e)))?;

                        let quad = QuadRef::new(
                            NamedNodeRef::new_unchecked(subject_str.as_str()),
                            NamedNodeRef::new_unchecked(predicate_str.as_str()),
                            NamedNodeRef::new_unchecked(object_str.as_str()),
                            GraphNameRef::DefaultGraph,
                        );

                        transaction.insert(quad)?;
                    }
                }
            }
            Result::<_, StorageError>::Ok(())
        })?;
        Ok(())
    }

    // remove rdf entries
    fn removeAsnTripples(&self, tripples: &Vec<RDFNT>) -> Result<(), Box<dyn std::error::Error>> {
        let mut remove_store = self.remove_store.lock().unwrap();
        remove_store.transaction(|mut transaction|{
            for tripple in tripples {
                let subject_str = String::from_utf8(tripple.ntSubject.to_vec())
                    .map_err(|e| StorageError::Other(Box::new(e)))?;
                let predicate_str = String::from_utf8(tripple.ntPredicate.to_vec())
                    .map_err(|e| StorageError::Other(Box::new(e)))?;
                let object_str = String::from_utf8(tripple.ntObject.to_vec())
                    .map_err(|e| StorageError::Other(Box::new(e)))?;

                let quad = QuadRef::new(
                    NamedNodeRef::new_unchecked(subject_str.as_str()),
                    NamedNodeRef::new_unchecked(predicate_str.as_str()),
                    NamedNodeRef::new_unchecked(object_str.as_str()),
                    GraphNameRef::DefaultGraph,
                );

                transaction.insert(quad)?;
            }
            Result::<_, StorageError>::Ok(())
        })?;
        Ok(())
    }

    fn removeAsnSubjects(&self, subjects: &Vec<RDFSubject>) -> Result<(), Box<dyn std::error::Error>> {
        let mut remove_store = self.remove_store.lock().unwrap();
        remove_store.transaction(|mut transaction|{
            for subject in subjects {
                for predicate in &subject.rdfPredicates {
                    for object in &predicate.rdfObjects {
                        let subject_str = String::from_utf8(subject.subject.to_vec())
                            .map_err(|e| StorageError::Other(Box::new(e)))?;
                        let predicate_str = String::from_utf8(predicate.predicate.to_vec())
                            .map_err(|e| StorageError::Other(Box::new(e)))?;
                        let object_str = String::from_utf8(object.value.to_vec())
                            .map_err(|e| StorageError::Other(Box::new(e)))?;

                        let quad = QuadRef::new(
                            NamedNodeRef::new_unchecked(subject_str.as_str()),
                            NamedNodeRef::new_unchecked(predicate_str.as_str()),
                            NamedNodeRef::new_unchecked(object_str.as_str()),
                            GraphNameRef::DefaultGraph,
                        );

                        transaction.insert(quad)?;
                    }
                }
            }
            Result::<_, StorageError>::Ok(())
        })?;
        Ok(())
    }

    fn query(&self, query: &String) -> Result<RdfResultSet, Box<dyn std::error::Error>> {
        let mut store = self.store.lock().unwrap();

        let mut column_headings: HashSet<String> = HashSet::new();
        let mut result_set_rows: Vec<HashMap<String, RdfCell>> = Vec::new();
        if let QueryResults::Solutions(mut solutions) = store.query(query)? {
            for solution in solutions {
                let row = solution?;
                let mut result_set_row: HashMap<String, RdfCell> = HashMap::new();
                for (variable, term) in row.iter() {
                    let cellValue = RdfCell::newCellForValueTerm(variable, term)?;
                    column_headings.insert(cellValue.name.clone());
                    result_set_row.insert(cellValue.name.clone(), cellValue);
                }
                result_set_rows.insert(result_set_rows.len(),result_set_row);
            }
        }
        Ok(RdfResultSet{column_headings:column_headings.into_iter().collect(),rows:result_set_rows})
    }

    // transaction management
    fn commit(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.active {
            return Err(Box::new(OxigraphStoreError{message:"State is no longer active.".to_string()}));
        }
        println!("Before accesssing the store");
        let store_ref = self.store.lock().unwrap();
        println!("Before creating the transaction");
        store_ref.transaction(|mut transaction|{
            println!("Remove the entries");
            for q in 
            self.remove_store.lock().unwrap().quads_for_pattern(None, None, None, None) {
                let q = q?;
                println!("Remove entry");
                transaction.remove(QuadRef::new(&q.subject, &q.predicate, &q.object, &q.graph_name))?;
            };
            println!("Insert the entries");
            for q in 
            self.change_store.lock().unwrap().quads_for_pattern(None, None, None, None) {
                let q = q?;
                println!("Insert entry");
                transaction.insert(QuadRef::new(&q.subject, &q.predicate, &q.object, &q.graph_name))?;
            };
            println!("Return the results");
            Result::<_, StorageError>::Ok(())
        })?;
        println!("Change the state");
        self.active = false;
        Ok(())
    }


    fn rollback(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.active {
            return Err(Box::new(OxigraphStoreError{message:"State is no longer active.".to_string()}));
        }
        self.active = false;
        Ok(())
    }

    // store 
    fn increment(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.ref_count += 1;
        Ok(())
    }

    fn release(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        self.ref_count -= 1;
        if self.ref_count <= 0 {
            if self.active {
                self.rollback();
            }
            return Ok(true);
        } else {
            return Ok(false);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use oxigraph::store::Store;
    use std::error::Error;
    use rasn::types::OctetString;

    #[test]
    fn test_store_state_subject_commit() -> Result<(), Box<dyn Error>> {
        let store = Arc::new(Mutex::new(Store::new()?));

        let store_state = OxigraphStoreState::new(store.clone())?;

        
        {
            let mut store_state_ref = store_state.lock().unwrap();

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

            store_state_ref.persistAsnSubjects(&subjects);

            // commit
            store_state_ref.commit();
        }

        {
            let query_str = "SELECT ?s ?p ?o WHERE { ?s ?p ?o }";

            let mut store_state_ref = store_state.lock().unwrap();

            let result = store_state_ref.query(&query_str.clone().to_string())?;

            assert_eq!(3,result.rows.len());
            assert_eq!(3,result.column_headings.len());
        }

        Ok(())
    }

    #[test]
    fn test_store_state_triple_commit() -> Result<(), Box<dyn Error>> {
        let store = Arc::new(Mutex::new(Store::new()?));

        let store_state = OxigraphStoreState::new(store.clone())?;

        
        {
            let mut store_state_ref = store_state.lock().unwrap();

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

            store_state_ref.persistAsnTripples(&triples);

            // commit
            store_state_ref.commit();
        }

        {
            let query_str = "SELECT ?s ?p ?o WHERE { ?s ?p ?o }";

            let mut store_state_ref = store_state.lock().unwrap();

            let result = store_state_ref.query(&query_str.clone().to_string())?;

            assert_eq!(3,result.rows.len());
            assert_eq!(3,result.column_headings.len());
        }

        Ok(())
    }

    #[test]
    fn test_store_state_subject_remove() -> Result<(), Box<dyn Error>> {
        let store = Arc::new(Mutex::new(Store::new()?));

        let store_state = OxigraphStoreState::new(store.clone())?;

        
        {
            let mut store_state_ref = store_state.lock().unwrap();

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

            store_state_ref.persistAsnSubjects(&subjects);

            // commit
            store_state_ref.commit();
        }

        {
            let query_str = "SELECT ?s ?p ?o WHERE { ?s ?p ?o }";

            let mut store_state_ref = store_state.lock().unwrap();

            let result = store_state_ref.query(&query_str.clone().to_string())?;

            assert_eq!(3,result.rows.len());
            assert_eq!(3,result.column_headings.len());
        }

        let store_state_remove = OxigraphStoreState::new(store.clone())?;

        {
            let mut store_state_ref = store_state_remove.lock().unwrap();

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

            store_state_ref.removeAsnSubjects(&subjects);

            // commit
            store_state_ref.commit();
        }

        {
            let query_str = "SELECT ?s ?p ?o WHERE { ?s ?p ?o }";

            let mut store_state_ref = store_state_remove.lock().unwrap();

            let result = store_state_ref.query(&query_str.clone().to_string())?;

            assert_eq!(0,result.column_headings.len());
            assert_eq!(0,result.rows.len());
            
        }

        Ok(())
    }

    #[test]
    fn test_store_state_tripple_remove() -> Result<(), Box<dyn Error>> {
        let store = Arc::new(Mutex::new(Store::new()?));

        let store_state = OxigraphStoreState::new(store.clone())?;

        {
            let mut store_state_ref = store_state.lock().unwrap();

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

            store_state_ref.persistAsnTripples(&triples);

            // commit
            store_state_ref.commit();
        }

        {
            let query_str = "SELECT ?s ?p ?o WHERE { ?s ?p ?o }";

            let mut store_state_ref = store_state.lock().unwrap();

            let result = store_state_ref.query(&query_str.clone().to_string())?;

            assert_eq!(3,result.rows.len());
            assert_eq!(3,result.column_headings.len());
        }

        let store_state_remove = OxigraphStoreState::new(store.clone())?;

        {
            let mut store_state_ref = store_state_remove.lock().unwrap();

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

            store_state_ref.removeAsnTripples(&triples);

            // commit
            store_state_ref.commit();
        }

        {
            let query_str = "SELECT ?s ?p ?o WHERE { ?s ?p ?o }";

            let mut store_state_ref = store_state_remove.lock().unwrap();

            let result = store_state_ref.query(&query_str.clone().to_string())?;

            assert_eq!(0,result.column_headings.len());
            assert_eq!(0,result.rows.len());
            
        }

        Ok(())
    }

    #[test]
    fn test_store_state_subject_rollback() -> Result<(), Box<dyn Error>> {
        let store = Arc::new(Mutex::new(Store::new()?));

        let store_state = OxigraphStoreState::new(store.clone())?;

        {
            let mut store_state_ref = store_state.lock().unwrap();

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

            store_state_ref.persistAsnSubjects(&subjects);


            store_state_ref.rollback()?;
        }

        {
            let query_str = "SELECT ?s ?p ?o WHERE { ?s ?p ?o }";

            let mut store_state_ref = store_state.lock().unwrap();

            let result = store_state_ref.query(&query_str.clone().to_string())?;

            assert_eq!(0,result.rows.len());
            assert_eq!(0,result.rows.len());
        }
        Ok(())
    }
}