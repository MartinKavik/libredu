//use actix_identity::Identity;
use actix_session::{Session};
use crate::forms::*;

use diesel::RunQueryDsl;

use crate::util::database::pool;

use actix_web::{
    HttpRequest, HttpResponse, Result, http
};
use actix_web::web::{Json, Data};
use crate::request::Request;
use crate::models::others::{Cities, Subject};
use crate::models::school::SchoolType;

pub async fn index(tmpl: Data<tera::Tera>, session: Session, req: HttpRequest, city: Option<Json<NewCity>> )
    -> Result<HttpResponse> {
    let mut context = tera::Context::new();
    use http::Method;
    if !req.is_auth(&session){
        Ok(HttpResponse::Found().header("location", "/").finish())
    }
    else{
        let mut user = req.user(&session);
        if user.as_mut().unwrap().is_admin != Some(true){
            Ok(HttpResponse::Found().header("location", "/").finish())
        }
        else{
            let conn = pool();
            context.insert("user",&user.unwrap());
            context.insert("is_auth", &true);
            let cities = crate::schema::city::table
                .load::<Cities>(&conn);
            let s_types = crate::schema::school_type::table
                .load::<SchoolType>(&conn);
            context.insert("cities", &cities.unwrap());
            context.insert("school_type", &s_types.unwrap());
            let s = tmpl.render(&"admin/index.html", &context);
            match *req.method(){ 
                Method::GET=>{
            //let mut user = req.user(&session);
                    Ok(HttpResponse::Ok().content_type("text/html").body(s.unwrap()))
                },
                Method::POST=>{
                    match city{
                        Some(c)=>{
                            let _add_city = diesel::insert_into(crate::schema::city::table).values(&c.into_inner())
                                .load::<NewCity>(&conn);
                            Ok(HttpResponse::Ok().content_type("text/html").body(s.unwrap()))
                        },
                        None=>{
                            Ok(HttpResponse::Ok().content_type("text/html").body(s.unwrap()))
                        }
                    }
                },
                _=>{
                    Ok(HttpResponse::Found().header("location", "/").finish())
                }
            }
        }
    }
}  

pub async fn town(tmpl: Data<tera::Tera>, session: Session, req: HttpRequest, town: Option<Json<NewTown>>)
    -> Result<HttpResponse> {
    let mut context = tera::Context::new();
    use http::Method;
    if !req.is_auth(&session){
        Ok(HttpResponse::Found().header("location", "/").finish())
    }
    else{
        let mut user = req.user(&session);
        if user.as_mut().unwrap().is_admin != Some(true){
            Ok(HttpResponse::Found().header("location", "/").finish())
        }
        else{
            let conn = pool();
            context.insert("user",&user.unwrap());
            context.insert("is_auth", &true);
            let cities = crate::schema::city::table
                .load::<Cities>(&conn);
            let s_types = crate::schema::school_type::table
                .load::<SchoolType>(&conn);
            context.insert("cities", &cities.unwrap());
            context.insert("school_type", &s_types.unwrap());
            let s = tmpl.render(&"admin/index.html", &context);
            match *req.method(){ 
                Method::GET=>{
            //let mut user = req.user(&session);
                    Ok(HttpResponse::Ok().content_type("text/html").body(s.unwrap()))
                },
                Method::POST=>{
                    match town{
                        Some(c)=>{
                            let _add_town = diesel::insert_into(crate::schema::town::table).values(&c.into_inner())
                                .load::<NewTown>(&conn);
                            Ok(HttpResponse::Ok().content_type("text/html").body(s.unwrap()))
                        },
                        None=>{
                            Ok(HttpResponse::Ok().content_type("text/html").body(s.unwrap()))
                        }
                    }
                },
                _=>{
                    Ok(HttpResponse::Found().header("location", "/admin").finish())
                }
            }
        }
    }
}

pub async fn subject(tmpl: Data<tera::Tera>, session: Session, req: HttpRequest, subject: Option<Json<NewSubjectsForm>>)
    -> Result<HttpResponse> {
    let mut context = tera::Context::new();
    use http::Method;
    if !req.is_auth(&session){
        Ok(HttpResponse::Found().header("location", "/").finish())
    }
    else{
        let mut user = req.user(&session);
        if user.as_mut().unwrap().is_admin != Some(true){
            Ok(HttpResponse::Found().header("location", "/").finish())
        }
        else{
            let conn = pool();
            context.insert("user",&user.unwrap());
            context.insert("is_auth", &true);
            let cities = crate::schema::city::table
                .load::<Cities>(&conn);
            let s_types = crate::schema::school_type::table
                .load::<SchoolType>(&conn);
            context.insert("cities", &cities.unwrap());
            context.insert("school_type", &s_types.unwrap());
            let s = tmpl.render(&"admin/index.html", &context);
            match *req.method(){ 
                Method::GET=>{
            //let mut user = req.user(&session);
                    Ok(HttpResponse::Ok().content_type("text/html").body(s.unwrap()))
                },
                Method::POST=>{
                    match subject{
                        Some(c)=>{
                            let _add_subject = diesel::insert_into(crate::schema::subjects::table).values(&c.into_inner())
                                .load::<Subject>(&conn);
                            Ok(HttpResponse::Ok().content_type("text/html").json(serde_json::to_string("kaydedildi").unwrap()))
                        },
                        None=>{
                            Ok(HttpResponse::Ok().content_type("text/json").json(serde_json::to_string("kaydedilmedi").unwrap()))
                        }
                    }
                },       
                _=>{
                    Ok(HttpResponse::Found().header("location", "/admin").finish())
                }
            }
        }
    }
}
    