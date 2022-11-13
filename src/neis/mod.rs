use anyhow::Result;
use chrono::NaiveDate;

use self::{
    common::Response,
    meal::{MealData, MealRawRequest, MealType},
    time_table::{TimeTableData, TimeTableRawRequest},
};

mod common;
pub(crate) mod meal;
pub(crate) mod time_table;

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
    ) -> Result<Response<MealData>> {
        let res = MealRawRequest::new(&self.key, region_code, school_code, meal_type, date)
            .send()
            .await?;
        Ok(res)
    }
    pub(crate) async fn time_table(
        &self,
        region_code: &str,
        school_code: &str,
        grade: u8,
        date: Option<NaiveDate>,
    ) -> Result<Response<TimeTableData>> {
        let res = TimeTableRawRequest::new(&self.key, region_code, school_code, grade, date)
            .send()
            .await?;
        Ok(res)
    }
}
