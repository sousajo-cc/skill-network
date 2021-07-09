#![allow(clippy::nonstandard_macro_braces)]

pub mod employee;
pub mod employee_skill;
pub mod skill;

pub use employee::EmployeeApi;
pub use employee_skill::EmployeeSkillApi;
pub use skill::SkillApi;
