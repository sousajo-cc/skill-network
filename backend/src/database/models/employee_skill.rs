use crate::database::models::employee::Employee;
use crate::database::models::skill::Skill;
use crate::database::models::tables::employee_skill::*;
use crate::result::BackendError;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use crate::LogsDbConn;

#[derive(Queryable, Serialize, Identifiable, Associations, Clone, Debug)]
#[belongs_to(Employee, foreign_key = "employee_number")]
#[belongs_to(Skill)]
pub struct EmployeesSkill {
    pub id: i32,
    pub employee_number: String,
    pub skill_id: i32,
}

impl EmployeesSkill {
    pub fn list(conn: &LogsDbConn) -> QueryResult<Vec<EmployeesSkill>> {
        employees_skills_table.load::<EmployeesSkill>(conn)
    }

    pub fn find(conn: &LogsDbConn, employee_skill_id: i32) -> QueryResult<EmployeesSkill> {
        employees_skills_table
            .filter(id.eq(employee_skill_id))
            .get_result::<EmployeesSkill>(conn)
    }

    pub fn filter_by_employee(
        conn: &LogsDbConn,
        employee: Employee,
    ) -> QueryResult<Vec<EmployeesSkill>> {
        EmployeesSkill::belonging_to(&employee).load::<EmployeesSkill>(conn)
    }

    pub fn filter_by_employees(
        conn: &LogsDbConn,
        employees: Vec<Employee>,
    ) -> QueryResult<Vec<EmployeesSkill>> {
        EmployeesSkill::belonging_to(&employees).load::<EmployeesSkill>(conn)
    }

    pub fn filter_by_skill(
        conn: &LogsDbConn,
        skill: Skill,
    ) -> QueryResult<Vec<EmployeesSkill>> {
        EmployeesSkill::belonging_to(&skill).load::<EmployeesSkill>(conn)
    }

    pub fn filter_by_skills(
        conn: &LogsDbConn,
        skills: Vec<Skill>,
    ) -> QueryResult<Vec<EmployeesSkill>> {
        EmployeesSkill::belonging_to(&skills).load::<EmployeesSkill>(conn)
    }

    pub fn filter(
        conn: &LogsDbConn,
        employees: Vec<Employee>,
        skills: Vec<Skill>,
    ) -> QueryResult<Vec<EmployeesSkill>> {
        let skills: Vec<i32> = skills.iter().map(|skill| skill.id).collect();
        EmployeesSkill::belonging_to(&employees)
            .filter(skill_id.eq_any(skills))
            .load::<EmployeesSkill>(conn)
    }
}

#[derive(Insertable, Deserialize, Clone)]
#[table_name = "employees_skills"]
pub struct NewEmployeesSkill {
    pub employee_number: String,
    pub skill_id: i32,
}

impl NewEmployeesSkill {
    pub fn insert(self, conn: &LogsDbConn) -> QueryResult<usize> {
        diesel::insert_into(employees_skills_table)
            .values(self)
            .execute(conn)
    }

    pub fn insert_batch(
        conn: &LogsDbConn,
        values: Vec<NewEmployeesSkill>,
    ) -> QueryResult<usize> {
        diesel::insert_into(employees_skills_table)
            .values(values)
            .execute(conn)
    }
}

pub trait ListSkills {
    fn list_skills(self) -> Result<Vec<Skill>, BackendError>;
}

impl ListSkills for Employee {
    fn list_skills(self) -> Result<Vec<Skill>, BackendError> {
        unimplemented!();
    }
}

pub trait ListEmployees {
    fn list_employees(self) -> Result<Vec<Employee>, BackendError>;
}

impl ListEmployees for Skill {
    fn list_employees(self) -> Result<Vec<Employee>, BackendError> {
        unimplemented!();
    }
}
