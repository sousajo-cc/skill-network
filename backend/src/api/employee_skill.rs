use rocket::Rocket;
use rocket_contrib::json::Json;

use crate::database::models::employee::Employee;
use crate::database::models::employee_skill::{EmployeesSkill, NewEmployeesSkill};
use crate::database::models::skill::Skill;
use crate::result::BackendError;
use crate::DbConn;

#[get("/")]
fn get_all(connection: DbConn) -> Result<Json<Vec<EmployeesSkill>>, BackendError> {
    let employee_skill_list = EmployeesSkill::list(&connection.0)?;
    Ok(Json(employee_skill_list))
}

#[get("/<id>")]
fn get_by_id(id: i32, connection: DbConn) -> Result<Json<EmployeesSkill>, BackendError> {
    let employee_skill = EmployeesSkill::find(&connection.0, id)?;
    Ok(Json(employee_skill))
}

#[get("/list_employees_with_skill/<skill>")]
fn list_employees_with_skill(
    skill: i32,
    connection: DbConn,
) -> Result<Json<Vec<Employee>>, BackendError> {
    use diesel::QueryResult; //TODO: use our own error types so we don't have the outer api dependending on diesel

    let skill = Skill::find(&connection.0, skill)?;
    let employees = EmployeesSkill::filter_by_skill(&connection.0, skill)?
        .into_iter()
        .map(|relation| Employee::find(&connection.0, &relation.employee_number))
        .collect::<QueryResult<Vec<Employee>>>()?;
    Ok(Json(employees))
}

#[get("/list_skills_for_employee/<employee>")]
fn list_skills_for_employee(
    employee: String,
    connection: DbConn,
) -> Result<Json<Vec<Skill>>, BackendError> {
    use diesel::QueryResult; //TODO: use our own error types so we don't have the outer api dependending on diesel

    let employee = Employee::find(&connection.0, &employee)?;
    let skills = EmployeesSkill::filter_by_employee(&connection.0, employee)?
        .into_iter()
        .map(|relation| Skill::find(&connection.0, relation.skill_id))
        .collect::<QueryResult<Vec<Skill>>>()?;
    Ok(Json(skills))
}

#[get("/?search&<skill>&<name>&<employeenumber>")]
fn search(
    skill: Option<String>,
    name: Option<String>,
    employeenumber: Option<String>,
    connection: DbConn,
) -> Result<Json<Vec<EmployeesSkill>>, BackendError> {
    let matched_employees = match_employees(&connection, name, employeenumber)?;
    let matched_skills = match skill {
        Some(skill_name) => Skill::filter(&connection.0, &skill_name),
        None => Skill::list(&connection.0),
    }?;
    let result = EmployeesSkill::filter(&connection.0, matched_employees, matched_skills)?;
    Ok(Json(result))
}

fn match_employees(
    connection: &DbConn,
    name: Option<String>,
    employeenumber: Option<String>,
) -> Result<Vec<Employee>, BackendError> {
    let matched_employees_by_name =
        name.map(|employee_name| Employee::filter(&connection.0, &employee_name));
    let matched_employees_by_employeenumber =
        employeenumber.map(|employeenumber| Employee::find(&connection.0, &employeenumber));
    let matched_employees = match (
        matched_employees_by_name,
        matched_employees_by_employeenumber,
    ) {
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
        }
    };
    let matched_employees = match matched_employees {
        Some(vec) => vec,
        None => Employee::list(&connection.0)?,
    };
    Ok(matched_employees)
}

#[post("/", data = "<employee_skill>")]
fn insert(
    employee_skill: Json<NewEmployeesSkill>,
    connection: DbConn,
) -> Result<Json<usize>, BackendError> {
    let employee_skill = employee_skill.into_inner();
    let insert = employee_skill.insert(&connection.0)?;
    Ok(Json(insert))
}

#[post("/batch", data = "<employee_skills>")]
fn insert_batch(
    employee_skills: Json<Vec<NewEmployeesSkill>>,
    connection: DbConn,
) -> Result<Json<usize>, BackendError> {
    let employee_skills = employee_skills.into_inner();
    let insert = NewEmployeesSkill::insert_batch(&connection.0, employee_skills)?;
    Ok(Json(insert))
}

pub trait EmployeeSkillApi {
    fn mount_employee_skill_api(self, base: &str) -> Self;
}

impl EmployeeSkillApi for Rocket {
    fn mount_employee_skill_api(self, base: &str) -> Self {
        self.mount(
            base,
            routes![
                get_all,
                get_by_id,
                list_employees_with_skill,
                list_skills_for_employee,
                search,
                insert,
                insert_batch,
            ],
        )
    }
}
