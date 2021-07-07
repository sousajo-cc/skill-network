use crate::database::models::tables::employee::*;
use crate::database::sanitize_search_string::Sanitize;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use crate::LogsDbConn;

#[derive(Insertable, Queryable, Serialize, Deserialize, Identifiable, PartialEq, Clone, Debug)]
#[primary_key(employee_number)]
pub struct Employee {
    pub employee_number: String,
    pub name: String,
}

impl Employee {
    pub fn list(conn: &LogsDbConn) -> QueryResult<Vec<Employee>> {
        employees_table.load::<Employee>(&conn.0)
    }

    pub fn find(conn: &LogsDbConn, employeenumber: &str) -> QueryResult<Employee> {
        employees_table
            .filter(employee_number.eq(employeenumber))
            .get_result::<Employee>(&conn.0)
    }

    #[allow(clippy::ptr_arg)]
    pub fn filter(conn: &LogsDbConn, employee_name: &String) -> QueryResult<Vec<Employee>> {
        let employee_name = employee_name.sanitize();
        let employee_name = format!("%{}%", employee_name);
        employees_table
            .filter(name.like(employee_name))
            .load::<Employee>(&conn.0)
    }

    pub fn insert(self, conn: &LogsDbConn) -> QueryResult<usize> {
        diesel::insert_into(employees_table)
            .values(self)
            .execute(&conn.0)
    }

    pub fn insert_batch(conn: &LogsDbConn, values: Vec<Employee>) -> QueryResult<usize> {
        diesel::insert_into(employees_table)
            .values(values)
            .execute(&*conn.0)
    }
}
