use seed::{*, prelude::*};
use shared::models::user::{AuthUser, UserId};
mod page;

const LOGIN: &str = "login";
const SCHOOL: &str = "school";
const SIGN_IN: &str = "sign_in";
const LOGOUT: &str = "logout";
const USER: &str = "user";

// ------ ------
//     Init
// ------ ------

const STORAGE_KEY: &str = "user";

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.subscribe(Msg::UrlChanged);

    Model {
        page: Page::Home,
        ctx: Context {
            base_url: url.to_base_url(),
            user: {
                let store = storage::get_storage().expect("get local storage");
                storage::load_data(&store, STORAGE_KEY)
            }
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

#[derive(Debug)]
pub struct Context {
    pub base_url: Url,
    pub user: Option<AuthUser>,
}


// ------ Page ------

pub enum Page {
    Home,
    Login(page::login::Model),
    School,
    NotFound,
    Logout,
    SignIn(page::sign_in::Model)
}

impl Page {
    fn init(mut url: Url) -> Self {
        match url.next_path_part() {
            Some("") | None => Self::Home,
            Some(LOGIN) => Self::Login(page::login::init()),
            Some(SCHOOL) => Self::School,
            Some(LOGOUT) => Self::Logout,
            Some(SIGN_IN) => Self::SignIn(page::sign_in::init()),
            _ => Self::NotFound,
        }
    }
}

// ------ ------
//     Urls
// ------ ------

struct_urls!();
impl<'a> Urls<'a> {
    pub fn home(self) -> Url { self.base_url() }
    pub fn sign_in(self) -> Url { self.base_url().add_path_part(SIGN_IN) }
    pub fn login(self) -> Url { self.base_url().add_path_part(LOGIN) }
    pub fn logout(self) -> Url { self.base_url().add_path_part(LOGOUT) }
    pub fn user_detail(self, user_id: UserId) -> Url {
        self.base_url().add_path_part(USER).add_path_part(user_id.to_string())
    }
}

// ------ ------
//    Update
// ------ ------

#[derive(Debug)]
pub enum Msg{
    UrlChanged(subs::UrlChanged),
    LoginMsg(page::login::Msg),
    SignIn(page::sign_in::Msg),
    Logout
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    let ctx = &mut model.ctx;
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            model.page = Page::init(url);
        },
        Msg::LoginMsg(msg) => {
            if let Page::Login(model) = &mut model.page {
                page::login::update(msg, model, &mut orders.proxy(Msg::LoginMsg), ctx)
            }
        },
        Msg::Logout=>{
            storage::get_storage()
                .expect("get local storage")
                .clear()
                .expect("clear local storage");
            ctx.user = None;
            orders.notify(subs::UrlRequested::new(Urls::new(&ctx.base_url).home()));
        },
        Msg::SignIn(msg) =>{

        }
    };
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Node<Msg> {
    let ctx = &model.ctx;
    match &model.page {
        Page::Home => {
            nav![
                C!["navbar", "is-fixed-top"],
                view_navbar_brand(),
                view_navbar_end(ctx)
            ]
        },
        Page::Login(model) => {
            div![
                nav![
                    C!["navbar", "is-fixed-top"],
                    view_navbar_brand(),
                    view_navbar_end(ctx)
                ],
                page::login::view(model, ctx).map_msg(Msg::LoginMsg)
            ]
        }
        Page::School => nav![C!["navbar", "is-fixed-top"], div!["I'm school/timetable"]],
        Page::Logout => { nav![C!["navbar", "is-fixed-top"], div!["I'm school/timetable"]]},
        Page::SignIn(model) => {
            div![
                nav![
                    C!["navbar", "is-fixed-top"],
                    view_navbar_brand(),
                    view_navbar_end(ctx)
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

fn view_navbar_end(ctx: &Context) -> Node<Msg>{
    match &ctx.user{
        Some(user)=>{
            div![
                C!{"navbar-end"},
                div![
                    C!{"navbar-item has-dropdown is-hoverable"},
                    a![
                        C!{"navbar-link"},
                        attrs!{At::Href => Urls::new(&ctx.base_url).user_detail(user.id)},
                        &user.first_name
                    ],
                    div![
                        C!{"navbar-dropdown"},
                        a![
                            C!{"navbar-item"},"Kişisel Bilgiler",
                            attrs!{At::Href => Urls::new(&ctx.base_url).user_detail(user.id)}
                        ],
                        a![
                            C!{"navbar-item"},
                            ev(Ev::Click, |event| {
                                event.prevent_default();
                                Msg::Logout
                            }),
                            "Çıkış"
                        ],
                    ]
                ]
            ]
        },
        None=>{
            div![
                C!{"navbar-end"},
                a![C!{"navbar-item"}, attrs!{At::Href => Urls::new(&ctx.base_url).login()}, "Giriş Yap"],
                a![C!{"navbar-item"}, attrs!{At::Href => Urls::new(&ctx.base_url).sign_in()}, "Üye Ol"]
            ]
        }
    }

}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
