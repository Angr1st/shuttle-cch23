use axum::{
    body::Body,
    extract::Path,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use base64::{engine::general_purpose, Engine};
use serde::{Deserialize, Serialize};
use tracing::info;

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

async fn ok_response() -> impl IntoResponse {
    StatusCode::OK
}

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

#[derive(Deserialize, Debug)]
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
    info!("Started cursed candy eating contest!");
    for reindeer in reindeers.iter() {
        info!("{:?}", reindeer)
    }
    let fastest = reindeers
        .iter()
        .map(|deer| {
            (deer, unsafe {
                (deer.speed * 10 as f64).to_int_unchecked::<u32>()
            })
        })
        .max_by_key(|deer| deer.1)
        .ok_or(AppError::new())?;
    info!("Fastest: {}", fastest.0.name);
    let tallest = reindeers
        .iter()
        .max_by_key(|deer| deer.height)
        .ok_or(AppError::new())?;
    info!("Tallest: {}", tallest.name);
    let magician = reindeers
        .iter()
        .max_by_key(|deer| deer.snow_magic_power)
        .ok_or(AppError::new())?;
    info!("Magician: {}", magician.name);
    let consumer = reindeers
        .iter()
        .max_by_key(|deer| deer.candies_eaten_yesterday)
        .ok_or(AppError::new())?;
    info!("Consumer: {}", consumer.name);
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
        consumer: format!(
            "{} ate lots of candies, but also some {}",
            consumer.name, consumer.favorite_food
        ),
    }))
}

#[derive(Serialize)]
struct ElfCounter {
    elf: usize,
    #[serde(rename = "elf on a shelf", skip_serializing_if = "Option::is_none")]
    elf_on_a_shelf: Option<usize>,
    #[serde(
        rename = "shelf with no elf on it",
        skip_serializing_if = "Option::is_none"
    )]
    shelf_with_no_elf: Option<usize>,
}

async fn elf_counter(body: String) -> Json<ElfCounter> {
    info!("{}", &body);
    let elf = body.matches("elf").count();
    let elf_on_a_shelf = body.matches("elf on a shelf").count();
    let elf_on_a_shelf = if elf_on_a_shelf == 0 {
        None
    } else {
        Some(elf_on_a_shelf)
    };
    let shelf_with_no_elf_matches: Vec<(usize, &str)> = body.match_indices("shelf").collect();
    let mut shelf_with_no_elf = 0;
    for shelf in shelf_with_no_elf_matches {
        //let space_before = shelf.0.checked_sub(10);
        //if let Some(index_space_before) = space_before {
        //    if &body[index_space_before..shelf.0] == " elf on a " {
        //        continue;
        //    }
        //}
        let Some(index_before) = shelf.0.checked_sub(10) else {
            continue;
        };
        info!("{}", &body[index_before..shelf.0]);
        if &body[index_before..shelf.0] == " elf on a " {
            continue;
        } else {
            shelf_with_no_elf = shelf_with_no_elf + 1;
        }
    }
    let shelf_with_no_elf = if elf_on_a_shelf.is_none() && shelf_with_no_elf == 0 {
        None
    } else {
        Some(shelf_with_no_elf)
    };
    Json(ElfCounter {
        elf,
        elf_on_a_shelf,
        shelf_with_no_elf,
    })
}

async fn get_cookie_recipie(headers: HeaderMap) -> impl IntoResponse {
    let header = headers.get("Cookie");
    if header.is_some() {
        let header = header.unwrap();
        let decoded = general_purpose::STANDARD
            .decode(header.to_str().unwrap().split_at(6).0)
            .unwrap();
        let decoded_string = String::from_utf8(decoded);
        return decoded_string.unwrap().into_response();
    }
    return "error".into_response();
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(ok_response))
        .route("/1/:num1", get(pow_three))
        .route("/1/:num1/:num2", get(xor_pow_three))
        .route("/1/:num1/:num2/*rest", get(sled_id))
        .route("/4/strength", post(reindeer_cheer))
        .route("/4/contest", post(cursed_candy_eating_contest))
        .route("/6", post(elf_counter))
        .route("/7/decode", get(get_cookie_recipie))
        .route("/-1/error", get(respond_internal_server_error));

    Ok(router.into())
}
