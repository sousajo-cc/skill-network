table! {
    employees (employee_number) {
        employee_number -> Text,
        name -> Text,
    }
}

table! {
    employees_skills (id) {
        id -> Integer,
        employee_number -> Text,
        skill_id -> Integer,
    }
}

table! {
    skills (id) {
        id -> Integer,
        skill -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    employees,
    employees_skills,
    skills,
);
