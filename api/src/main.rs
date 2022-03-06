#[macro_use]
extern crate rocket;
use rocket::http::Status;
use rocket::response::status;
use sqlx::postgres::PgPoolOptions;
use sqlx::postgres::PgPool;
use dotenv::dotenv;
use std::env;
use rocket::serde::{Serialize, json::Json};
use tracing::{info,warn};
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}


#[derive(Serialize)]
struct Sound {
    file_url: String,
    volume: i16,
    name: String,
}

#[get("/sounds")]
async fn sounds_list(pool: &rocket::State<PgPool>) -> Result<Json<Vec<Sound>>, Status> {
    Ok(
        sqlx::query_as!(
            Sound,
            "SELECT file_url, volume, name FROM sounds"
            )
        .fetch_all(&**pool)
        .await
        .unwrap()
        .into()
      )
}

#[get("/sounds/<name>")]
async fn sounds_detail(name: String, pool: &rocket::State<PgPool>) -> Result<Json<Sound>, status::NotFound<String>> {
    Ok(
        sqlx
        ::query_as!(
            Sound,
            "SELECT file_url, volume, name FROM sounds WHERE name = $1",
            name
            )
        .fetch_optional(&**pool)
        .await
        .unwrap()
        .ok_or_else(|| status::NotFound(format!("rule {} not found", name)))?
        .into()
      )
}

#[get("/ok")]
fn healthcheck() -> String {
    "ok".to_string()
}

#[rocket::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt::init();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").expect("missing `DATABASE_URL` env variable"))
        .await
        .expect("error connecting to the db");

    sqlx::migrate!()
        .run(&pool)
        .await.unwrap();

    rocket::build()
        .mount(
            "/",
            routes![
            healthcheck,
            sounds_list,
            sounds_detail,
            ])
        .attach(CORS)
        .manage(pool)
        .launch()
        .await
        .unwrap();
}
