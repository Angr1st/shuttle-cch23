use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;

async fn pow_three(Path(num1): Path<i32>) -> impl IntoResponse {
    num1.pow(3).to_string()
}

async fn xor_pow_three(Path((num1, num2)): Path<(i32, i32)>) -> impl IntoResponse {
    (num1 ^ num2).pow(3).to_string()
}

async fn sled_id(Path((num1, num2, rest)): Path<(i32, i32, String)>) -> impl IntoResponse {
    let mut result = vec![num1, num2];
    if rest.contains('/') {
        result.append(
            &mut rest
                .split('/')
                .map(|rest_path| {
                    rest_path
                        .parse::<i32>()
                        .expect("Should be parsable into i32!")
                })
                .collect::<Vec<i32>>(),
        );
    } else {
        result.push(rest.parse::<i32>().expect("Should be parsable into i32!"))
    }
    let result = result
        .into_iter()
        .reduce(|acc, x| acc ^ x)
        .expect("Should be reducable!");
    result.pow(3).to_string()
}

async fn respond_internal_server_error() -> (StatusCode, &'static str) {
    (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
}

#[derive(Deserialize)]
struct Reindeer {
    name: String,
    strength: u32,
}

async fn reindeer_cheer(Json(reindeers): Json<Vec<Reindeer>>) -> impl IntoResponse {
    reindeers
        .into_iter()
        .map(|deer| deer.strength)
        .sum::<u32>()
        .to_string()
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/1/:num1", get(pow_three))
        .route("/1/:num1/:num2", get(xor_pow_three))
        .route("/1/:num1/:num2/*rest", get(sled_id))
        .route("/4/strength", post(reindeer_cheer))
        .route("/-1/error", get(respond_internal_server_error));

    Ok(router.into())
}
