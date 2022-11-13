use std::env;

use axum::{
    extract::Query,
    response::{IntoResponse, Response},
    Json,
};
use chrono::NaiveDate;
use reqwest::StatusCode;

use crate::{
    neis::{time_table::TimeTableData, NeisClient},
    subjects::Subjects,
    AppResponse,
};

pub(crate) async fn time_table(Query(params): Query<TimeTableQuery>) -> Response {
    let neis_client = NeisClient::new(&env::var("API_KEY").expect("API_KEY env missing"));

    let date = match params.date {
        Some(d) => match NaiveDate::parse_from_str(&d, "%Y%m%d") {
            Ok(d) => Some(d),
            Err(err) => {
                return (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(AppResponse::<Vec<TimeTableData>>::error_with_message(
                        &err.to_string(),
                    )),
                )
                    .into_response()
            }
        },
        None => None,
    };

    let res = neis_client
        .time_table(&params.region_code, &params.school_code, params.grade, date)
        .await;

    match res {
        Ok(data) => {
            let res = match data.result() {
                Some(time_table) => {
                    let subjects = Subjects::from_time_table(time_table);
                    AppResponse::success(subjects)
                }
                None => AppResponse::error(data.status().to_app_response_error()),
            };
            Json(res).into_response()
        }
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(AppResponse::<Vec<TimeTableData>>::error_with_message(
                &err.to_string(),
            )),
        )
            .into_response(),
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct TimeTableQuery {
    region_code: String,
    school_code: String,
    grade: u8,
    date: Option<String>,
}
