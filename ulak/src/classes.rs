//use actix_web::http::{header, Method, StatusCode};
use actix_web::{
    HttpRequest, HttpResponse, Result, http
};
use actix_web::web::{Path, Json, Data};
//use actix_identity::Identity;
use actix_session::{Session};
//use crate::middlewares::UserAuthentication;
//use std::collections::HashMap;
use crate::forms::*;
use crate::schema::*;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
//use diesel::BelongingToDsl;
use crate::util::database::pool;
pub type Pool = r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;
use crate::request::*;
use crate::models::others::{ClassAvailable, Class, Subject, Timetable, Day};
use crate::models::school::{SchoolTime, School};
use crate::models::user::{User};
use crate::models::activity::{Activities, Activity};

pub async fn detail(path: Path<(i32,i32)>, session: Session, tmpl: Data<tera::Tera>,req: HttpRequest)-> Result<HttpResponse> {
    //use crate::schema::city::dsl::*;
    use http::Method;
    //use actix_web::HttpMessage;
    let pth = path.into_inner();
    let mut context = req.context(&session);
    let school_auth = req.authorized(&session, pth.0, None, Some(pth.1));
    match school_auth{
        SchoolAuth::AUTH=>{
            match *req.method(){
                Method::GET=>{
                    let conn = pool();

                    use crate::schema::teachers_menu::dsl::*;

                    let mut schl = crate::schema::school::table.filter(crate::schema::school::code.eq(pth.0))
                        .get_result::<School>(&conn);
                    context.insert("school", &schl.as_mut().unwrap());
                    let class = crate::schema::classes::table.filter(crate::schema::classes::id.eq(pth.1))
                        .get_result::<Class>(&conn);
                    let classes = crate::schema::classes::table.filter(crate::schema::classes::school.eq(pth.0))
                        .get_results::<Class>(&conn);
                    context.insert("class", &class.unwrap());
                    context.insert("classes", &classes.unwrap());
                    let menu = teachers_menu
                        .select((title, link))
                        .load::<(Option<String>, Option<String>)>(&conn);
                    context.insert("menus", &menu.unwrap());
                    let s = tmpl.render("class_detail.html", &context);
                    Ok(HttpResponse::Ok().content_type("text/html").body(s.unwrap()))
                },
                _=>{
                    Ok(HttpResponse::Ok().content_type("application/json").json("Metod desteklenmiyor"))
                }
            }
        },
        _=>{
            context.insert("status", &"400");
            context.insert("error", &"Yetkiniz yok");
            Ok(HttpResponse::Ok().content_type("text/html").json(context))
        }

    }
}

pub async fn activities( path: Path<(i32,i32)>, pays: Option<Json<NewActivitiesForm>>,req: HttpRequest,tmpl: Data<tera::Tera>,session: Session)
-> Result<HttpResponse>{
    let mut context = req.context(&session);
    use http::Method;
    let pth = path.into_inner();
    let school_auth = req.authorized(&session,pth.0, None, Some(pth.1));
    match school_auth{
        SchoolAuth::AUTH=>{
            match *req.method(){
                Method::GET=>{
                    let conn = pool();
                    //let teacher_id = path.into_inner().clone().1;
                    let schl = req.school(pth.0).unwrap();
                    context.insert("school", &schl);
                    use crate::schema::teachers_menu::dsl::*;
                    let menu = teachers_menu
                        .select((title, link))
                        .load::<(Option<String>, Option<String>)>(&conn);
                    context.insert("menus", &menu.unwrap());
                    let class = crate::schema::classes::table.filter(crate::schema::classes::id.eq(pth.1))
                        //.select(crate::schema::classes::id)
                        .get_result::<Class>(&conn).unwrap();
                    context.insert("class", &class);
                    let sbjcts = crate::schema::class_subjects::table.filter(crate::schema::class_subjects::class.eq(pth.1))
                        .select(crate::schema::class_subjects::subject)
                        .get_results::<i32>(&conn);
                    let mut subjects = crate::schema::subjects::table.filter(crate::schema::subjects::id.eq_any(sbjcts.unwrap()))
                        .get_results::<Subject>(&conn).unwrap();
                    if subjects.len()==0{
                        subjects = subjects::table
                            .filter(subjects::kademe.eq(class.kademe))
                            .filter(subjects::school_type.eq(schl.school_type))
                            .order(subjects::optional)
                            .order(subjects::name)
                            .get_results::<Subject>(&conn).unwrap();
                    }
                    context.insert("subjects", &subjects);
                    let teachers = crate::schema::school_users::table
                        .filter(crate::schema::school_users::school_id.eq(schl.code))
                        .select(crate::schema::school_users::user_id)
                        .get_results::<i32>(&conn);
                    let teachers = crate::schema::users::table.filter(crate::schema::users::id.eq_any(teachers.unwrap()))
                        //.select(id,sube,kademe,school)
                        .order(crate::schema::users::first_name)
                        .get_results::<User>(&conn);
                    context.insert("teachers", &teachers.unwrap());

                    let classes = crate::schema::classes::table.filter(crate::schema::classes::school.eq(pth.0))
                        .get_results::<Class>(&conn);
                    context.insert("classes", &classes.unwrap());
                    use crate::schema::users::dsl::*;
                    let acts = crate::schema::activities::table.inner_join(users)
                        .inner_join(crate::schema::classes::table)
                        .inner_join(crate::schema::subjects::table)
                        .filter(crate::schema::activities::class.eq(class.id))
                        .select((crate::schema::activities::id
                                 ,(crate::schema::subjects::all_columns)
                                 ,(crate::schema::users::id, crate::schema::users::first_name, crate::schema::users::last_name, crate::schema::users::username, crate::schema::users::email, crate::schema::users::is_admin)
                                 ,(crate::schema::classes::all_columns)
                                 ,(crate::schema::activities::hour)
                                 ,(crate::schema::activities::split)))
                        .load::<Activities>(&conn);
                    //println!("{:?}", acts.unwrap());
                    context.insert("activities", &acts.unwrap());
                    let s = tmpl.render("classes/activities.html",&context);
                    Ok(HttpResponse::Ok().body(s.unwrap()))
                },
                Method::POST=>{
                    let conn = pool();
                    use diesel::insert_into;
                    use crate::schema::activities::dsl::*;
                    let _schl = crate::schema::school::table.filter(crate::schema::school::code.eq(pth.0))
                        .get_result::<School>(&conn);
                    let sbjcts = insert_into(activities).values(pays.unwrap().into_inner())
                        //.select(user_id, school_id)
                        .get_result::<Activity>(&conn);
                    Ok(HttpResponse::Ok().content_type("application/json").json(&sbjcts.unwrap()))
                },
                _=>{
                    Ok(HttpResponse::Ok().content_type("application/json").json("Metod desteklenmiyor"))
                }
            }
        },
        _=>{
            context.insert("status", &"400");
            context.insert("error", &"Yetkiniz yok");
            Ok(HttpResponse::Ok().content_type("text/html").json(context))
        }
    }
}

pub async fn limitations(path: Path<(i32,i32)>, pays: Option<Json<Vec<ClassAvailableForm>>>,req: HttpRequest,tmpl: Data<tera::Tera>,session: Session)
-> Result<HttpResponse>{
    let mut context = req.context(&session);
   // use diesel::insert_into;
    use http::Method;
    let pth = path.into_inner();
    let school_auth = req.authorized(&session, pth.0, None, Some(pth.1));
    match school_auth{
        SchoolAuth::AUTH=>{
            match *req.method(){
                Method::GET=>{
                    let conn = pool();
                    //let teacher_id = path.into_inner().clone().1;
                    let mut schl = crate::schema::school::table.filter(crate::schema::school::code.eq(pth.0))
                        .get_result::<School>(&conn);
                    context.insert("school", &schl.as_mut().unwrap());
                    //use crate::schema::teachers_menu::dsl::*;
                    let menu = crate::schema::teachers_menu::table
                        .select((crate::schema::teachers_menu::title, crate::schema::teachers_menu::link))
                        .load::<(Option<String>, Option<String>)>(&conn);
                    context.insert("menus", &menu.unwrap());
                    let _hrs = crate::schema::school_time::table.filter(crate::schema::school_time::school.eq(schl.as_mut().unwrap().code))
                        .get_results::<SchoolTime>(&conn);
                    let days = crate::schema::days::table.get_results::<(i32,String)>(&conn);
                    context.insert("days", &days.unwrap());
                    let class = crate::schema::classes::table.filter(crate::schema::classes::id.eq(pth.1))
                        .get_result::<Class>(&conn);
                    context.insert("class", &class.unwrap());
                    let classes = crate::schema::classes::table.filter(crate::schema::classes::school.eq(pth.0))
                        .get_results::<Class>(&conn);
                    context.insert("classes", &classes.unwrap());
                    use crate::schema::*;
                    let class_available_hours = class_available::table
                        .filter(class_available::class_id.eq(pth.1))
                        .get_results::<ClassAvailable>(&conn);
                    match class_available_hours{
                        Ok(e)=>{
                            let total_hour = schl.unwrap().clone().hour;
                            if e.len() == 0 || e[0].hours.len() != total_hour as usize{
                                println!("aa");
                                let _sil = diesel::delete(class_available::table).filter(class_available::class_id.eq(pth.1)).execute(&conn);
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
                                    let new_av = ClassAvailable{
                                        class_id: pth.1,
                                        //school_id: pth.0,
                                        day: i as i32,
                                        hours: hour_vec.clone()
                                    };

                                    let _ekle = diesel::insert_into(class_available::table).values(&new_av).execute(&conn);
                                }
                                let _e = class_available::table
                                    .filter(class_available::class_id.eq(pth.1))
                                    //.filter(school_id.eq(pth.0))
                                    .get_results::<ClassAvailable>(&conn).unwrap();
                                context.insert("hours", &_e);
                            }
                            else{
                                context.insert("hours", &e);
                            }
                        },
                        Err(_e)=>{
                        }
                    }
                    //context.insert("hours", &class_available_hours.unwrap());
                    let s = tmpl.render("classes/limitations.html",&context);
                    Ok(HttpResponse::Ok().body(s.unwrap()))
                },
                Method::POST=>{
                    let pys = pays.unwrap().into_inner();
                    let conn = pool();
                    use diesel::insert_into;
                    use crate::schema::class_available::dsl::*;
                    let class_available_hours = class_available
                        .filter(class_id.eq(pth.1))
                        .get_results::<ClassAvailable>(&conn);
                    match class_available_hours{
                        Ok(hour)=>{
                            for h in hour.iter(){
                                let p = pys.iter().find(|d| d.day == h.day ).unwrap();
                                println!("{:?}", p);
                                let _sil = diesel::update(class_available)
                                    .filter(class_id.eq(pth.1))
                                    .filter(day.eq(p.day))
                                    .set(hours.eq(p.hours.as_ref().unwrap()))
                                    .execute(&conn);
                                println!("{:?}", _sil);
                            };
                            Ok(HttpResponse::Ok().content_type("application/json").json(&hour))
                        },
                        Err(_hour)=>{
                            let class_available_hour = insert_into(class_available).values(&pys)
                                .get_results::<ClassAvailable>(&conn);

                            Ok(HttpResponse::Ok().content_type("application/json").json(serde_json::to_string(&class_available_hour.unwrap()).unwrap()))
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
    let school_auth = req.authorized(&session, school.0, None, Some(school.1));
    match school_auth{
        SchoolAuth::AUTH=>{
            let class = crate::schema::classes::table
                .filter(crate::schema::classes::id.eq(school.1))
                //.select((crate::schema::users::id,users::first_name, users::last_name, users::username,users::email, users::is_admin))
                .get_result::<Class>(&conn).unwrap();
            context.insert("class", &class);
            let s = req.school(school.0).unwrap();
            let classes = crate::schema::classes::table
                //.inner_join(crate::schema::users::table)
                .filter(crate::schema::classes::school.eq(school.0))
                //.select((crate::schema::users::id,users::first_name, users::last_name, users::username,users::email, users::is_admin))
                .get_results::<Class>(&conn).unwrap();
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
                .filter(activities::class.eq(&school.1))
                .get_results::<Timetable>(&conn).unwrap();
            if s.hour == 8 {
                class_timetables.push((0, 7, timetables.clone()));
            }
            if s.hour == 7 {
                class_timetables.push((0, 6, timetables.clone()));
            }
            if s.hour == 14 {
                if timetables.len()>0{
                    if timetables[0].hour==0 {
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
            context.insert("classes",&classes);
            let s = tmpl.render(&"classes/timetable.html", &context);
            Ok(HttpResponse::Ok().body(&s.unwrap()))
        },
        _=>{
            Ok(HttpResponse::Ok().json("merhaba"))
        }
    }
}