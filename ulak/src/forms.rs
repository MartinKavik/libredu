use chrono::NaiveDateTime;
use crate::schema::post;
use crate::schema::users;
use crate::schema::classes;
use crate::schema::subjects;
use crate::schema::activities;
use crate::schema::teacher_available;
use crate::schema::class_available;
use crate::schema::city;
use crate::schema::town;
use crate::schema::class_subjects;
#[derive(Deserialize, Debug, Serialize)]
pub struct LoginForm{
 pub username: String,
 pub password: String,
}

#[derive(Deserialize, Insertable, Serialize, Debug)]
#[table_name = "post"]
pub struct PostForm{
 //pub title: Option<String>,
 pub only_teacher: Option<bool>,
 pub body: String,
 pub pub_date: Option<NaiveDateTime>,
 pub sender: i32,
 //pub school: i32
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SignForm{
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email : Option<String>,
    pub password1: Option<String>,
    pub password2: Option<String>,
    pub last_login: Option<NaiveDateTime>,
    pub gender : Option<String>,
    pub is_active: Option<bool>,
    pub is_admin: Option<bool>,
    pub is_staff: Option<bool>,
    pub username: Option<String>,
    pub tel: Option<String>,

}

#[derive(Deserialize, Serialize, Debug, Insertable)]
#[table_name = "users"]
pub struct AddTeacherForm{
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub tel: Option<String>,
    pub date_join: Option<NaiveDateTime>,
    pub is_staff: Option<bool>
}

#[derive(Deserialize, Serialize, Debug, Insertable)]
#[table_name = "users"]
pub struct SignTeacherForm{
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub tel: Option<String>,
    //pub date_join: NaiveDateTime,
}

#[derive(Deserialize, Serialize, Debug, Insertable)]
#[table_name = "classes"]
pub struct NewClassForm{
    pub sube: String,
    pub kademe: i16,
    pub school: i32,
}

#[derive(Deserialize, Serialize, Debug, Insertable)]
#[table_name = "subjects"]
pub struct NewSubjectsForm{
    pub name: Option<String>,
    pub kademe: Option<i16>,
    pub school_type: Option<i32>,
    pub optional: Option<bool>
}

#[derive(Deserialize, Serialize, Debug, Insertable)]
#[table_name = "class_subjects"]
pub struct NewClassSubjectsForm{
    pub class: Option<i32>,
    pub subject: Option<i32>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ClassSubjectsForm{
    pub classes: Vec<i32>,
    pub subjects: Vec<i32>,
}

#[derive(Deserialize, Serialize, Debug, Insertable, Clone)]
#[table_name = "activities"]
pub struct NewActivitiesForm{
    pub subject: Option<i32>,
    pub teacher: i32,
    pub hour: Option<i16>,
    pub class: i32,
    pub split: Option<bool>,
}

#[derive(Queryable, PartialEq, Debug, Serialize, Deserialize, Insertable, AsChangeset)]
#[table_name =  "teacher_available"]
pub struct TeacherAvailableForm {
    pub user_id: i32,
    pub school_id: i32,
    pub day: i32,
    pub hours: Option<Vec<bool>>,
}

#[derive(Queryable, PartialEq, Debug, Serialize, Deserialize, Insertable, AsChangeset)]
#[table_name =  "class_available"]
#[primary_key(class_id, day)]
pub struct ClassAvailableForm {
    pub class_id: i32,
    pub day: i32,
    pub hours: Option<Vec<bool>>,
}

#[derive(Queryable, PartialEq, Debug, Serialize, Deserialize, Insertable, AsChangeset)]
#[table_name =  "city"]
pub struct NewCity {
    pub pk: i32,
    pub name: String,
    //pub hours: Option<Vec<bool>>,
}

#[derive(Queryable, PartialEq, Debug, Serialize, Deserialize, Insertable, AsChangeset)]
#[table_name =  "town"]
pub struct NewTown {
    pub pk: i32,
    pub name: String,
    pub city: Option<i32>
    //pub hours: Option<Vec<bool>>,
}