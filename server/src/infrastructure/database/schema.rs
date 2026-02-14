// @generated automatically by Diesel CLI.

diesel::table! {
    brawlers (id) {
        id -> Int4,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        #[max_length = 50]
        display_name -> Varchar,
        #[max_length = 512]
        avatar_url -> Nullable<Varchar>,
        #[max_length = 255]
        avatar_public_id -> Nullable<Varchar>,
    }
}

diesel::table! {
    crew_memberships (mission_id, brawler_id) {
        mission_id -> Int4,
        brawler_id -> Int4,
        joined_at -> Timestamp,
    }
}

diesel::table! {
    missions (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        #[max_length = 255]
        status -> Varchar,
        chief_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
        max_crew -> Int4,
        #[max_length = 50]
        difficulty -> Varchar,
    }
}

diesel::joinable!(crew_memberships -> brawlers (brawler_id));
diesel::joinable!(crew_memberships -> missions (mission_id));
diesel::joinable!(missions -> brawlers (chief_id));

diesel::allow_tables_to_appear_in_same_query!(
    brawlers,
    crew_memberships,
    missions,
);
