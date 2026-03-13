use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::app::handler::{emails, health, mobiles, persons};
use crate::app::state::AppState;

pub fn build_router(state: AppState) -> Router {
    let health_routes = Router::new()
        .route("/livez", get(health::livez))
        .route("/readyz", get(health::readyz));

    let person_routes = Router::new()
        .route("/persons", get(persons::list_persons).post(persons::create_person))
        .route("/persons/test", get(|| async { "test ok" }))
        .route("/persons/:id",
            get(persons::get_person)
            .put(persons::update_person)
            .delete(persons::delete_person),
        );

    let mobile_routes = Router::new()
        .route("/persons/:person_id/mobiles",
            get(mobiles::list_mobiles).post(mobiles::create_mobile),
        )
        .route("/mobiles/:id",
            get(mobiles::get_mobile)
            .put(mobiles::update_mobile)
            .delete(mobiles::delete_mobile),
        );

    let email_routes = Router::new()
        .route("/persons/:person_id/emails",
            get(emails::list_emails).post(emails::create_email),
        )
        .route("/emails/:id",
            get(emails::get_email)
            .put(emails::update_email)
            .delete(emails::delete_email),
        );

    Router::new()
        .merge(health_routes)
        .nest("/api/v1", person_routes)
        .nest("/api/v1", mobile_routes)
        .nest("/api/v1", email_routes)
        .with_state(state)
}