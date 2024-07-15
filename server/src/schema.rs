// @generated automatically by Diesel CLI.

diesel::table! {
    words (id) {
        id -> Int4,
        #[max_length = 255]
        word -> Varchar,
    }
}
