use serde::*;

pub type UserId = i32;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthUser{
    pub id: UserId,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: Option<String>,
    pub is_admin: bool,
    //pub is_active: bool,
    //pub is_staff: bool,
}
