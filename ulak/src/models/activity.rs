use crate::models::others::{Subject, Class};
use crate::models::user::{AuthUser};
use crate::schema::*;

#[derive(Deserialize, Serialize, Debug, Queryable, PartialEq,Clone)]
pub struct Activities{
    pub id: i32,
    pub subject: Subject,
    pub teacher: AuthUser,
    pub class: Class,
    pub hour: Option<i16>,
    pub split: bool
}

#[derive(Deserialize, Serialize, Debug, Queryable, PartialEq, Clone, Identifiable)]
#[table_name="activities"]
pub struct Activity{
    pub id: i32,
    pub subject: Option<i32>,
    pub teacher: Option<i32>,
    pub hour: Option<i16>,
    pub class: i32,
    pub split: bool,
    pub splitted: Option<i16>,
    pub placed: bool,
    pub day: Option<i16>,
    pub hrs: Option<i16>,
    pub act_id: Option<i32>
}