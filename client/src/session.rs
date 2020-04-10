use shared::models::user::AuthUser;

#[derive(Clone, Debug)]
pub enum Session {
    LoggedIn(AuthUser),
    Guest,
}

impl<'a> Default for Session {
    fn default() -> Self {
        Self::Guest
    }
}

impl<'a> Session {
    pub fn new(user: Option<AuthUser>) -> Self {
        match user {
            Some(user) => Self::LoggedIn(user),
            None => Self::Guest,
        }
    }
    pub fn user(&self) -> Option<&AuthUser> {
        match self {
            Self::LoggedIn(user) => Some(user),
            Self::Guest => None,
        }
    }
}