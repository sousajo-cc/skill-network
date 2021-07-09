#![allow(clippy::nonstandard_macro_braces)]

#![deny(warnings)]
#![feature(proc_macro_hygiene, decl_macro)]

mod api;
mod database;
mod result;

use crate::database::models::employee_skill::NewEmployeesSkill;
use database::establish_connection;
use database::models::skill::NewSkill;
use dotenv::dotenv;
use result::BackendError;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Error};

#[database("sqlite_logs")]
pub struct LogsDbConn(pub diesel::SqliteConnection);

extern crate dotenv;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket_contrib;

pub fn employee_skill_example() -> Result<(), BackendError> {
    dotenv().ok();
    let connection = establish_connection();
    let relations = vec![
        NewEmployeesSkill {
            skill_id: 1,
            employee_number: "00767".to_string(),
        },
        NewEmployeesSkill {
            skill_id: 1,
            employee_number: "00965".to_string(),
        },
        NewEmployeesSkill {
            skill_id: 2,
            employee_number: "00965".to_string(),
        },
        NewEmployeesSkill {
            skill_id: 3,
            employee_number: "00965".to_string(),
        },
        NewEmployeesSkill {
            skill_id: 4,
            employee_number: "00965".to_string(),
        },
        NewEmployeesSkill {
            skill_id: 5,
            employee_number: "00965".to_string(),
        },
    ];
    NewEmployeesSkill::insert_batch(&connection, relations)?;
    Ok(())
}

pub fn skills_example() -> Result<(), BackendError> {
    //TODO: increase list and move to migration
    dotenv().ok();
    let connection = establish_connection();
    let skills = vec![
        NewSkill {
            skill: "C++".to_string(),
        },
        NewSkill {
            skill: "C#".to_string(),
        },
        NewSkill {
            skill: "C".to_string(),
        },
        NewSkill {
            skill: "Rust".to_string(),
        },
        NewSkill {
            skill: "D".to_string(),
        },
        NewSkill {
            skill: "F#".to_string(),
        },
        NewSkill {
            skill: "Darklang".to_string(),
        },
        NewSkill {
            skill: "Zig".to_string(),
        },
        NewSkill {
            skill: "Java".to_string(),
        },
        NewSkill {
            skill: "Python".to_string(),
        },
        NewSkill {
            skill: "Assembly family".to_string(),
        },
        NewSkill {
            skill: "Javascript".to_string(),
        },
    ];
    NewSkill::insert_batch(&connection, skills)?;
    Ok(())
}

pub fn example() -> Result<(), BackendError> {
    dotenv().ok();
    //TODO: put this as an env var -> use format! (dont commit this as this as personal info)
    /*let mut workbook: Xlsx<_> = open_workbook("people.xlsx".to_string())?;
    let range = workbook.worksheet_range("People")
        .ok_or(calamine::Error::Msg("Cannot find Sheet 'People'"))??;
    let connection = establish_connection();

    let iter = RangeDeserializerBuilder::new().from_range(&range)?;
    for val in iter {
        let (name, _full_name, employee_number): (String, String, String) = val?;
        let employee = Employee { employee_number, name };
        employee.insert(&connection)?;
    }*/
    Ok(())
}

pub fn main() -> Result<(), Error> {
    use api::*;

    let allowed_origins = AllowedOrigins::all();

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;
    example()
        .or_else(|e| {
            if let BackendError::DatabaseError(_) = e {
                println!("Database already has values!");
                Ok(())
            } else {
                Err(e)
            }
        })
        .unwrap();

    skills_example()
        .or_else(|e| {
            if let BackendError::DatabaseError(_) = e {
                println!("Database already has values!");
                Ok(())
            } else {
                Err(e)
            }
        })
        .unwrap();

    // employee_skill_example().or_else(|e|
    //     if let BackendError::DatabaseError(_) = e {
    //         println!("Database already has values!");
    //         Ok(())
    //     } else {
    //         Err(e)
    //     }
    // ).unwrap();

    rocket::ignite()
        .mount_employee_api("/employee")
        .mount_skill_api("/skill")
        .mount_employee_skill_api("/")
        .attach(cors)
        .attach(LogsDbConn::fairing())
        .launch();
    Ok(())
}
