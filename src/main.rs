use actix_web::{App, HttpServer, web};
use dotenv::dotenv;

// local folders
mod config;
mod models;
mod handlers;
mod schemas;
mod routes;
mod utils;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Load the config
    let cnf: config::config::Config = config::config::Config::from_env();
    let address: String = cnf.listen_address();

    // Connecting to database
    let pool = config::db::connect_db(&cnf.database_url)
        .await
        .expect("Failed to connect to database");

    println!("Listening on {}", address);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::auth::auth_routes)
    })
    .bind(address)?
    .run()
    .await
}
