use crate::pages::{Model, Msg};
use seed::{*, prelude::*};

pub fn get_log_form(model: &Model)-> Node<Msg>{
    let login_page = div![
            class!{"columns"}, div![class!{"column is-2"}],
                div![class!{"column is-4"},
                    form![attrs!{At::Action=>"/login", At::Method=>"Post"},
                        div![class!{"field"}, label![class!{"label"}, "Giriş Yap"],
                            p![class!{"control has-icons-left"},
                                input![class!{"input"}, attrs!{At::Type=>"text", At::Placeholder=>"E-posta veya telefon numarası", At::Name=>"username", At::Id=>"email"}],
                                span![class!{"icon is-small is-left"}, i![class!{"fa fa-envelop"}]]
                            ]
                        ],
                        div![class!{"field"},
                            p![class!{"control has-icons-left"},
                                input![class!{"input"}, attrs!{At::Type=>"password", At::Placeholder=>"Şifreniz", At::Name=>"username", At::Id=>"password"}],
                                span![class!{"icon is-small is-left"}, i![class!{"fa fa-envelop"}]],
                                span![&model.form.email]
                            ]
                        ],
                        div![class!{"field"},
                            p![class!{"control has-icons-left"},
                                input![class!{"button is-primary"}, attrs!{At::Type=>"button", At::Value=>"Giriş Yap", At::Id=>"login_button"},
                                    input_ev(Ev::Click, Msg::FormSubmit)
                                ],
                                span![class!{"icon is-small is-left"}, i![class!{"fa fa-envelop"}]]
                            ]
                        ],
                        div![class!{"field"},
                            "Üye olmak için", a![attrs!{At::Href=>"/signin"}, " tıklayınız"]
                        ]
                    ]]
        ];
    login_page
}