#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Skill {
    pub id: i32,
    pub skill: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Employee {
    pub employee_number: String,
    pub name: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct EmployeeSkill {
    pub employee_number: String,
    pub skill_id: i32,
}
