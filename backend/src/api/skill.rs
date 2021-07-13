use rocket::Rocket;
use rocket_contrib::json::Json;

use crate::database::models::skill::{NewSkill, Skill};
use crate::result::BackendError;
use crate::DbConn;

#[get("/")]
fn get_all(connection: DbConn) -> Result<Json<Vec<Skill>>, BackendError> {
    let skill_list = Skill::list(&connection)?;
    Ok(Json(skill_list))
}

#[get("/<id>")]
fn get_by_id(id: i32, connection: DbConn) -> Result<Json<Skill>, BackendError> {
    let skill = Skill::find(&connection, id)?;
    Ok(Json(skill))
}

#[get("/search/<name>")]
fn search_by_name(name: String, connection: DbConn) -> Result<Json<Vec<Skill>>, BackendError> {
    let skill = Skill::filter(&connection, &name)?;
    Ok(Json(skill))
}

#[post("/", data = "<skill>")]
fn insert(skill: Json<NewSkill>, connection: DbConn) -> Result<Json<usize>, BackendError> {
    let skill = skill.0;
    let insert = skill.insert(&connection)?;
    Ok(Json(insert))
}

#[post("/batch", data = "<skills>")]
fn insert_batch(
    skills: Json<Vec<NewSkill>>,
    connection: DbConn,
) -> Result<Json<usize>, BackendError> {
    let skills = skills.0;
    let insert = NewSkill::insert_batch(&connection, skills)?;
    Ok(Json(insert))
}

pub trait SkillApi {
    fn mount_skill_api(self, base: &str) -> Self;
}

impl SkillApi for Rocket {
    fn mount_skill_api(self, base: &str) -> Self {
        self.mount(
            base,
            routes![get_all, get_by_id, search_by_name, insert, insert_batch,],
        )
    }
}
