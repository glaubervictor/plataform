// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 120]
        email -> Varchar,
        #[max_length = 200]
        password -> Varchar,
    }
}
