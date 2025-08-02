use rocket::serde::Deserialize;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct User{
    pub id:u8,
    pub name:String
}