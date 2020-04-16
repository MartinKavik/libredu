use actix_web::http::{Method};
use actix_web::*;
use actix_web::web::{Path, Data, Json};
//use actix_identity::Identity;

use actix_session::{Session};
//use crate::middlewares::UserAuthentication;
use std::collections::HashMap;
use crate::forms::*;
use diesel::{ExpressionMethods};
use diesel::QueryDsl;
use diesel::RunQueryDsl;
//use diesel::BelongingToDsl;
use crate::util::database::pool;
use crate::request::Request;
//use uuid::Uuid;
use chrono::Utc;
use crate::models::others::{Post, Town};
use crate::models::user::{AuthUser, User, NewUser};

pub async fn index(tmpl: Data<tera::Tera>, req: HttpRequest, session: Session, payload: Option<Json<PostForm>>)
    -> Result<HttpResponse> {
    let mut context = req.context(&session);
    //println!("{:?}", serde_json(req.get_session()));
    let conn = pool();
    match *req.method() {
        Method::GET=>{
            use crate::schema::post::dsl::*;
            let posts: Vec<Post> = post
                .order(pub_date.desc())
                .limit(10)
                .load::<Post>(&conn).unwrap_or(vec![]);
            context.insert("posts", &posts);
            let s = tmpl.render(&"index.html", &context);
            Ok(HttpResponse::Ok().content_type("text/html").body(s.unwrap()))
        },
        Method::POST=>{
            match payload{
                Some(_pst) => {
                    if req.is_auth(&session){
                        let admin = req.user(&session);
                        match admin {
                            Some(u) => {
                                if u.is_admin.unwrap() {
                                    use diesel::insert_into;
                                    use crate::schema::post::dsl::*;
                                    let mut pst = _pst.into_inner();
                                    pst.pub_date = Some(Utc::now().naive_utc());
                                    pst.body = pst.body.replace("\n","<br>");
                                    //let _r = req.headers().get(http::header::ACCEPT);
                                    let _post = insert_into(post)
                                        .values(&pst)
                                        .get_result::<Post>(&conn);
                                    Ok(HttpResponse::Ok().content_type("application/json").json(_post.unwrap()))
                                }
                                else{
                                    Ok(HttpResponse::Ok().content_type("application/json").json("Yetkiniz yok"))
                                }
                            },
                            None => {
                                Ok(HttpResponse::Ok().content_type("application/json").json("Yetkiniz yok"))
                            }
                        }
                    }
                    else{
                        Ok(HttpResponse::Ok().content_type("application/json").json("Giriş Yapınız"))
                    }

                },
                None => {
                    Ok(HttpResponse::Ok().content_type("application/json").json("Verilerinizde hata var"))
                }
            }

        },
        _=>{
            Ok(HttpResponse::Ok().content_type("application/json").json("Metod desteklenmiyor"))
        }

    }
}


pub async fn login(tmpl: Data<tera::Tera>, session: Session, req: HttpRequest, payload:Option<Json<LoginForm>>) -> Result<HttpResponse> {
    let mut context = tera::Context::new();
    if req.is_auth(&session){
        println!("aaa");
        use crate::schema::users::dsl::*;
        let conn = pool();
        let user: Option<AuthUser> = session.get("user").unwrap();
        Ok(HttpResponse::Ok().content_type("application/json").json(user.unwrap()))
        //Ok(HttpResponse::Found().header("location", "/").finish())
    }
    else{
        match *req.method(){
            Method::GET=>{
                let s = tmpl.render(&"login.html", &context);
                Ok(HttpResponse::Ok().content_type("text/html").body(s.unwrap()))
            },
            Method::POST=>{
                println!("bb");
                match payload{
                    Some(post)=>{
                        println!("c");
                        let log = req.login(&post.username, &post.password);
                        if log {
                            use crate::schema::users::dsl::*;
                            let conn = pool();
                            //i.remember(post.username.clone());
                            let user = users
                                .filter(email.eq(&post.username))
                                .select((id,first_name,last_name,username, email, is_admin))
                                .get_result::<AuthUser>(&conn).unwrap();
                            session.clear();
                            session.set("user", &user).unwrap();
                            session.set("is_auth", &true).unwrap();
                            context.insert("status", &"202");
                            //println!("{:?}\n", req.headers());
                            //println!("mime{:?}", req.mime_type().unwrap().unwrap());
                            Ok(HttpResponse::Ok().content_type("application/json").json(user))
                        }
                        else{
                            context.insert("status", &"401");
                            context.insert("error", &"Kullanıcı adı veya şifre hatalı");
                            Ok(HttpResponse::Ok().content_type("application/json").json(context))
                        }
                    },
                    None=>{
                        println!("dd");
                        context.insert("status", &"400");
                        context.insert("error", &"Giriş bilgileri hatalı");
                        Ok(HttpResponse::Ok().content_type("application/json").json(context))
                    }
                }
            },
            _=>{
                println!("ee");
                context.insert("status", &"405");
                context.insert("error", &"Metod desteklenmiyor");
                Ok(HttpResponse::Ok().content_type("application/json").json(context))
            }
        }

    }

}


pub async fn signin(mut pays: Option<Json<SignForm>>, tmpl: Data<tera::Tera>, req: HttpRequest, session: Session)->Result<HttpResponse> {

    if req.is_auth(&session){
        Ok(HttpResponse::Found().header("location", "/").finish())
    }
    else {
        let mut context = tera::Context::new();
        match *req.method() {
            Method::POST => {
                let conn = pool();
                //let sign_user = pays.clone();
                //let mail = pays.as_mut().unwrap().email.as_mut();
                let mobile = pays.as_mut().unwrap().tel.clone();
                let mail = pays.as_mut().unwrap().email.clone();
                use crate::schema::users::dsl::*;
                let user = users.filter(tel.eq(mobile)).or_filter(email.eq(mail.clone()))
                    .get_result::<User>(&conn);
                match user{
                    Ok(_u)=>{
                        let mut err=HashMap::new();
                        err.insert(String::from("tel"), "Bu telefon numarası veya eposta hesabı kayıtlı");
                        context.insert("error",&err);
                        let _s = tmpl.render(&"signin.html", &context);
                        Ok(HttpResponse::Ok().content_type("application/json").json(err))
                    },
                    Err(_e)=>{
                        use diesel::insert_into;
                        let psw1 = pays.as_mut().unwrap().password1.as_mut().cloned().unwrap();
                        let psw2 = pays.as_mut().unwrap().password2.as_mut().cloned().unwrap();
                        if psw1 == psw2 || &psw1.len() > &3{
                            use bcrypt::{hash};
                            let u = NewUser {
                                username: None,
                                first_name: pays.as_mut().unwrap().first_name.as_mut().cloned(),
                                last_name: pays.as_mut().unwrap().last_name.as_mut().cloned(),
                                email: pays.as_mut().unwrap().email.as_mut().cloned(),
                                password: Some(hash(psw1.clone(), 8).unwrap()),
                                date_join: Some(Utc::now().naive_utc()),
                                last_login: Some(Utc::now().naive_utc()),
                                is_active: Some(true),
                                is_staff: Some(true),
                                is_admin: Some(false),
                                tel: pays.as_mut().unwrap().tel.as_mut().cloned(),
                                gender: pays.as_mut().unwrap().gender.clone(),
                                img: None
                            };
                            if &u.clone().first_name.unwrap().len()>&0 && &u.clone().last_name.unwrap().len()>&0{
                                let user = insert_into(users).values(u)
                                    .get_result::<User>(&conn);
                                match user{
                                    Ok(_user)=>{

                                        if req.login(&mail.unwrap(), &psw1){
                                            let auth_user = users
                                                .filter(id.eq(&_user.id))
                                                .select((id,first_name,last_name,username, email, is_admin))
                                                .get_result::<AuthUser>(&conn);
                                            session.clear();
                                            session.set("user", auth_user.unwrap()).unwrap();
                                            session.set("is_auth", &true).unwrap();
                                            Ok(HttpResponse::Ok().content_type("application/json").json("Giriş başarılı".to_string()))
                                        }
                                        else{
                                            Ok(HttpResponse::Ok().content_type("application/json").json(("error", &"Giriş yapılamıyor".to_string())))
                                        }

                                    },
                                    Err(_e)=>{
                                        Ok(HttpResponse::Ok().content_type("application/json").json(("error", &"Veritabanı hatası".to_string())))
                                    }
                                }
                                //Ok(HttpResponse::Found().header("location", "/").finish())
                            }
                            else{
                                context.insert("error",&"İsimler boş geçilemez");
                                //let s = tmpl.render(&"signin.html", &context);
                                Ok(HttpResponse::Ok().content_type("application/json").json(context))
                            }

                            //let s = tmpl.render(&"signin.html", &context);

                        }
                        else{
                            let mut err=HashMap::new();
                            err.insert(String::from("email"), "Şifreler uyuşmuyor veya boş");
                            context.insert("error",&err);
                            let _s = tmpl.render(&"signin.html", &context);
                            Ok(HttpResponse::Ok().content_type("application/json").json(err))
                        }
                    }
                }

            },
            Method::GET => {
                let s = tmpl.render(&"signin.html", &context);
                Ok(HttpResponse::Ok().content_type(req.content_type()).body(s.unwrap()))
            },
            _ => {
                let s = tmpl.render(&"signin.html", &context);
                Ok(HttpResponse::Ok().content_type(req.content_type()).body(s.unwrap()))
            }
        }
    }

}


pub async fn logout(session: Session)->Result<HttpResponse>{

    session.purge();
    session.set("is_auth", false).unwrap();
    Ok(HttpResponse::Found().header("location", "/").finish())
}

pub async fn cities(path: Path<i32>)-> Result<HttpResponse> {
    use crate::schema::town::dsl::*;
    let conn = pool();
    let towns = town.filter(city.eq(path.into_inner()))
        .get_results::<Town>(&conn);
    
    Ok(HttpResponse::Ok().json(serde_json::to_string_pretty(&towns.unwrap()).unwrap()))
}