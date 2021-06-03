#[derive(Clone, Debug, Deserialize)]
pub struct Skill {
    pub id: i32,
    pub skill: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Employee {
    pub employee_number: String,
    pub name: String,
}
