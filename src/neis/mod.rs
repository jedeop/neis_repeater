use anyhow::Result;

use self::{
    common::Response,
    meal::{MealData, MealRawRequest, MealType},
};

mod common;
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
    ) -> Result<Response<MealData>> {
        let res = MealRawRequest::new(&self.key, region_code, school_code, meal_type, date)
            .send()
            .await?;
        Ok(res)
    }
}
