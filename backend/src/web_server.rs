use std::option;

use actix_cors::Cors;
use actix_web::dev::ServiceRequest;
use actix_web::error::ErrorUnauthorized;
use actix_web::http::header;
use actix_web::middleware::DefaultHeaders;
use actix_web::{get, http, options, web, App, HttpResponse, HttpServer};
use actix_web::{Error, Responder};
use actix_web_httpauth::extractors::basic::BasicAuth;
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::server::ServerService;
use crate::storage::{Database, Repository, Storage};
use crate::user::{verify_password, UserService};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/server/{id}")]
async fn server_config(path: web::Path<u32>) -> impl Responder {
    let storage = Storage::new(Database::init().unwrap());
    let server_service = ServerService::new(&storage);

    let id = path.into_inner();

    let config = server_service.get_config(id);

    match config {
        Ok(config) => HttpResponse::Ok().body(config),
        Err(e) => HttpResponse::BadRequest().body(e),
    }
}

#[get("/servers")]
async fn servers() -> impl Responder {
    let storage = Storage::new(Database::init().unwrap());
    let server_service = ServerService::new(&storage);

    let servers = server_service.list();

    HttpResponse::Ok().body(serde_json::to_string(&servers.unwrap()).unwrap())
}

/// Damn rewerite the piece of shit

async fn auth_validator(
    req: ServiceRequest,
    credentials: BasicAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let storage = Storage::new(Database::init().unwrap());
    let user_service = UserService::new(&storage);

    let username = credentials.user_id();
    let password = credentials.password();

    match password {
        Some(password) => {
            let user = user_service.get_user(username.to_string());

            match user {
                Ok(user) => match verify_password(password.to_string(), user.password) {
                    true => Ok(req),
                    false => Err((ErrorUnauthorized("Invalid username of password"), req)),
                },
                Err(_) => Err((ErrorUnauthorized("Invalid username of password"), req)),
            }
        }
        None => Err((ErrorUnauthorized("No credentials"), req)),
    }
}

pub async fn run_web_server() -> std::io::Result<()> {
    HttpServer::new(move || {
        let auth = {
            HttpAuthentication::basic(move |req: ServiceRequest, credentials: BasicAuth| {
                auth_validator(req, credentials)
            })
        };

        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods([http::Method::GET])
            .allowed_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
            .max_age(86_400);

        let security_headers = DefaultHeaders::new()
            .header(
                header::STRICT_TRANSPORT_SECURITY,
                "max-age=3153600 ; includeSubDomains",
            )
            .header(header::X_FRAME_OPTIONS, "deny")
            .header(header::X_CONTENT_TYPE_OPTIONS, "nosniff")
            .header(
                header::CONTENT_SECURITY_POLICY,
                "default-src 'self'; frame-ancestors 'none';",
            )
            .header(
                header::CACHE_CONTROL,
                "no-cache, no-store, max-age=0, must-revalidate",
            )
            .header(header::PRAGMA, "no-cache")
            .header(header::EXPIRES, "0");

        App::new()
            .wrap(auth)
            .wrap(cors)
            .wrap(security_headers)
            .service(index)
            .service(servers)
            .service(server_config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
