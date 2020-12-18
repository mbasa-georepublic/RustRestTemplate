use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Employee{
    pub name  : String,
    pub age   : u16,
    pub dept  : String,
    pub skills: Vec<Skill>//[Skill;3]
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Skill {
    pub id: u16,
    pub desc: String
}
/*
impl Skill {
    pub fn new() -> Skill {
        Skill{id:0,desc:String::from("x")}
    }
}
*/