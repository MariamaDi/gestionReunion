use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;
//Gestion de reunions
use crate::{models::participants::NewParticipants, services::participants_service, AppState};
pub async fn create_participants(
    state: web::Data<AppState>,
    new_participants: web::Json<NewParticipants>,
) -> impl Responder {
    let new_participants = new_participants.into_inner();
    match participants_service::create_participants(&mut state.get_conn(), new_participants).await {
        Ok(inserted_participants) => HttpResponse::Created().json(inserted_participants),
        Err(err) => HttpResponse::InternalServerError()
            .body(format!("Failed to insert participants: {}", err)),
    }
}

pub async fn get_participants(state: web::Data<AppState>) -> impl Responder {
    match participants_service::get_participants(&mut state.get_conn()).await {
        Ok(results) => HttpResponse::Ok().json(results),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to load participant: {}", err))
        }
    }
}

pub async fn update_participants(
    state: web::Data<AppState>,
    new_participants: web::Json<NewParticipants>,
    participants_id: web::Path<Uuid>,
) -> impl Responder {
    match participants_service::update_participants(
        &mut state.get_conn(),
        new_participants.into_inner(),
        participants_id.into_inner(),
    )
    .await
    {
        Ok(inserted_participants) => HttpResponse::Ok().json(inserted_participants),
        Err(err) => HttpResponse::InternalServerError()
            .body(format!("Failed to insert participants: {}", err)),
    }
}

pub async fn delete_participants(
    state: web::Data<AppState>,
    participants_id: web::Path<Uuid>,
) -> HttpResponse {
    let conn = &mut state
        .conn
        .get()
        .expect("Couldn't get DB connection from pool");
    match participants_service::delete_participants(conn, participants_id.into_inner()).await {
        Ok(participants) => HttpResponse::Ok().json(participants),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_participant_by_id(
    state: web::Data<AppState>,
    participant_id: web::Path<Uuid>,
) -> impl Responder {
    match participants_service::get_participant_by_id(
        &mut state.get_conn(),
        participant_id.into_inner(),
    )
    .await
    {
        Ok(participants) => HttpResponse::Ok().json(participants),
        Err(err) => HttpResponse::InternalServerError()
            .body(format!("Failed to load participants: {}", err)),
    }
}
