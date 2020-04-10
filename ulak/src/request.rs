use actix_web::{HttpRequest, Result, http::Method};
//use actix_identity::Identity;
use actix_session::{Session};
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use crate::schema::*;
use diesel::RunQueryDsl;
//use diesel::BelongingToDsl;
use crate::util::database::pool;
use crate::models::others::Class;
use crate::models::user::{User, AuthUser};
use crate::models::school::School;

#[derive(Debug)]
pub enum SchoolAuth{
    AUTH,
    NOTLOGGED,
    NOTAUTH,
    NOTFOUND,
    NOTEQ
}
pub trait Request{

    fn user(&self, session: &Session)->Option<AuthUser>;
    fn is_auth(&self, session: &Session)->bool;
    //fn init<T: Response>(&self, item: &T, ids: Identity, tmpl: Data<tera::Tera>, session: Session)-> Result<HttpResponse, Error>;
    fn login(&self, uname: &String, pas: &String)->  bool;
    fn schools(&self, session: &Session)->Result<Vec<School>,diesel::result::Error>;
    fn school(&self, id: i32)->Result<School, diesel::result::Error>;
    fn authorized(&self, session: &Session, school: i32, teacher: Option<i32>, class:Option<i32>)->SchoolAuth;
    fn context(&self, session: &Session)->tera::Context;
    fn teachers(&self, school: i32)->Vec<User>;
}

impl Request for HttpRequest{

    fn user(&self, session: &Session)->Option<AuthUser>{
        //use crate::schema::users::dsl::*;
        //let conn = pool();
        let user = session.get::<AuthUser>("user");
        user.unwrap()
    }

    fn schools(&self, session: &Session)->Result<Vec<School>,diesel::result::Error>{
        let conn = pool();
        let user = self.user(&session);
        let schls = crate::schema::school_users::table
            .inner_join(crate::schema::school::table)
            .filter(crate::schema::school_users::user_id.eq(user.unwrap().id))
            .select(crate::schema::school::all_columns)
            .get_results::<School>(&conn);

        schls
    }
    fn teachers(&self, school: i32)->Vec<User>{
        let conn = pool();
        let tchrs = crate::schema::school_users::table.filter(crate::schema::school_users::school_id.eq(school))
            .inner_join(crate::schema::users::table)
            .select(crate::schema::users::all_columns)
            .order(crate::schema::users::first_name)
            .get_results::<User>(&conn).unwrap();
        tchrs
    }
    fn school(&self, id: i32)->Result<School, diesel::result::Error>{
        let conn = pool();
        let schl = crate::schema::school::table
            .filter(crate::schema::school::code.eq(id))
            .select(crate::schema::school::all_columns)
            .get_result::<School>(&conn);
        schl
    }

    fn authorized(&self, session: &Session, schl: i32, teacher: Option<i32>, class: Option<i32>)-> SchoolAuth{
        let rslt: SchoolAuth;
        let conn = pool();
        if self.is_auth(session){
            if self.user(session).unwrap().is_admin.unwrap(){
                match self.school(schl){
                    Ok(_t)=>{
                        match teacher{
                            Some(id)=>{
                                let t = school_users::table.filter(school_users::school_id.eq(schl)).filter(school_users::user_id.eq(id)).select(school_users::user_id).get_result::<i32>(&conn);
                                match t{
                                    Ok(_tch)=>{
                                        rslt = SchoolAuth::AUTH;
                                    },
                                    Err(_e)=>{
                                        rslt = SchoolAuth::NOTEQ;
                                    }
                                }
                            },
                            None=>{
                                match class{
                                    Some(c_id)=>{
                                        let cls = classes::table.filter(classes::id.eq(c_id)).get_result::<Class>(&conn);
                                        match cls{
                                            Ok(c)=>{
                                                if c.school == schl{
                                                    rslt = SchoolAuth::AUTH;
                                                }
                                                else{
                                                    rslt = SchoolAuth::NOTEQ;
                                                }
                                            },
                                            Err(_e)=>{
                                                rslt = SchoolAuth::NOTFOUND;
                                            }
                                        }
                                    },
                                    None=>{
                                        rslt = SchoolAuth::AUTH;
                                    }
                                }
                            }
                        }
                    },
                    Err(_e)=>{
                        rslt = SchoolAuth::NOTFOUND;
                    }
                }

            }
            else {
                match self.school(schl){
                    Ok(s)=>{
                        if self.user(session).unwrap().id == s.manager.unwrap(){
                            match teacher{
                                Some(t)=>{
                                    use crate::schema::school_users::dsl::*;
                                    let school_teacher = school_users
                                        .select(user_id)
                                        .filter(user_id.eq(t))
                                        .filter(school_id.eq(s.code))
                                        .get_result::<i32>(&conn);
                                    match school_teacher{
                                        Ok(_st)=>{
                                            rslt = SchoolAuth::AUTH;
                                        },
                                        Err(_e)=>{
                                            rslt = SchoolAuth::NOTEQ
                                        }
                                    }
                                },
                                None=>{
                                    match class{
                                        Some(c)=>{
                                            use crate::schema::classes::dsl::*;
                                            let school_classes = classes
                                                .filter(id.eq(c))
                                                .get_result::<Class>(&conn);
                                            match school_classes{
                                                Ok(_c)=>{
                                                    if _c.school == schl{
                                                        rslt = SchoolAuth::AUTH
                                                    }
                                                    else{
                                                        rslt = SchoolAuth::NOTEQ
                                                    }
                                                },
                                                Err(_e)=>{
                                                    rslt = SchoolAuth::NOTFOUND
                                                }
                                            }
                                        },
                                        None=>{
                                            rslt = SchoolAuth::AUTH
                                        }
                                    }
                                }
                            }
                        }
                        else{
                            match teacher{
                                Some(t)=>{
                                    use crate::schema::school_users::dsl::*;
                                    let school_teacher = school_users
                                        .select(user_id)
                                        .filter(user_id.eq(t))
                                        .filter(school_id.eq(s.code))
                                        .get_result::<i32>(&conn);
                                    match school_teacher{
                                        Ok(st)=>{
                                            if st == self.user(session).unwrap().id{
                                                rslt = SchoolAuth::AUTH;
                                            }
                                            else{
                                                rslt = SchoolAuth::NOTEQ
                                            }
                                        },
                                        Err(_e)=>{
                                            rslt = SchoolAuth::NOTFOUND;
                                        }
                                    }
                                },
                                None=>{
                                    match class{
                                        Some(c)=>{
                                            use crate::schema::classes::dsl::*;
                                            let school_classes = classes
                                                .filter(id.eq(c))
                                                .get_result::<Class>(&conn);
                                            match school_classes{
                                                Ok(_c)=>{
                                                    if _c.school == schl{
                                                        rslt = SchoolAuth::AUTH
                                                    }
                                                    else{
                                                        rslt = SchoolAuth::NOTEQ
                                                    }
                                                },
                                                Err(_e)=>{
                                                    rslt = SchoolAuth::NOTFOUND
                                                }
                                            }
                                        },
                                        None=>{
                                            match *self.method(){
                                                Method::GET=>{
                                                    use crate::schema::school_users::dsl::*;
                                                    let school_teacher = school_users
                                                        .select(user_id)
                                                        .filter(user_id.eq(self.user(session).unwrap().id))
                                                        .filter(school_id.eq(schl))
                                                        .get_result::<i32>(&conn);
                                                    match school_teacher{
                                                        Ok(_t)=>{
                                                            rslt = SchoolAuth::AUTH;
                                                        },
                                                        Err(_e)=>{
                                                            rslt = SchoolAuth::NOTAUTH;
                                                        }
                                                    }

                                                },
                                                _ =>{
                                                    rslt = SchoolAuth::NOTAUTH;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    },
                    Err(_e)=>{
                        rslt = SchoolAuth::NOTFOUND;
                    }
                }
            }

        }
        else{
            rslt = SchoolAuth::NOTLOGGED
        }

        return rslt;
    }
    fn is_auth(&self, session: &Session)->bool{
        let user = session.get::<bool>("is_auth");
        match user.unwrap(){
            Some(u)=>{
                u
            },
            None=>{
                false
            }
        }
    }

    fn login(&self, uname: &String, pas: &String)-> bool{
        //use crate::schema::users;
        use crate::schema::users::dsl::*;
        let conn = pool();

        let user = users.filter(email.eq(uname))
            .get_result::<User>(&conn);
        match user{
            Ok(usr)=>{
                use bcrypt::{verify};
                let is_valid = verify(pas, &usr.password.unwrap()).unwrap();
                if is_valid{
                    true
                }
                else{
                    false
                }
            }
            Err(_err)=>{

                println!("{:?}", "Bilgiler hatalı".to_string());
                false
            }
        }
    }
    fn context(&self, session: &Session)->tera::Context{
        let mut cntxt = tera::Context::new();
        if self.is_auth(session){
            cntxt.insert("schools", &self.schools(&session).unwrap_or(vec![]));
            cntxt.insert("user", &self.user(&session).unwrap());
            //cntxt.insert("is_auth", &req.is_auth(&session));
        }
        cntxt.insert("is_auth", &self.is_auth(&session));
        cntxt
    }
}