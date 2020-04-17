use seed::{*, prelude::*};
use serde::{Deserialize, Serialize};

use crate::{Context, STORAGE_KEY};
use shared::models::user::AuthUser;
//use seed::app::subs::url_requested::UrlRequest;

// ------ ------
//     Init
// ------ ------

pub fn init() -> Model {
    Model::default()
}

// ------ ------
//     Model
// ------ ------

#[derive(Default)]
pub struct Model {
    form: LoginForm
}

// TODO: It should be probably in the `shared` crate.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LoginForm{
     username: String,
     password: String
}

// ------ ------
//    Update
// ------ ------

#[derive(Debug)]
pub enum Msg{
    EmailChanged(String),
    PasswordChanged(String),
    SubmitForm,
    Fetched(fetch::Result<AuthUser>),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>, ctx: &mut Context) {
    match msg {
        Msg::EmailChanged(email) => {
            model.form.username = email;
        },
        Msg::PasswordChanged(password) => {
            model.form.password = password;
        },
        Msg::SubmitForm => {
            orders.perform_cmd({
                // `request` has to be outside of the async function because we can't pass reference
                // to the form (`&model.form`) into the async function (~= `Future`).
                // (As a workaround we can `clone` the form, but then there will be unnecessary cloning.)
                let request = Request::new("/login")
                    .method(Method::Post)
                    .json(&model.form);
                // The first `async` is just the function / `Future` / command
                // that will be executed by `orders.perform_cmd`.
                // ---
                // The second `async` function + its `await` allow us to write async code
                // that returns `Result` (consumed by `Msg::Fetched`) and contains `await`s
                // and early returns (`?`).
                async { Msg::Fetched(async {
                    request?
                        .fetch()
                        .await?
                        .check_status()?
                        .json()
                        .await
                }.await)}
            });
        },
        Msg::Fetched(Ok(auth_user)) => {
            let store = storage::get_storage().expect("get local storage");
            storage::store_data(&store, STORAGE_KEY, &auth_user);
            ctx.user = Some(auth_user);
            orders.notify(
                subs::UrlRequested::new(crate::Urls::new(&ctx.base_url).home())
            );
        }

        Msg::Fetched(Err(fetch_error)) => {
            log!("fetch AuthUser error:", fetch_error);
            orders.skip();
        }
    }
}

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model, ctx: &Context)-> Node<Msg>{
    div![C!{"columns"},
        div![C!{"column is-2"}],
        div![C!{"column is-4"},
            form![attrs!{At::Action=>"/login", At::Method=>"Post"},
                div![C!{"field"},
                    label![C!{"label"}, "Giriş Yap"],
                    p![C!{"control has-icons-left"},
                        input![C!{"input"},
                            attrs!{
                                At::Type=>"text",
                                At::Placeholder=>"E-posta veya telefon numarası",
                                // TODO: `username` vs `email`?
                                At::Name=>"username",
                                At::Id=>"email"
                                At::Value => &model.form.username,
                            },
                            input_ev(Ev::Input, Msg::EmailChanged),
                        ],
                        span![C!{"icon is-small is-left"}, i![C!{"fa fa-envelop"}]]
                    ]
                ],
                div![C!{"field"},
                    p![C!{"control has-icons-left"},
                        input![C!{"input"},
                            attrs!{
                                At::Type=>"password",
                                At::Placeholder=>"Şifreniz",
                                // TODO: `username` vs `password`?
                                At::Name=>"password",
                                At::Id=>"password"
                                At::Value => &model.form.password,
                            },
                            input_ev(Ev::Input, Msg::PasswordChanged),
                        ],
                        span![C!{"icon is-small is-left"}, i![C!{"fa fa-envelop"}]],
                        span![&model.form.username]
                    ]
                ],
                div![C!{"field"},
                    p![C!{"control has-icons-left"},
                        input![C!{"button is-primary"},
                            attrs!{
                                At::Type=>"button",
                                At::Value=>"Giriş Yap",
                                At::Id=>"login_button"
                            },
                            ev(Ev::Click, |event| {
                                event.prevent_default();
                                Msg::SubmitForm
                            })
                        ],
                        span![C!{"icon is-small is-left"}, i![C!{"fa fa-envelop"}]]
                    ]
                ],
                div![C!{"field"},
                    "Üye olmak için",
                    a![attrs!{ At::Href => crate::Urls::new(&ctx.base_url).sign_in() },
                        " tıklayınız"
                    ]
                ]
            ]
        ]
    ]
}
