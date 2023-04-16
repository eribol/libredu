use config::CONFIG;
use moon::{
    actix_cors::Cors,
    actix_http::StatusCode,
    actix_web::{
        middleware::{Compat, Condition, ErrorHandlers, Logger},
        App,
    },
    *,
};
mod connection;
mod frontend;
mod up_msg_handler;
mod user;
pub mod send_mail;

#[moon::main]
async fn main() -> std::io::Result<()> {
    let app = || {
        let redirect = Redirect::new()
            .http_to_https(CONFIG.https)
            .port(CONFIG.redirect.port, CONFIG.port);

        App::new()
            .wrap(Condition::new(
                CONFIG.redirect.enabled,
                Compat::new(redirect),
            ))
            // https://docs.rs/actix-web/4.0.0-beta.8/actix_web/middleware/struct.Logger.html
            .wrap(Logger::new(r#""%r" %s %b "%{Referer}i" %T"#))
            .wrap(Cors::default().allowed_origin_fn(move |origin, _| {
                if CONFIG.cors.origins.contains("*") {
                    return true;
                }
                let origin = match origin.to_str() {
                    Ok(origin) => origin,
                    // Browsers should always send a valid Origin.
                    // We don't care about invalid Origin sent from non-browser clients.
                    Err(_) => return false,
                };
                CONFIG.cors.origins.contains(origin)
            }))
            .wrap(
                ErrorHandlers::new()
                    .handler(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        error_handler::internal_server_error,
                    )
                    .handler(StatusCode::NOT_FOUND, error_handler::not_found),
            )
    };
    start_with_app(
        frontend::frontend,
        up_msg_handler::up_msg_handler,
        app,
        |_| {},
    )
    .await
}
