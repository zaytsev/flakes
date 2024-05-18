use crate::handler;
use crate::{assets::Assets, context::HandlerContext};
use axum::{routing::get, Router};

pub fn routes(db: db::ConnectionPool) -> Router {
    let context = HandlerContext::new(db);
    Router::new()
        .route("/", get(handler::index))
        .with_state(context)
        .nest_service("/assets", Assets::service())
}
