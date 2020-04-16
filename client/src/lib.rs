use seed::{*, prelude::*};
use shared::models::user::AuthUser;
use serde::*;
mod page;

const LOGIN: &str = "login";
const SCHOOL: &str = "school";
const SIGN_IN: &str = "sign_in";
const LOGOUT: &str = "logout";

// ------ ------
//     Init
// ------ ------

const STORAGE_KEY: &str = "user";

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders
        .subscribe(Msg::UrlChanged);

    let store = seed::storage::get_storage().expect("get local storage");

    Model {
        page: Page::Home,
        ctx: Context {
            //user: None
            user: seed::storage::load_data(&store, STORAGE_KEY)
        }
    }
}

// ------ ------
//     Model
// ------ ------

struct Model {
    ctx: Context,
    page: Page
}

// ------ Context ------
#[derive(Debug, Serialize, Deserialize)]
pub struct Context {
    pub user: Option<AuthUser>,
}


// ------ Page ------

pub enum Page {
    Home,
    Login(page::login::Model),
    School,
    NotFound,
    Logout,
    Sign_in(page::sign_in::Model)
}

impl Page {
    fn init(url: Url) -> Self {
        // It will be easier / more readable soon -
        // https://github.com/MartinKavik/seed/blob/fix/routing/examples/pages/src/lib.rs#L50
        let mut path = url.path().into_iter();
        match path.next().as_ref().map(&String::as_str) {
            Some("") | None => Self::Home,
            Some(LOGIN) => Self::Login(page::login::init()),
            Some(SCHOOL) => Self::School,
            Some(LOGOUT) => Self::Logout,
            Some(SIGN_IN) => Self::Sign_in(page::sign_in::init()),
            _ => Self::NotFound,
        }
    }
}

// ------ ------
//     Urls
// ------ ------

// It will be officially supported soon and it also allow to build the remaining path parts in nested
// moduiles - https://github.com/MartinKavik/seed/blob/fix/routing/examples/pages/src/lib.rs#L62-L70
struct Urls;
impl Urls {
    pub fn sign_in() -> String {
        format!("/{}", SIGN_IN)
    }
    pub fn login() -> String {
        format!("/{}", LOGIN)
    }
    pub fn logout() -> String {
        format!("/{}", LOGOUT)
    }
    pub fn home() -> String { format!("/")   }
}

// ------ ------
//    Update
// ------ ------

// #[derive(Debug)]   TODO: Uncomment once MartinKavik implemented `Clone` for `subs::UrlChanged`.
pub enum Msg{
    UrlChanged(subs::UrlChanged),
    LoginMsg(page::login::Msg),
    SignIn(page::sign_in::Msg),
    Logout
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    let ctx = &mut model.ctx;
    let p = &mut model.page;
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            model.page = Page::init(url);
        },
        Msg::LoginMsg(msg) => {
            page::login::update(msg, p, &mut orders.proxy(Msg::LoginMsg), ctx)
            /*if let Page::Login(m) = &mut model.page {
                page::login::update(msg, model.page, &mut orders.proxy(Msg::LoginMsg), ctx)
            }*/
        },
        Msg::Logout=>{
            //use crate::STORAGE_KEY;
            let store = seed::storage::get_storage();
            model.ctx = Context{user: None};
            store.unwrap().clear();
            //model.ctx = Context{user: None};
            model.page = Page::Home;
        },
        Msg::SignIn(msg) =>{

        }
    };
}

// ------ ------
//     View
// ------ ------

// `view` should return `Vec<Node<Msg>` or `Node<Msg>` or hide the specific type under `impl IntoNodes<Msg>`.
fn view(model: &Model) -> Node<Msg> {
    let user = &model.ctx;
    match &model.page {
        Page::Home => {
            nav![
                C!["navbar", "is-fixed-top"],
                view_navbar_brand(),
                view_navbar_end(user)
            ]
        },
        Page::Login(model) => {
            div![
                nav![
                    C!["navbar", "is-fixed-top"],
                    view_navbar_brand(),
                    view_navbar_end(user)
                ],
                page::login::view(model).map_msg(Msg::LoginMsg)
            ]
        }
        Page::School => nav![C!["navbar", "is-fixed-top"], div!["I'm school/timetable"]],
        Page::Logout => { nav![C!["navbar", "is-fixed-top"], div!["I'm school/timetable"]]},
        Page::Sign_in(model) => {
            div![
                nav![
                    C!["navbar", "is-fixed-top"],
                    view_navbar_brand(),
                    view_navbar_end(user)
                ],
                page::sign_in::view(model).map_msg(Msg::SignIn)
            ]
        }
        Page::NotFound => nav![C!["navbar", "is-fixed-top"], div!["404"]],
    }
}

fn view_navbar_brand() -> Vec<Node<Msg>>{
    raw!(r#"
        <div class="navbar-brand" id="navbarMenu1">
            <a class="navbar-item" href="/">
            ULAK
            </a>
            <div class="navbar-item">
                <div class="columns">
                    <div class="column is-narrow">
                        <div class="field">
                        </div>
                    </div>
                </div>
            </div>
            <span role="button" class="navbar-burger burger" aria-label="menu" aria-expanded="false" data-target="navbarBasicExample">
                <span aria-hidden="true" style="width: 25px; height: 2px;"></span>
                <span aria-hidden="true" style="width: 25px; height: 2px;"></span>
                <span aria-hidden="true" style="width: 25px; height: 2px;"></span>
            </span>
        </div>
    "#)
}

fn view_navbar_end(user: &Context) -> Node<Msg>{
    match &user.user{
        Some(u)=>{
            div![
                C!{"navbar-end"},
                div![
                    C!{"navbar-item has-dropdown is-hoverable"},
                    a![
                        C!{"navbar-link"},&u.first_name,
                        attrs!{At::Href=>"/user/".to_owned()+&u.id.to_string()}
                    ],
                    div![
                        C!{"navbar-dropdown"},
                        a![
                            C!{"navbar-item"},"Kişisel Bilgiler",
                            attrs!{At::Href=>"/user/".to_owned()+&u.id.to_string()}
                        ],
                        a![
                            C!{"navbar-item"},
                            ev(Ev::Click, |event| {
                                event.prevent_default();
                                Msg::Logout
                            }), "Çıkış"
                        ],
                    ]
                ]

            ]
        },
        None=>{
            div![
                C!{"navbar-end"},
                a![C!{"navbar-item"}, attrs!{At::Href => Urls::login()}, "Giriş Yap"],
                a![C!{"navbar-item"}, attrs!{At::Href => Urls::sign_in()}, "Üye Ol"]
            ]
        }
    }

}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    // New Seed init API.
    App::start("app", init, update, view);
}
