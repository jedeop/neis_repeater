use std::env;

use axum::{
    extract::Query,
    response::{IntoResponse, Response},
    Json,
};
use reqwest::StatusCode;

use crate::neis::{
    meal::{MealData, MealType},
    NeisClient,
};

pub(crate) async fn meal(Query(params): Query<MealQuery>) -> Response {
    let neis_client = NeisClient::new(&env::var("API_KEY").expect("API_KEY env missing"));
    let res = neis_client
        .meal(
            &params.region_code,
            &params.school_code,
            &params.meal_type.unwrap_or(MealType::All),
            &params.date,
        )
        .await;

    match res {
        Ok(data) => Json(Meal::from_meal_data(&data)).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", err),
        )
            .into_response(),
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct MealQuery {
    region_code: String,
    school_code: String,
    date: String,
    meal_type: Option<MealType>,
}

#[derive(serde::Serialize)]
struct Meal {
    meal_type: MealType,
    date: String,
    dish_names: Vec<String>,
}

impl Meal {
    fn from_meal_data(meal: &[MealData]) -> Vec<Meal> {
        meal.iter()
            .map(|meal| Meal {
                meal_type: meal.meal_type(),
                date: meal.date().to_string(),
                dish_names: meal.dish_names(),
            })
            .collect()
    }
}
