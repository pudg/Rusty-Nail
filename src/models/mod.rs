use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, AsChangeset, Insertable)]
#[diesel(table_name = crate::schema::recipes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
/// Represents a Recipe record in the database.
pub struct Recipe {
    /// A recipe has a unique id.
    pub id: i32,
    /// A recipe has a name.
    pub drink_name: String,
    /// A recipe has at least one ingredient.
    pub ingredients: serde_json::Value,
    /// A recipe has instructions.
    pub instructions: String,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = crate::schema::recipes)]
/// Represents Recipe records in the database.
pub struct Recipes {
    /// All recipes have a name.
    pub drink_name: String,
    /// All recipes have at least one ingredient.
    pub ingredients: serde_json::Value,
    /// All recipes have instructions.
    pub instructions: String,
}
