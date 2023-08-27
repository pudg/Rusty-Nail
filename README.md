# Rust Nail
REST API written in Rust using Actix-web/Diesel frameworks, and PostgreSQL database.

# Install
## Setup Database
The project uses _PostgreSQL_ and _diesel\_cli_  for the backends database management.

    cargo install diesel_cli --no-default-features --features postgres

Within the project directory, open a terminal and run the following. Change the `username` and `password` portions of the `DATABASE_URL` to your corresponding postgres database.

    echo DATABASE_URL=postgres://username:password@localhost/rusty_nail > .env

Create database, initial migrations, and table to store cocktail recipes.

    diesel setup
    diesel migration generate cocktail_recipes

Apply migrations to generate `schema.rs` file.

    diesel migration run


## TODO
- [X] Add database schema
- [ ] Add database connection module
- [ ] Add API endpoints
  - [ ] Add create recipe functionality (POST)
  - [ ] Add read all/detail recipe functionality (GET)
  - [ ] Add update recipe functionality (PUT)
  - [ ] Add delete recipe functionality (DELETE)
- [ ] Add pagination
- [ ] Add search filtering
- [ ] Deploy