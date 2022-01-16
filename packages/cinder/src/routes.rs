use crate::data::UpdateInfo;
use crate::entities::{
    apps, apps::Entity as AppEntity, deploys, deploys::Entity as DeployEntity,
};
use crate::errors::ApiError;
use crate::AppState;
use actix_web::{get, post, web, HttpResponse};
use sea_orm::{entity::*, query::*};
use serde_json::json;

#[post("/update")]
pub async fn update_app(
    data: web::Data<AppState>,
    app_name: web::Path<String>,
    update_info: web::Json<UpdateInfo>,
) -> Result<HttpResponse, ApiError> {
    let possible_app = AppEntity::find_by_id(app_name.clone())
        .one(&data.conn)
        .await?;
    Ok(match possible_app {
        Some(app) => {
            let active_deploy = deploys::ActiveModel {
                sha: Set(update_info.sha.clone()),
                app_name: Set(app.name.clone()),
                ..Default::default()
            };
            let deploy = active_deploy.insert(&data.conn).await?;
            HttpResponse::Ok().json(json!({
                "app": app,
                "deploy": deploy
            }))
        }
        None => {
            let active_app = apps::ActiveModel {
                name: Set(app_name.to_string()),
                ..Default::default()
            };
            let app = active_app.insert(&data.conn).await?;
            let active_deploy = deploys::ActiveModel {
                sha: Set(update_info.sha.clone()),
                app_name: Set(app.name.clone()),
                ..Default::default()
            };
            let deploy = active_deploy.insert(&data.conn).await?;
            HttpResponse::Ok().json(json!({
                "app": app,
                "deploy": deploy
            }))
        }
    })
}

#[get("/")]
async fn get_all_apps(data: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let apps = AppEntity::find()
        .order_by_desc(apps::Column::CreatedAt)
        .all(&data.conn)
        .await?;
    Ok(HttpResponse::Ok().json(apps))
}

#[get("")]
async fn find_app_by_name(
    data: web::Data<AppState>,
    app_name: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let app = AppEntity::find_by_id(app_name.clone())
        .one(&data.conn)
        .await?;
    Ok(match app {
        Some(app) => {
            let deploy = app
                .find_related(DeployEntity)
                .order_by_desc(deploys::Column::ExecutedAt)
                .one(&data.conn)
                .await?;
            HttpResponse::Ok().json(json!({
                "app": app,
                "deploy": deploy
            }))
        }
        None => HttpResponse::NotFound().json(json!({
            "message": format!("An app with name='{}' does not exist", &app_name)
        })),
    })
}
