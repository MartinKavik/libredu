use seed::{self, prelude::*, *};
use seed::prelude::Node;
use shared::models::user::AuthUser;
use serde::*;


pub mod pages;
pub mod login;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginForm{
    pub email: String,
    pub password: String
}

#[derive(Clone, Debug)]
pub struct Model{
    pub form: LoginForm
}

impl Default for Model{
    fn default() -> Self {
        let form = LoginForm{
            email: "".to_string(),
            password: "".to_string()
        };
        let model = Model{
            form: form,
        };
        model
    }
}

pub fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::FormSubmit(_s) => {
            //use seed::html_document;
            let mut fetch_data ="aa".to_string();
            let email = seed::document().get_element_by_id("email");
            let password = seed::document().get_element_by_id("password");
            model.form.email = seed::util::get_value(&email.unwrap()).unwrap();
            model.form.password = seed::util::get_value(&password.unwrap()).unwrap();
            _orders.skip().perform_cmd({
                async { Msg::Fetched(send_message(&model.form).await) }
            });
            //send_message(&model.form);
            //log!(req);
        },
        Msg::Fetched(Ok(response_data)) => {
            let user = Some(response_data);
        }

        Msg::Fetched(Err(fetch_error)) => {
            log!("Example_A error:", fetch_error);
            orders.skip();
        }
    }
}

async fn send_message(form: &LoginForm) -> fetch::Result<shared::models::user::AuthUser> {

    Request::new("/login")
        .method(Method::Post)
        .json(form)?
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}
#[derive(Clone)]
pub enum Pages{
    Home,
    Login(Model),
    Timetable
}

impl Default for Pages {
    fn default() -> Self {
        Pages::Home
    }
}

#[derive(Clone,Debug)]
pub enum Msg{
    FormSubmit(String),
    Fetched(fetch::Result<shared::models::user::AuthUser>),
}

impl Pages{
    pub fn init(&self) -> Vec<Node<Msg>>{
        let mut navbar: Vec<Node<Msg>>;
        match self{
            Pages::Login(m)=> {
                use login::get_log_form;
                navbar = vec![nav![class!{"navbar is-fixed-top"}, self.get_navbar()]];
                navbar.push(get_log_form(m))
            },
            Pages::Home => {
                navbar = vec![nav![class!{"navbar is-fixed-top"}, self.get_navbar()]];
            },
            Pages::Timetable=>{
                navbar = vec![];
            }
        }
        //let mut navbar = self.get_navbar();
        navbar
    }
    pub fn get_nav_end(&self)->Vec<Node<Msg>>{
        let log_menu = raw!(r#"
        <div class="navbar-end">
            <a class="navbar-item"  href="/login">
            Giriş Yap
            </a>
            <a class="navbar-item" href ="/school">
            Üye Ol
            </a>
        </div>
      "#);
        log_menu
    }

    pub fn get_navbar(&self) -> Vec<Node<Msg>>{
        let mut navbar_brand = raw!(r#"
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
        "#);
        navbar_brand.append(self.get_nav_end().as_mut());
        navbar_brand
    }
}



