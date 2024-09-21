// use actix_web::{web, HttpResponse, Responder};
use diesel::{prelude::*, r2d2::ConnectionManager, result::Error};
use r2d2::PooledConnection;
use uuid::Uuid;

use crate::{
    models::participants::{NewParticipants, Participants},
    schema::participants::{dsl::email, dsl::id, dsl::nom, dsl::participants},
};

pub async fn create_participants(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    new_participants: NewParticipants,
) -> Result<Participants, Error> {
    let new_participants = NewParticipants {
        nom: new_participants.nom.clone(),
        email: new_participants.email.clone(),
    };

    diesel::insert_into(participants)
        .values(&new_participants)
        .get_result::<Participants>(conn)
}
pub async fn get_participants(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<Vec<Participants>, Error> {
    participants.load::<Participants>(conn)
}

pub async fn delete_participants(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    participants_id: Uuid,
) -> Result<Participants, Error> {
    diesel::delete(participants.filter(id.eq(participants_id))).get_result::<Participants>(conn)
}

pub async fn update_participants(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    new_participants: NewParticipants,
    participants_id: Uuid,
) -> Result<Participants, Error> {
    diesel::update(participants.filter(id.eq(participants_id)))
        .set((
            id.eq(&participants_id),
            nom.eq(new_participants.nom),
            email.eq(new_participants.email),
        ))
        .get_result(conn)
}

pub async fn get_participant_by_id(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    participants_id: Uuid,
) -> Result<Participants, Error> {
    participants
        .filter(id.eq(participants_id))
        .first::<Participants>(conn)
}

pub fn validate_participants(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    participant_ids: &Vec<Uuid>,
) -> Result<bool, diesel::result::Error> {
    // Récupérer les UUID des participants existants
    let existing_participants = participants
        .filter(id.eq_any(participant_ids)) // Vérifie que les UUID dans la liste existent
        .select(id)
        .load::<Uuid>(conn)?;
    Ok(existing_participants.len() == participant_ids.len())
}
