// use actix_web::{web, HttpResponse, Responder};
use diesel::{prelude::*, r2d2::ConnectionManager, result::Error};
use r2d2::PooledConnection;
use uuid::Uuid;

use crate::{
    models::reunions::{NewReunions, Reunions},
    schema::reunions::dsl::*,
    services::participants_service::validate_participants,
};
pub async fn create_reunion(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    new_reunion: NewReunions,
) -> Result<Reunions, Error> {
    let new_reunion = NewReunions {
        titre: new_reunion.titre.clone(),
        date_reunion: new_reunion.date_reunion,
        heure_debut: new_reunion.heure_debut,
        heure_fin: new_reunion.heure_fin,
        participants: new_reunion.participants.clone(),
        notes: new_reunion.notes.clone(),
    };
    let valid_participants = validate_participants(conn, &new_reunion.participants)?;
    if !valid_participants {
        return Err(diesel::result::Error::NotFound);
    }

    diesel::insert_into(reunions)
        .values(&new_reunion)
        .get_result::<Reunions>(conn)
}

pub async fn get_reunions(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<Vec<Reunions>, Error> {
    reunions.load::<Reunions>(conn)
}

pub async fn get_reunion_by_id(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    reunion_id: Uuid,
) -> Result<Reunions, Error> {
    reunions
        .filter(id.eq(reunion_id)) // Filtrer par l'ID de la réunion
        .first::<Reunions>(conn) // Récupérer la première correspondance
}

pub async fn delete_reunions(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    reunions_id: Uuid,
) -> Result<Reunions, Error> {
    diesel::delete(reunions.filter(id.eq(reunions_id))).get_result::<Reunions>(conn)
}

pub async fn update_reunions(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    new_reunions: NewReunions,
    reunions_id: Uuid,
) -> Result<Reunions, Error> {
    diesel::update(reunions.filter(id.eq(reunions_id)))
        .set((
            id.eq(&reunions_id),
            titre.eq(new_reunions.titre),
            date_reunion.eq(new_reunions.date_reunion),
            heure_debut.eq(new_reunions.heure_debut),
            heure_fin.eq(new_reunions.heure_fin),
            participants.eq(new_reunions.participants),
            notes.eq(new_reunions.notes),
        ))
        .get_result(conn)
}
