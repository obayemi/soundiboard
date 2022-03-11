#[macro_use]
extern crate rocket;
use rocket::State;
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
use rocket::outcome::try_outcome;
use rocket::fairing::{Fairing, Info, Kind};
use serde::Deserialize;
use bcrypt::{DEFAULT_COST, hash, verify};
use rusoto_credential::EnvironmentProvider;
use rusoto_core::Region;
use rusoto_s3::S3Client;
use rusoto_s3::S3;
use rusoto_core::request::HttpClient;
use std::default::Default;
use url::Url;

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
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, PUT, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}


#[derive(Debug, Deserialize, Serialize)]
struct Sound {
    file_url: String,
    volume: i16,
    name: String,
}

#[options("/sounds")]
fn sounds_option() {}

#[get("/sounds")]
async fn sounds_list(pool: &State<PgPool>) -> Result<Json<Vec<Sound>>, Status> {
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

#[options("/sounds/<_>")]
fn sounds_detail_option() {}

#[get("/sounds/<name>")]
async fn sounds_detail(name: String, pool: &State<PgPool>) -> Option<Json<Sound>> {
    info!(name=name.as_str(), "sounds_detail");
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
struct FilesApi {
    client: S3Client,
    bucket: String,
    file_server_url: Url
}

impl FilesApi {
    async fn save_file(&self, new_file: &str) -> anyhow::Result<String> {
        let decoded = base64::decode(new_file).unwrap();
        let filename = uuid::Uuid::new_v4();
        let file_key = format!("boards/sensiboard/{}.mp3", filename.to_simple());
        info!("new file: {:?}", decoded);
        self.client.put_object(
            rusoto_s3::PutObjectRequest{
                acl: Some("public-read".to_string()),
                body: Some(decoded.into()),
                bucket: self.bucket.clone(),
                key: file_key.clone(),
                ..Default::default()
            }
            ).await?;
        Ok(self.file_server_url.join(&file_key)?.to_string())
    }
}

#[put("/sounds/<name>", data = "<sound>")]
async fn sounds_update(name: String, sound: Json<Sound>, _user: LoggedUser, pool: &State<PgPool>, files: &State<FilesApi>) -> Option<Json<Sound>> {
    info!(name=name.as_str(), sound=format!("{:?}", sound).as_str(), "sounds_update");
    let file_url = if sound.file_url.starts_with("data:") {
        let file_payload = &sound.file_url[23..];
        info!("file_payload: {}", &file_payload);
        let new = files.save_file(file_payload).await.ok()?;
        new
    } else {
        sound.file_url.clone()
    };

    Some(
        sqlx
        ::query_as!(
            Sound,
            "UPDATE sounds SET file_url = $1, volume = $2, name = $3 WHERE name = $4 RETURNING file_url, volume, name",
            file_url, sound.volume, sound.name, name
            )
        .fetch_optional(&**pool)
        .await
        .unwrap()?
        .into()
        )
}

#[post("/sounds", data = "<sound>")]
async fn sounds_create(sound: Json<Sound>, _user: LoggedUser, pool: &State<PgPool>, files: &State<FilesApi>) -> Option<Json<Sound>> {
    info!(sound=format!("{:?}", sound).as_str(), "sounds_create");
    let file_url = if sound.file_url.starts_with("data:") {
        let file_payload = &sound.file_url[23..];
        info!("file_payload: {}", &file_payload);
        let new = files.save_file(file_payload).await.ok()?;
        new
    } else {
        sound.file_url.clone()
    };

    Some(
        sqlx
        ::query_as!(
            Sound,
            "INSERT INTO sounds (file_url, name, volume) VALUES ($1, $2, 75) RETURNING file_url, volume, name",
            file_url, sound.name
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
async fn signup(user: Json<UserCredentials>, pool: &State<PgPool>) -> Result<Json<User>, Status> {
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
async fn list_users(pool: &State<PgPool>) -> Result<Json<Vec<User>>, Status>{
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
async fn login(credentials: Json<UserCredentials>, pool: &State<PgPool>, jwt_secret: &State<JwtSecret>) -> Result<Json<TokenResponse>, Status> {
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
            &jsonwebtoken::EncodingKey::from_secret(jwt_secret.as_ref())
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
    Internal,
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

struct JwtSecret(String);
use std::ops::Deref;
impl Deref for JwtSecret {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
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
                    info!("token doesn't start with {}", BEARER);
                    return Outcome::Failure((Status::Unauthorized, JwtError::Invalid))
                }
                let token = bearer.trim_start_matches(BEARER).to_owned();
                info!("token: {}, {}", bearer, token);
                let secret = try_outcome!(req.guard::<&State<JwtSecret>>().await.map_failure(|_| (Status::InternalServerError, JwtError::Internal)));
                let decoded = {
                    match jsonwebtoken::decode::<TokenClaim>(&token, &jsonwebtoken::DecodingKey::from_secret(secret.as_ref()), &jsonwebtoken::Validation::default()) {
                        Ok(decoded) => decoded,
                        Err(e) => {
                            info!("decode error: {:?}", e);
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

    let jwt_secret = env::var("JWT_SECRET").expect("missing `JWT_SECRET` env variable");

    let files_api = {
        let s3_creds = EnvironmentProvider::default();
        //let s3_creds = EnvironmentProvider::default().credentials().await.expect("missing spaces s3 credentials");
        let s3_region = Region::Custom {
            name: env::var("SPACE_REGION").expect("missing `SPACE_REGION` env variable"),
            endpoint: env::var("SPACE_ENDPOINT").expect("missing `SPACE_ENDPOINT` env variable")
        };
        let bucket = env::var("SPACE_BUCKET").expect("missing `SPACE_BUCKET` env variable");

        let s3_client = S3Client::new_with(HttpClient::new().unwrap(), s3_creds, s3_region);
        //info!("bucket content: {:?}", s3_bucket.list("/".to_string(), Some("/".to_string())).await);
        //info!("{:?}", 
        //s3_client.list_objects(rusoto_s3::ListObjectsRequest{bucket, ..Default::default()}).await
        //);
        let file_server_url = Url::parse(&env::var("FILE_SERVER_URL").expect("missing `FILE_SERVER_URL` env variable")).expect("invalid `FILE_SERVER_URL` env variable");
        FilesApi { client: s3_client, bucket,  file_server_url}
    };

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
            sounds_create,
            sounds_detail_option,
            ])
        .attach(CORS)
        .manage(pool)
        .manage(JwtSecret(jwt_secret))
        .manage(files_api)
        .launch()
        .await
        .unwrap();
}
