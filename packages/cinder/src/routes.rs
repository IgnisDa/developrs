use crate::{
    actions,
    api_errors::ApiError,
    db,
    models::{App, NewApp},
    DbPool,
};
use actix_web::{get, post, web, Error, HttpResponse};
use diesel::prelude::*;

#[post("")]
pub async fn create_app(
    pool: web::Data<DbPool>,
    form: web::Json<NewApp>,
) -> Result<HttpResponse, Error> {
    let app = web::block(move || {
        let conn = pool.get()?;
        actions::insert_new_app(&form.name, &conn)
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().json(app))
}

#[get("")]
async fn get_all_apps(pool: web::Data<DbPool>) -> Result<HttpResponse, ApiError> {
    use crate::schema::apps::dsl::*;
    let conn = db::connection()?;
    let all_apps = apps.load::<App>(&conn)?;
    Ok(HttpResponse::Ok().json(all_apps))
}

#[get("/{app_name}")]
async fn find_app_by_id(
    pool: web::Data<DbPool>,
    app_name: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    use crate::schema::apps::dsl::*;
    use crate::schema::deploys::dsl::*;
    // let conn = db::connection()?;
    // let app =
    Ok(HttpResponse::Ok().json("{}"))
}
