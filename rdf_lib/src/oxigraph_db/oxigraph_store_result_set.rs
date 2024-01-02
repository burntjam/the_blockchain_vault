use crate::store_result_set::*;
use oxigraph::model::*;
use oxigraph::sparql::*;
use oxigraph::model::vocab::{rdf, xsd};


impl RdfCellValue {
    fn newCellValueFromTerm(term: &Term) -> Result<Self, Box<dyn std::error::Error>> {
        match term {
            Term::NamedNode(iri) => {
                print!("This is a node");
                return  Ok(RdfCellValue::Node(iri.clone().to_string()));
            },
            Term::BlankNode(bnode) => {
                return Ok(RdfCellValue::Blank());
            },
            Term::Literal(literal) => {
                match literal.datatype() {
                    xsd::INTEGER => {
                        let value_str = literal.value();
                        return Ok(RdfCellValue::Int(value_str.parse::<i32>().unwrap()));
                    }
                    xsd::FLOAT => {
                        let value_str = literal.value();
                        return Ok(RdfCellValue::Float(value_str.parse::<f64>().unwrap()));
                    }
                    xsd::STRING => {
                        return Ok(RdfCellValue::Text(literal.value().to_string()));
                    }
                    _ => {
                        return Ok(RdfCellValue::Text(literal.value().to_string()));
                    }
                }
            },
            Term::Triple(var) => {
                print!("This is a triple");
            },
            _ => {
                return Ok(RdfCellValue::Blank());
            }
        }

        Ok(RdfCellValue::Blank())
    }
}

impl RdfCell {
    pub fn newCellForValueTerm(variable: &Variable, term: &Term) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(RdfCell{name: variable.clone().into_string(), value: RdfCellValue::newCellValueFromTerm(term).unwrap()})
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use oxigraph::io::{DatasetFormat, GraphFormat};
    use oxigraph::model::vocab::{rdf, xsd};
    use oxigraph::model::*;
    use oxigraph::store::Store;
    use std::error::Error;
    use std::fs::File;
    use std::io::BufReader;
    use std::collections::HashMap;

    #[test]
    fn test_rdf_cell() -> Result<(), Box<dyn Error>> {
        let store = Store::new()?;

        let file_path = "test/data/small_test_dataset_50_triples.ttl"; // Update with the actual path

        // Open the file in read-only mode
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        // Load the data from the Turtle file into the store
        store.load_graph(reader, GraphFormat::Turtle, &GraphName::DefaultGraph, None)?;

        // Write a SPARQL query
        let query_str = "SELECT ?s ?p ?o WHERE { ?s ?p ?o }";

        if let QueryResults::Solutions(mut solutions) = store.query(query_str)? {
            for solution in solutions {
                let row = solution?;
                let mut result_set_row: HashMap<String, RdfCell> = HashMap::new();
                for (variable, term) in row.iter() {
                    let result = RdfCell::newCellForValueTerm(variable, term)?;
                    println!("Cell name: {} {}", result.name, result.value.to_string());
                }
            }
        } else {
            panic!("Test failed its gone away");
        }

        Ok(())
    }
}

