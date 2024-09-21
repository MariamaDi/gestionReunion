// @generated automatically by Diesel CLI.

diesel::table! {
    participants (id) {
        id -> Uuid,
        #[max_length = 255]
        nom -> Varchar,
        #[max_length = 255]
        email -> Varchar,
    }
}

diesel::table! {
    reunions (id) {
        id -> Uuid,
        #[max_length = 255]
        titre -> Varchar,
        date_reunion -> Date,
        heure_debut -> Time,
        heure_fin -> Time,
        participants -> Array<Nullable<Uuid>>,
        notes -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    participants,
    reunions,
);
