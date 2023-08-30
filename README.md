# Rusty Nail
REST API written in Rust using Actix-web, and Diesel/PostgreSQL database.

# Install
## Setup Database
The project uses _PostgreSQL_ and _diesel\_cli_  for the backends database management.

    cargo install diesel_cli --no-default-features --features postgres

Within the project directory, open a terminal and run the following. Change the `username` and `password` portions of the `DATABASE_URL` to your corresponding postgres database.

    echo DATABASE_URL=postgres://username:password@localhost/rusty_nail > .env
    echo HOST="127.0.0.1" > .env
    echo PORT=8080 > .env

Create database, initial migrations, and table to store cocktail recipes.

    diesel setup
    diesel migration generate cocktail_recipes

Apply migrations to generate `schema.rs` file.

    diesel migration run

## Run
Build and run the project binary.

    cargo build --release
    ./target/release/rusty_nail

## API

`POST /recipes`

    Creates a Recipe object given the specified json Recipe information.

`GET /recipes`

    Returns a list of all Recipes.

`GET /recipes/<int:id>`

    Returns information about the specified Recipe.

`UPDATE /recipes/<int:id>`

    Updates or Creates specified Recipe with given json Recipe object.

`DELETE /recipes/<int:id>`

    Deletes the specified Recipe from the database.



## TODO
- [X] Add database schema
- [X] Add database connection module
- [ ] Add API endpoints
  - [ ] Add create recipe functionality (POST)
  - [ ] Add read all/detail recipe functionality (GET)
  - [ ] Add update recipe functionality (PUT)
  - [ ] Add delete recipe functionality (DELETE)
- [ ] Add pagination
- [ ] Add search filtering
- [ ] Deploy