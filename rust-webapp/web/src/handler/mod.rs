use axum::{extract::State, response::IntoResponse};
use maud::html;

use crate::{context::HandlerContext, view};

pub async fn index(
    State(_ctx): State<HandlerContext>,
) -> Result<impl IntoResponse, crate::error::Error> {
    Ok(view::common::page_template(
        "Main",
        html! {
            h1 {
                "Index"
            }
        },
    ))
}
