use chrono::NaiveDateTime;
use crate::schema::*;
use crate::models::activity::Activities;
use crate::models::user::User;

#[derive(Deserialize, Serialize, Debug, Queryable, PartialEq, AsExpression, Clone)]
pub struct Class{
    pub id: i32,
    pub kademe: i16,
    pub sube: String,
    pub school: i32,
    pub teacher: Option<i32>,
}

#[derive(Deserialize, Serialize, Debug, Queryable, PartialEq, Clone)]
pub struct Subject{
    pub id: i32,
    pub name: Option<String>,
    pub kademe: Option<i16>,
    pub school_type: Option<i32>,
    pub optional: Option<bool>
}



#[derive(Queryable, PartialEq, Debug, Serialize, Deserialize)]
pub struct Cities {
    pub pk: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewCities {
    pub pk: i32,
    pub name: String,
}

#[derive(Queryable, PartialEq, Debug, Serialize, Deserialize)]
pub struct Town {
    pub pk: i32,
    pub name: String,
    pub city: Option<i32>
}

#[derive(Serialize, Queryable, Identifiable, Deserialize, Associations, PartialEq, Debug)]
#[belongs_to(User, foreign_key = "sender")]
#[table_name = "post"]
pub struct Post {
    pub id: i32,
    pub title: Option<String>,
    pub only_teacher: Option<bool>,
    pub body: String,
    pub pub_date: Option<NaiveDateTime>,
    pub sender: i32,
    pub school: Option<i32>,
}


#[derive(Queryable, PartialEq, Debug, Insertable, Serialize, Deserialize, Clone)]
#[table_name="teacher_available"]
pub struct TeacherAvailable {
    pub user_id: i32,
    pub school_id: i32,
    pub day: i32,
    pub hours: Vec<bool>,
}

#[derive(Queryable, PartialEq, Debug, Insertable, Serialize, Deserialize, Clone, AsChangeset)]
#[table_name="class_available"]
#[primary_key(class_id, day)]
pub struct ClassAvailable {
    pub class_id: i32,
    pub day: i32,
    pub hours: Vec<bool>,
}

#[derive(Queryable, PartialEq, Debug, Serialize, Deserialize, Clone, Identifiable)]
#[table_name="class_timetable"]
pub struct ClassTimetable {
    pub id: i32,
    pub class_id: Option<i32>,
    pub day_id: Option<i32>,
    pub hour: Option<i16>,
    pub activities: Option<i32>
}

#[derive(Queryable, PartialEq, Debug, Insertable,Serialize, Deserialize, Clone)]
#[table_name="class_timetable"]
pub struct NewClassTimetable {
    pub class_id: Option<i32>,
    pub day_id: Option<i32>,
    pub hour: Option<i16>,
    pub activities: Option<i32>
}

#[derive(Deserialize, Serialize, Debug, Queryable, PartialEq, Clone)]
pub struct Timetable {
    pub id: i32,
    //pub class_id: Class,
    pub day_id: i32,
    pub hour: i16,
    //pub class: Class,
    pub activities: Activities
}

#[derive(Queryable, PartialEq, Debug, Serialize, Deserialize)]
pub struct Day {
    pub id: i32,
    pub name: String
}