use rocket::serde::json::Json;
use crate::models::content::Content;

#[get("/hi/<user>",)]
pub fn say_hello(user:&str)->Json<Content>{
    Json(
        Content {content : format!("{} Haçan Yaylaya Hoş Celdun",user)}
    )
}