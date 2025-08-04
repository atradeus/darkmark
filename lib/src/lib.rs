use serde::{Deserialize, Serialize};

pub mod user;

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SelectOpt {
    pub value: String,
    pub label: String,
}

impl SelectOpt {
    // pub fn new(value: &'static str, label: &'static str) -> Option {
    //     Option { value, label }
    // }
    pub fn new(value: String, label: String) -> SelectOpt {
        SelectOpt { value, label }
    }
}


#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Region {
    pub code: String,
    pub name: String,
    pub geography: String,
}

