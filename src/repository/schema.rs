// @generated automatically by Diesel CLI.

diesel::table! {
    alerts (id) {
        id -> Int4,
        source -> Varchar,
        source_component -> Varchar,
        alert_type -> Varchar,
        description -> Nullable<Text>,
        subject_type -> Varchar,
        subject_reference_number -> Varchar,
        subject_description -> Nullable<Text>,
        content -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}
