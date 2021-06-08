use rocket::Rocket;
use rocket_contrib::json::Json;

use crate::database::establish_connection;
use crate::database::models::skill::{Skill, NewSkill};
use crate::result::BackendError;

#[get("/")]
fn get_all() -> Result<Json<Vec<Skill>>, BackendError> {
    let connection = establish_connection();
    let skill_list = Skill::list(&connection)?;
    Ok(Json(skill_list))
}

#[get("/<id>")]
fn get_by_id(id: i32) -> Result<Json<Skill>, BackendError> {
    let connection = establish_connection();
    let skill = Skill::find(&connection, id)?;
    Ok(Json(skill))
}

#[get("/search/<name>")]
fn search_by_name(name: String) -> Result<Json<Vec<Skill>>, BackendError> {
    let connection = establish_connection();
    let skill = Skill::filter(&connection, &name)?;
    Ok(Json(skill))
}

#[post("/", data="<skill>")]
fn insert(skill: Json<NewSkill>) -> Result<Json<usize>, BackendError> {
    let connection = establish_connection();
    let skill = skill.0;
    let insert = skill.insert(&connection)?;
    Ok(Json(insert))
}

#[post("/batch", data="<skills>")]
fn insert_batch(skills: Json<Vec<NewSkill>>) -> Result<Json<usize>, BackendError> {
    let connection = establish_connection();
    let skills = skills.0;
    let insert = NewSkill::insert_batch(&connection, skills)?;
    Ok(Json(insert))
}

pub trait SkillApi {
    fn mount_skill_api(self, base: &str) -> Self;
}

impl SkillApi for Rocket {
    fn mount_skill_api(self, base: &str) -> Self {
        self.mount(base, routes![
            get_all,
            get_by_id,
            search_by_name,
            insert,
            insert_batch,
        ])
    }
}