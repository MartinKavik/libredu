use seed::{*, prelude::*};
use shared::models::user::AuthUser;

mod page;

const LOGIN: &str = "login";
const SCHOOL: &str = "school";
const SIGN_IN: &str = "sign_in";

// ------ ------
//     Init
// ------ ------

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders
        .subscribe(Msg::UrlChanged);
    let store = seed::storage::get_storage();
    let user = seed::storage::load_data::<AuthUser>(&store.unwrap(), "user");
    let mut model = Model::default();
    model.ctx = Context{user: user};
    model
}

// ------ ------
//     Model
// ------ ------

struct Model {
    ctx: Context,
    page: Page
}

impl Default for Model{
    fn default()-> Self{
        Model{
            ctx: Context{ user: None},
            page: Page::Home
        }
    }
}
// ------ Context ------

pub struct Context {
    pub user: Option<AuthUser>,
}


// ------ Page ------

pub enum Page {
    Home,
    Login(page::login::Model),
    School,
    NotFound,
}

impl Page {
    fn init(url: Url) -> Self {
        // It will be easier / more readable soon -
        // https://github.com/MartinKavik/seed/blob/fix/routing/examples/pages/src/lib.rs#L50
        let mut path = url.path.into_iter();
        match path.next().as_ref().map(String::as_str) {
            Some("") | None => Self::Home,
            Some(LOGIN) => Self::Login(page::login::init()),
            Some(SCHOOL) => Self::School,
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
}

// ------ ------
//    Update
// ------ ------

// #[derive(Debug)]   TODO: Uncomment once MartinKavik implemented `Clone` for `subs::UrlChanged`.
pub enum Msg{
    UrlChanged(subs::UrlChanged),
    LoginMsg(page::login::Msg)
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
                div!["I'm home"], view_navbar_end(user)
            ]
        },
        Page::Login(model) => nav![
            C!["navbar", "is-fixed-top"],
            page::login::view(model).map_msg(Msg::LoginMsg)
        ],
        Page::School => nav![C!["navbar", "is-fixed-top"], div!["I'm school/timetable"]],
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
                    a![
                        C!{"navbar-item"},&u.first_name
                    ]
            ]
        },
        None=>{
            div![
                C!{"navbar-end"},
                    a![C!{"navbar-item"}, attrs!{At::Href => Urls::login()}, "Giriş Yap"]
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
