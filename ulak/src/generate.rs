use actix_session::{Session};
use actix_redis::{Command, RedisActor};
use actix::prelude::*;
//use futures::future::{join_all};
//use actix_web::http::{header, Method, StatusCode};

use actix_web::{HttpResponse, HttpRequest, Result};
use actix_web::web::{Path, Json, Data};
use crate::request::Request;

pub type ClassSingleHours = (i32, i16, i16, usize, bool);
pub type Pool = r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

#[derive(Clone, Serialize, Deserialize)]
pub struct Params {
    max_day_hour: i32,
}
pub async fn generate(
    path: Path<i32>,
    _session: Session,
    _tmpl: Data<tera::Tera>,
    _db: Data<Pool>,
    redis: Data<Addr<RedisActor>>,
    _req: HttpRequest,
    mdh: Option<Json<Params>>
)
-> Result<HttpResponse>{
    if !_req.is_auth(&_session){
        Ok(HttpResponse::Ok().content_type("text/plain").body("Giriş yapınız"))
    }
    else {
        //let _conn = db.get().unwrap();
        let mdh = match mdh{
            Some(m)=>{
                let max_day_hour = m.into_inner();
                if max_day_hour.max_day_hour > 4{
                    4.to_string()
                }
                else if max_day_hour.max_day_hour < 2{
                    2.to_string()
                }
                else{
                    max_day_hour.max_day_hour.to_string()
                }
            },
            None=>{
                4.to_string()
            }
        };
        let params = mdh+&path.into_inner().to_string();
        let one = redis.send(Command(resp_array!["LRANGE", "generates"].append(vec!["0".to_string(),"-1".to_string()])));
        //let result = one.await?;
        match one.await{
            Ok(res)=>{
                use redis_async::resp::FromResp;
                let res2: Result<Option<Vec<String>>, _> = res.and_then(|value| FromResp::from_resp(value).map_err(actix_redis::Error::Redis));
                match res2 {
                    Ok(r)=>{
                        match r{
                            Some(r2)=>{
                                if r2.iter().clone().any(|prms| prms == &params){
                                    Ok(HttpResponse::Ok().content_type("application/json").json("Verileriniz kayıtlı"))
                                }
                                else{
                                    let add_params = redis.send(Command(resp_array!["RPUSH", "generates"].append(vec![params])));
                                    match add_params.await{
                                        Ok(_t)=>{
                                            Ok(HttpResponse::Ok().content_type("application/json").json("Verileriniz sıraya eklendi"))
                                        },
                                        Err(_e)=>{
                                            Ok(HttpResponse::Ok().content_type("application/json").json("Sunucu hatası"))
                                        }
                                    }
                                }
                            },
                            None=>{
                                let add_params = redis.send(Command(resp_array!["RPUSH", "generates"].append(vec![params])));
                                match add_params.await{
                                    Ok(_t)=>{
                                        Ok(HttpResponse::Ok().content_type("application/json").json("Verileriniz sıraya eklendi"))
                                    },
                                    Err(_e)=>{
                                        Ok(HttpResponse::Ok().content_type("application/json").json("Sunucu hatası"))
                                    }
                                }
                            }
                        }
                    },
                    Err(_e)=>{
                        let delete = redis.send(Command(resp_array!["DEL", "generates"]));
                        match delete.await{
                            Ok(_t)=>{
                                Ok(HttpResponse::Ok().content_type("application/json").json("Veriler sıfırlandı"))
                            },
                            Err(_e)=>{
                                Ok(HttpResponse::Ok().content_type("application/json").json("Sunucu hatası"))
                            }
                        }
                    }
                }
            },
            Err(_e)=>{
                Ok(HttpResponse::InternalServerError().json("Sunucu hatası"))
            }

            /*Ok(Ok(RespValue::BulkString(x)))=>{
                println!("{:?}", x);
                Ok(HttpResponse::Ok().content_type("text/plain").json("Merhaba X"))
            },
            Ok(Ok(RespValue::Nil)) =>{
                let mut school = vec![params.unwrap().into_inner().max_day_hour.to_string(), path.into_inner().to_string()];
                let two = redis.send(Command(resp_array!["RPUSH", "generates"].append(school)));
                match two.await{
                    Ok(Ok(RespValue::Error(x)))=>{
                        println!("eklenmedi");
                        Ok(HttpResponse::Ok().content_type("application/json").json("Merhaba"))
                    },
                    _ =>{
                        println!("eklendi");
                        Ok(HttpResponse::Ok().content_type("application/json").json("Merhab"))
                    }
                }
            },
            Ok(Ok(RespValue::SimpleString(x)))=>{
                println!("{:?}", x);
                Ok(HttpResponse::Ok().content_type("text/plain").json("Merha"))
            },
            Ok(Ok(RespValue::Error(x)))=>{
                println!("{:?}", x);
                Ok(HttpResponse::Ok().content_type("text/plain").json("Merha"))
            },
            _ =>{
                println!("son");
                Ok(HttpResponse::Ok().content_type("text/plain").json("Merha"))
            }*/
        }
        //
        /*let info_set = join_all(vec![one].into_iter());
    //let mut okey = "AA".to_string();
    info_set
        .map_err(AWError::from)
        .and_then(|res: Vec<Result<RespValue, ARError>>|
            // successful operations return "OK", so confirm that all returned as so
            if !res.iter().all(|res| match res {
                Ok(RespValue::Integer(x)) => true,
                _ => false
            }) {
                //okey = "Tamamlanmadı".to_string();
                return Ok(HttpResponse::InternalServerError().finish())
            } else {
                //okey = "Tamamlandı".to_string();
                return Ok(HttpResponse::Ok().json("successfully cached values"))
            }
        )*/
        //Ok(HttpResponse::Ok().content_type("text/plain").body("Merhaba"))
    }
}
    //Add all timetables array to database   
