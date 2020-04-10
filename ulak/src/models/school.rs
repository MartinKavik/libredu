use chrono::NaiveTime;
use crate::schema::*;
#[derive(Queryable, PartialEq, Insertable, Debug, Serialize, Deserialize, Clone)]
#[table_name="school"]
pub struct School {
    pub code: i32,
    pub name: Option<String>,
    pub email: Option<String>,
    pub pansiyon: Option<bool>,
    pub dersane: Option<bool>,
    pub is_active: Option<bool>,
    pub manager: Option<i32>,
    pub city: Option<i32>,
    pub town: Option<i32>,
    pub school_type: Option<i32>,
    pub hour: i16,
}

#[derive(Queryable, PartialEq, Insertable, Debug, Serialize, Deserialize, Clone)]
#[table_name="school"]
pub struct NewSchool {
    //pub code: i32,
    pub name: Option<String>,
    pub email: Option<String>,
    pub city: Option<i32>,
    pub town: Option<i32>,
    pub school_type: Option<i32>,
    pub manager: Option<i32>,
    pub hour: i16
}
#[derive(Queryable, PartialEq, Insertable, Debug, Serialize, Deserialize, AsChangeset)]
#[table_name="school"]
pub struct UpdateSchool {
    //pub code: i32,
    pub name: Option<String>,
    pub email: Option<String>,
    pub city: Option<i32>,
    pub town: Option<i32>,
    pub hour: i16
}
#[derive(Queryable, PartialEq, Debug, Serialize, Deserialize)]
pub struct SchoolType {
    pub name: String,
    pub id: i32
}

#[derive(Queryable, PartialEq, Debug, Serialize, Deserialize)]
pub struct SchoolMenu {
    pub title: Option<String>,
    pub link: Option<String>,
    pub school_type: Option<i32>,
    pub id: i32,
}

#[derive(Queryable, PartialEq, Debug, Serialize, Deserialize, Insertable)]
#[table_name="school_users"]
pub struct SchoolTeacher {
    pub user_id: i32,
    pub school_id: i32,
    pub auth: Option<i32>
}

#[derive(Queryable, PartialEq, Debug, Serialize, Deserialize, Insertable)]
#[table_name="school_time"]
pub struct SchoolTime {
    pub id: i32,
    pub school: Option<i32>,
    pub day: Option<i32>,
    pub start: Option<NaiveTime>,
    pub finish: Option<NaiveTime>,
    pub hour: Option<i32>,
}