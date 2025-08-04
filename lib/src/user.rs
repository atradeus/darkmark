use std::collections::HashSet;
use std::fmt;
use strum_macros::EnumString;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub anonymous: bool,
    pub roles: HashSet<Role>,
}

impl User {
    pub fn has_role(&self, role: Role) -> bool {
        self.roles.contains(&role)
    }

    pub fn name(&self) -> String {
        format!("{} {}", self.first_name, &self.last_name)
    }
}

#[derive(Serialize, Deserialize, Clone, Hash, Eq, PartialEq, Debug, EnumString)]
pub enum Role {
    Admin,
    User,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Credentials {
    pub email: String,
    pub password: String,
    pub remember: Option<bool>,
}

impl fmt::Display for Credentials {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Credentials(email={}, password=*****)", self.email)
    }
}