use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::participants)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Participants {
    pub id: Uuid,
    pub nom: String,
    pub email: String,
}
#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::participants)]
pub struct NewParticipants {
    pub nom: String,
    pub email: String,
}
