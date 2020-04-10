use chrono::NaiveDateTime;
use crate::schema::*;
#[derive(Clone, Debug, Identifiable, Serialize, Deserialize, Queryable, PartialEq)]
#[table_name = "users"]
#[primary_key(id)]
pub struct User {
    pub id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub date_join: Option<NaiveDateTime>,
    pub last_login: Option<NaiveDateTime>,
    pub is_active: Option<bool>,
    pub is_staff: Option<bool>,
    pub is_admin: Option<bool>,
    pub tel: Option<String>,
    pub gender:Option<String>,
    pub img:Option<String>,
}

#[derive(Clone, Debug, Serialize, Identifiable, Insertable, Deserialize, Queryable, PartialEq)]
#[table_name = "users"]
pub struct AuthUser {
    pub id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    //pub is_active: Option<bool>,
    //pub is_staff: Option<bool>,
    pub is_admin: Option<bool>,
    //pub tel: Option<String>,
    //pub gender:Option<String>,
    //pub img:Option<String>,
}


#[derive(Insertable, Deserialize, Serialize, Debug, Clone)]
#[table_name="users"]
pub struct NewUser {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub date_join: Option<NaiveDateTime>,
    pub last_login: Option<NaiveDateTime>,
    pub is_active: Option<bool>,
    pub is_staff: Option<bool>,
    pub is_admin: Option<bool>,
    pub tel:Option<String>,
    pub gender:Option<String>,
    pub img:Option<String>,
}