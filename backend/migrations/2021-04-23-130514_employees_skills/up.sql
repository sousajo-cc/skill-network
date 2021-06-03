-- Your SQL goes here
CREATE TABLE employees_skills (
    id integer primary key AUTOINCREMENT not null,
    employee_number text not null,
    skill_id integer not null
)