//use actix_web::http::{header, Method, StatusCode};
use actix_web::{
    HttpRequest, HttpResponse, http, HttpMessage, Result
};
use actix_web::web::{Path, Json, Data};
//use actix_identity::Identity;
use actix_session::{Session};
//use crate::middlewares::UserAuthentication;
use std::collections::HashMap;
use crate::forms::*;
use crate::models::school::*;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
//use diesel::BelongingToDsl;
use crate::util::database::pool;
use crate::request::*;
use crate::models::others::{Cities, Town, TeacherAvailable, Class, ClassAvailable, Subject};
use crate::models::user::User;
use crate::models::activity::{Activities, Activity};

pub async fn add(school_params: Option<Json<NewSchool>>, session: Session, tmpl: Data<tera::Tera>,req: HttpRequest)-> Result<HttpResponse> {
    //use crate::schema::school::dsl::*;
    //use crate::schema::city::dsl::*;
    use http::Method;
    //use actix_web::HttpMessage;
    if !req.is_auth(&session){
        Ok(HttpResponse::Found().header("location", "/login").finish())
    }
    else {
        match *req.method() {
            Method::GET => {
                let conn = pool();
                let cities = crate::schema::city::table
                    .load::<Cities>(&conn);
                let types = crate::schema::school_type::table
                    .load::<SchoolType>(&conn);
                let mut context = req.context(&session);
                context.insert("cities", &cities.unwrap());
                context.insert("school_type", &types.unwrap());
                let s = tmpl.render("school_add.html", &context);
                Ok(HttpResponse::Ok().content_type("text/html").body(s.unwrap()))
            },
            Method::POST => {
                match school_params {
                    Some(mut _sch) => {
                        use crate::schema::school::dsl::*;
                        let conn = pool();
                        use diesel::insert_into;
                        let u_school = school.filter(manager.eq(req.user(&session).unwrap().id)).select(code).get_result::<i32>(&conn);
                        match u_school {
                            Ok(_u_s) => {
                                let mut resp: HashMap<String, String>= HashMap::new();
                                resp.insert("status".to_string(), "203".to_string());
                                resp.insert("error".to_string(), "Daha önce kurum eklemişsiniz".to_string());
                                Ok(HttpResponse::build(http::StatusCode::from_u16(203).unwrap()).json(resp))
                            },
                            Err(_e) => {
                                _sch.manager = Some(req.user(&session).unwrap().id);
                                let new_school = insert_into(school).values(_sch.into_inner())
                                    .get_result::<School>(&conn);
                                let school_teacher = SchoolTeacher {
                                    user_id: req.user(&session).unwrap().id,
                                    school_id: new_school.unwrap().code.clone(),
                                    auth: Some(0)
                                };
                                let _s_t = insert_into(crate::schema::school_users::table).values(school_teacher)
                                    .get_result::<SchoolTeacher>(&conn);
                                let mut resp: HashMap<String, String>= HashMap::new();
                                resp.insert("status".to_string(), "200".to_string());
                                resp.insert("code".to_string(), _s_t.unwrap().school_id.to_string());
                                Ok(HttpResponse::build(http::StatusCode::from_u16(200).unwrap()).json(resp))
                            }
                        }
                    },
                    None => {
                        println!("Hatalı bilgiler");
                        let mut context = tera::Context::new();
                        let mut error: HashMap<String, String> = HashMap::new();
                        error.insert("error".to_string(), "Girdiğiniz bilgiler eksik veya yanlış".to_string());
                        context.insert("error", &error);
                        let s = tmpl.render("school_add.html", &context);
                        Ok(HttpResponse::Ok().content_type("text/html").body(s.unwrap()))
                    }
                }
            },
            _ => {
                let mut error: HashMap<String, String> = HashMap::new();
                error.insert("error".to_string(), "Method desteklenmiyor".to_string());
                let mut context = tera::Context::new();
                context.insert("error", &error);
                Ok(HttpResponse::Ok().body(serde_json::to_string(&context).unwrap()))
            }
        }
    }
}

pub async fn detail(path: Path<i32>, req: HttpRequest,tmpl: Data<tera::Tera>,js: Option<Json<UpdateSchool>>,session: Session)
-> Result<HttpResponse>{
    let mut context = req.context(&session);
    //use diesel::insert_into;
    use http::Method;
    use crate::schema::school::dsl::*;
    //let new_school = insert_into(school).values(sch)
    //                    .get_result::<School>(&conn);
    //context.insert("school", &new_school.unwrap());
    let schl_id = path.into_inner().clone();

    let school_auth = req.authorized(&session, schl_id, None, None);
    match school_auth {
        SchoolAuth::AUTH => {
            match *req.method() {
                Method::GET => {
                    //println!("{:?}", &AUTH);
                    let conn = pool();
                    let schl = school.filter(code.eq(schl_id))
                        .get_result::<School>(&conn).unwrap();
                    context.insert("school", &schl);
                    use crate::schema::school_menu::dsl::*;
                    let menu = school_menu
                        .load::<SchoolMenu>(&conn);
                    context.insert("menus", &menu.unwrap());
                    let cities = crate::schema::city::table
                        .load::<Cities>(&conn).unwrap();
                    context.insert("cities", &cities);
                    let towns = crate::schema::town::table
                        .filter(crate::schema::town::city.eq(schl.city))
                        .get_results::<Town>(&conn).unwrap();
                    context.insert("towns", &towns);
                    let s = tmpl.render("school_detail.html", &context);
                    Ok(HttpResponse::Ok().body(s.unwrap()))
                },
                Method::PATCH => {
                    use crate::schema::school::dsl::*;
                    let schl = school.filter(code.eq(schl_id));
                    let conn = pool();
                    let schl = diesel::update(schl).set(&js.unwrap().into_inner())
                        .load::<School>(&conn);
                    Ok(HttpResponse::Ok().content_type("application/json").json(schl.unwrap()))
                },
                _ => {
                    Ok(HttpResponse::Ok().content_type("application/json").json("Metod desteklenmiyor"))
                }
            }
        },
        _ => {
            context.insert("error", &true);
            let s = tmpl.render("school_detail.html", &context);
            Ok(HttpResponse::Ok().body(s.unwrap()))
        }
    }
}

pub async fn teachers(path: Path<i32>, pays: Option<Json<AddTeacherForm>>,req: HttpRequest,tmpl: Data<tera::Tera>,session: Session)
-> Result<HttpResponse>{
    let mut context = req.context(&session);
    //use diesel::insert_into;
    use http::Method;
    //let new_school = insert_into(school).values(sch)
    //                    .get_result::<School>(&conn);
    //context.insert("school", &new_school.unwrap());
    let schl_id = path.into_inner().clone();
    let conn = pool();
    let menu = crate::schema::school_menu::table
        .load::<SchoolMenu>(&conn);
    context.insert("menus", &menu.unwrap());
    context.insert("school", &req.school(schl_id).unwrap());
    match req.authorized(&session,schl_id, None, None) {
        SchoolAuth::AUTH => {
            context.insert("error", &false);
            match *req.method() {
                Method::GET => {
                    use crate::schema::school_users::dsl::*;
                    let teachers = school_users.filter(school_id.eq(schl_id))
                        .select(user_id)
                        .get_results::<i32>(&conn);
                    use crate::schema::users::dsl::*;
                    let school_teachers = users.filter(id.eq_any(teachers.unwrap()))
                        .order(first_name)
                        .get_results::<User>(&conn);
                    context.insert("teachers", &school_teachers.unwrap());

                    let s = tmpl.render("teachers.html", &context);
                    Ok(HttpResponse::Ok().body(s.unwrap()))
                },
                Method::POST => {
                    match pays {
                        None => {
                            Ok(HttpResponse::Ok().content_type("application/json").json("Bilgiler hatalı veya eksik"))
                        },
                        Some(mut form) => {
                            use diesel::insert_into;
                            use chrono::Utc;
                            form.date_join = Some(Utc::now().naive_utc());
                            form.tel = None;
                            let mut u = insert_into(crate::schema::users::table).values(form.into_inner())
                                .get_result::<User>(&conn);
                            use crate::schema::school_users::dsl::*;
                            let teacher = SchoolTeacher {
                                user_id: u.as_mut().unwrap().id,
                                school_id: schl_id,
                                auth: Some(2),
                            };
                            //use crate::schema::teacher_available::dsl::*;
                            let teacher_available_hours = crate::schema::teacher_available::table
                                .filter(crate::schema::teacher_available::user_id.eq(u.as_mut().unwrap().id))
                                .filter(crate::schema::teacher_available::school_id.eq(schl_id))
                                .order(crate::schema::teacher_available::day)
                                .get_results::<TeacherAvailable>(&conn);
                            match teacher_available_hours {
                                Ok(e) => {
                                    let schl = crate::schema::school::table.filter(crate::schema::school::code.eq(schl_id))
                                        .get_result::<School>(&conn);
                                    let total_hour = schl.unwrap().clone().hour;
                                    if e.len() == 0 || e[0].hours.len() != total_hour as usize {
                                        for i in 1..8 {
                                            let mut hour_vec: Vec<bool> = Vec::new();
                                            for _i in 0..total_hour {
                                                if i <= 5 {
                                                    hour_vec.push(true);
                                                } else {
                                                    hour_vec.push(false);
                                                }
                                            }
                                            let new_av = TeacherAvailable {
                                                user_id: u.as_mut().unwrap().id,
                                                school_id: schl_id,
                                                day: i,
                                                hours: hour_vec.clone()
                                            };
                                            let _ekle = diesel::insert_into(crate::schema::teacher_available::table).values(&new_av).execute(&conn);
                                        }
                                        let _e = crate::schema::teacher_available::table
                                            .filter(crate::schema::teacher_available::user_id.eq(u.as_mut().unwrap().id))
                                            .filter(crate::schema::teacher_available::school_id.eq(schl_id))
                                            .get_results::<TeacherAvailable>(&conn).unwrap();
                                        context.insert("hours", &_e);
                                    } else {
                                        context.insert("hours", &e);
                                    }
                                },
                                Err(_e) => {}
                            }
                            let _teacher = insert_into(school_users).values(teacher)
                                //.select(user_id, school_id)
                                .get_result::<SchoolTeacher>(&conn);
                            Ok(HttpResponse::Ok().content_type("application/json").json(u.unwrap()))
                        }
                    }
                },

                _ => {
                    Ok(HttpResponse::Ok().content_type("application/json").json("Metod desteklenmiyor"))
                }
            }
        },
        SchoolAuth::NOTEQ=>{
            context.insert("status", &"400");
            context.insert("error", &"Kullanıcı kuruma kayıtlı değil");
            let s = tmpl.render("teachers.html", &context);
            Ok(HttpResponse::Ok().body(s.unwrap()))
        },
        _=>{
            context.insert("error", &true);
            let s = tmpl.render("teachers.html", &context);
            Ok(HttpResponse::Ok().body(s.unwrap()))
            //Ok(HttpResponse::Ok().content_type("application/json").json("Yetkiniz yok"))
        }
    }
}

pub async fn classes(path: Path<i32>, pays: Option<Json<NewClassForm>>,req: HttpRequest,tmpl: Data<tera::Tera>,session: Session)
-> Result<HttpResponse>{
    let mut context = req.context(&session);
    //use diesel::insert_into;
    use http::Method;
    let schl_id = path.into_inner().clone();
    let school_auth = req.authorized(&session, schl_id, None, None);
    match school_auth{
        SchoolAuth::AUTH=>{
            match *req.method(){
                Method::GET => {
                    let conn = pool();
                    let mut schl = crate::schema::school::table.filter(crate::schema::school::code.eq(schl_id))
                        .get_result::<School>(&conn);
                    context.insert("school", &schl.as_mut().unwrap());
                    let menu = crate::schema::school_menu::table
                        .get_results::<SchoolMenu>(&conn);
                    println!("{:?}", menu);
                    context.insert("menus", &menu.unwrap());
                    let class = crate::schema::classes::table.filter(crate::schema::classes::school.eq(schl_id))
                        //.select(id,sube,kademe,school)
                        .order((crate::schema::classes::kademe, crate::schema::classes::sube))
                        .get_results::<Class>(&conn);
                    context.insert("classes", &class.unwrap());
                    let s = tmpl.render("classes.html", &context);
                    Ok(HttpResponse::Ok().body(s.unwrap()))
                },
                Method::POST => {
                    let conn = pool();
                    //println!("{:?}", pays);
                    let form = pays.unwrap().into_inner();
                    use diesel::insert_into;
                    //use chrono::Utc;
                    use crate::schema::classes::dsl::*;
                    let _schl = crate::schema::school::table.filter(crate::schema::school::code.eq(schl_id))
                        .get_result::<School>(&conn);
                    let cls = insert_into(classes).values(form)
                        //.select(user_id, school_id)
                        .get_result::<Class>(&conn);
                    let cls_id = cls.unwrap().clone().id;
                    //use crate::schema::class_available::dsl::*;
                    let class_available_hours = crate::schema::class_available::table
                        .filter(crate::schema::class_available::class_id.eq(cls_id))
                        .get_results::<ClassAvailable>(&conn);
                    match class_available_hours{
                        Ok(e)=>{
                            let total_hour = _schl.unwrap().clone().hour;
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
                                    let new_av = ClassAvailable{
                                        class_id: cls_id,
                                        day: i as i32,
                                        hours: hour_vec.clone()
                                    };

                                    let _ekle = diesel::insert_into(crate::schema::class_available::table).values(&new_av).execute(&conn);
                                }
                                let _e = crate::schema::class_available::table
                                    .filter(crate::schema::class_available::class_id.eq(cls_id))
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
                    Ok(HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&"Eklendi").unwrap()))
                },
                _ => {
                    Ok(HttpResponse::Ok().content_type("text/plain").json("Metod desteklenmiyor"))
                }
            }
        },
        _=>{
            context.insert("status", &"400");
            context.insert("error", &"Yetkiniz yok");
            let s = tmpl.render("classes.html", &context);
            Ok(HttpResponse::Ok().body(s.unwrap()))
        }
    }
}

pub async fn subjects(path: Path<i32>, pays: Option<Json<ClassSubjectsForm>>,req: HttpRequest,tmpl: Data<tera::Tera>,session: Session)
-> Result<HttpResponse>{
    let mut context = tera::Context::new();
   // use diesel::insert_into;
    use http::Method;
    let schl_id = path.into_inner().clone();
    if req.is_auth(&session) {
        /*if !req.authorized(&session, schl_id, None, &req) {
            context.insert("error", &"Bu sayfayı görüntüleme yetkiniz yok".to_string());
            let s = tmpl.render("subjects.html", &context);
            return Ok(HttpResponse::Ok().content_type("text/html").body(s.unwrap()))
        }*/
        match *req.method() {
            Method::GET => {
                let conn = pool();
                let mut schl = crate::schema::school::table.filter(crate::schema::school::code.eq(schl_id))
                    .get_result::<School>(&conn);
                context.insert("school", &schl.as_mut().unwrap());
                context.insert("is_auth", &true);
                context.insert("user", &req.user(&session).unwrap());
                context.insert("schools", &req.schools(&session).unwrap());

                let menu = crate::schema::school_menu::table
                    .load::<SchoolMenu>(&conn);
                context.insert("menus", &menu.unwrap());
                //use crate::schema::school_users::dsl::*;
                //use crate::schema::users::dsl::*;
                let class = crate::schema::classes::table.filter(crate::schema::classes::school.eq(schl.as_mut().unwrap().code))
                    //.select(id,sube,kademe,school)
                    .get_results::<Class>(&conn);
                context.insert("classes", &class.unwrap());
                let subjects = crate::schema::subjects::table.filter(crate::schema::subjects::school_type.eq(schl.as_mut().unwrap().school_type))
                    //.select(id,sube,kademe,school)
                    .get_results::<Subject>(&conn);
                let grade = crate::schema::school_grade::table
                    .select(crate::schema::school_grade::grade)
                    .filter(crate::schema::school_grade::school_type.eq(schl.as_mut().unwrap().school_type.unwrap()))
                    .order(crate::schema::school_grade::grade)
                    .get_results::<i16>(&conn);
                context.insert("grades", &grade.unwrap());
                context.insert("subjects", &subjects.unwrap());
                let s = tmpl.render("subjects.html", &context);
                Ok(HttpResponse::Ok().body(s.unwrap()))
            },
            Method::POST => {
                let conn = pool();
                use diesel::insert_into;
                use crate::schema::class_subjects::dsl::*;
                let mut class_subjects2: Vec<NewClassSubjectsForm> = Vec::new();
                let payload = pays.unwrap().into_inner();
                for p in &payload.classes {
                    for s in &payload.subjects {
                        let class_subject = NewClassSubjectsForm {
                            class: Some(*p),
                            subject: Some(*s)
                        };

                        class_subjects2.push(class_subject);
                    }
                }
                //let _schl = crate::schema::school::table.filter(crate::schema::school::code.eq(path.into_inner()))
                //    .get_result::<School>(&conn);
                let _del_class_sb = diesel::delete(class_subjects)
                    .filter(crate::schema::class_subjects::class.eq_any(&payload.classes))
                    .execute(&conn);
                let sbjcts = insert_into(class_subjects).values(&class_subjects2)
                    //.select(user_id, school_id)
                    .get_results::<(i32, i32)>(&conn);
                Ok(HttpResponse::Ok().content_type("application/json").json(serde_json::to_string(&sbjcts.unwrap()).unwrap()))
            },
            _ => {
                Ok(HttpResponse::Ok().content_type("text/plain").body(serde_json::to_string(&schl_id).unwrap()))
            }
        }
    }
    else{
        Ok(HttpResponse::Ok().content_type("text/plain").body(serde_json::to_string(&schl_id).unwrap()))
    }
}

pub async fn activities(path: Path<i32>, pays: Option<Json<NewActivitiesForm>>,req: HttpRequest,tmpl: Data<tera::Tera>,session: Session)
-> Result<HttpResponse>{
    let mut context = req.context(&session);
   // use diesel::insert_into;
    use http::Method;
    let schl_id = path.into_inner();
    let mut acts: Vec<Activities>=Vec::new();
    let school_auth = req.authorized(&session, schl_id, None, None);
    match school_auth{
        SchoolAuth::AUTH=>{
            match *req.method(){
                Method::POST => {
                    let conn = pool();
                    use diesel::insert_into;
                    use crate::schema::activities::dsl::*;
                    match pays {
                        Some(mut pay) => {
                            let act_class = crate::schema::classes::table
                                .filter(crate::schema::classes::id.eq(pay.class))
                                .get_result::<Class>(&conn);
                            match act_class {
                                Ok(_a_c) => {
                                    let act_teacher = crate::schema::school_users::table
                                        .select(crate::schema::school_users::user_id)
                                        .filter(crate::schema::school_users::user_id.eq(pay.teacher))
                                        .get_result::<i32>(&conn);
                                    match act_teacher {
                                        Ok(_a_t) => {
                                            if pay.hour > Some(2) {
                                                pay.split = Some(false);
                                                loop {
                                                    let mut act = pay.clone();
                                                    if pay.hour > Some(2) {
                                                        act.hour = Some(2);
                                                    }
                                                    let _act = insert_into(activities).values(act.clone())
                                                        //.select(user_id, school_id)
                                                        .get_result::<Activity>(&conn);
                                                    let act2 = crate::schema::activities::table.inner_join(crate::schema::users::table)
                                                        .inner_join(crate::schema::classes::table)
                                                        .inner_join(crate::schema::subjects::table)
                                                        .filter(crate::schema::activities::id.eq(_act.unwrap().id))
                                                        .select((crate::schema::activities::id
                                                                 ,(crate::schema::subjects::all_columns)
                                                                 ,(crate::schema::users::id, crate::schema::users::first_name, crate::schema::users::last_name, crate::schema::users::username, crate::schema::users::email, crate::schema::users::is_admin)
                                                                 , (crate::schema::classes::all_columns)
                                                                 , (crate::schema::activities::hour)
                                                                 , (crate::schema::activities::split)))
                                                        .get_result::<Activities>(&conn);
                                                    acts.push(act2.unwrap());
                                                    pay.hour = Some(pay.hour.unwrap() - act.hour.unwrap());
                                                    if pay.hour == Some(0) {
                                                        break;
                                                    }
                                                }
                                                Ok(HttpResponse::Ok().content_type("application/json").json(acts))
                                            }
                                            else {
                                                let _act = insert_into(activities).values(pay.clone())
                                                    .get_result::<Activity>(&conn);
                                                let act2 = crate::schema::activities::table.inner_join(crate::schema::users::table)
                                                    .inner_join(crate::schema::classes::table)
                                                    .inner_join(crate::schema::subjects::table)
                                                    .filter(crate::schema::activities::id.eq(_act.unwrap().id))
                                                    .select((crate::schema::activities::id
                                                             , (crate::schema::subjects::all_columns)
                                                             ,(crate::schema::users::id, crate::schema::users::first_name, crate::schema::users::last_name, crate::schema::users::username, crate::schema::users::email, crate::schema::users::is_admin)
                                                             , (crate::schema::classes::all_columns)
                                                             , (crate::schema::activities::hour)
                                                             , (crate::schema::activities::split)))
                                                    .get_result::<Activities>(&conn);

                                                acts.push(act2.unwrap());
                                                Ok(HttpResponse::Ok().content_type("application/json").json(acts))
                                            }

                                        },
                                        Err(_e) => {
                                            let mut cntxt = tera::Context::new();
                                            cntxt.insert("status", &"401");
                                            cntxt.insert("error", &"Ders öğretmeni kurumunuzda kayıtlı değil.");
                                            Ok(HttpResponse::Ok().content_type("application/json").json(cntxt))
                                        }
                                    }
                                },
                                Err(_e) => {
                                    let mut cntxt = tera::Context::new();
                                    cntxt.insert("status", &"401");
                                    cntxt.insert("error", &"Ders sınıfı kurumunuzda kayıtlı değil.");
                                    Ok(HttpResponse::Ok().content_type("application/json").json(cntxt))
                                }
                            }
                        },
                        None => {
                            let mut cntxt = tera::Context::new();
                            cntxt.insert("status", &"401");
                            cntxt.insert("error", &"Girdiğiniz bilgiler hatalı");
                            Ok(HttpResponse::Ok().content_type("application/json").json(cntxt))
                        }
                    }
                },
                _=> {
                    Ok(HttpResponse::Ok().content_type("application/json").json("Metod desteklenmiyor"))
                }
            }
        }
        _=>{
            match req.mime_type() {
                Ok(_mime) => {
                    context.insert("error", &"Yetkiniz yok veya okul, öğretmen değeri uyumsuz.");
                    let s = tmpl.render("teachers/activities.html", &context);
                    Ok(HttpResponse::Ok().content_type("text/html").body(s.unwrap()))
                },
                Err(_e) => {
                    Ok(HttpResponse::Ok().content_type("application/json").json("Yetkiniz yok veya okul, öğretmen değeri uyumsuz.".to_string()))
                }
            }
        }
    }
}

pub async fn activities_detail(path: Path<(i32,i32)>, req: HttpRequest, _tmpl: Data<tera::Tera>, session: Session)-> Result<HttpResponse>{
    use diesel::delete;
    let conn = pool();
    use crate::schema::activities::dsl::*;
    let mut context = tera::Context::new();
    let schl_id = path.into_inner();
    let act = crate::schema::activities::table
        //.select(crate::schema::activities.id)
        .filter(crate::schema::activities::id.eq(schl_id.1))
        .get_result::<Activity>(&conn);
    match act{
        Ok(a)=>{
            let school_auth = req.authorized(&session, schl_id.0, a.teacher,None);
            match school_auth{
                SchoolAuth::AUTH=>{
                    let act_class = crate::schema::classes::table
                        .filter(crate::schema::classes::school.eq(schl_id.0))
                        .filter(crate::schema::classes::id.eq(a.class))
                        .get_result::<Class>(&conn);
                    match act_class{
                        Ok(_a_c)=>{
                            let _del = delete(activities.filter(id.eq(schl_id.1))).execute(&conn);
                            match _del{
                                Ok(_d)=>{
                                    context.insert("status", &"200");
                                    Ok(HttpResponse::Ok().content_type("application/json").json(context))
                                },
                                Err(_e)=>{
                                    context.insert("status", &"500");
                                    context.insert("error", &_e.to_string());
                                    Ok(HttpResponse::Ok().content_type("application/json").json(context))
                                }
                            }
                        },
                        Err(_e)=>{
                            context.insert("status", &"500");
                            context.insert("error", &"Yetkiniz yok veya veritabanı hatası oluştu.");
                            Ok(HttpResponse::Ok().content_type("application/json").json(context))
                        }
                    }

                },
                _=>{
                    Ok(HttpResponse::Ok().content_type("application/json").json(&"Yetkiniz yok".to_string()))
                }
            }
        },
        Err(_e)=>{
            context.insert("status", &"500");
            context.insert("error", &"Yetkiniz yok veya veritabanı hatası oluştu.");
            Ok(HttpResponse::Ok().content_type("application/json").json(context))
        }
    }
}



pub async fn school_teachers(path: Path<i32>, req: HttpRequest,tmpl: Data<tera::Tera>,session: Session)-> Result<HttpResponse>{
    let conn = pool();
    let mut context = req.context(&session);
    let schl_id = path.into_inner().clone();
    let mut schl = crate::schema::school::table.filter(crate::schema::school::code.eq(schl_id))
            .get_result::<School>(&conn);

    /*if !req.authorized(&session, schl_id, None, &req){
        context.insert("error", &"Bu sayfayı görüntüleme yetkiniz yok".to_string());
        let s = tmpl.render("school_detail.html",&context);
        return Ok(HttpResponse::Ok().content_type("text/plain").body(s.unwrap()))
    }*/
    context.insert("school", &schl.as_mut().unwrap());
    let tchrs = crate::schema::school_users::table.filter(crate::schema::school_users::school_id.eq(schl.unwrap().code))
                    .select(crate::schema::school_users::user_id)
                    .get_results::<i32>(&conn);
    let teachers = crate::schema::users::table.filter(crate::schema::users::id.eq_any(tchrs.unwrap()))
                        .get_results::<User>(&conn);
    context.insert("teachers", &teachers.unwrap());
    let s = tmpl.render("timetables.html",&context);
    Ok(HttpResponse::Ok().body(s.unwrap()))
}

pub async fn class_timetables(path: Path<i32>, req: HttpRequest,tmpl: Data<tera::Tera>,session: Session)-> Result<HttpResponse>{
    let conn = pool();
    let mut context = tera::Context::new();
    let schl_id = path.into_inner().clone();
    let mut schl = crate::schema::school::table.filter(crate::schema::school::code.eq(schl_id))
            .get_result::<School>(&conn);
    /*if !req.authorized(&session, schl_id, None, &req){
        context.insert("error", &"Bu sayfayı görüntüleme yetkiniz yok".to_string());
        let s = tmpl.render("school_detail.html",&context);
        return Ok(HttpResponse::Ok().content_type("text/plain").body(s.unwrap()))
    }*/
    context.insert("school", &schl.as_mut().unwrap());
    context.insert("is_auth", &req.is_auth(&session));
    context.insert("user", &req.user(&session).unwrap());
    context.insert("schools", &req.schools(&session).unwrap());
    let tchrs = crate::schema::school_users::table.filter(crate::schema::school_users::school_id.eq(schl.unwrap().code))
                    .select(crate::schema::school_users::user_id)
                    .get_results::<i32>(&conn);
    let teachers = crate::schema::users::table.filter(crate::schema::users::id.eq_any(tchrs.unwrap()))
                        .get_results::<User>(&conn);
    context.insert("teachers", &teachers.unwrap());
    let s = tmpl.render("timetables.html",&context);
    Ok(HttpResponse::Ok().body(s.unwrap()))
}