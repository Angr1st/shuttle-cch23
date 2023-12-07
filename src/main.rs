use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

struct AppError {}

impl AppError {
    fn new() -> Self {
        AppError {}
    }
}

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
//impl<E> From<E> for AppError {
//    fn from(_: E) -> Self {
//        AppError {}
//    }
//}

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

#[derive(Deserialize)]
struct ReindeerDetails {
    name: String,
    strength: u32,
    speed: f64,
    height: u32,
    antler_width: u32,
    snow_magic_power: u32,
    favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies_eaten_yesterday: u32,
}

#[derive(Serialize)]
struct ReindeerChallengeResponse {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

async fn cursed_candy_eating_contest(
    Json(reindeers): Json<Vec<ReindeerDetails>>,
) -> Result<Json<ReindeerChallengeResponse>, AppError> {
    let fastest = reindeers
        .iter()
        .map(|deer| {
            (deer, unsafe {
                (deer.speed * 10 as f64).to_int_unchecked::<u32>()
            })
        })
        .max_by_key(|deer| deer.1)
        .ok_or(AppError::new())?;
    let tallest = reindeers
        .iter()
        .max_by_key(|deer| deer.height)
        .ok_or(AppError::new())?;
    let magician = reindeers
        .iter()
        .max_by_key(|deer| deer.snow_magic_power)
        .ok_or(AppError::new())?;
    let consumer = reindeers
        .iter()
        .max_by_key(|deer| deer.candies_eaten_yesterday)
        .ok_or(AppError::new())?;
    Ok(Json::from(ReindeerChallengeResponse {
        fastest: format!(
            "Speeding past the finish line with a strength of {} is {}",
            fastest.0.strength, fastest.0.name
        ),
        tallest: format!(
            "{} is standing tall with his {} cm wide antlers",
            tallest.name, tallest.antler_width
        ),
        magician: format!(
            "{} could blast you away with a snow magic power of {}",
            magician.name, magician.snow_magic_power
        ),
        consumer: format!("{} ate lots of candies, but also some grass", consumer.name),
    }))
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/1/:num1", get(pow_three))
        .route("/1/:num1/:num2", get(xor_pow_three))
        .route("/1/:num1/:num2/*rest", get(sled_id))
        .route("/4/strength", post(reindeer_cheer))
        .route("/4/contest", post(cursed_candy_eating_contest))
        .route("/-1/error", get(respond_internal_server_error));

    Ok(router.into())
}
