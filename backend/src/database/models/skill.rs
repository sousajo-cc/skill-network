use crate::database::models::tables::skill::*;
use crate::database::sanitize_search_string::Sanitize;
use diesel::prelude::*;
//use diesel::LogsDbConn;
use crate::LogsDbConn;

#[derive(Queryable, Serialize, Identifiable, Clone, Debug)]
pub struct Skill {
    pub id: i32,
    pub skill: String,
}

impl Skill {
    pub fn list(conn: &LogsDbConn) -> QueryResult<Vec<Skill>> {
        skills_table.load::<Skill>(conn)
    }

    pub fn find(conn: &LogsDbConn, skill_id: i32) -> QueryResult<Skill> {
        skills_table
            .filter(id.eq(skill_id))
            .get_result::<Skill>(conn)
    }

    #[allow(clippy::ptr_arg)]
    pub fn filter(conn: &LogsDbConn, skill_name: &String) -> QueryResult<Vec<Skill>> {
        let skill_name = skill_name.sanitize();
        let skill_name = format!("%{}%", skill_name);
        skills_table
            .filter(skill.like(skill_name))
            .load::<Skill>(conn)
    }
}

#[derive(Insertable, Deserialize, Clone)]
#[table_name = "skills"]
pub struct NewSkill {
    pub skill: String,
}

impl NewSkill {
    pub fn insert(self, conn: &LogsDbConn) -> QueryResult<usize> {
        diesel::insert_into(skills_table).values(self).execute(conn)
    }

    pub fn insert_batch(conn: &LogsDbConn, values: Vec<NewSkill>) -> QueryResult<usize> {
        diesel::insert_into(skills_table)
            .values(values)
            .execute(conn)
    }
}
