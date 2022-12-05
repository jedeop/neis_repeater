use anyhow::Result;
use chrono::{Datelike, Duration, NaiveDate, Utc, Weekday};

use super::common::{RawResponseContent, Response};

#[derive(serde::Serialize)]
pub(super) struct TimeTableRawRequest {
    #[serde(rename = "KEY")]
    key: String,
    #[serde(rename = "Type")]
    res_type: String,
    #[serde(rename = "pSize")]
    p_size: u32,
    #[serde(rename = "ATPT_OFCDC_SC_CODE")]
    region_code: String, // 시도교육청코드
    #[serde(rename = "SD_SCHUL_CODE")]
    school_code: String, // 표준학교코드
    #[serde(rename = "AY")]
    year: String,
    #[serde(rename = "GRADE")]
    grade: Option<String>,
    #[serde(rename = "TI_FROM_YMD")]
    date_start: String,
    #[serde(rename = "TI_TO_YMD")]
    date_end: String,
}
impl TimeTableRawRequest {
    pub(super) fn new(
        key: &str,
        region_code: &str,
        school_code: &str,
        grade: Option<u8>,
        date: Option<NaiveDate>,
    ) -> Self {
        let date = match date {
            Some(d) => d,
            None => Utc::now().date_naive(),
        };
        let week = date.week(Weekday::Mon);
        let start_date = week.first_day();
        let end_date = week.last_day() - Duration::days(2);

        // println!("{}, {}", start_date, end_date);

        TimeTableRawRequest {
            key: key.to_string(),
            res_type: "json".to_string(),
            p_size: 1000,
            region_code: region_code.to_string(),
            school_code: school_code.to_string(),
            year: date.year().to_string(),
            grade: grade.and_then(|g| Some(g.to_string())),
            date_start: start_date.format("%Y%m%d").to_string(),
            date_end: end_date.format("%Y%m%d").to_string(),
        }
    }

    pub(super) async fn send(&self) -> Result<Response<TimeTableData>> {
        let client = reqwest::Client::new();
        let res = client
            .get("https://open.neis.go.kr/hub/hisTimetable")
            .query(self)
            .send()
            .await?;

        let res_data = res.json::<TimeTableRawResponse>().await?;
        let res_data = Response::from_raw(&res_data.time_table)?;

        Ok(res_data)
    }
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct TimeTableRawResponse {
    #[serde(rename = "hisTimetable")]
    time_table: Vec<RawResponseContent<TimeTableData>>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub(crate) struct TimeTableData {
    #[serde(rename(deserialize = "ALL_TI_YMD"))]
    pub(crate) date: String,
    #[serde(rename(deserialize = "GRADE"))]
    pub(crate) grade: String,
    #[serde(rename(deserialize = "CLRM_NM"))]
    pub(crate) classroom: String,
    #[serde(rename(deserialize = "CLASS_NM"))]
    pub(crate) class: Option<String>,
    #[serde(rename(deserialize = "PERIO"))]
    pub(crate) perio: String,
    #[serde(rename(deserialize = "ITRT_CNTNT"))]
    pub(crate) subject: String,
}
