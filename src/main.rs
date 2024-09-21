use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use controllers::{participants_controller, reunions_controller};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use models::database::AppState;
use r2d2::Pool;
use std::env;

mod controllers;
mod models;
mod schema;
mod services;

pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

fn get_pool() -> PostgresPool {
    dotenv::dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mgr = ConnectionManager::<PgConnection>::new(url);
    r2d2::Pool::builder()
        .build(mgr)
        .expect("could not build connection pool")
}

fn logging_setup() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    logging_setup();
    let pool = get_pool();
    let state = AppState { conn: pool };
    println!("Backend launched!");
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(state.clone()))
            .service(
                web::scope("/participants")
                    .route(
                        "",
                        web::post().to(participants_controller::create_participants),
                    )
                    .route("", web::get().to(participants_controller::get_participants))
                    .route(
                        "/{participant_id}",
                        web::put().to(participants_controller::update_participants),
                    )
                    .route(
                        "/{participant_id}",
                        web::delete().to(participants_controller::delete_participants),
                    )
                    .route(
                        "/{participant_id}",
                        web::get().to(participants_controller::get_participant_by_id),
                    ),
            )
            .service(
                web::scope("/reunions")
                    .route("", web::get().to(reunions_controller::get_reunions))
                    .route("", web::post().to(reunions_controller::create_reunion))
                    .route(
                        "/{reunions_id}",
                        web::get().to(reunions_controller::get_reunion_by_id),
                    )
                    .route(
                        "/{reunions_id}",
                        web::delete().to(reunions_controller::delete_reunions),
                    )
                    .route(
                        "/{reunions_id}",
                        web::put().to(reunions_controller::update_reunions),
                    ),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
