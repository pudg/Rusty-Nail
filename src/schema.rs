// @generated automatically by Diesel CLI.

diesel::table! {
    recipes (id) {
        id -> Int4,
        drink_name -> Varchar,
        ingredients -> Json,
        instructions -> Text,
    }
}
