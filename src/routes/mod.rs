use crate::models::Recipe;
use serde_json::json;
use actix_web::{delete, get, post, put, web, HttpResponse, Error};

///CRUD operation handlers for API.

/// Returns a list of all Recipes database.
#[get("/recipes")]
async fn find_all() -> Result<HttpResponse, Error> {
    let all_recipes = web::block(|| Recipe::find_all())
    .await??;

    Ok(HttpResponse::Ok().json(all_recipes))
    // Ok(HttpResponse::Ok().json(json!({"TODO": "IMPLEMENT GET /find_all"})))
}

/// Returns information about specified Recipe if it exists.
/// 
/// # Arguments
///  * `id` - An integer that identifies a specific Recipe.
/// 
#[get("/recipes/{id}")]
async fn find(id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let recipe = Recipe::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(recipe))
}

/// Creates a new Recipe record in the database.
/// 
/// # Arguments
///     * `recipes` - json object denoting a Recipe.
/// 
/// # Examples
/// ```
///     Recipe object has the following structure:
///     
///     {
///         "id": 7,
///         "drink_name": "rusty nail",
///         "ingredients": {
///             "ingredient1": "vodka",
///             "ingredient2": "lime",
///             ...
///             "ingredientN": "cherries"
///         },
///         "instructions": "shake it up!"
///     }
/// ```
#[post("/recipes")]
async fn create(recipes: web::Json<Recipe>) -> Result<HttpResponse, Error> {
    let recipe = Recipe::create(recipes.into_inner())?;
    Ok(HttpResponse::Ok().json(recipe))
}

/// Creates or Updates a Recipe in the database.
/// 
/// # Arguments
///     * `id` - An integer denoting the Recipe to be updated, or created if it doesn't exist.
///     * `recipe` - A json object denoting Recipe to be updated or created.
#[put("/recipes/{id}")]
async fn update(id: web::Path<i32>, recipe: web::Json<Recipe>) -> Result<HttpResponse, Error> {
    let recipe = Recipe::update(id.into_inner(), recipe.into_inner())?;
    Ok(HttpResponse::Ok().json(recipe))
}

/// Deletes a recipe from the database.
/// 
/// # Arguments
///     * `id` - An integer denoting the Recipe to be deleted.
#[delete("recipes/{id}")]
async fn delete(id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let recipe = Recipe::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({"deleted": recipe})))
}

/// Registers all route handlers.
pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(find);
    config.service(create);
    config.service(update);
    config.service(delete);
}