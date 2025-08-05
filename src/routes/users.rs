use rocket::{http::Status, serde::json::Json, State};
use crate::{models::{answer::{Answer}, user::User}, state::UserList};

#[get("/users")]
pub fn list(lists: &State<UserList>) ->Json<Vec<User>> {
    let datas = lists.lock().unwrap();
    Json(datas.clone())
}

#[post("/users",format = "json", data = "<data>")]
pub fn add_user(data:Json<User>, list: &State<UserList>) ->Result<Json<Answer>,Status> {
    let mut datas = list.lock().unwrap();
    if datas.iter().any(|k| k.id == data.id) {
        return Err(Status::Conflict)
    }
    datas.push(data.into_inner());
    Ok(Json(Answer { mesaj: "Kullanıcı Eklendi".to_string() }))
}
#[put("/users/<id>",format = "json" , data = "<data>")]
pub fn update_user(data:Json<User>,id:u32,list:&State<UserList>)->Result<Json<Answer>,Status>{
    let mut datas = list.lock().unwrap();
    if let Some(user) = datas.iter_mut().find(|k| k.id == id){
        *user = data.into_inner();
        Ok(Json(Answer { mesaj: "Kullanıcı Güncellendi".to_string() }))
    }  else {
        Err(Status::NotFound)
    }
}
#[delete("/users/<id>")]
pub fn delete_user(id:u32,list: &State<UserList>) -> Result<Json<Answer>,Status> {
    let mut datas = list.lock().unwrap();
    let first_len = datas.len();
    datas.retain(|k | k.id != id);
    if datas.len() < first_len {
        Ok(Json(Answer { mesaj: "Başarıyla Silindi".to_string() }))
    }else {
        Err(Status::NotFound)
    }
}

