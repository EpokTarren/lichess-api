use crate::model::{Body, Request};
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostQuery;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Reason {
    Generic,
    Later,
    TooFast,
    TooSlow,
    TimeControl,
    Rated,
    Casual,
    Standard,
    Variant,
    NoBot,
    OnlyBot,
}

#[derive(Clone, Debug, Serialize)]
pub struct DeclineReason {
    pub reason: Reason,
}

pub type PostRequest = Request<PostQuery, DeclineReason>;

impl PostRequest {
    pub fn new(challenge_id: String, reason: Reason) -> Self {
        let path = format!("/api/challenge/{challenge_id}/decline");
        Self::post(path, None, Body::Form(DeclineReason { reason }), None)
    }
}
