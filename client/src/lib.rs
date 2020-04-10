use seed::{*, prelude::*};
mod session;
mod pages;
use crate::pages::{Pages};
mod routes;
use crate::routes::Route;
use shared::models::user::AuthUser;
use std::borrow::BorrowMut;

#[derive(Clone)]
struct Model {
    user: Option<AuthUser>,
    page: Pages
}



impl Default for Model {
    fn default() -> Self {
        Model {
            user: None,
            page: Pages::Home
        }
    }
}

#[derive(Debug)]
pub enum Msg{
    RouteChanged(Option<Route>),
    LoginMsg(pages::Msg)
}

fn update(mut msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::RouteChanged(route) => {
            match route{
                Some(route)=>{
                    match route{
                        Route::Login=> {
                            //let model2 = pages::Model::default();
                            model.page = Pages::Login(pages::Model::default())
                        },
                        Route::Home=> model.page = Pages::Home,
                        Route::School=>{
                            model.page = Pages::Timetable
                        }
                    }
                },
                None=> model.page = Pages::Home
            }
        },
        Msg::LoginMsg(s)=>{
            match model.page.borrow_mut(){
                Pages::Login(m)=>{
                    pages::update(s, m, &mut _orders.proxy(Msg::LoginMsg));
                },
                _=>{}
            }
        }
        //pages::update(s,&mut pages::Model{ form: pages::LoginForm{email: "b".to_string(), password: "b".to_string()}}, &mut orders.proxy(Msg::LoginMsg));
    };
}


fn view(model: &Model) -> impl View<Msg> {
    model.page.init().els().map_msg(Msg::LoginMsg)
}

fn routes(url: Url) -> Option<Msg> {
    if url.path.is_empty() {
        return Some(Msg::RouteChanged(Some(Route::Home)))
    }

    Some(match url.path[0].as_ref() {
        "login" => {
            Msg::RouteChanged(Some(Route::Login))
            // Determine if we're at the main guide page, or a subpage
        },
        "school" => {
            Msg::RouteChanged(Some(Route::School))
            // Determine if we're at the main guide page, or a subpage
        },
        _ => Msg::RouteChanged(Some(Route::Home)),
    })
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view)
        .routes(routes)
        .build_and_start();
}
