use rocket::serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize,Serialize,Clone)]
#[serde(crate = "rocket::serde")]
pub struct User{
    pub id:u32,
    pub name:String,
    pub age: u8,
}