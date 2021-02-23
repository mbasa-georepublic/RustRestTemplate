use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Employee {
    pub name: String,
    pub age: u16,
    pub dept: String,
    pub skills: Vec<Skill>, 
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Skill {
    pub id: u16,
    pub desc: String,
}
