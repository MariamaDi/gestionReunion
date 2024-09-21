use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;

use crate::{models::reunions::NewReunions, services::reunions_service, AppState};
pub async fn create_reunion(
    state: web::Data<AppState>,
    new_reunions: web::Json<NewReunions>,
) -> impl Responder {
    let new_reunions = new_reunions.into_inner();
    match reunions_service::create_reunion(&mut state.get_conn(), new_reunions).await {
        Ok(inserted_participants) => HttpResponse::Created().json(inserted_participants),
        Err(err) => HttpResponse::InternalServerError()
            .body(format!("Failed to insert participants: {}", err)),
    }
}

pub async fn get_reunions(state: web::Data<AppState>) -> impl Responder {
    match reunions_service::get_reunions(&mut state.get_conn()).await {
        Ok(results) => HttpResponse::Ok().json(results),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to load reunions: {}", err))
        }
    }
}

pub async fn get_reunion_by_id(
    state: web::Data<AppState>,
    reunions_id: web::Path<Uuid>,
) -> impl Responder {
    match reunions_service::get_reunion_by_id(&mut state.get_conn(), reunions_id.into_inner()).await
    {
        Ok(reunions) => HttpResponse::Ok().json(reunions),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to load reunions: {}", err))
        }
    }
}

pub async fn update_reunions(
    state: web::Data<AppState>,
    new_reunions: web::Json<NewReunions>,
    reunions_id: web::Path<Uuid>,
) -> impl Responder {
    match reunions_service::update_reunions(
        &mut state.get_conn(),
        new_reunions.into_inner(),
        reunions_id.into_inner(),
    )
    .await
    {
        Ok(inserted_reunions) => HttpResponse::Ok().json(inserted_reunions),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to insert reunions: {}", err))
        }
    }
}

pub async fn delete_reunions(
    state: web::Data<AppState>,
    reunions_id: web::Path<Uuid>,
) -> HttpResponse {
    let conn = &mut state
        .conn
        .get()
        .expect("Couldn't get DB connection from pool");
    match reunions_service::delete_reunions(conn, reunions_id.into_inner()).await {
        Ok(reunions) => HttpResponse::Ok().json(reunions),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
