use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RdfCellValue {
    Int(i32),
    Float(f64),
    Text(String),
    Node(String),
    Blank(),
}

impl fmt::Display for RdfCellValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        match self {
            RdfCellValue::Int(value) => {
                write!(f, "({})", value)
            }
            RdfCellValue::Float(value) => {
                write!(f, "({})", value)
            }
            RdfCellValue::Text(value) => {
                write!(f, "({})", value)
            }
            RdfCellValue::Node(value) => {
                write!(f, "({})", value)
            }
            _ => {
                write!(f, "NA")
            }
        }
    }
}

// Define your message types using structs
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RdfCell{
    pub name: String,
    pub value: RdfCellValue,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RdfResultSet {
    pub column_headings: Vec<String>,
    pub rows: Vec<HashMap<String, RdfCell>>,
}