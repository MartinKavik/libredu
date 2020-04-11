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

    Model {
        // TODO: Load the user (e.g. from LocalStorage).
        ctx: Context { user: None },
        page: Page::init(url),
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
fn view(model: &Model) -> Vec<Node<Msg>> {
    match &model.page {
        Page::Home => vec![view_navbar(), div!["I'm home"]],
        Page::Login(model) => vec![
            view_navbar(),
            page::login::view(model).map_msg(Msg::LoginMsg)
        ],
        Page::School => vec![view_navbar(), div!["I'm school/timetable"]],
        Page::NotFound => vec![view_navbar(), div!["404"]],
    }
}

fn view_navbar() -> Node<Msg> {
    // `C!` is a better alternative to `class!` (`class!` will be deprecated).
    nav![C!["navbar", "is-fixed-top"],
        view_navbar_brand(),
        view_navbar_end()
    ]
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

fn view_navbar_end() -> Vec<Node<Msg>>{
    raw!(r#"
        <div class="navbar-end">
            <a class="navbar-item"  href="/login">
            Giriş Yap
            </a>
            <a class="navbar-item" href ="/school">
            Üye Ol
            </a>
        </div>
      "#)
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    // New Seed init API.
    App::start("app", init, update, view);
}
