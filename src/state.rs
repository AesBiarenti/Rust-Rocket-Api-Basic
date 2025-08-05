use std::sync::Mutex;
use crate::models::user::User;

pub type UserList = Mutex<Vec<User>>;