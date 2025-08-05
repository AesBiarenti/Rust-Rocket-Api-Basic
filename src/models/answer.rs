use serde::Serialize;



#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Answer {
    pub mesaj:String
}