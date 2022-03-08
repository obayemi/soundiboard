#[macro_use]
extern crate rocket;
use rocket::http::Status;
use sqlx::postgres::PgPoolOptions;
use sqlx::postgres::PgPool;
use dotenv::dotenv;
use std::env;
use rocket::serde::{Serialize, json::Json};
use tracing::{info};
use rocket::http::Header;
use rocket::{Request, Response, };
use chrono::Utc;
use rocket::request::{self, FromRequest, Outcome};
use rocket::fairing::{Fairing, Info, Kind};
use serde::Deserialize;
use bcrypt::{DEFAULT_COST, hash, verify};

pub struct CORS;

const BEARER: &str = "BEARER ";

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        info!("set_cors");
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}


#[derive(Deserialize, Serialize)]
struct Sound {
    file_url: String,
    volume: i16,
    name: String,
}

#[options("/sounds")]
fn sounds_option() {}

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
async fn sounds_detail(name: String, pool: &rocket::State<PgPool>) -> Option<Json<Sound>> {
    Some(
        sqlx
        ::query_as!(
            Sound,
            "SELECT file_url, volume, name FROM sounds WHERE name = $1",
            name
            )
        .fetch_optional(&**pool)
        .await
        .unwrap()?
        .into()
        )
}

#[put("/sounds/<name>", data = "<sound>")]
async fn sounds_update(name: String, sound: Json<Sound>, _user: LoggedUser, pool: &rocket::State<PgPool>) -> Option<Json<Sound>> {
    Some(
        sqlx
        ::query_as!(
            Sound,
            "UPDATE sounds SET file_url = $1, volume = $2, name = $3 WHERE name = $4 RETURNING file_url, volume, name",
            sound.file_url, sound.volume, sound.name, name
            )
        .fetch_optional(&**pool)
        .await
        .unwrap()?
        .into()
        )
}

#[derive(Deserialize)]
struct UserCredentials {
    username: String,
    password: String
}

#[derive(Serialize)]
struct User {
    id: uuid::Uuid,
    username: String,
}


#[post("/signup", data = "<user>")]
async fn signup(user: Json<UserCredentials>, pool: &rocket::State<PgPool>) -> Result<Json<User>, Status> {
    if Some("true") == env::var("SIGNUP_ACTIVE").as_deref().ok() {
        info!(username=user.username.as_str(), password=user.password.as_str(), "signup");
        let hashed = hash(&user.password, DEFAULT_COST).unwrap();
        Ok(sqlx::
            query_as!(
                User,
                "INSERT INTO users (username, password) VALUES ($1, $2) RETURNING id, username",
                user.username,
                hashed
                )
            .fetch_one(&**pool)
            .await
            .unwrap()
            .into()
          )
    } else {
        Err(Status::Unauthorized)
    }
}

#[get("/users")]
async fn list_users(pool: &rocket::State<PgPool>) -> Result<Json<Vec<User>>, Status>{
    Ok(
        sqlx::query_as!(
            User,
            "SELECT id, username FROM users"
            )
        .fetch_all(&**pool)
        .await
        .unwrap()
        .into()
      )
}

#[derive(Serialize)]
struct TokenResponse {
    token: String
}

#[derive(Serialize, Deserialize)]
struct TokenClaim {
    sub: uuid::Uuid,
    username: String,
    exp: usize,
}

#[post("/login", data = "<credentials>")]
async fn login(credentials: Json<UserCredentials>, pool: &rocket::State<PgPool>) -> Result<Json<TokenResponse>, Status> {
    info!(username=credentials.username.as_str(), password=credentials.password.as_str(), "login");
    let user = 
        sqlx::
        query!(
            "SELECT id, username, password FROM users WHERE username = $1",
            credentials.username,
            )
        .fetch_one(&**pool)
        .await
        .map_err(|_| Status::Unauthorized)?;
    info!(user_id=user.id.to_string().as_str(), password=user.password.as_deref(), username=credentials.username.as_str(), password=credentials.password.as_str(), "login");
    let hashed = user.password.ok_or(Status::Unauthorized)?;
    info!("user has password");
    if verify(&credentials.password, &hashed).unwrap() {
        info!("user is verified");
        let exp = Utc::now()
            .checked_add_signed(chrono::Duration::seconds(60))
            .expect("valid timestamp")
            .timestamp();
        let claims = TokenClaim{
            sub: user.id,
            username: user.username,
            exp: exp.try_into().unwrap()
        };
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret("secret".as_ref())
            ).unwrap();
        Ok(TokenResponse{token}.into())
    } else  {
        info!("user is unverified");
        Err(Status::Unauthorized)
    }
}

#[derive(Debug)]
enum JwtError {
    Missing,
    Invalid,
}

#[derive(Debug)]
struct LoggedUser {
    id: uuid::Uuid,
    username: String,
}

impl From<TokenClaim> for LoggedUser {
    fn from(claim: TokenClaim) -> Self {
        Self {
            id: claim.sub,
            username: claim.username,
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for LoggedUser {
    type Error = JwtError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        info!("foo: {:?}", req.headers());
        match req.headers().get_one("Authorization") {
            Some(bearer) => {
                info!("token: {}", bearer);
                if !bearer.starts_with(BEARER) {
                    return Outcome::Failure((Status::Unauthorized, JwtError::Invalid))
                }
                let token = bearer.trim_start_matches(BEARER).to_owned();
                info!("token: {}, {}", bearer, token);
                let decoded = {
                    match jsonwebtoken::decode::<TokenClaim>(&token, &jsonwebtoken::DecodingKey::from_secret("secret".as_ref()), &jsonwebtoken::Validation::default()) {
                        Ok(decoded) => decoded,
                        Err(e) => {
                            info!("{:?}", e);
                            return Outcome::Failure((Status::Unauthorized, JwtError::Invalid))
                        }
                    }
                };
                Outcome::Success(decoded.claims.into())
            },
            None => Outcome::Failure((Status::Unauthorized, JwtError::Missing))
        }
    }
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
            signup,
            sounds_update,
            login,
            list_users,
            sounds_option,
            ])
        .attach(CORS)
        .manage(pool)
        .launch()
        .await
        .unwrap();
}
