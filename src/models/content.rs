use serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Content{
   pub content:String
}