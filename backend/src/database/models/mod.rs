pub mod employee;
pub mod skill;
pub mod employee_skill;

//TODO: clean this
pub mod tables {
    pub mod employee {
        pub use crate::database::schema::employees::dsl::{
            employees as employees_table,
            employee_number,
            name
        };
        pub use crate::database::schema::employees;
    }
    pub mod skill {
        pub use crate::database::schema::skills::dsl::{
            skills as skills_table,
            id,
            skill
        };
        pub use crate::database::schema::skills;
    }
    pub mod employee_skill {
        pub use crate::database::schema::employees_skills::dsl::{
            employees_skills as employees_skills_table,
            id,
            employee_number,
            skill_id
        };
        pub use crate::database::schema::employees_skills;
    }
}
