pub mod employee;
pub mod employee_skill;
pub mod skill;

//TODO: clean this
pub mod tables {
    pub mod employee {
        pub use crate::database::schema::employees;
        pub use crate::database::schema::employees::dsl::{
            employee_number, employees as employees_table, name,
        };
    }
    pub mod skill {
        pub use crate::database::schema::skills;
        pub use crate::database::schema::skills::dsl::{id, skill, skills as skills_table};
    }
    pub mod employee_skill {
        pub use crate::database::schema::employees_skills;
        pub use crate::database::schema::employees_skills::dsl::{
            employee_number, employees_skills as employees_skills_table, id, skill_id,
        };
    }
}
