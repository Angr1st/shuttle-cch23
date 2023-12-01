use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Router};

async fn xor_pow_three(Path((num1, num2)): Path<(i32, i32)>) -> impl IntoResponse {
    (num1 ^ num2).pow(3).to_string()
}

async fn respond_internal_server_error() -> (StatusCode, &'static str) {
    (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/:num1/:num2", get(xor_pow_three))
        .route("/-1/error", get(respond_internal_server_error));

    Ok(router.into())
}
