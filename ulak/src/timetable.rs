use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use actix_redis::{Command, RedisActor};
use actix::prelude::*;
use actix_web::{
    HttpRequest, HttpResponse, Result
};
use actix_web::web::{Path, Data};
use crate::request::{Request, SchoolAuth};
use actix_session::Session;
use crate::models::others::*;
use crate::models::user::*;
use crate::schema::{users, activities, subjects, classes, class_timetable};

pub type ClassSingleHours = (i32, i16, i16, usize, bool);
pub type Pool = r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

pub async fn timetable(path: Path<i32>, tmpl: Data<tera::Tera>,db: Data<Pool>, req: HttpRequest, session: Session, redis: Data<Addr<RedisActor>>)
                       -> Result<HttpResponse>{
    let conn = db.get().unwrap();
    let school = path.into_inner();
    //let (mut tat, mut class_hours, mut _acts, _teachers, classes) = get_timetables_data(school, &conn);
    let mut context = req.context(&session);
    context.insert("school", &req.school(school).unwrap());
    let _classes = crate::schema::classes::table
        .filter(crate::schema::classes::school.eq(school))
        .select(crate::schema::classes::id)
        .get_results::<i32>(&conn).unwrap();
    //println!("{:?}", copy_tt2);
    let get_school = redis.send(Command(resp_array!["GET", school.to_string()+&":generated"]));
    match get_school.await{
        Ok(s)=>{
            use redis_async::resp::FromResp;
            let s2: Result<Option<String>, _> = s.and_then(|value| FromResp::from_resp(value).map_err(actix_redis::Error::Redis));
            match s2{
                Ok(s3)=>{
                    match s3{
                        Some(_s)=>{
                            if _s =="true".to_string(){
                                context.insert("generated", &true);
                            }
                            else{
                                context.insert("generated", &false);
                            }
                        },
                        None=>{
                            context.insert("generated", &false);
                        }
                    }

                },
                Err(_e)=>{
                    context.insert("generated", &false);
                }

            }
        },
        Err(_e)=>{

        }
    }
    //let mut class_timetables : Vec<(usize, usize, Vec<Timetable>)>= Vec::new();

    let s = tmpl.render(&"timetables/index.html", &context);
    return Ok(HttpResponse::Ok().content_type("text/html").body(s.unwrap()))
    //return Ok(HttpResponse::Ok().content_type("text/html").body(copy_tt.len().to_string()))
}

pub async fn class_timetable(path: Path<i32>, tmpl: Data<tera::Tera>,db: Data<Pool>, req: HttpRequest, session: Session)
                -> Result<HttpResponse>{
    let conn = db.get().unwrap();
    let school = path.into_inner();
    let mut context = req.context(&session);
    let school_auth = req.authorized(&session, school, None, None);
    match school_auth{
        SchoolAuth::AUTH=>{
            let s = req.school(school).unwrap();
            context.insert("school", &s);
            let classes = classes::table
                .filter(classes::school.eq(s.code))
                .select(classes::id)
                .get_results::<i32>(&conn).unwrap();
            //println!("{:?}", copy_tt2);
            let mut class_timetables : Vec<(usize, usize, Vec<Timetable>)>= Vec::new();
            let timetables = class_timetable::table
                //.inner_join(crate::schema::classes::table)
                .inner_join(activities::table
                    .inner_join(subjects::table)
                    .inner_join(users::table)
                    .inner_join(classes::table))
                .order(classes::id)
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
                .filter(class_timetable::class_id.eq_any(&classes))
                .get_results::<Timetable>(&conn);
            match timetables{
                Ok(tt)=>{
                    if tt.len()>0 {
                        for c in &classes {
                            let class_timetable: Vec<Timetable> = tt.iter().cloned()
                                .filter(|ct| ct.activities.class.id == *c).collect();
                            let class_available = crate::schema::class_available::table
                                .filter(crate::schema::class_available::class_id.eq(*c))
                                .filter(crate::schema::class_available::day.eq(1))
                                .first::<ClassAvailable>(&conn);
                            match class_available {
                                Ok(class_av) => {
                                    if s.hour == 8 {
                                        class_timetables.push((0, 7, class_timetable.clone()));
                                    }
                                    if s.hour == 7 {
                                        class_timetables.push((0, 6, class_timetable.clone()));
                                    }
                                    if s.hour == 14 {
                                        if class_av.hours[0] {
                                            class_timetables.push((0, 6, class_timetable.clone()));
                                        } else {
                                            class_timetables.push((7, 13, class_timetable.clone()));
                                        }
                                    }
                                },
                                Err(_e) => {}
                            }
                        };
                        let days = crate::schema::days::table
                            .filter(crate::schema::days::id.lt(6))
                            .get_results::<Day>(&conn);
                        let classes = crate::schema::classes::table
                            .filter(crate::schema::classes::school.eq(school))
                            .select(crate::schema::classes::all_columns)
                            .get_results::<Class>(&conn).unwrap();
                        context.insert("days", &days.unwrap());
                        context.insert("timetables", &class_timetables);
                        context.insert("classes", &classes);
                        context.insert("school", &s);
                        let s = tmpl.render(&"timetables/classes.html", &context);
                        return Ok(HttpResponse::Ok().content_type("text/html").body(s.unwrap()))
                    }
                    else{
                        context.insert("status",&"500");
                        context.insert("error", &"Kayıtlı ders programınız yok");
                        let s = tmpl.render(&"timetables/classes.html", &context);
                        return Ok(HttpResponse::Ok().content_type("text/html").body(s.unwrap()))
                    }
                },
                Err(_e)=>{
                    context.insert("status",&"501");
                    context.insert("error", &"Veritabanı hatası");
                    let s = tmpl.render(&"timetables/classes.html", &context);
                    return Ok(HttpResponse::Ok().content_type("text/html").body(s.unwrap()))
                }
            }

        },
        SchoolAuth::NOTFOUND=>{
            context.insert("status",&"501");
            context.insert("error", &"Böyle bir okul kayıtlı değil");
            let s = tmpl.render(&"timetables/classes.html", &context);
            return Ok(HttpResponse::Ok().content_type("text/html").body(s.unwrap()))
        },
        _=>{
            context.insert("status",&"400");
            context.insert("error", &"Yetkiniz yok");
            let s = tmpl.render(&"timetables/classes.html", &context);
            return Ok(HttpResponse::Ok().content_type("text/html").body(s.unwrap()))
        }
    }
}

pub async fn teacher_timetable(path: Path<i32>, tmpl: Data<tera::Tera>,db: Data<Pool>, req: HttpRequest, session: Session)
                 -> Result<HttpResponse>{
    let conn = db.get().unwrap();
    let school = path.into_inner();
    let mut context = req.context(&session);
    let school_auth = req.authorized(&session, school, None, None);
    use crate::request::*;
    match school_auth{
        SchoolAuth::AUTH=>{
            //let (mut tat, mut class_hours, mut _acts, _teachers, classes) = get_timetables_data(school, &conn);
            let teachers = crate::schema::school_users::table
                .inner_join(crate::schema::users::table)
                .filter(crate::schema::school_users::school_id.eq(school))
                .select(crate::schema::users::all_columns)
                .get_results::<User>(&conn).unwrap();
            let schl = req.school(school);
            context.insert("school", &schl.unwrap());
            let mut teacher_timetables : Vec<(usize, usize, Vec<Timetable>)>= Vec::new();
            let timetables = crate::schema::class_timetable::table
                .inner_join(crate::schema::activities::table
                    .inner_join(crate::schema::subjects::table)
                    .inner_join(crate::schema::users::table)
                    .inner_join(crate::schema::classes::table))
                .select((crate::schema::class_timetable::id,
                         crate::schema::class_timetable::day_id,
                         crate::schema::class_timetable::hour,
                         (crate::schema::activities::id
                          ,crate::schema::subjects::all_columns
                          ,(users::id,users::first_name, users::last_name, users::username, users::email, users::is_admin)
                          ,crate::schema::classes::all_columns
                          ,crate::schema::activities::hour
                          ,crate::schema::activities::split)))
                .filter(crate::schema::classes::school.eq(&school))
                .get_results::<Timetable>(&conn);
            match timetables{
                Ok(timetable)=>{
                    if timetable.len()>0{
                        for t in &teachers{
                            let teacher_timetable: Vec<Timetable> = timetable.iter().cloned()
                                .filter(|ct| ct.activities.teacher.id == t.id).collect();
                            let ranges = crate::schema::school::table
                                .filter(crate::schema::school::code.eq(&school))
                                .select(crate::schema::school::hour)
                                .first::<i16>(&conn);
                            let range = ranges.unwrap();
                            if range == 8{
                                teacher_timetables.push((0, 7, teacher_timetable.clone()));
                            }
                            if range == 7{
                                teacher_timetables.push((0, 6, teacher_timetable.clone()));
                            }
                            if range == 14{
                                if timetable[0].hour <= 6{
                                    teacher_timetables.push((0, 6, teacher_timetable.clone()));
                                }
                                else{
                                    teacher_timetables.push((7, 13, teacher_timetable.clone()));
                                }
                            }
                        }
                        let days = crate::schema::days::table
                            .filter(crate::schema::days::id.lt(6))
                            .get_results::<Day>(&conn);
                        context.insert("days", &days.unwrap());
                        context.insert("teachers", &teachers);
                        context.insert("timetables", &teacher_timetables);

                        let s = tmpl.render(&"timetables/teachers.html", &context);
                        Ok(HttpResponse::Ok().content_type("text/html").body(s.unwrap()))
                    }
                    else{
                        context.insert("status", &"300");
                        context.insert("error", &"Hazır programınız yok");
                        let s = tmpl.render(&"timetables/teachers.html", &context);
                        Ok(HttpResponse::Ok().content_type("text/html").body(s.unwrap()))
                    }
                },
                Err(_e)=>{
                    context.insert("status", &"500");
                    context.insert("error", &"Veritabanı hatası");
                    let s = tmpl.render(&"timetables/teachers.html", &context);
                    return Ok(HttpResponse::Ok().content_type("text/html").body(s.unwrap()))
                }
            }
        },
        SchoolAuth::NOTFOUND=>{
            context.insert("status", &"501");
            context.insert("error", &"Böyle bir okul yok");
            let s = tmpl.render(&"timetables/teachers.html", &context);
            return Ok(HttpResponse::Ok().content_type("text/html").body(s.unwrap()))
        }
        _=>{
            let schl = req.school(school);
            context.insert("school", &schl.unwrap());
            context.insert("status", &"500");
            context.insert("error", &"Yetkiniz yok veya sistem hatası");
            let s = tmpl.render(&"timetables/teachers.html", &context);
            return Ok(HttpResponse::Ok().content_type("text/html").body(s.unwrap()))
        }
    }

}