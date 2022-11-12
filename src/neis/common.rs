use anyhow::{anyhow, Result};

use crate::AppResponseError;

#[derive(serde::Deserialize, Debug)]
pub(super) enum RawResponse<T> {
    #[serde(rename = "head")]
    Head(Vec<HeadElement>),
    #[serde(rename = "row")]
    Row(Vec<T>),
}

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub(super) enum HeadElement {
    Count(Count),
    Status(Status),
}

#[derive(serde::Deserialize, Debug)]
pub(super) struct Count {
    list_total_count: u64,
}

#[derive(serde::Deserialize, Debug)]
pub(super) struct Status {
    #[serde(rename = "RESULT")]
    result: StatusResult,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub(crate) struct StatusResult {
    #[serde(rename = "CODE")]
    code: String,
    #[serde(rename = "MESSAGE")]
    message: String,
}
impl StatusResult {
    pub(crate) fn to_app_response_error(&self) -> AppResponseError {
        AppResponseError {
            code: self.code.parse().unwrap(),
            message: self.message.to_string(),
        }
    }
}

pub(crate) struct Response<T> {
    status: StatusResult,
    result: Option<Vec<T>>,
}

impl<T: Clone> Response<T> {
    pub(super) fn from_raw(raw: &[RawResponse<T>]) -> Result<Self> {
        if let (RawResponse::Head(head), RawResponse::<T>::Row(row)) = (&raw[0], &raw[1]) {
            if let HeadElement::Status(status) = &head[1] {
                Ok(Response {
                    status: status.result.clone(),
                    result: Some(row.to_vec()),
                })
            } else {
                Err(anyhow!("wrong response from original api"))
            }
        } else {
            Err(anyhow!("wrong response from original api"))
        }
    }

    pub(crate) fn status(&self) -> &StatusResult {
        &self.status
    }

    pub(crate) fn result(&self) -> Option<&Vec<T>> {
        self.result.as_ref()
    }
}
