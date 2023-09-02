mod database;
mod schema;
mod models;
mod routes;
mod errors;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use listenfd::ListenFd;
use std::env;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let mut ssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())
    .unwrap();
    ssl_builder
    .set_private_key_file("key.pem", SslFiletype::PEM)
    .expect("Failed to open/read key.pem");
    ssl_builder
    .set_certificate_chain_file("cert.pem")
    .expect("Failed to open/read cert.pem");

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| App::new().configure(routes::init_routes));
    server = match listenfd.take_tcp_listener(0) ? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Set HOST in .env");
            let port = env::var("PORT").expect("Set PORT in .env");
            let port = port.parse::<i32>().ok().expect("Expected an int for PORT in .env");
            println!("Listening on {}:{}", host, port);
            server.bind_openssl(format!("{}:{}", host, port), ssl_builder)?
        }
    };
    server.run().await
}