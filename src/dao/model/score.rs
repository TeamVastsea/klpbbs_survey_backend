use crate::dao::entity::score;
use sea_orm::ActiveValue::Set;
use sea_orm::NotSet;
use serde_json::Value;

impl score::ActiveModel {
    pub fn new(user: &str, content: Value, survey: i32) -> Self {
        Self {
            id: NotSet,
            survey: Set(survey),
            user: Set(user.to_string()),
            answer: Set(serde_json::to_string(&content).unwrap()),
            completed: NotSet,
            update_time: Set(chrono::Utc::now().naive_local()),
            judge: NotSet,
            judge_time: NotSet,
            scores: NotSet,
            user_scores: NotSet,
            full_scores: NotSet,
            passed: NotSet,
        }
    }
}