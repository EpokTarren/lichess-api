use crate::model::challenges::CreateChallenge;
use crate::model::{Body, Request};
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostQuery;

pub type PostRequest = Request<PostQuery, CreateChallenge>;

impl PostRequest {
    pub fn new(username: &str, challenge: CreateChallenge) -> Self {
        let path = format!("/api/challenge/{username}");
        Self::post(path, None, Body::Form(challenge), None)
    }
}
