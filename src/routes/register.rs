use rocket::serde::json::Json;

use crate::models::{content::Content, user::User};


#[post("/register", format = "json", data = "<user>")]
pub fn register(user:Json<User>)->Json<Content>{
    Json(Content { content: format!("Register succesfull daa! id:{} name:{}",user.id,user.name) })
}