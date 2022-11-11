use anyhow::Result;

use self::meal::{MealData, MealRawRequest, MealType};

pub(crate) mod meal;

pub(crate) struct NeisClient {
    key: String,
}
impl NeisClient {
    pub(crate) fn new(key: &str) -> Self {
        NeisClient {
            key: key.to_string(),
        }
    }
    pub(crate) async fn meal(
        &self,
        region_code: &str,
        school_code: &str,
        meal_type: &MealType,
        date: &str,
    ) -> Result<Vec<MealData>> {
        let res = MealRawRequest::new(&self.key, region_code, school_code, meal_type, date)
            .send()
            .await?;
        Ok(res)
    }
}

#[derive(serde::Deserialize, Debug)]
pub(crate) enum Response<T> {
    #[serde(rename = "head")]
    Head(Vec<HeadElement>),
    #[serde(rename = "row")]
    Row(Vec<T>),
}

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub(crate) enum HeadElement {
    Count(Count),
    Status(Status),
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct Count {
    list_total_count: u64,
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct Status {
    #[serde(rename = "RESULT")]
    result: StatusResult,
}

#[derive(serde::Deserialize, Debug)]
struct StatusResult {
    #[serde(rename = "CODE")]
    code: String,
    #[serde(rename = "MESSAGE")]
    message: String,
}
