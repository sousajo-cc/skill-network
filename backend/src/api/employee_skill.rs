use rocket::Rocket;
use rocket_contrib::json::Json;

use crate::database::establish_connection;
use crate::database::models::employee_skill::{EmployeesSkill, NewEmployeesSkill};
use crate::database::models::skill::Skill;
use crate::database::models::employee::Employee;
use crate::result::BackendError;
use diesel::SqliteConnection;

#[get("/")]
fn get_all() -> Result<Json<Vec<EmployeesSkill>>, BackendError> {
    let connection = establish_connection();
    let employee_skill_list = EmployeesSkill::list(&connection)?;
    Ok(Json(employee_skill_list))
}

#[get("/<id>")]
fn get_by_id(id: i32) -> Result<Json<EmployeesSkill>, BackendError> {
    let connection = establish_connection();
    let employee_skill = EmployeesSkill::find(&connection, id)?;
    Ok(Json(employee_skill))
}

#[get("/list_employees_with_skill/<skill>")]
fn list_employees_with_skill(skill: i32) -> Result<Json<Vec<Employee>>, BackendError> {
    use diesel::QueryResult; //TODO: use our own error types so we don't have the outter api dependending on diesel

    let connection = establish_connection();
    let skill = Skill::find(&connection, skill)?;
    let employees  = EmployeesSkill::filter_by_skill(&connection, skill)?
        .into_iter()
        .map(|relation| Employee::find(&connection, &relation.employee_number))
        .collect::<QueryResult<Vec<Employee>>>()?;
    Ok(Json(employees))
}

#[get("/?search&<skill>&<name>&<employeenumber>")]
fn search(
    skill: Option<String>,
    name: Option<String>,
    employeenumber: Option<String>,
) -> Result<Json<Vec<EmployeesSkill>>, BackendError> {
    let connection = establish_connection();
    let matched_employees = match_employees(&connection, name, employeenumber)?;
    let matched_skills = match skill {
        Some(skill_name) => Skill::filter(&connection, &skill_name),
        None => Skill::list(&connection),
    }?;
    let result = EmployeesSkill::filter(&connection, matched_employees, matched_skills)?;
    Ok(Json(result))
}

fn match_employees(
    connection: &SqliteConnection,
    name: Option<String>,
    employeenumber: Option<String>
) -> Result<Vec<Employee>, BackendError> {
    let matched_employees_by_name = name.map(|employee_name|
        Employee::filter(connection, &employee_name)
    );
    let matched_employees_by_employeenumber = employeenumber.map(|employeenumber|
        Employee::find(connection, &employeenumber)
    );
    let matched_employees = match (matched_employees_by_name, matched_employees_by_employeenumber) {
        (None, None) => None,
        (None, Some(employee)) => Some(vec![employee?]),
        (Some(employees), None) => Some(employees?),
        (Some(employees), Some(employee)) => {
            let employee = employee?;
            if employees?.contains(&employee) {
                Some(vec![employee])
            } else {
                None
            }
        },
    };
    let matched_employees = match matched_employees {
        Some(vec) => vec,
        None => Employee::list(&connection)?,
    };
    Ok(matched_employees)
}

#[post("/", data="<employee_skill>")]
fn insert(employee_skill: Json<NewEmployeesSkill>) -> Result<Json<usize>, BackendError> {
    let connection = establish_connection();
    let employee_skill = employee_skill.into_inner();
    let insert = employee_skill.insert(&connection)?;
    Ok(Json(insert))
}

#[post("/batch", data="<employee_skills>")]
fn insert_batch(employee_skills: Json<Vec<NewEmployeesSkill>>) -> Result<Json<usize>, BackendError> {
    let connection = establish_connection();
    let employee_skills = employee_skills.into_inner();
    let insert = NewEmployeesSkill::insert_batch(&connection, employee_skills)?;
    Ok(Json(insert))
}

pub trait EmployeeSkillApi {
    fn mount_employee_skill_api(self, base: &str) -> Self;
}

impl EmployeeSkillApi for Rocket {
    fn mount_employee_skill_api(self, base: &str) -> Self {
        self.mount(base, routes![
            get_all,
            get_by_id,
            list_employees_with_skill,
            search,
            insert,
            insert_batch,
        ])
    }
}
