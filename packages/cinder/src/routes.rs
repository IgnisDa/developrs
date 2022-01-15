use actix_web::{get, post, web, Error, HttpResponse};

use crate::{actions, models::NewApp, DbPool};

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
async fn get_all_apps(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let users = web::block(move || {
        let conn = pool.get()?;
        actions::get_all_apps(&conn)
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().json(users))
}

#[get("/{app_id}")]
async fn find_app_by_id(
    pool: web::Data<DbPool>,
    app_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let app_id = app_id.into_inner();

    // use web::block to offload blocking Diesel code without blocking server thread
    let user = web::block(move || {
        let conn = pool.get()?;
        actions::find_app_by_id(app_id, &conn)
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        let res =
            HttpResponse::NotFound().body(format!("No app found with id: {:?}", app_id));
        Ok(res)
    }
}
