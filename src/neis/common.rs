use anyhow::{anyhow, Result};

use crate::AppResponseError;

#[derive(serde::Deserialize, Debug)]
pub(super) enum RawResponseContent<T> {
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
#[allow(dead_code)]
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
    pub(super) fn from_raw(raw: &[RawResponseContent<T>]) -> Result<Self> {
        if let (RawResponseContent::Head(head), RawResponseContent::<T>::Row(row)) =
            (&raw[0], &raw[1])
        {
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

#[derive(serde::Deserialize)]
pub(super) enum RawResponseType<T> {
    Error(Status),
    Success(T),
}

// impl<T> RawResponseType<T> {
//     fn to_response(&self) -> Response<T> {
//         match self {
//             RawResponseType::Error(err) => Response { status: err, result: None },
//             RawResponseType::Success(res) => ,
//         }
//     }
// }
