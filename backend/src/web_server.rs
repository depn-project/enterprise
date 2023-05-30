use actix_web::dev::ServiceRequest;
use actix_web::error::ErrorUnauthorized;
use actix_web::Error;
use actix_web::{web, App, HttpServer};
use actix_web_httpauth::extractors::basic::BasicAuth;
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::storage::{Database, Repository, Storage};
use crate::user::{verify_password, UserService};

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

        App::new()
            .wrap(auth)
            .route("/", web::get().to(|| async { "Hello, middleware!" }))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
