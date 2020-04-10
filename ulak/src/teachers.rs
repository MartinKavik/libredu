//use actix_web::http::{header, Method, StatusCode};
use actix_web::{
    HttpRequest, HttpResponse, Result, http, HttpMessage
};
use actix_web::web::{Path, Json, Data};
//use actix_identity::Identity;
use actix_session::{Session};
//use crate::middlewares::UserAuthentication;
//use std::collections::HashMap;
use crate::forms::*;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
//use diesel::BelongingToDsl;
use crate::util::database::pool;
pub type Pool = r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;
use crate::request::*;
use crate::schema::*;
use crate::models::school::{School, SchoolTime};
use crate::models::user::{User, AuthUser};
use crate::models::others::{Class, Subject, TeacherAvailable, Timetable, Day};
use crate::models::activity::Activities;

pub async fn detail(path: Path<(i32,i32)>,session: Session, tmpl: Data<tera::Tera>,req: HttpRequest)-> Result<HttpResponse> {
    //use crate::schema::city::dsl::*;
    use http::Method;
    let mut context = req.context(&session);
    //use actix_web::HttpMessage;
    let pth = path.into_inner();
    let _r = req.headers().get(http::header::ACCEPT).unwrap();
    //println!("{:?}", _r);
    //println!("mime{:?}", req.mime_type().unwrap());
    let school_auth = req.authorized(&session, pth.0, Some(pth.1), None);
    match school_auth{
        SchoolAuth::AUTH=>{
            match *req.method() {
                Method::GET => {
                    let conn = pool();
                    use crate::schema::teachers_menu::dsl::*;
                    let mut schl = crate::schema::school::table.filter(crate::schema::school::code.eq(pth.0))
                        .get_result::<School>(&conn);
                    context.insert("school", &schl.as_mut().unwrap());
                    let teacher = crate::schema::users::table.filter(crate::schema::users::id.eq(pth.1))

                        .get_result::<User>(&conn).unwrap();
                    context.insert("teacher", &teacher);
                    let thcrs = req.teachers(pth.0);
                    context.insert("teachers", &thcrs);
                    let index = thcrs.iter()
                        .enumerate()
                        .find(|i| i.1.id == teacher.id).unwrap();
                    let menu = teachers_menu
                        .select((title, link))
                        .load::<(Option<String>, Option<String>)>(&conn);
                    context.insert("index",&index.0);
                    context.insert("menus", &menu.unwrap());
                    let s = tmpl.render("teacher_detail.html", &context);
                    Ok(HttpResponse::Ok().content_type("text/html").body(s.unwrap()))
                },
                Method::DELETE => {
                    let conn = pool();
                    let user= crate::schema::users::table
                        .filter(crate::schema::users::id.eq(pth.1))
                        .get_result::<User>(&conn);
                    match user {
                        Ok(u)=>{
                            let _del_teacher_times = diesel::delete(crate::schema::teacher_available::table)
                                .filter(crate::schema::teacher_available::user_id.eq(&pth.1))
                                .filter(crate::schema::teacher_available::school_id.eq(&pth.0))
                                .execute(&conn);
                            let _del_teacher = diesel::delete(crate::schema::school_users::table)
                                .filter(crate::schema::school_users::user_id.eq(&pth.1))
                                .filter(crate::schema::school_users::school_id.eq(&pth.0))
                                .execute(&conn);
                            if u.is_staff == Some(false){
                                let del_teacher = diesel::delete(crate::schema::users::table)
                                    .filter(crate::schema::users::id.eq(u.id))
                                    .execute(&conn);
                                match del_teacher{
                                    Ok(_t)=>{
                                        Ok(HttpResponse::Ok().content_type("application/json").json(&"Kullanıcı tamamen silindi"))
                                    },
                                    Err(_e)=>{
                                        let classes: Vec<i32> = crate::schema::classes::table
                                            .select(crate::schema::classes::id)
                                            .filter(crate::schema::classes::school.eq(pth.0))
                                            .get_results::<i32>(&conn).unwrap_or(vec![]);
                                        let _del_teacher = diesel::delete(crate::schema::activities::table)
                                            .filter(crate::schema::activities::class.eq_any(classes))
                                            .filter(crate::schema::activities::teacher.eq(u.id))
                                            .execute(&conn);
                                        Ok(HttpResponse::Ok().content_type("application/json").json(&"Kullanıcı aktiviteleriyle beraber tamamen silindi"))
                                    }
                                }

                            }
                            else{
                                Ok(HttpResponse::Ok().content_type("application/json").json(serde_json::to_string(&"Öğretmen okulunuzdan çıkarıldı").unwrap()))
                            }
                        },
                        Err(_e)=>{
                            Ok(HttpResponse::Ok().content_type("application/json").json(&_e.to_string()))
                        }
                    }
                },
                _ => {
                    Ok(HttpResponse::Ok().content_type("application/json").json("Metod desteklenmiyor".to_string()))
                }
            }
        },
        _=>{
            match req.mime_type(){
                Ok(_mime)=>{
                    context.insert("error", &"Yetkiniz yok veya okul, öğretmen değeri uyumsuz.");
                    let s = tmpl.render("teacher_detail.html", &context);
                    Ok(HttpResponse::Ok().content_type("text/html").body(s.unwrap()))
                },
                Err(_e)=>{
                    Ok(HttpResponse::Ok().content_type("application/json").json("Yetkiniz yok veya okul, öğretmen değeri uyumsuz.".to_string()))
                }
            }

        }
    }
}

pub async fn activities(path: Path<(i32,i32)>, req: HttpRequest,tmpl: Data<tera::Tera>,session: Session)
-> Result<HttpResponse>{
    let mut context = req.context(&session);
    let conn = pool();
   // use diesel::insert_into;
    use http::Method;
    let pth = path.into_inner();
    let school_auth = req.authorized(&session, pth.0, Some(pth.1), None);
    match school_auth{
        SchoolAuth::AUTH=>{
            match *req.method(){
                Method::GET=>{
                    //let teacher_id = path.into_inner().clone().1;
                    let mut schl = crate::schema::school::table.filter(crate::schema::school::code.eq(pth.0))
                        .get_result::<School>(&conn);
                    context.insert("school", &schl.as_mut().unwrap());

                    use crate::schema::teachers_menu::dsl::*;
                    let menu = teachers_menu
                        .select((title, link))
                        .load::<(Option<String>, Option<String>)>(&conn);
                    context.insert("menus", &menu.unwrap());

                    let classes = crate::schema::classes::table.filter(crate::schema::classes::school.eq(schl.as_mut().unwrap().code))
                        //.select(id,sube,kademe,school)
                        .order((crate::schema::classes::kademe, crate::schema::classes::sube))
                        .get_results::<Class>(&conn);
                    match classes{
                        Ok(class)=>{
                            context.insert("classes", &class);
                            if class.len() == 0{
                                context.insert("error", &"Sınıf eklemelisiniz!");
                                let s = tmpl.render("teachers/activities.html", &context);
                                return Ok(HttpResponse::Ok().content_type("text/html").body(s.unwrap()));
                            }
                            let mut sbjcts = crate::schema::subjects::table
                                .inner_join(crate::schema::class_subjects::table)
                                .filter(crate::schema::class_subjects::class.eq(class[0].id))
                                .order(crate::schema::subjects::name)
                                .select(crate::schema::subjects::all_columns)
                                .get_results::<Subject>(&conn).unwrap();
                            if sbjcts.len()==0{
                                sbjcts = subjects::table
                                    .filter(subjects::kademe.eq(class[0].kademe))
                                    .filter(subjects::school_type.eq(schl.as_mut().unwrap().school_type))
                                    .order((subjects::optional,subjects::name))
                                    .select(subjects::all_columns)
                                    .get_results::<Subject>(&conn).unwrap();
                            }
                            context.insert("subjects", &sbjcts);
                            let teachers = crate::schema::school_users::table.filter(crate::schema::school_users::school_id.eq(schl.as_mut().unwrap().code))
                                .inner_join(crate::schema::users::table)
                                .select(crate::schema::users::all_columns)
                                .order(crate::schema::users::first_name)
                                .get_results::<User>(&conn);
                            context.insert("teachers", &teachers.unwrap());
                            let class = crate::schema::classes::table.filter(crate::schema::classes::school.eq(schl.as_mut().unwrap().code))
                                .select(crate::schema::classes::id)
                                .get_results::<i32>(&conn);
                            use crate::schema::users::dsl::*;
                            let mut teacher = crate::schema::users::table.filter(crate::schema::users::id.eq(pth.1))
                                .get_result::<User>(&conn);
                            context.insert("teacher", &teacher.as_mut().unwrap());
                            let acts = crate::schema::activities::table.inner_join(users)
                                .inner_join(crate::schema::classes::table)
                                .inner_join(crate::schema::subjects::table)
                                .filter(crate::schema::activities::class.eq_any(class.unwrap()))
                                .filter(crate::schema::activities::teacher.eq(teacher.unwrap().id))
                                .select((crate::schema::activities::id
                                         ,(crate::schema::subjects::all_columns)
                                         ,(crate::schema::users::id, crate::schema::users::first_name, crate::schema::users::last_name, crate::schema::users::username, crate::schema::users::email, crate::schema::users::is_admin)
                                         ,(crate::schema::classes::all_columns)
                                         ,(crate::schema::activities::hour)
                                         ,(crate::schema::activities::split)))
                                .load::<Activities>(&conn);
                            //println!("{:?}", acts.unwrap());
                            context.insert("activities", &acts.unwrap());
                            let s = tmpl.render("teachers/activities.html",&context);
                            Ok(HttpResponse::Ok().body(s.unwrap()))
                        },
                        Err(_e)=>{
                            context.insert("error", &"Sınıf eklemelisiniz!");
                            let s = tmpl.render("teachers/activities.html", &context);
                            Ok(HttpResponse::Ok().content_type("text/html").body(s.unwrap()))
                        }
                    }


                },
                _=>{
                    Ok(HttpResponse::Ok().content_type("application/json").json(&"Metod desteklenmiyor"))
                }
            }
        },
        _=>{
            match req.mime_type(){
                Ok(_mime)=>{
                    context.insert("error", &"Yetkiniz yok veya okul, öğretmen değeri uyumsuz.");
                    let s = tmpl.render("teachers/activities.html", &context);
                    Ok(HttpResponse::Ok().content_type("text/html").body(s.unwrap()))
                },
                Err(_e)=>{
                    Ok(HttpResponse::Ok().content_type("application/json").json("Yetkiniz yok veya okul, öğretmen değeri uyumsuz.".to_string()))
                }
            }
        }
    }
}

pub async fn limitations( path: Path<(i32,i32)>, pays: Option<Json<Vec<TeacherAvailableForm>>>,req: HttpRequest,tmpl: Data<tera::Tera>,session: Session)
-> Result<HttpResponse>{
    use http::Method;
    let pth = path.into_inner();
    let mut context = req.context(&session);
    match req.authorized(&session, pth.0, Some(pth.1), None){
        SchoolAuth::AUTH=>{
            match *req.method(){
                Method::GET=>{
                    let conn = pool();
                    let mut schl = crate::schema::school::table.filter(crate::schema::school::code.eq(pth.0))
                        .get_result::<School>(&conn);
                    context.insert("school", &schl.as_mut().unwrap());
                    //use crate::schema::teachers_menu::dsl::*;
                    let menu = crate::schema::teachers_menu::table
                        .select((crate::schema::teachers_menu::title, crate::schema::teachers_menu::link))
                        .load::<(Option<String>, Option<String>)>(&conn);
                    let teachers = crate::schema::school_users::table.filter(crate::schema::school_users::school_id.eq(pth.0))
                        .inner_join(crate::schema::users::table)
                        .select(crate::schema::users::all_columns)
                        .order(crate::schema::users::first_name)
                        .get_results::<User>(&conn);
                    context.insert("teachers", &teachers.unwrap());
                    context.insert("menus", &menu.unwrap());
                    let _hrs = crate::schema::school_time::table.filter(crate::schema::school_time::school.eq(schl.as_mut().unwrap().code))
                        .get_results::<SchoolTime>(&conn);
                    let days = crate::schema::days::table.get_results::<(i32,String)>(&conn);
                    context.insert("days", &days.unwrap());
                    let teacher = crate::schema::users::table.filter(crate::schema::users::id.eq(pth.1))
                        .get_result::<User>(&conn);
                    context.insert("teacher", &teacher.unwrap());
                    use crate::schema::teacher_available::dsl::*;
                    let teacher_available_hours = teacher_available
                        .filter(user_id.eq(pth.1))
                        .filter(school_id.eq(pth.0))
                        .order(crate::schema::teacher_available::day)
                        .get_results::<TeacherAvailable>(&conn);
                    match teacher_available_hours{
                        Ok(e)=>{
                            let total_hour = schl.unwrap().clone().hour;
                            if e.len() == 0 || e[0].hours.len() != total_hour as usize{
                                for i in 1..8{
                                    let mut hour_vec: Vec<bool>=Vec::new();
                                    for _i in 0..total_hour{
                                        if i <= 5{
                                            hour_vec.push(true);
                                        }
                                        else{
                                            hour_vec.push(false);
                                        }
                                    }
                                    let new_av = TeacherAvailable{
                                        user_id: pth.1,
                                        school_id: pth.0,
                                        day: i,
                                        hours:hour_vec.clone()
                                    };
                                    let _ekle = diesel::insert_into(teacher_available).values(&new_av).execute(&conn);
                                }
                                let _e = teacher_available
                                    .filter(user_id.eq(pth.1))
                                    .filter(school_id.eq(pth.0))
                                    .get_results::<TeacherAvailable>(&conn).unwrap();
                                context.insert("hours", &_e);
                            }
                            else{
                                context.insert("hours", &e);
                            }
                        },
                        Err(_e)=>{
                        }
                    }

                    let s = tmpl.render("teachers/limitations.html",&context);
                    Ok(HttpResponse::Ok().body(s.unwrap()))
                },
                Method::POST=>{
                    let conn = pool();
                    use diesel::insert_into;
                    let pay = pays.unwrap().into_inner();
                    use crate::schema::teacher_available::dsl::*;

                    let _schl = crate::schema::school::table.filter(crate::schema::school::code.eq(pth.0))
                        .get_result::<School>(&conn).unwrap();

                    let teacher_available_hours = teacher_available
                        .filter(user_id.eq(pth.1))
                        .filter(school_id.eq(pth.0))
                        //.select((user_id,school_id))
                        .get_results::<TeacherAvailable>(&conn);
                    match teacher_available_hours{
                        Ok(hour)=>{
                            for h in hour.iter(){
                                let p = pay.iter().find(|t| t.day == h.day).unwrap();
                                let _update = diesel::update(teacher_available)
                                    .filter(user_id.eq(pth.1))
                                    .filter(school_id.eq(pth.0))
                                    .filter(day.eq(p.day))
                                    .set(hours.eq(p.hours.as_ref().unwrap()))
                                    .execute(&conn);
                            };
                            Ok(HttpResponse::Ok().content_type("application/json").json(&pay))
                        },
                        Err(hour)=>{
                            let _teacher_available_hour = insert_into(teacher_available).values(&pay)
                                .get_results::<TeacherAvailable>(&conn);

                            Ok(HttpResponse::Ok().content_type("application/json").json(&hour.to_string()))
                        }
                    }
                },
                _=>{
                    Ok(HttpResponse::Ok().content_type("application/json").json("Metod desteklenmiyor"))
                }
            }
        },
        _=>{
            Ok(HttpResponse::Ok().content_type("application/json").json("Yetkiniz yok"))
        }

    }
}

pub async fn timetable( path: Path<(i32,i32)>, req: HttpRequest,tmpl: Data<tera::Tera>,db: Data<Pool>, session: Session)
                          -> Result<HttpResponse>{
    let conn = db.get().unwrap();
    let school = path.into_inner();
    let mut context = req.context(&session);
    let school_auth = req.authorized(&session, school.0, Some(school.1), None);
    match school_auth{
        SchoolAuth::AUTH=>{
            let teacher = crate::schema::users::table
                .filter(crate::schema::users::id.eq(school.1))
                .select((crate::schema::users::id,users::first_name, users::last_name, users::username,users::email, users::is_admin))
                .get_result::<AuthUser>(&conn).unwrap();
            context.insert("teacher", &teacher);
            let s = req.school(school.0).unwrap();
            let teachers = crate::schema::school_users::table
                .inner_join(crate::schema::users::table)
                .filter(crate::schema::school_users::school_id.eq(school.0))
                .select((crate::schema::users::id,users::first_name, users::last_name, users::username,users::email, users::is_admin))
                .get_results::<AuthUser>(&conn).unwrap();
            use crate::schema::*;
            context.insert("school", &s);
            let mut class_timetables : Vec<(usize, usize, Vec<Timetable>)>= Vec::new();
            let timetables = class_timetable::table
                //.inner_join(crate::schema::classes::table)
                .inner_join(activities::table
                    .inner_join(subjects::table)
                    .inner_join(users::table)
                    .inner_join(classes::table))
                .order(class_timetable::day_id)
                .order(class_timetable::hour)
                .select((class_timetable::id,
                         //crate::schema::classes::all_columns,
                         class_timetable::day_id,
                         class_timetable::hour,
                         (activities::id
                          ,subjects::all_columns
                          ,(users::id,users::first_name, users::last_name, users::username, users::email, users::is_admin)
                          ,classes::all_columns
                          ,activities::hour
                          ,activities::split)))
                .filter(activities::teacher.eq(&Some(school.1)))
                .get_results::<Timetable>(&conn).unwrap();
            if s.hour == 8 {
                class_timetables.push((0, 7, timetables.clone()));
            }
            if s.hour == 7 {
                class_timetables.push((0, 6, timetables.clone()));
            }
            if s.hour == 14 {
                if timetables.len()>0{
                    if timetables[0].hour<6 {
                        class_timetables.push((0, 6, timetables.clone()));
                    } else {
                        class_timetables.push((7, 13, timetables.clone()));
                    }
                }
                else{
                    class_timetables.push((0, 6, timetables.clone()));
                }

            }
            let days = crate::schema::days::table
                .filter(crate::schema::days::id.lt(6))
                .get_results::<Day>(&conn);
            let menu = crate::schema::teachers_menu::table
                .select((crate::schema::teachers_menu::title, crate::schema::teachers_menu::link))
                .load::<(Option<String>, Option<String>)>(&conn);
            context.insert("menus", &menu.unwrap());
            context.insert("days", &days.unwrap());
            context.insert("timetables",&class_timetables);
            context.insert("teachers",&teachers);
            let s = tmpl.render(&"teachers/timetable.html", &context);
            Ok(HttpResponse::Ok().body(&s.unwrap()))
        },
        _=>{
            Ok(HttpResponse::Ok().json("merhaba"))
        }
    }
}
