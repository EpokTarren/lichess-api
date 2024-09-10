use crate::model::Body;
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery {
    pub with_current_games: bool,
}

pub type PostRequest = crate::model::Request<PostQuery, Vec<String>>;

impl PostRequest {
    pub fn new(user_ids: Vec<String>, with_current_games: bool) -> Self {
        Self::post(
            "/api/stream/games-by-users",
            PostQuery { with_current_games },
            Body::PlainText(user_ids.join(",")),
            None,
        )
    }
}
