use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::{errors::CustomError, database::create_connection};

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

/// Implement set of functions for Recipes.
impl Recipe {
    pub fn create(recipe: Recipe) -> Result<Self, CustomError> {
        use crate::schema::recipes;
        
        let mut connection = create_connection()?;
        let recipe = diesel::insert_into(recipes::table)
        .values(recipe)
        .returning(Recipe::as_returning())
        .get_result(&mut connection)
        .expect("Error saving Recipe to database.");

        Ok(recipe)
    }

    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        use crate::schema::recipes::dsl::*;
        let mut connection = create_connection()?;
        let all_recipes = recipes
        .select(Recipe::as_select())
        .load(&mut connection)
        .expect("Error loading Recipes");
        Ok(all_recipes)
    }

    pub fn find(id: i32) -> Result<Self, CustomError> {
        use crate::schema::recipes::dsl::recipes;
        let mut connection = create_connection()?;
        let recipe = recipes
        .find(id)
        .select(Recipe::as_select())
        .first(&mut connection)?;
        
        Ok(recipe)
    }

    pub fn update(id: i32, recipe: Recipe) -> Result<Self, CustomError> {
        use crate::schema::recipes::dsl::recipes;
        let mut connection = create_connection()?;
        let recipe = diesel::update(recipes.find(id))
        .set(recipe)
        .get_result(&mut connection)?;
        
        Ok(recipe)
    }

    pub fn delete(id: i32) -> Result<usize, CustomError> {
        use crate::schema::recipes;

        let mut connection = create_connection()?;
        let response = diesel::delete(recipes::table.filter(recipes::id.eq(id)))
        .execute(&mut connection)?;

        Ok(response)
    }
}
