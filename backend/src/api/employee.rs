use rocket::Rocket;
use rocket_contrib::json::Json;

//use crate::database::establish_connection;
use crate::database::models::employee::Employee;
use crate::result::BackendError;
use crate::LogsDbConn;

#[get("/")]
fn get_all(
    connection: LogsDbConn
) -> Result<Json<Vec<Employee>>, BackendError> {
    let employee_list = Employee::list(&connection)?;
    Ok(Json(employee_list))
}

#[get("/<employeenumber>")]
fn get_by_employeenumber(
                    employeenumber: String,
                    connection: LogsDbConn
) -> Result<Json<Employee>, BackendError> {
    // try this with http://localhost:8000/employee/get_by_employeenumber/00767
    let employee_by_q_nr = Employee::find(&connection, &employeenumber)?;
    Ok(Json(employee_by_q_nr))
}

#[get("/search/<name>")]
fn search_by_name(
                name: String,
                connection: LogsDbConn
) -> Result<Json<Vec<Employee>>, BackendError> {
    // try this with http://localhost:8000/employee/get_by_name/Jorge
    let employee_by_name = Employee::filter(&connection, &name)?;
    Ok(Json(employee_by_name))
}

#[post("/", data = "<employee>")]
fn insert(
        employee: Json<Employee>,
        connection: LogsDbConn
) -> Result<Json<usize>, BackendError> {
    let employee = employee.into_inner();
    let insert = employee.insert(&connection)?;
    Ok(Json(insert))
}

#[post("/batch", data = "<employees>")]
fn insert_batch(
            employees: Json<Vec<Employee>>,
            connection: LogsDbConn
) -> Result<Json<usize>, BackendError> {
    let employees = employees.to_vec();
    let insert = Employee::insert_batch(&connection, employees)?;
    Ok(Json(insert))
}

pub trait EmployeeApi {
    fn mount_employee_api(self, base: &str) -> Self;
}

impl EmployeeApi for Rocket {
    fn mount_employee_api(self, base: &str) -> Self {
        self.mount(
            base,
            routes![
                get_all,
                get_by_employeenumber,
                search_by_name,
                insert,
                insert_batch,
            ],
        )
    }
}
