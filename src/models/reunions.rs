use chrono::NaiveDate;
use chrono::NaiveTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::reunions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Reunions {
    pub id: Uuid,
    pub titre: String,
    pub date_reunion: NaiveDate,
    pub heure_debut: NaiveTime,
    pub heure_fin: NaiveTime,
    pub participants: Vec<Option<Uuid>>,
    pub notes: Option<String>,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::reunions)]
pub struct NewReunions {
    pub titre: String,
    pub date_reunion: NaiveDate,
    pub heure_debut: NaiveTime,
    pub heure_fin: NaiveTime,
    pub participants: Vec<Uuid>,
    pub notes: Option<String>,
}
