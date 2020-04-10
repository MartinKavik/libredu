use std::convert::TryFrom;
use std::fmt;
#[derive(Clone, Debug)]
pub enum Route{
    Login,
    Home,
    School
}

impl Route {
    pub fn path(&self) -> Vec<&str> {
        match self {
            Route::Login => vec!["login"],
            Route::Home=> vec![""],
            Route::School=>vec!["school"]
        }
    }
}

impl<'a> fmt::Display for Route {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "/{}", self.path().join("/"))
    }
}

impl TryFrom<seed::Url> for Route {
    type Error = ();

    fn try_from(url: seed::Url) -> Result<Self, Self::Error> {
        let mut path = url.path.into_iter();

        match path.next().as_ref().map(String::as_str) {
            None | Some("") => Some(Route::Home),
            Some("login") => Some(Route::Login),
            Some("school") => Some(Route::School),
            _ => None,
        }
            .ok_or(())
    }
}