use crate::models::employee;
use rocket_contrib::json::Json;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>/<age>")]
pub fn hello(name: String, age: u8) -> String {
    let mage = age + 2;
    format!("Hello, {} year old named {}!", mage, name)
}

#[ post("/employee",format = "json",data="<emp>") ]
pub fn post_employee(emp : Json<Vec<employee::Employee>>) -> 
    Json<Vec<employee::Employee>> {
    
    let mut employee_rec : Vec<employee::Employee> = Vec::new();
    
    for i in 0..emp.len()  {
        let e = employee::Employee{
          name   : emp[i].name.to_string(),
          age    : emp[i].age,
          dept   : emp[i].dept.to_string(),
          skills : emp[i].skills.clone()
        };
        
        employee_rec.push( e );            
    }
    
    Json( employee_rec )
}


#[get("/employee")]
pub fn employee() -> Json<employee::Employee> {
    let skill1 = employee::Skill {
        id: 1,
        desc: String::from("programmer"),
    };
    let skill2 = employee::Skill {
        id: 2,
        desc: String::from("designer"),
    };
    let skill3 = employee::Skill {
        id: 3,
        desc: String::from("debugger"),
    };

    let mut skills: Vec<employee::Skill> = Vec::new();
    skills.push(skill1);
    skills.push(skill2);
    skills.push(skill3);

    let _emp = employee::Employee {
        name: String::from("Mario Basa"),
        age: 42,
        dept: String::from("Production"),
        skills: skills,
    };

    Json(_emp)
}
