use anyhow::Result;

use super::common::{RawResponse, Response};

#[derive(serde::Serialize)]
pub(super) struct MealRawRequest {
    #[serde(rename = "KEY")]
    key: String,
    #[serde(rename = "Type")]
    res_type: String,
    #[serde(rename = "ATPT_OFCDC_SC_CODE")]
    region_code: String, // 시도교육청코드
    #[serde(rename = "SD_SCHUL_CODE")]
    school_code: String, // 표준학교코드
    #[serde(rename = "MMEAL_SC_CODE")]
    meal_type_code: Option<String>, // 식사코드
    #[serde(rename = "MLSV_YMD")]
    date: String, // 급식일자
}
impl MealRawRequest {
    pub(super) fn new(
        key: &str,
        region_code: &str,
        school_code: &str,
        meal_type: &MealType,
        date: &str,
    ) -> Self {
        MealRawRequest {
            key: key.to_string(),
            res_type: "json".to_string(),
            region_code: region_code.to_string(),
            school_code: school_code.to_string(),
            meal_type_code: meal_type.to_code(),
            date: date.to_string(),
        }
    }

    pub(super) async fn send(&self) -> Result<Response<MealData>> {
        let client = reqwest::Client::new();
        let res = client
            .get("https://open.neis.go.kr/hub/mealServiceDietInfo")
            .query(self)
            .send()
            .await?;
        let res_data = res.json::<MealRawResponse>().await?.to_response()?;
        Ok(res_data)
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) enum MealType {
    Lunch,
    Dinner,
    All,
    None,
}

impl MealType {
    fn to_code(&self) -> Option<String> {
        match self {
            MealType::All | MealType::None => None,
            MealType::Lunch => Some("2".to_string()),
            MealType::Dinner => Some("3".to_string()),
        }
    }

    fn from_string(str: &str) -> MealType {
        match str {
            "2" => MealType::Lunch,
            "3" => MealType::Dinner,
            _ => MealType::None,
        }
    }
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct MealRawResponse {
    #[serde(rename = "mealServiceDietInfo")]
    meal: Vec<RawResponse<MealData>>,
}
impl MealRawResponse {
    fn to_response(&self) -> Result<Response<MealData>> {
        Response::from_raw(&self.meal)
    }
}

#[derive(serde::Deserialize, Debug, Clone)]
pub(crate) struct MealData {
    #[serde(rename = "MMEAL_SC_CODE")]
    meal_type_code: String, // 식사코드
    #[serde(rename = "MLSV_YMD")]
    date: String, // 급식일자
    #[serde(rename = "DDISH_NM")]
    dish_names: String, // 요리명
}

impl MealData {
    pub(crate) fn meal_type(&self) -> MealType {
        MealType::from_string(&self.meal_type_code)
    }

    pub(crate) fn date(&self) -> &str {
        self.date.as_ref()
    }

    pub(crate) fn dish_names(&self) -> Vec<String> {
        self.dish_names.split("<br/>").map(String::from).collect()
    }
}
