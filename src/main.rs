#[macro_use] extern crate rocket;

mod models;
mod routes;
mod state;

use std::sync::Mutex;

use routes::{hello::say_hello , home::hello,register::register,users::{list,add_user,update_user,delete_user}};

use crate::state::UserList;

#[launch]
fn rocket()->_{
    let user_list:UserList = Mutex::new(Vec::new()); 
    rocket::build()
    .manage(user_list)
    .mount("/", routes![say_hello,hello,register,list,add_user,update_user,delete_user])
}