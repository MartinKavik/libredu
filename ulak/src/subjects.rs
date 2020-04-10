//use actix_web::http::{header, Method, StatusCode};
use actix_web::{
    HttpRequest, HttpResponse, Result
};
use actix_web::web::{Json, Data};
use actix_session::{Session};

use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
//use diesel::BelongingToDsl;
use crate::util::database::pool;
use crate::models::others::{Subject, Class};
use crate::schema::*;
use crate::request::Request;


#[derive(Deserialize, Serialize,)]
pub struct ClassId{
    class: i32
}

pub async fn index(_tmpl: Data<tera::Tera>, _session: Session, _req: HttpRequest, class: Option<Json<ClassId>> )
-> Result<HttpResponse>
{
    let conn = pool();
    let cls = class.unwrap().into_inner();
    let cls = classes::table.filter(classes::id.eq(cls.class)).get_result::<Class>(&conn).unwrap();
    let schl = _req.school(cls.school).unwrap();
    let sbjcts = crate::schema::class_subjects::table.filter(crate::schema::class_subjects::class.eq(cls.id))
        .select(crate::schema::class_subjects::subject)
        .get_results::<i32>(&conn).unwrap();
    let mut subjects = crate::schema::subjects::table.filter(crate::schema::subjects::id.eq_any(sbjcts))
        .get_results::<Subject>(&conn).unwrap();
    if subjects.len()==0{
        subjects = subjects::table.filter(subjects::kademe.eq(cls.kademe)).filter(subjects::school_type.eq(schl.school_type))
            .get_results::<Subject>(&conn).unwrap();
    }

    Ok(HttpResponse::Ok().content_type("applications/json").json(subjects))

}